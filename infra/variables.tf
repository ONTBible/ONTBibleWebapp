variable "region" {
  description = "La région où vit la Lambda. Paris, comme l'API de l'app."
  type        = string
  default     = "eu-west-3"
}

variable "profil" {
  description = "Le profil AWS local. `ont` porte l'utilisateur ont-app."
  type        = string
  default     = "ont"
}

variable "paquet" {
  description = "Le zip du binaire Lambda, produit par scripts/deployer.sh."
  type        = string
  default     = "../target/lambda/ontbible/paquet.zip"
}

variable "courriel" {
  description = "Où part l'alerte de dépense."
  type        = string
  default     = "ybikouta@icloud.com"
}
