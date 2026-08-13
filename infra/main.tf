// L'infrastructure du site — Lambda pour le HTML, CloudFront et S3 pour le reste.
//
// ## Pourquoi deux origines et pas une
//
// Le site sert deux choses de nature opposée. Le HTML est **calculé** : il
// dépend du jour (le verset), de l'adresse, du corpus. Le WASM, les fontes et
// les images sont **figés** : ils ne changent qu'au déploiement, et ils portent
// leur empreinte dans leur nom.
//
// Les faire passer tous les deux par la Lambda coûterait cher et lentement :
// 1,3 Mo de WASM à chaque première visite, encodés en base64 par le runtime,
// facturés à la durée d'exécution. Ils vivent donc sur S3, et CloudFront les
// sert depuis le cache d'un point de présence proche du lecteur.
//
// La Lambda ne rend alors que du HTML — quelques kilo-octets — et son démarrage
// à froid ne touche plus les fichiers.
//
// ## Ce que ce fichier ne fait pas
//
// Il ne touche **pas** au DNS. Le site est servi sur l'adresse que CloudFront
// attribue, et rien d'autre. La bascule de `ontbible.com` suit un ordre strict
// — créer `api.ontbible.com`, y basculer l'app, vérifier, et seulement ensuite
// donner la racine au site. Inversé, les liens universels déjà partagés
// tombent, et ça ne se voit pas tout de suite : iOS met le fichier
// d'association en cache.

terraform {
  required_version = ">= 1.5"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region  = var.region
  profile = var.profil
}

// CloudFront n'accepte ses certificats que depuis la Virginie du Nord, quelle
// que soit la région où vit le reste. Ce fournisseur n'existe que pour eux.
provider "aws" {
  alias   = "virginie"
  region  = "us-east-1"
  profile = var.profil
}

locals {
  nom = "ont-site"
  // Le nom d'un seau S3 est global à tout AWS : le compte le rend unique sans
  // qu'on ait à y coller un nombre au hasard.
  seau = "ont-site-${data.aws_caller_identity.moi.account_id}"
}

data "aws_caller_identity" "moi" {}

// ───────────────────────────── la Lambda ──────────────────────────────────────

resource "aws_iam_role" "lambda" {
  name = "${local.nom}-lambda"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
}

// Le strict nécessaire : écrire dans CloudWatch. Le site ne lit aucune base, ne
// touche à aucun seau — le corpus est **dans le binaire**. Un rôle plus large
// serait un droit que personne n'utilise et que personne ne surveille.
resource "aws_iam_role_policy_attachment" "journaux" {
  role       = aws_iam_role.lambda.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_cloudwatch_log_group" "lambda" {
  name = "/aws/lambda/${local.nom}"
  // Un journal sans expiration se garde pour toujours et se facture au
  // gigaoctet. Trente jours suffisent à diagnostiquer ce qui se diagnostique.
  retention_in_days = 30
}

resource "aws_lambda_function" "site" {
  function_name = local.nom
  role          = aws_iam_role.lambda.arn

  // `provided.al2023` : le binaire Rust **est** le runtime, il n'y a pas
  // d'interpréteur à charger. C'est ce qui rend le démarrage à froid supportable.
  runtime       = "provided.al2023"
  handler       = "bootstrap"
  architectures = ["arm64"]

  filename         = var.paquet
  source_code_hash = filebase64sha256(var.paquet)

  // 1024 Mo, et ce n'est pas pour la mémoire. Lambda alloue le processeur
  // **proportionnellement** à la mémoire déclarée : à 256 Mo le rendu d'une
  // page prendrait trois fois plus longtemps, pour un coût identique puisqu'on
  // paie la mémoire multipliée par la durée. Au-delà de 1024, le gain se tasse.
  memory_size = 1024

  // Dix secondes. Une page se rend en dizaines de millisecondes ; ce délai
  // n'existe que pour qu'un démarrage à froid pathologique ne soit pas coupé
  // au milieu.
  timeout = 10

  environment {
    variables = {
      LEPTOS_OUTPUT_NAME = "ontbible"
      // Les fichiers vivent sur S3, pas à côté du binaire. La Lambda ne les
      // sert jamais — CloudFront route `/pkg`, `/images` et `/fontes` vers le
      // seau avant qu'elle ne les voie.
      LEPTOS_SITE_ROOT    = "site"
      LEPTOS_SITE_PKG_DIR = "pkg"
      LEPTOS_ENV          = "PROD"
      // Sans ça, le HTML référencerait `ontbible.js` au lieu de
      // `ontbible.<empreinte>.js` — et un visiteur qui revient exécuterait
      // l'ancien WASM contre le nouveau HTML, en silence.
      LEPTOS_HASH_FILES = "true"
      LEPTOS_HASH_FILE  = "hash.txt"
      RUST_BACKTRACE    = "1"
    }
  }

  depends_on = [aws_cloudwatch_log_group.lambda]
}

