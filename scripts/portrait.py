#!/usr/bin/env python3

"""Reprend le détourage du portrait, depuis l'original.

## Le défaut

Le détourage a été fait sur un fond **blanc**, et les pixels de bord en ont
gardé la couleur : leur moyenne est RGB(190, 180, 179) là où le sujet est à
(115, 95, 85). Sur un fond clair ça ne se voit pas — le halo se confond avec la
page. Sur une nuit d'aubergine, il dessine un liseré lumineux autour des
cheveux et du col, et le sujet a l'air découpé aux ciseaux.

## La correction

Un pixel de bord observé est un mélange du sujet et du fond, pondéré par
l'opacité :

    C = α·F + (1−α)·B          avec B = blanc

On connaît C, α et B : on retrouve donc F.

    F = (C − (1−α)·B) / α

C'est une **décontamination**, pas une retouche : on ne devine rien, on retire
ce qui a été ajouté. Le calcul devient instable quand α tend vers zéro — la
division amplifie le bruit — d'où le seuil en dessous duquel on préfère rendre
le pixel franchement transparent.

## Le rééchantillonnage

Réduire une image à alpha droit fait baver les pixels transparents dans le
sujet : leur couleur, qui ne devrait compter pour rien, entre dans la moyenne.
On prémultiplie donc avant de réduire, et on démultiplie après. C'est la moitié
du halo qu'on évite, et c'est gratuit.

    ./scripts/portrait.py
"""

import pathlib

import numpy as np
from PIL import Image

Image.MAX_IMAGE_PIXELS = None

ORIGINAL = pathlib.Path(
    "/Volumes/Workspace/Projectground/Doneground/CV/done/photo CV/export/pro.png"
)
CIBLE = pathlib.Path(__file__).resolve().parent.parent / "public" / "images"

# Le fond sur lequel le détourage a été fait.
FOND = np.float32([255.0, 255.0, 255.0])

# En dessous, la division amplifie le bruit plus qu'elle ne récupère de sujet :
# ces pixels ne portent presque rien et valent mieux transparents.
SEUIL = 0.10

# Un resserrement de l'opacité. La décontamination corrige la *couleur* du
# bord, pas son étendue : il reste une frange large de deux ou trois pixels,
# héritée du flou de l'objectif sur les mèches fines. L'exposant la rend plus
# franche sans manger les mèches elles-mêmes — au-delà de 1,3, les cheveux
# commencent à se couper net et le détourage se voit.
GAMMA = 1.2

# Les largeurs servies. `sizes` dans le composant demande 14 rem — 224 px, donc
# 448 en densité double. On monte à 640 pour couvrir une densité triple sans
# fournir un fichier que personne ne charge.
LARGEURS = [640, 1024]


def decontamine(image: Image.Image) -> Image.Image:
    """Retire le fond blanc de la couleur des pixels de bord."""
    source = np.asarray(image, dtype=np.float32)
    hauteur = source.shape[0]
    sortie = np.empty_like(source)

    # Par bandes : l'original fait 4910 × 6844, et le convertir d'un bloc en
    # flottants demanderait plus d'un gigaoctet.
    for haut in range(0, hauteur, 512):
        bas = min(haut + 512, hauteur)
        bande = source[haut:bas]
        rgb, alpha = bande[..., :3], bande[..., 3:4] / 255.0

        sujet = np.where(alpha > 0, (rgb - (1.0 - alpha) * FOND) / np.maximum(alpha, 1e-6), rgb)
        np.clip(sujet, 0.0, 255.0, out=sujet)

        opacite = np.where(alpha < SEUIL, 0.0, np.power(alpha, GAMMA))
        sortie[haut:bas, :, :3] = sujet
        sortie[haut:bas, :, 3:4] = opacite * 255.0

    return Image.fromarray(sortie.round().astype(np.uint8), "RGBA")


def reduire(image: Image.Image, largeur: int) -> Image.Image:
    """Réduit en prémultipliant, pour que le transparent ne bave pas."""
    a = np.asarray(image, dtype=np.float32)
    alpha = a[..., 3:4] / 255.0
    premultiplie = np.concatenate([a[..., :3] * alpha, a[..., 3:4]], axis=2)

    hauteur = round(image.height * largeur / image.width)
    reduit = np.asarray(
        Image.fromarray(premultiplie.round().astype(np.uint8), "RGBA").resize(
            (largeur, hauteur), Image.LANCZOS
        ),
        dtype=np.float32,
    )

    alpha = reduit[..., 3:4] / 255.0
    couleur = np.where(alpha > 0, reduit[..., :3] / np.maximum(alpha, 1e-6), 0.0)
    np.clip(couleur, 0.0, 255.0, out=couleur)
    return Image.fromarray(
        np.concatenate([couleur, reduit[..., 3:4]], axis=2).round().astype(np.uint8), "RGBA"
    )


def mesure(image: Image.Image, nom: str) -> None:
    a = np.asarray(image)
    alpha = a[..., 3].astype(int)
    bord = (alpha > 20) & (alpha < 235)
    if not bord.any():
        return
    contour = a[..., :3][bord].mean(axis=0)
    sujet = a[..., :3][alpha == 255].mean(axis=0)
    ecart = contour.mean() - sujet.mean()
    print(f"  {nom:22} contour {contour.round(0)}  sujet {sujet.round(0)}  écart {ecart:+.0f}")


def main() -> None:
    if not ORIGINAL.exists():
        raise SystemExit(f"original introuvable : {ORIGINAL}")

    print("→ lecture de l'original")
    original = Image.open(ORIGINAL).convert("RGBA")
    mesure(original, "avant")

    print("→ décontamination")
    propre = decontamine(original)
    mesure(propre, "après")

    for largeur in LARGEURS:
        reduit = reduire(propre, largeur)
        fichier = CIBLE / f"portrait-{largeur}.png"
        reduit.save(fichier, optimize=True)
        mesure(reduit, f"portrait-{largeur}")
        print(f"  → {fichier.name}  {reduit.size[0]}×{reduit.size[1]}  "
              f"{fichier.stat().st_size // 1024} Ko")


if __name__ == "__main__":
    main()
