#!/usr/bin/env python3

"""Normalise les SVG exportés d'Affinity.

Un export Affinity n'est pas utilisable tel quel sur le web, pour trois
raisons — et ce sont trois défauts silencieux : le fichier s'affiche, mais mal.

1. **Pas de `viewBox`.** Sans elle, le SVG a une taille fixe en pixels et ne
   se met pas à l'échelle. C'est précisément ce qu'on attend d'un vecteur, et
   c'est ce qu'on perd.

2. **La couleur est écrite en dur.** L'or de l'export vaut `rgb(207,189,122)`
   là où la marque vaut `#CDBE83` — un décalage de profil colorimétrique. Plus
   gênant : une couleur fixe ne suit pas le thème sombre. On la remplace par
   `currentColor`, ce qui rend le fichier obéissant au jeton CSS.

   Et on pose l'or de la marque en **attribut de présentation** sur la racine.
   C'est le seul niveau qui donne le bon comportement partout : un fichier
   ouvert seul — une favicon, par exemple — s'affiche dans sa couleur au lieu
   d'un noir de repli, tandis que la moindre règle CSS l'emporte dessus, car
   un attribut de présentation a une spécificité nulle. Une valeur par défaut,
   pas une contrainte.

3. **Du bruit d'éditeur** — déclaration XML, DOCTYPE, espaces de noms `serif:`
   et `xlink` inutilisés, `xml:space`. Une centaine d'octets, et surtout du
   bruit qui masque le contenu quand on relit le fichier.

Le script est **idempotent** : on peut le relancer sur un fichier déjà
normalisé, ou après un nouvel export, sans réfléchir. C'est la condition pour
qu'il serve encore dans six mois.

    ./scripts/normaliser-svg.py            # tout public/images/*.svg
    ./scripts/normaliser-svg.py a.svg b.svg
"""

import pathlib
import re
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
IMAGES = RACINE / "public" / "images"

# Ce qu'Affinity ajoute et dont le web n'a rien à faire.
BRUIT = [
    re.compile(r"<\?xml[^>]*\?>"),
    re.compile(r"<!DOCTYPE[^>]*>"),
    re.compile(r'\s+xmlns:(?:xlink|serif)="[^"]*"'),
    re.compile(r'\s+xml:space="[^"]*"'),
    re.compile(r'\s+version="1\.1"'),
    re.compile(r'\s+serif:id="[^"]*"'),
]

TAILLE = re.compile(r'\s(width|height)="([\d.]+)(?:px)?"')
COULEUR = re.compile(r"fill:\s*(?:rgb\([^)]*\)|#[0-9a-fA-F]{3,8})")

# L'or de la marque, relevé au pixel sur le combination mark — et non celui
# qu'Affinity exporte, décalé par la conversion de profil.
OR = "#cdbe83"


def normalise(brut: str) -> str:
    svg = brut
    for motif in BRUIT:
        svg = motif.sub("", svg)

    # La `viewBox` est déduite de la taille d'export, avant de retirer
    # `width`/`height` : c'est le seul endroit où cette information existe.
    # Les transformations de l'export ramènent déjà le dessin à l'origine.
    if "viewBox" not in svg:
        dimensions = dict(TAILLE.findall(svg[: svg.index(">")]))
        largeur, hauteur = dimensions.get("width"), dimensions.get("height")
        if not (largeur and hauteur):
            raise SystemExit("ni viewBox ni width/height : rien pour déduire le cadre")
        svg = svg.replace("<svg", f'<svg viewBox="0 0 {largeur} {hauteur}"', 1)

    # Sans taille intrinsèque, c'est la CSS qui décide — donc le composant qui
    # affiche l'image, et non le fichier. `preserveAspectRatio` vaut `meet` par
    # défaut : le dessin ne se déforme pas pour autant.
    svg = TAILLE.sub("", svg, count=2)

    # La couleur passe au jeton. `currentColor` suit `color`, donc le thème.
    svg = COULEUR.sub("fill:currentColor", svg)

    # L'or par défaut, en attribut de présentation — écrasé par n'importe
    # quelle règle CSS, mais présent quand le fichier est ouvert seul.
    if 'color="' not in svg[: svg.index(">")]:
        svg = svg.replace("<svg", f'<svg color="{OR}"', 1)

    return svg.strip() + "\n"


def main() -> None:
    cibles = [pathlib.Path(a) for a in sys.argv[1:]] or sorted(IMAGES.glob("*.svg"))
    if not cibles:
        raise SystemExit(f"aucun SVG dans {IMAGES}")

    for chemin in cibles:
        avant = chemin.read_text()
        apres = normalise(avant)
        chemin.write_text(apres)
        etat = "inchangé" if avant == apres else f"{len(avant)} → {len(apres)} o"
        print(f"  {chemin.name:24} {etat}")


if __name__ == "__main__":
    main()
