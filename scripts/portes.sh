#!/usr/bin/env bash

# La planche de comparaison des trois dessins de seuil.
#
#     ./scripts/portes.sh
#
# ## Pourquoi une planche et non trois visites
#
# Une porte au défilement ne se juge pas sur une capture — c'est le piège du
# §7 bis, et il a déjà coûté une heure. Mais une porte **figée** se capture,
# et c'est la seule façon de mettre trois dessins côte à côte : regardés l'un
# après l'autre, on ne compare plus des dessins, on compare des souvenirs.
#
# `?ouverture=` fige la scène, coupe son animation et la pose **par-dessus la
# page** : le simulateur ne sait pas défiler, et QuickLook ne rend que le
# premier écran. Une porte laissée à sa place dans le flux n'entre donc dans
# aucun des deux — quatre aperçus sont sortis identiques avant qu'on s'en
# aperçoive.
#
# Le mouvement, lui, ne s'y voit pas. Pour lui :
#
#     xcrun simctl io booted recordVideo /tmp/porte.mov
#
# ## Ce script est temporaire
#
# Il disparaît avec les deux dessins écartés, en même temps que les paramètres
# d'URL qu'il emploie. Ce n'est pas un outil du site, c'est l'instrument d'un
# choix.

set -euo pipefail
cd "$(dirname "$0")/.."

DOSSIER="${DOSSIER:-/tmp/ont-portes}"
DESSINS=(montagne nus voile portail)
OUVERTURES=(0 0.35 0.7 1)

rm -rf "$DOSSIER"
mkdir -p "$DOSSIER"

for dessin in "${DESSINS[@]}"; do
  for o in "${OUVERTURES[@]}"; do
    sortie="$DOSSIER/$dessin-$o.png"
    printf '  %s à %s…\n' "$dessin" "$o"
    ATTENTE="${ATTENTE:-9}" ./scripts/sim.sh "/fr?porte=$dessin&ouverture=$o" "$sortie" >/dev/null
  done
done

# La planche : une ligne par dessin, une colonne par progression. Les vignettes
# sont réduites à la même largeur, sinon un écran de simulateur en occupe la
# moitié et l'on ne voit plus rien des autres.
python3 - "$DOSSIER" "${DESSINS[@]}" -- "${OUVERTURES[@]}" <<'PY'
import sys, pathlib
from PIL import Image, ImageDraw

dossier = pathlib.Path(sys.argv[1])
sep = sys.argv.index("--")
dessins = sys.argv[2:sep]
ouvertures = sys.argv[sep + 1:]

LARGEUR, MARGE, TITRE = 300, 16, 34

vignettes = {}
for d in dessins:
    for o in ouvertures:
        c = dossier / f"{d}-{o}.png"
        if not c.exists():
            raise SystemExit(f"capture manquante : {c}")
        im = Image.open(c).convert("RGB")
        vignettes[(d, o)] = im.resize(
            (LARGEUR, round(im.height * LARGEUR / im.width)), Image.LANCZOS
        )

hauteur = max(v.height for v in vignettes.values())
planche = Image.new(
    "RGB",
    (len(ouvertures) * (LARGEUR + MARGE) + MARGE,
     len(dessins) * (hauteur + TITRE + MARGE) + MARGE),
    "#18090d",  # la nuit du site : une planche sur fond blanc ment sur les valeurs
)
crayon = ImageDraw.Draw(planche)

for ligne, d in enumerate(dessins):
    y = MARGE + ligne * (hauteur + TITRE + MARGE)
    for colonne, o in enumerate(ouvertures):
        x = MARGE + colonne * (LARGEUR + MARGE)
        crayon.text((x, y + 8), f"{d} — ouverture {o}", fill="#cdbe83")
        planche.paste(vignettes[(d, o)], (x, y + TITRE))

chemin = dossier / "planche.png"
planche.save(chemin)
print(chemin)
PY
