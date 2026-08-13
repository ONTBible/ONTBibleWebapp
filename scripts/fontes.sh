#!/usr/bin/env bash

# Porte les fontes de l'app vers le site, en woff2.
#
# ## Pourquoi un script et non une copie à la main
#
# Les fontes ont **une** source : `ONTBibleApp/app/Resources/Fonts/`. C'est là
# qu'elles sont choisies, licenciées et versionnées. Les recopier à la main
# créerait une seconde vérité, et le jour où une coupe change dans l'app,
# personne ne penserait à la reporter ici.
#
# Le script est donc rejouable : on le relance, il écrase, et les deux projets
# restent d'accord.
#
# ## Licences
#
# Toutes les fontes portées ici sont sous OFL, donc redistribuables — et leur
# licence part avec elles, ce que l'OFL exige. **SBL Hebrew** (EULA
# propriétaire) et **Taamey Frank CLM** (GPL dont l'exception ne couvre que les
# documents composés, pas un binaire) ne doivent jamais entrer dans un livrable
# web : elles ne sont pas listées, et ce n'est pas un oubli.

set -euo pipefail

SOURCE="$(cd "$(dirname "$0")/../../ONTBibleApp/app/Resources/Fonts" && pwd)"
CIBLE="$(cd "$(dirname "$0")/.." && pwd)/public/fontes"

# Ce que le site emploie, et rien de plus. Une fonte qu'aucune règle CSS ne
# nomme est un fichier que le dépôt porte pour rien.
COUPES=(
  # La voix du site — la géométrique de l'édition imprimée et du logo.
  Jost-Regular Jost-Italic Jost-SemiBold
  # En comparaison, le temps que le corps du site soit tranché (§9). À retirer
  # d'ici — script, `_fontes.scss`, jetons — dès que la décision est prise :
  # une fonte que plus aucune règle ne nomme est un fichier porté pour rien.
  EBGaramond-Regular EBGaramond-Italic EBGaramond-SemiBold
  # La voix d'une citation de l'ONT — les fontes de l'app, à l'identique.
  Literata-Regular Literata-Italic Literata-SemiBold
  FrankRuhlLibre-Medium
  EzraSIL
)

LICENCES=(
  Jost-OFL.txt EBGaramond-OFL.txt Literata-OFL.txt
  FrankRuhlLibre-OFL.txt EzraSIL-Licenses.txt OFL-FAQ.txt
)

mkdir -p "$CIBLE"
rm -f "$CIBLE"/*.woff2 "$CIBLE"/*.txt

for coupe in "${COUPES[@]}"; do
  ttf="$SOURCE/$coupe.ttf"
  [ -f "$ttf" ] || { echo "fonte absente : $ttf" >&2; exit 1; }
  cp "$ttf" "$CIBLE/$coupe.ttf"
  woff2_compress "$CIBLE/$coupe.ttf" >/dev/null
  rm "$CIBLE/$coupe.ttf"
  printf '  %-24s %s\n' "$coupe" "$(du -h "$CIBLE/$coupe.woff2" | cut -f1)"
done

for licence in "${LICENCES[@]}"; do
  [ -f "$SOURCE/$licence" ] || { echo "licence absente : $licence" >&2; exit 1; }
  cp "$SOURCE/$licence" "$CIBLE/$licence"
done

echo "→ $(ls "$CIBLE"/*.woff2 | wc -l | tr -d ' ') fontes, $(du -sh "$CIBLE" | cut -f1) au total"