// ───────────────────────────── la porte HTTP ─────────────────────────────────
//
// ## Pourquoi une passerelle et pas une adresse de fonction
//
// Une adresse de fonction (`Lambda function URL`) aurait été plus simple et
// gratuite. Elle a été écrite, appliquée, et **elle ne répond pas** : la
// fonction rend « Forbidden » à tout appelant, y compris en appel direct, avec
// `AuthType = NONE` et une permission `Principal: "*"` conforme — vérifiée dans
// la politique de ressource, adresse recréée de zéro, permission reposée.
//
// Le compte refuse donc les adresses de fonction, très probablement par une
// politique d'organisation que cet utilisateur n'a pas le droit de lire
// (`organizations:DescribeOrganization` est refusé).
//
// Le backend de l'app, lui, tourne sur ce même compte depuis des mois — par
// une passerelle HTTP. On prend le chemin qui est prouvé ici plutôt que celui
// qui devrait marcher.
//
// ## Ce que ça coûte
//
// 1 $ par million de requêtes. À trente mille visites par mois, douze
// centimes. La contrepartie, c'est que CloudFront reste la seule porte : la
// passerelle n'est pas devinable, et rien ne la publie.

resource "aws_apigatewayv2_api" "site" {
  name          = local.nom
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "site" {
  api_id = aws_apigatewayv2_api.site.id

  integration_type   = "AWS_PROXY"
  integration_uri    = aws_lambda_function.site.invoke_arn
  integration_method = "POST"
  // Format 2.0 : c'est celui que `lambda_http` attend, et celui qui porte le
  // chemin brut sans le réécrire.
  payload_format_version = "2.0"
}

// Une seule route, attrape-tout. Le routage est le travail d'axum : le
// dédoubler ici ferait deux tables de routes à tenir d'accord, et c'est
// toujours celle qu'on oublie qui répond 404.
resource "aws_apigatewayv2_route" "tout" {
  api_id    = aws_apigatewayv2_api.site.id
  route_key = "$default"
  target    = "integrations/${aws_apigatewayv2_integration.site.id}"
}

// Le stage `$default` sert à la racine du domaine de la passerelle, sans
// préfixe. Un stage nommé collerait son nom devant chaque chemin — et
// `/.well-known/apple-app-site-association` deviendrait
// `/prod/.well-known/…`, qu'Apple n'irait jamais chercher.
resource "aws_apigatewayv2_stage" "defaut" {
  api_id      = aws_apigatewayv2_api.site.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_lambda_permission" "passerelle" {
  statement_id  = "AutoriserPasserelle"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.site.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.site.execution_arn}/*/*"
}

// ───────────────────────────── le seau ────────────────────────────────────────

resource "aws_s3_bucket" "site" {
  bucket = local.seau
}

// Aucun accès public direct. Le seau n'est joignable que par CloudFront, qui
// signe ses requêtes. Un seau ouvert se retrouve indexé, aspiré, et facturé.
resource "aws_s3_bucket_public_access_block" "site" {
  bucket                  = aws_s3_bucket.site.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_policy" "site" {
  bucket = aws_s3_bucket.site.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "cloudfront.amazonaws.com" }
      Action    = "s3:GetObject"
      Resource  = "${aws_s3_bucket.site.arn}/*"
      Condition = {
        StringEquals = {
          "AWS:SourceArn" = aws_cloudfront_distribution.site.arn
        }
      }
    }]
  })
  depends_on = [aws_s3_bucket_public_access_block.site]
}

// ───────────────────────────── CloudFront ─────────────────────────────────────

resource "aws_cloudfront_origin_access_control" "seau" {
  name                              = "${local.nom}-seau"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

// Le HTML n'est pas caché par défaut, et c'est délibéré : la page d'accueil
// porte le **verset du jour**, qui change à minuit chez le lecteur. Un cache
// d'une heure servirait le verset de la veille à qui arrive à 00 h 05.
//
// `Origin-Cache-Control` reste honoré : le jour où une page voudra être cachée,
// elle le dira dans son en-tête et CloudFront suivra, sans qu'on touche ici.
// ── Les en-têtes de sécurité ────────────────────────────────────────────────
//
// Servis par CloudFront et non par la Lambda : ils doivent valoir pour **tout**
// ce qui sort du domaine — le HTML calculé comme les fichiers du seau. Posés
// dans le serveur, ils manqueraient sur `/images/` et `/pkg/`.
resource "aws_cloudfront_response_headers_policy" "securite" {
  name = "${local.nom}-securite"

  security_headers_config {
    // HSTS ferme la fenêtre du premier contact. `redirect-to-https` renvoie
    // bien un visiteur arrivé en clair — mais ce renvoi lui-même voyage en
    // clair, et c'est là qu'un intermédiaire se place. Une fois l'en-tête reçu,
    // le navigateur n'essaiera plus jamais le http, même si on le lui demande.
    //
    // Deux ans, avec les sous-domaines. Pas de `preload` : l'inscription à la
    // liste des navigateurs est **irréversible en pratique**, et elle
    // engagerait `api.ontbible.com` avec le reste.
    strict_transport_security {
      access_control_max_age_sec = 63072000
      include_subdomains         = true
      preload                    = false
      override                   = true
    }

    // Interdit au navigateur de deviner un type qu'on lui a donné. Sans lui,
    // un fichier servi avec le mauvais type peut être exécuté comme du script.
    content_type_options {
      override = true
    }

    // L'origine seule part vers un site tiers, jamais le chemin. Un lien
    // partagé porte le passage dans son adresse — `/fr/lire/bereshit/…?v=1-3`
    // dit ce que quelqu'un lisait, et ça ne regarde pas le site d'en face.
    referrer_policy {
      referrer_policy = "strict-origin-when-cross-origin"
      override        = true
    }

    // Personne ne met ce site dans un cadre. C'est la défense contre le
    // détournement de clic, et `frame-ancestors` la dit mieux que l'ancien
    // `X-Frame-Options` : lui accepte une liste d'origines.
    content_security_policy {
      content_security_policy = "frame-ancestors 'none'"
      override                = true
    }
  }

  custom_headers_config {
    // Aucune de ces interfaces n'est employée. Les refuser toutes vaut mieux
    // que de les laisser disponibles à un script qu'on n'aurait pas prévu.
    items {
      header   = "Permissions-Policy"
      value    = "camera=(), microphone=(), geolocation=(), payment=(), usb=(), interest-cohort=()"
      override = true
    }
  }
}

resource "aws_cloudfront_cache_policy" "html" {
  name        = "${local.nom}-html"
  default_ttl = 0
  min_ttl     = 0
  max_ttl     = 31536000

  parameters_in_cache_key_and_forwarded_to_origin {
    enable_accept_encoding_brotli = true
    enable_accept_encoding_gzip   = true
    cookies_config { cookie_behavior = "none" }
    headers_config { header_behavior = "none" }
    // La sélection de versets d'un lien partagé — `?v=1-3` — fait partie de la
    // page : elle change les métadonnées d'aperçu. Elle doit donc entrer dans
    // la clé de cache, sinon deux liens différents recevraient la même réponse.
    query_strings_config {
      query_string_behavior = "whitelist"
      query_strings { items = ["v"] }
    }
  }
}

// Les fichiers de `/pkg` portent leur empreinte dans leur nom : un contenu
// nouveau a un nom nouveau. Ils peuvent donc être cachés un an sans risque —
// c'est exactement ce que l'empreinte achète.
resource "aws_cloudfront_cache_policy" "figes" {
  name        = "${local.nom}-figes"
  default_ttl = 31536000
  min_ttl     = 31536000
  max_ttl     = 31536000

  parameters_in_cache_key_and_forwarded_to_origin {
    enable_accept_encoding_brotli = true
    enable_accept_encoding_gzip   = true
    cookies_config { cookie_behavior = "none" }
    headers_config { header_behavior = "none" }
    query_strings_config { query_string_behavior = "none" }
  }
}

// Les images et les fontes, elles, gardent un **nom fixe** : `logomark.svg`
// reste `logomark.svg` quand son dessin change. Les cacher un an les figerait
// pour un an chez tous ceux qui les ont vues — c'est le piège dont on vient de
// sortir sur le WASM, à l'échelle de la marque.
//
// `min_ttl = 0` laisse l'origine décider : le script de déploiement pose une
// journée sur ces fichiers, et CloudFront revalide ensuite.
resource "aws_cloudfront_cache_policy" "statiques" {
  name        = "${local.nom}-statiques"
  default_ttl = 86400
  min_ttl     = 0
  max_ttl     = 31536000

  parameters_in_cache_key_and_forwarded_to_origin {
    enable_accept_encoding_brotli = true
    enable_accept_encoding_gzip   = true
    cookies_config { cookie_behavior = "none" }
    headers_config { header_behavior = "none" }
    query_strings_config { query_string_behavior = "none" }
  }
}

// Le manifeste du corpus : court, mais pas nul.
//
// Nul obligerait chaque lancement d'app à réveiller S3. Cinq minutes suffisent
// — une correction met de toute façon plus longtemps à traverser le pipeline et
// la CI — et cent mille lancements dans la même fenêtre ne coûtent qu'une seule
// lecture d'objet.
resource "aws_cloudfront_cache_policy" "manifeste" {
  name        = "${local.nom}-manifeste"
  default_ttl = 300
  min_ttl     = 0
  max_ttl     = 300

  parameters_in_cache_key_and_forwarded_to_origin {
    enable_accept_encoding_brotli = true
    enable_accept_encoding_gzip   = true
    cookies_config { cookie_behavior = "none" }
    headers_config { header_behavior = "none" }
    query_strings_config { query_string_behavior = "none" }
  }
}

resource "aws_cloudfront_distribution" "site" {
  enabled             = true
  is_ipv6_enabled     = true
  comment             = "La Bible ONT — le site"
  price_class         = "PriceClass_100" // Europe et Amérique du Nord : le lectorat.
  default_root_object = ""

  origin {
    origin_id   = "lambda"
    domain_name = replace(aws_apigatewayv2_api.site.api_endpoint, "https://", "")

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  origin {
    origin_id                = "seau"
    domain_name              = aws_s3_bucket.site.bucket_regional_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.seau.id
  }

  // Tout ce qui n'est pas un fichier va à la Lambda. C'est le bon défaut :
  // `/sitemap.xml` et `/.well-known/apple-app-site-association` sont **calculés**,
  // et une règle qui les enverrait au seau les ferait disparaître en silence.
  default_cache_behavior {
    target_origin_id           = "lambda"
    viewer_protocol_policy     = "redirect-to-https"
    allowed_methods            = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods             = ["GET", "HEAD"]
    cache_policy_id            = aws_cloudfront_cache_policy.html.id
    response_headers_policy_id = aws_cloudfront_response_headers_policy.securite.id
    // Sans politique, CloudFront n'enverrait aucun en-tête à l'origine, et les
    // fonctions serveur de Leptos — qui répondent en POST — perdraient leur
    // type de contenu.
    // `AllViewerExceptHostHeader`, la politique gérée d'AWS. Le nom dit ce qui
    // compte : elle transmet tout **sauf** le `Host`.
    //
    // Et c'est ce qui bloquait. Une politique maison transmettait le `Host` du
    // visiteur — `d1158….cloudfront.net` — à une passerelle qui n'accepte que
    // le sien. Elle répondait 403, et le message ne disait rien.
    origin_request_policy_id = "b689b0a8-53d0-40ab-baf2-68738e2966ac"
    compress                 = true

    // Le renvoi vers le nom canonique, exécuté au point de présence avant
    // toute autre chose. Il n'existe que s'il y a des domaines à renvoyer.
    dynamic "function_association" {
      for_each = local.actif && length(local.a_renvoyer) > 0 ? [1] : []
      content {
        event_type   = "viewer-request"
        function_arn = aws_cloudfront_function.canonique[0].arn
      }
    }
  }

  // Le manifeste du corpus — le point d'entrée de l'app.
  //
  // **Avant** `/corpus/*`, et l'ordre est ce qui compte : CloudFront retient la
  // première règle qui correspond, et `/corpus/*` engloberait celle-ci.
  //
  // Cinq minutes de cache. C'est le seul fichier du corpus qui porte un nom
  // fixe, donc le seul qui puisse mentir — un cache d'un an y figerait
  // l'ensemble du corpus pour un an. Cinq minutes, c'est le délai entre une
  // correction publiée et le moment où un lecteur peut la recevoir.
  ordered_cache_behavior {
    path_pattern               = "/corpus/manifeste.json"
    target_origin_id           = "seau"
    viewer_protocol_policy     = "https-only"
    allowed_methods            = ["GET", "HEAD"]
    cached_methods             = ["GET", "HEAD"]
    cache_policy_id            = aws_cloudfront_cache_policy.manifeste.id
    response_headers_policy_id = aws_cloudfront_response_headers_policy.securite.id
    compress                   = true
  }

  // Le corpus lui-même. Chaque fichier porte l'empreinte de son contenu, donc
  // une adresse ne désigne jamais qu'une version : un an, sans revalidation.
  ordered_cache_behavior {
    path_pattern               = "/corpus/*"
    target_origin_id           = "seau"
    viewer_protocol_policy     = "https-only"
    allowed_methods            = ["GET", "HEAD"]
    cached_methods             = ["GET", "HEAD"]
    cache_policy_id            = aws_cloudfront_cache_policy.figes.id
    response_headers_policy_id = aws_cloudfront_response_headers_policy.securite.id
    compress                   = true
  }

  // Ce qui porte son empreinte : un an, sans revalidation.
  ordered_cache_behavior {
    path_pattern               = "/pkg/*"
    target_origin_id           = "seau"
    viewer_protocol_policy     = "redirect-to-https"
    allowed_methods            = ["GET", "HEAD"]
    cached_methods             = ["GET", "HEAD"]
    cache_policy_id            = aws_cloudfront_cache_policy.figes.id
    response_headers_policy_id = aws_cloudfront_response_headers_policy.securite.id
    compress                   = true
  }

  // Ce qui garde un nom fixe : une journée, puis on revalide.
  dynamic "ordered_cache_behavior" {
    for_each = ["/images/*", "/fontes/*", "/robots.txt"]
    content {
      path_pattern               = ordered_cache_behavior.value
      target_origin_id           = "seau"
      viewer_protocol_policy     = "redirect-to-https"
      allowed_methods            = ["GET", "HEAD"]
      cached_methods             = ["GET", "HEAD"]
      cache_policy_id            = aws_cloudfront_cache_policy.statiques.id
      response_headers_policy_id = aws_cloudfront_response_headers_policy.securite.id
      compress                   = true
    }
  }

  // Les noms que sert la distribution. Vides tant que `domaines` l'est : le
  // site répond alors sur l'adresse de CloudFront, et rien d'autre.
  aliases = var.domaines

  restrictions {
    geo_restriction { restriction_type = "none" }
  }

  // Tant qu'aucun domaine n'est déclaré, le certificat de CloudFront sur son
  // propre nom suffit. Dès qu'il y en a un, c'est le nôtre — et il faut
  // attendre sa **validation**, pas sa création : une distribution refuse un
  // certificat dont AWS n'a pas encore vérifié qu'on possède le domaine.
  viewer_certificate {
    cloudfront_default_certificate = local.actif ? false : true
    acm_certificate_arn            = local.actif ? aws_acm_certificate_validation.site[0].certificate_arn : null
    ssl_support_method             = local.actif ? "sni-only" : null
    // TLS 1.2 au minimum. Les versions antérieures sont cassées, et plus rien
    // ne les parle depuis des années.
    minimum_protocol_version = local.actif ? "TLSv1.2_2021" : null
  }
}

// ───────────────────────────── la garde ───────────────────────────────────────

// L'alerte de dépense.
//
// Elle est gratuite, et c'est la **seule** chose qui voie venir un abus. Le
// site tient dans le palier gratuit d'AWS à trente mille visites par mois ; ce
// qui coûterait, c'est quelqu'un qui tire le WASM en boucle — 1 To de sortie
// vaut environ 85 $. Rien d'autre ne le signale : il n'y a pas d'erreur, pas de
// panne, seulement une facture le mois suivant.
//
// Cinq dollars parce que le site normal en coûte zéro : le seuil n'a pas à
// laisser de marge, il a à se déclencher tôt.
resource "aws_budgets_budget" "garde" {
  name         = "${local.nom}-garde"
  budget_type  = "COST"
  limit_amount = "5"
  limit_unit   = "USD"
  time_unit    = "MONTHLY"

  notification {
    comparison_operator = "GREATER_THAN"
    threshold           = 100
    threshold_type      = "PERCENTAGE"
    // `FORECASTED` et non `ACTUAL` : prévenir quand la dépense *est* de cinq
    // dollars, c'est prévenir trop tard. AWS projette la fin du mois d'après
    // la tendance, donc l'alerte tombe au moment où la tendance s'emballe.
    notification_type          = "FORECASTED"
    subscriber_email_addresses = [var.courriel]
  }
}
