// Les domaines du site.
//
// ## Ce que ce fichier fait, et en combien de temps
//
// Le DNS est chez **Cloudflare**, pas chez AWS : Terraform ne peut donc pas
// poser les enregistrements lui-même. Il les **produit**, et l'application se
// fait en deux temps :
//
//     terraform apply -target=aws_acm_certificate.site    # crée le certificat
//     terraform output enregistrements_a_coller           # les lignes à poser
//     …                                                   # on les pose
//     terraform apply                                     # valide et attache
//
// Le second passage attend la validation du certificat. AWS ne la donne que
// lorsqu'il a vu les enregistrements : tant qu'ils ne sont pas posés, il
// tourne, puis renonce au bout de vingt minutes sans rien casser.
//
// ## Pourquoi tout est conditionnel
//
// `domaines` vide, et ce fichier ne crée rien. Le site continue de répondre sur
// l'adresse de CloudFront. C'est ce qui permet de committer le code sans forcer
// personne à basculer, et de revenir en arrière en vidant une variable.

variable "domaines" {
  description = <<-TXT
    Les noms que sert le site. Le **premier** est le canonique : les autres y
    sont renvoyés en 301. Vide, rien n'est créé.
  TXT
  type        = list(string)
  default     = []
}

locals {
  actif      = length(var.domaines) > 0
  canonique  = local.actif ? var.domaines[0] : ""
  a_renvoyer = local.actif ? slice(var.domaines, 1, length(var.domaines)) : []
}

// ── Le certificat ─────────────────────────────────────────────────────────────
//
// **En Virginie du Nord**, et c'est la confusion la plus courante : CloudFront
// n'accepte ses certificats que de `us-east-1`, quelle que soit la région où
// vit le reste. Le domaine personnalisé d'API Gateway, lui, exige l'inverse —
// un certificat de sa propre région. Les deux règles sont opposées, et chacune
// est absolue.
resource "aws_acm_certificate" "site" {
  count    = local.actif ? 1 : 0
  provider = aws.virginie

  domain_name               = local.canonique
  subject_alternative_names = local.a_renvoyer
  validation_method         = "DNS"

  // Un certificat ne se remplace pas sous une distribution qui s'en sert : il
  // faut créer le nouveau, l'attacher, puis retirer l'ancien.
  lifecycle {
    create_before_destroy = true
  }
}

// On ne peut pas valider tout seul : les enregistrements sont chez Cloudflare,
// posés à la main. Cette ressource attend qu'ils apparaissent, et renonce au
// bout de vingt minutes plutôt que de bloquer indéfiniment.
resource "aws_acm_certificate_validation" "site" {
  count           = local.actif ? 1 : 0
  provider        = aws.virginie
  certificate_arn = aws_acm_certificate.site[0].arn

  timeouts {
    create = "20m"
  }
}

// ── Le renvoi vers le nom canonique ───────────────────────────────────────────
//
// `labibleont.com` et les `www.` renvoient vers `ontbible.com`, **chemin et
// paramètres préservés** : un lien partagé vers un passage doit arriver sur ce
// passage, pas sur l'accueil.
//
// Une fonction CloudFront et non une seconde distribution : elle s'exécute au
// point de présence, avant le cache, en quelques microsecondes. Deux millions
// d'appels par mois sont gratuits — le site n'en fera pas le centième.
//
// Le 301 est **permanent**, contrairement à celui de `/` vers `/fr` : un
// domaine secondaire ne redeviendra pas canonique, et un moteur de recherche
// doit reporter sur `ontbible.com` tout ce qu'il sait de `labibleont.com`.
resource "aws_cloudfront_function" "canonique" {
  count   = local.actif && length(local.a_renvoyer) > 0 ? 1 : 0
  name    = "${local.nom}-canonique"
  runtime = "cloudfront-js-2.0"
  publish = true

  code = <<-JS
    function handler(event) {
      var requete = event.request;
      var hote = requete.headers.host ? requete.headers.host.value : '';

      // Une liste explicite, et non « tout ce qui n'est pas le canonique ».
      // Le domaine que CloudFront attribue doit continuer de répondre : c'est
      // par lui qu'on vérifie le site avant de basculer le DNS, et une règle
      // par défaut le renverrait sur un domaine qui ne pointe pas encore ici.
      var aRenvoyer = ${jsonencode(local.a_renvoyer)};
      if (aRenvoyer.indexOf(hote) === -1) {
        return requete;
      }

      // Les paramètres sont recomposés un à un : CloudFront les donne
      // décomposés, et un `?v=1-3` perdu ferait arriver le lecteur en haut
      // d'un chapitre de quarante-six versets sans savoir ce qu'on lui
      // montrait.
      var parametres = [];
      for (var cle in requete.querystring) {
        var valeur = requete.querystring[cle];
        if (valeur.multiValue) {
          for (var i = 0; i < valeur.multiValue.length; i++) {
            parametres.push(cle + '=' + valeur.multiValue[i].value);
          }
        } else {
          parametres.push(valeur.value === '' ? cle : cle + '=' + valeur.value);
        }
      }
      var suite = parametres.length ? '?' + parametres.join('&') : '';

      return {
        statusCode: 301,
        statusDescription: 'Moved Permanently',
        headers: { location: { value: 'https://${local.canonique}' + requete.uri + suite } }
      };
    }
  JS
}

// ── Ce qu'il reste à coller chez Cloudflare ───────────────────────────────────

output "enregistrements_a_coller" {
  description = "Les enregistrements DNS à créer chez Cloudflare. Nuage GRIS obligatoire."
  value = !local.actif ? [] : concat(
    [
      for o in aws_acm_certificate.site[0].domain_validation_options : {
        role   = "validation du certificat"
        type   = o.resource_record_type
        nom    = o.resource_record_name
        valeur = o.resource_record_value
      }
    ],
    [
      for d in var.domaines : {
        role   = "le domaine vers le site"
        type   = "CNAME"
        nom    = d
        valeur = aws_cloudfront_distribution.site.domain_name
      }
    ]
  )
}
