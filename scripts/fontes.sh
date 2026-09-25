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
#
# ## Les six familles de lecture, et pourquoi elles sont toutes là
#
# Depuis le 21 septembre 2026, le site offre le **choix de la fonte de
# lecture**, comme l'app — `ReadingFont`, sept entrées dont six embarquées.
# Georgia est la septième : le système la fournit des deux côtés, et personne
# n'en contrôle le fichier.
#
# Chacune vient en **trois coupes** — Regular, Italic, SemiBold — et les trois
# comptent. L'app le dit : *une famille amputée de son italique se résout quand
# même, en pente simulée, penchée à la main par le moteur de rendu.* Le web
# fait exactement pareil, et c'est pire chez nous : la translittération du
# niveau 3 est en italique, donc une famille incomplète abîme précisément la
# pièce que la liseuse existe pour montrer.
#
# EB Garamond n'est donc plus « en comparaison le temps que le corps soit
# tranché » : elle est une des six, à demeure.
COUPES=(
  # La voix du site — la géométrique de l'édition imprimée et du logo.
  Jost-Regular Jost-Italic Jost-SemiBold
  # Les six familles de lecture offertes au lecteur.
  Literata-Regular Literata-Italic Literata-SemiBold
  EBGaramond-Regular EBGaramond-Italic EBGaramond-SemiBold
  Spectral-Regular Spectral-Italic Spectral-SemiBold
  SourceSerif4-Regular SourceSerif4-Italic SourceSerif4-SemiBold
  Newsreader-Regular Newsreader-Italic Newsreader-SemiBold
  # Le titre hébreu de la marque.
  FrankRuhlLibre-Medium
  # L'hébreu du corpus — la seule qui positionne niqqud et te'amim.
  EzraSIL
)

LICENCES=(
  Jost-OFL.txt EBGaramond-OFL.txt Literata-OFL.txt
  Spectral-OFL.txt SourceSerif4-OFL.txt Newsreader-OFL.txt
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
