output "adresse" {
  description = "L'adresse du site, tant qu'il n'a pas de domaine."
  value       = "https://${aws_cloudfront_distribution.site.domain_name}"
}

output "distribution" {
  description = "L'identifiant CloudFront, pour invalider le cache."
  value       = aws_cloudfront_distribution.site.id
}

output "seau" {
  description = "Le seau S3 des fichiers figés."
  value       = aws_s3_bucket.site.id
}
