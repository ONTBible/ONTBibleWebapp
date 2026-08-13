#!/usr/bin/env python3

"""Engendre le QR code qui mène à la fiche App Store.

    ./scripts/qr-app.py            # pip install qrcode, si besoin

## Pourquoi un QR et pas seulement un lien

La page de l'application se lit surtout sur un **grand écran** — c'est là qu'on
tombe sur le site, depuis un moteur ou un lien partagé. Or l'app s'installe sur
un téléphone. Le badge App Store, cliqué depuis un ordinateur, ouvre iTunes ou
une page web : il ne pose rien sur l'appareil qui compte.

Le QR fait le pont. On sort son téléphone, on vise, on installe.

Sur téléphone il n'a aucun sens — on ne scanne pas l'écran qu'on tient — et la
page ne l'affiche donc qu'à partir de `sm`.

## Un SVG, pas un PNG

Un QR est fait de carrés à angles droits : c'est exactement ce qu'un vecteur
rend le mieux, et ce qu'une image matricielle rend le moins bien. Un PNG
mis à l'échelle brouille les bords, et un lecteur qui hésite est un lecteur
qui repart.

Le fichier fait quelques kilo-octets, contre une trentaine en PNG.

## La correction d'erreur est au minimum

`ERROR_CORRECT_L` — 7 %. Les niveaux supérieurs existent pour les QR imprimés
sur un carton qui se salit, se plie ou vieillit. Celui-ci est affiché sur une
dalle : il ne se dégrade pas. Monter le niveau ne ferait qu'ajouter des modules,
donc réduire la taille de chacun à surface égale — c'est-à-dire rendre la
lecture *plus* difficile, pas moins.
"""

import sys
from pathlib import Path

try:
    import qrcode
except ModuleNotFoundError:
    raise SystemExit("Il manque `qrcode` : pip install qrcode")

# La fiche, par son identifiant. Pas par un nom lisible : Apple les recompose
# quand le titre change, et l'identifiant, lui, ne bouge jamais.
CIBLE = "https://apps.apple.com/fr/app/id6801192372"
SORTIE = Path(__file__).resolve().parent.parent / "public/images/qr-app.svg"

# Le module en unités de la viewBox. Une marge de quatre modules est le « quiet
# zone » exigé par la norme : sans elle, un lecteur ne trouve pas les repères.
MARGE = 4


def main() -> None:
    qr = qrcode.QRCode(error_correction=qrcode.constants.ERROR_CORRECT_L, border=MARGE)
    qr.add_data(CIBLE)
    qr.make(fit=True)
    matrice = qr.get_matrix()
    cote = len(matrice)

    # Un rectangle par module noir. On ne fusionne pas les voisins en un seul
    # tracé : le gain de poids est marginal, et un fichier lisible se corrige.
    carres = [
        f'<rect x="{x}" y="{y}" width="1" height="1"/>'
        for y, ligne in enumerate(matrice)
        for x, plein in enumerate(ligne)
        if plein
    ]

    # `currentColor` sur les modules, et **aucun fond** : c'est la page qui pose
    # la pastille claire derrière. Un QR a besoin d'un fond clair pour être lu,
    # mais le décider ici l'imposerait à tous les emplacements futurs.
    SORTIE.write_text(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {cote} {cote}" '
        f'shape-rendering="crispEdges" role="img" '
        f'aria-label="Code QR vers La Bible ONT sur l\'App Store">'
        f'<g fill="currentColor">{"".join(carres)}</g></svg>\n'
    )
    print(f"  qr-app.svg  {cote}×{cote} modules  {SORTIE.stat().st_size // 1024} Ko  → {CIBLE}")


if __name__ == "__main__":
    sys.exit(main())
