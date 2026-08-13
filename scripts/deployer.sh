#!/usr/bin/env bash

# Déploie le site.
#
#     ./scripts/deployer.sh
#
# ## Ce qu'il fait, et pourquoi en deux constructions
#
# Le front et le serveur ne se construisent **pas** de la même façon, et ce
# n'est pas un choix :
#
# * le front (WASM, CSS) sort de `cargo leptos`, qui pose l'empreinte dans le
#   nom des fichiers et écrit `target/release/hash.txt` ;
# * le serveur sort de `cargo lambda`, qui compile pour Linux ARM avec zig.
#
# Et il **faut** que ce soit zig. Le linker d'Apple échoue en mode release sur
# ce projet — `ld: Assertion failed: (name.size() <= maxLength)`, une limite sur
# la longueur des noms de symboles que les types génériques de Leptos font
# exploser. `cargo leptos build --release` s'arrête donc là ; on ne lui demande
# que le front (`--frontend-only`), et zig fait le reste.
#
# ## L'ordre compte
#
# Les fichiers partent sur S3 **avant** que la Lambda ne change. L'inverse
# ouvrirait une fenêtre — quelques secondes, mais réelle — où le nouveau HTML
# réclamerait un WASM que le seau n'a pas encore. Les noms portent leur
# empreinte, donc l'ancien et le nouveau cohabitent sans se marcher dessus :
# c'est ce qui rend cet ordre possible.

set -euo pipefail

cd "$(dirname "$0")/.."

RACINE="$PWD"
PROFIL="${AWS_PROFILE:-ont}"
PAQUET="$RACINE/target/lambda/ontbible/paquet.zip"

etape() { printf '\n\033[1m── %s\033[0m\n' "$1"; }

# ── 1. Le front ───────────────────────────────────────────────────────────────

etape "Le front (WASM, CSS, empreintes)"
cargo leptos build --release --frontend-only

if [ ! -f target/release/hash.txt ]; then
  echo "hash.txt manquant : les fichiers n'ont pas d'empreinte, on s'arrête." >&2
  exit 1
fi

# ── 2. Le serveur ─────────────────────────────────────────────────────────────

etape "Le serveur (Linux ARM64, via zig)"
cargo lambda build --release --arm64 --features ssr --no-default-features

# Le paquet porte le binaire **et** `hash.txt`. Sans ce second fichier, la
# Lambda ne connaît pas les empreintes et écrit `ontbible.js` dans le HTML —
# une adresse que le seau ne sert pas. La page se charge alors sans son WASM,
# et rien ne le signale.
etape "Le paquet"
rm -f "$PAQUET"
cp target/release/hash.txt target/lambda/ontbible/hash.txt
(cd target/lambda/ontbible && zip -q "$PAQUET" bootstrap hash.txt)
printf '  %s  %s Ko\n' "$(basename "$PAQUET")" "$(($(stat -f%z "$PAQUET") / 1024))"

# ── 3. Les fichiers ───────────────────────────────────────────────────────────

SEAU=$(terraform -chdir=infra output -raw seau 2>/dev/null || true)

