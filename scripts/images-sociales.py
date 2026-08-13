#!/usr/bin/env python3

"""Compose l'image d'aperçu et l'icône d'écran d'accueil.

    ./scripts/images-sociales.py

## L'image d'aperçu — `apercu.png`, 1200 × 630

C'est ce qu'affiche une messagerie quand on y colle un lien. Le site servait
jusqu'ici la montagne seule, en 512 × 512 : un carré dans un cadre paysage, que
la plupart des messageries rognent ou entourent de blanc.

Elle est composée ici plutôt que dessinée à la main pour une raison : le fond,
l'or et la montagne viennent des mêmes jetons que le site. Une image d'aperçu
qui dérive de la marque est une image qui trahit le jour où la marque bouge.

## L'icône d'écran d'accueil — `touch-icon.png`, 180 × 180

iOS ne gère pas la transparence d'une icône d'accueil : il la remplit de noir.
La montagne dorée sur fond transparent y devient donc une tache dorée sur un
carré noir. On compose un fond d'aubergine opaque.
"""

import pathlib
import subprocess
import tempfile

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

RACINE = pathlib.Path(__file__).resolve().parent.parent
IMAGES = RACINE / "public" / "images"

# Les jetons, repris de `style/main.css`. S'ils y changent, ils changent ici —
# c'est la seule duplication, et elle est explicite.
NUIT = (24, 9, 13)
AUBERGINE = (66, 27, 38)
SURFACE_HAUTE = (53, 21, 30)
OR = (205, 190, 131)


def rasterise(svg: pathlib.Path, largeur: int, couleur: tuple[int, int, int]) -> Image.Image:
    """Rend un SVG dans la couleur demandée, sur fond transparent.

    Aucune bibliothèque Python du système ne lit le SVG ; QuickLook, lui,
    embarque WebKit. Mais il rend sur un **fond blanc opaque** : la transparence
    est perdue, et un détourage naïf récupère le carré entier.

    On la reconstruit. Le tracé n'a qu'une couleur — l'or de la marque — donc
    chaque pixel est un mélange de cet or et du blanc, et sa luminance donne
    exactement la couverture :

        α = (255 − L) / (255 − L_or)

    C'est la même arithmétique que la décontamination du portrait, à l'envers :
    là on retirait le fond d'une couleur connue, ici on déduit l'opacité d'un
    fond connu.
    """
    with tempfile.TemporaryDirectory() as dossier:
        subprocess.run(
            ["qlmanage", "-t", "-s", str(largeur * 2), "-o", dossier, str(svg)],
            capture_output=True,
            check=False,
        )
        rendus = list(pathlib.Path(dossier).glob("*.png"))
        if not rendus:
            raise SystemExit(f"QuickLook n'a pas rendu {svg.name}")
        rendu = np.asarray(Image.open(rendus[0]).convert("L"), dtype=np.float32)

    # L'or du fichier — celui que le script de normalisation y pose.
    or_svg = np.float32([205, 190, 131])
    luminance_or = float(0.299 * or_svg[0] + 0.587 * or_svg[1] + 0.114 * or_svg[2])
    alpha = np.clip((255.0 - rendu) / (255.0 - luminance_or), 0.0, 1.0)

    plein = np.zeros((*alpha.shape, 4), dtype=np.uint8)
    plein[..., :3] = couleur
    plein[..., 3] = (alpha * 255).round()
    im = Image.fromarray(plein, "RGBA")

    boite = im.getbbox()
    if boite:
        im = im.crop(boite)
    hauteur = round(im.height * largeur / im.width)
    return im.resize((largeur, hauteur), Image.LANCZOS)


def voute(taille: tuple[int, int]) -> Image.Image:
    """Le fond du site : la nuit, une lueur haute, l'aubergine qui remonte.

    Dessiné en dégradés radiaux comme en CSS, pour que l'image d'aperçu et la
    page se ressemblent au premier coup d'œil.
    """
    largeur, hauteur = taille
    fond = Image.new("RGB", taille, NUIT)

    # La lueur haute — un disque flou en or très dilué.
    lueur = Image.new("L", taille, 0)
    ImageDraw.Draw(lueur).ellipse(
        [-largeur * 0.2, -hauteur * 0.85, largeur * 1.2, hauteur * 0.55], fill=90
    )
    lueur = lueur.filter(ImageFilter.GaussianBlur(largeur // 8))
    fond = Image.composite(Image.new("RGB", taille, SURFACE_HAUTE), fond, lueur)

    halo = Image.new("L", taille, 0)
    ImageDraw.Draw(halo).ellipse(
        [largeur * 0.2, -hauteur * 0.5, largeur * 0.8, hauteur * 0.45], fill=60
    )
    halo = halo.filter(ImageFilter.GaussianBlur(largeur // 6))
    return Image.composite(Image.new("RGB", taille, AUBERGINE), fond, halo)


def apercu() -> None:
    largeur, hauteur = 1200, 630
    image = voute((largeur, hauteur))

    # Le massif en horizon, très pâle : le même geste que l'ouverture du site.
    massif = rasterise(IMAGES / "logomark.svg", int(largeur * 1.5), AUBERGINE)
    couche = Image.new("RGBA", (largeur, hauteur), (0, 0, 0, 0))
    couche.paste(massif, ((largeur - massif.width) // 2, hauteur - int(massif.height * 0.8)))
    image = Image.alpha_composite(image.convert("RGBA"), couche).convert("RGB")

    wordmark = rasterise(IMAGES / "wordmark.svg", 560, OR)
    image.paste(
        wordmark,
        ((largeur - wordmark.width) // 2, (hauteur - wordmark.height) // 2 - 20),
        wordmark,
    )

    fichier = IMAGES / "apercu.png"
    image.save(fichier, optimize=True)
    print(f"  {fichier.name:18} {image.width}×{image.height}  {fichier.stat().st_size // 1024} Ko")


def icone() -> None:
    cote = 180
    image = Image.new("RGB", (cote, cote), AUBERGINE)
    montagne = rasterise(IMAGES / "logomark.svg", int(cote * 0.72), OR)
    image.paste(
        montagne,
        ((cote - montagne.width) // 2, (cote - montagne.height) // 2),
        montagne,
    )
    fichier = IMAGES / "touch-icon.png"
    image.save(fichier, optimize=True)
    print(f"  {fichier.name:18} {cote}×{cote}  {fichier.stat().st_size // 1024} Ko")


if __name__ == "__main__":
    apercu()
    icone()
