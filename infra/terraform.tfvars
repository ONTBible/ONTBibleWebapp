# Les noms que sert le site. Le **premier** est le canonique : les autres y
# sont renvoyés en 301, chemin et paramètres préservés.
#
# Ici et pas dans une variable d'environnement : sans cette valeur, un
# `terraform apply` lancé depuis un autre terminal détacherait les domaines et
# le certificat sans le dire. Ce n'est pas un secret.
domaines = [
  "ontbible.com",
  "www.ontbible.com",
  "labibleont.com",
  "www.labibleont.com",
]