if [ -n "$SEAU" ]; then
  etape "Les fichiers vers S3"

  # Deux passes, parce que les deux familles ne se cachent pas pareil.
  #
  # `/pkg` porte l'empreinte du contenu dans le nom : un an, sans revalidation.
  # `immutable` dit au navigateur de ne même pas demander si ça a changé — le
  # nom le lui garantit déjà.
  aws --profile "$PROFIL" s3 sync target/site/pkg "s3://$SEAU/pkg" \
    --delete --exclude "*.ts" \
    --cache-control "public, max-age=31536000, immutable" \
    --no-progress

  # Les images et les fontes gardent un **nom fixe** : `logomark.svg` reste
  # `logomark.svg` quand son dessin change. Une journée, puis on revalide —
  # sinon une refonte de la marque resterait invisible un an.
  for dossier in images fontes; do
    [ -d "target/site/$dossier" ] || continue
    aws --profile "$PROFIL" s3 sync "target/site/$dossier" "s3://$SEAU/$dossier" \
      --delete --cache-control "public, max-age=86400" --no-progress
  done

  [ -f target/site/robots.txt ] && aws --profile "$PROFIL" s3 cp target/site/robots.txt \
    "s3://$SEAU/robots.txt" --cache-control "public, max-age=86400" --no-progress

  # `aws s3` devine le type d'après l'extension, et il ne connaît pas `.wasm` :
  # il le pose en `binary/octet-stream`. Un navigateur refuse alors de le
  # compiler en flux — `WebAssembly.instantiateStreaming` exige
  # `application/wasm` — et l'hydratation ne démarre jamais.
  for w in target/site/pkg/*.wasm; do
    [ -e "$w" ] || continue
    aws --profile "$PROFIL" s3 cp "$w" "s3://$SEAU/pkg/$(basename "$w")" \
      --content-type "application/wasm" \
      --cache-control "public, max-age=31536000, immutable" --no-progress
  done

  # ── Le corpus pour l'app ────────────────────────────────────────────────────
  #
  # Publié à côté du site, sur le même seau et derrière le même CloudFront.
  # C'est ce qui permettra à une correction de verset d'atteindre les lecteurs
  # sans passer par un build, un envoi à Apple et une revue.
  #
  # Les fichiers portent leur empreinte : un an de cache, sans revalidation.
  # Le **manifeste** porte un nom fixe et cinq minutes — c'est le seul que l'app
  # interroge, et le seul qui puisse mentir.
  etape "Le corpus pour l'app"
  ./scripts/corpus-publie.py

  aws --profile "$PROFIL" s3 sync target/corpus "s3://$SEAU/corpus" \
    --delete --exclude "manifeste.json" \
    --cache-control "public, max-age=31536000, immutable" --no-progress

  # Le manifeste **en dernier**, et c'est tout le sujet : il nomme des fichiers
  # qui doivent déjà être là. Publié avant eux, il enverrait les apps chercher
  # des adresses qui n'existent pas encore.
  aws --profile "$PROFIL" s3 cp target/corpus/manifeste.json "s3://$SEAU/corpus/manifeste.json" \
    --cache-control "public, max-age=300" --no-progress
else
  etape "Les fichiers vers S3 — sauté (le seau n'existe pas encore)"
fi

# ── 4. L'infrastructure ───────────────────────────────────────────────────────

etape "Terraform"
terraform -chdir=infra apply -auto-approve -input=false

# Au tout premier passage, le seau vient d'être créé : les fichiers n'ont pas pu
# partir plus haut. On rejoue cette étape seule.
if [ -z "$SEAU" ]; then
  etape "Les fichiers (première fois, le seau existe maintenant)"
  exec "$0" "$@"
fi

# ── 5. Le cache ───────────────────────────────────────────────────────────────

# Seulement le HTML. Les fichiers figés n'ont **pas** à être invalidés : leur
# nom a changé, donc l'ancien n'est plus demandé. Invalider `/*` coûterait des
# invalidations pour rien — les mille premières par mois sont gratuites, les
# suivantes se paient.
etape "Invalidation du HTML"
DISTRIBUTION=$(terraform -chdir=infra output -raw distribution)
aws --profile "$PROFIL" cloudfront create-invalidation \
  --distribution-id "$DISTRIBUTION" --paths "/" "/fr" "/fr/*" "/sitemap.xml" \
  --query 'Invalidation.Id' --output text

printf '\n\033[1m%s\033[0m\n' "$(terraform -chdir=infra output -raw adresse)"

# Le déploiement a **écrasé** `target/site/pkg` avec les fichiers de
# production, et c'est le même dossier que sert le serveur local. Celui-ci
# demande alors un CSS de développement que le dossier ne contient plus : la
# page arrive nue, sans style et sans WASM.
#
# Rien n'est cassé — il suffit de reconstruire. Mais ça ne se devine pas : on
# croit avoir cassé le site alors qu'on vient seulement de le déployer.
printf '\n\033[2m%s\033[0m\n' \
  "Le dossier target/site porte maintenant la production. Pour retrouver le site local : cargo leptos watch"
