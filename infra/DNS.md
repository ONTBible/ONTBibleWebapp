# Les enregistrements à créer chez Cloudflare

**Tous en nuage GRIS** (proxy désactivé). Le proxy orange présente le
certificat de Cloudflare : le nom ne correspond plus à celui d'AWS, et le TLS
casse — c'est déjà la règle pour le domaine actuel.

## 1. Validation des certificats — à poser maintenant

Ils prouvent à AWS que les domaines vous appartiennent. Ils sont **permanents** :
ACM les relit à chaque renouvellement automatique. Les supprimer plus tard fait
expirer le certificat.

| type | nom | valeur |
|---|---|---|
| CNAME | `_3e51755dc2832d14ddc030590ee9d8fd.labibleont.com` | `_9488b4e62edc7b4059e8602eced650ac.jkddzztszm.acm-validations.aws` |
| CNAME | `_3b74447866cb4d7f86bdf5234b3591e2.ontbible.com` | `_bedeab9bcc7ce7e1daa2c6d294320a9f.jkddzztszm.acm-validations.aws` |
| CNAME | `_fcb173a2224f3cabf768d15838ff3f73.www.labibleont.com` | `_e1a32f174e296168e14e407da1f7a2e2.jkddzztszm.acm-validations.aws` |
| CNAME | `_b3b69634a54e3eb8fea41dea6d5e5429.www.ontbible.com` | `_2bc28f913cd29690ad90784e1d455dc3.jkddzztszm.acm-validations.aws` |
| CNAME | `_8100c2082b0151c71784885e625804ad.api.ontbible.com` | `_6be34eba49fbdfde8846d12456d0ab35.jkddzztszm.acm-validations.aws` |

## 2. Les domaines eux-mêmes — à poser **après** la validation

`api.ontbible.com` peut être posé tout de suite : il est **nouveau**, il ne
remplace rien.

Les quatre autres **remplacent** l'enregistrement existant de `ontbible.com`,
qui pointe aujourd'hui la Lambda de l'API. C'est la bascule.

| type | nom | valeur | quand |
|---|---|---|---|
| CNAME | `api.ontbible.com` | *(donné par le second `terraform apply` du backend)* | tout de suite |
| CNAME | `ontbible.com` | `d1158mwsz5tj2z.cloudfront.net` | **la bascule** — remplace l'existant |
| CNAME | `www.ontbible.com` | `d1158mwsz5tj2z.cloudfront.net` | avec |
| CNAME | `labibleont.com` | `d1158mwsz5tj2z.cloudfront.net` | avec |
| CNAME | `www.labibleont.com` | `d1158mwsz5tj2z.cloudfront.net` | avec |
