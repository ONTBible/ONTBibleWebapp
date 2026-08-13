// Le rôle que GitHub emprunte pour déployer.
//
// ## Aucune clé nulle part
//
// La tentation est de coller la clé de `ont-app` dans un secret GitHub. C'est
// une clé **longue durée** : elle vaut tant qu'on ne la révoque pas, elle
// traîne dans les journaux d'un job mal écrit, et elle survit à la personne qui
// l'a posée.
//
// L'OIDC fait autrement. GitHub signe un jeton qui dit « je suis le workflow de
// telle branche de tel dépôt » ; AWS le vérifie contre la clé publique de
// GitHub et prête ce rôle **pour la durée du job**. Rien à stocker, rien à
// faire tourner, rien à révoquer.
//
// ## La condition sur `sub` est la serrure
//
// Sans elle, n'importe quel dépôt GitHub du monde pourrait emprunter ce rôle :
// le fournisseur ne dit que « ce jeton vient bien de GitHub Actions ». C'est la
// condition qui dit **lequel**.
//
// Elle est épinglée à `refs/heads/main`. Une branche de travail, une pull
// request, un fork : rien de tout ça ne peut déployer.

// Le fournisseur existe déjà sur ce compte — AWS n'en accepte qu'un par
// émetteur, et un autre projet l'a créé. On le référence.
data "aws_iam_openid_connect_provider" "github" {
  url = "https://token.actions.githubusercontent.com"
}

variable "depot" {
  description = "Le dépôt GitHub autorisé à déployer, sous la forme proprietaire/nom."
  type        = string
  default     = "ONTBible/ONTBibleWebapp"
}

resource "aws_iam_role" "ci" {
  name = "${local.nom}-github"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Federated = data.aws_iam_openid_connect_provider.github.arn }
      Action    = "sts:AssumeRoleWithWebIdentity"
      Condition = {
        StringEquals = {
          "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
          "token.actions.githubusercontent.com:sub" = "repo:${var.depot}:ref:refs/heads/main"
        }
      }
    }]
  })
}

// Le strict nécessaire pour **déployer**, et rien pour **changer
// l'infrastructure**.
//
// C'est délibéré : Terraform garde son état en local, donc un job de CI qui
// l'exécuterait travaillerait sans savoir ce qui existe déjà — il recréerait
// tout, ou détruirait ce qu'il ne connaît pas. La CI pose des fichiers et
// remplace du code ; la forme de l'infrastructure reste une décision prise à
// la main.
resource "aws_iam_role_policy" "ci" {
  name = "deployer"
  role = aws_iam_role.ci.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid      = "LesFichiers"
        Effect   = "Allow"
        Action   = ["s3:PutObject", "s3:DeleteObject", "s3:ListBucket", "s3:GetObject"]
        Resource = [aws_s3_bucket.site.arn, "${aws_s3_bucket.site.arn}/*"]
      },
      {
        Sid    = "LeCodeDeLaLambda"
        Effect = "Allow"
        // `UpdateFunctionCode` seulement. Pas `UpdateFunctionConfiguration` :
        // la mémoire, le délai et les variables d'environnement sont décrits
        // par Terraform, et un job qui pourrait les changer les ferait diverger
        // de ce que le dépôt déclare.
        Action   = ["lambda:UpdateFunctionCode", "lambda:GetFunction"]
        Resource = aws_lambda_function.site.arn
      },
      {
        Sid    = "LInvalidation"
        Effect = "Allow"
        // Pas `UpdateDistribution` : la CI vide le cache, elle ne redessine pas
        // le routage.
        Action   = ["cloudfront:CreateInvalidation", "cloudfront:GetInvalidation"]
        Resource = aws_cloudfront_distribution.site.arn
      }
    ]
  })
}

output "role_ci" {
  description = "Le rôle à donner au workflow GitHub."
  value       = aws_iam_role.ci.arn
}
