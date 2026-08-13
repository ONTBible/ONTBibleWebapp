#!/usr/bin/env python3

"""Détoure le portrait, depuis le fichier brut.

    ./scripts/portrait.py [chemin/vers/IMG_3655.DNG]

## Pourquoi repartir du brut

Les versions précédentes de ce script *réparaient* un détourage existant : la
seule image disponible était déjà découpée, sur fond blanc, et ses pixels de
bord en avaient gardé la couleur. On pouvait retirer cette contamination et
réestimer l'opacité, mais pas inventer les mèches que la découpe avait mangées
— l'information n'y était plus.

Avec le fichier brut, le fond est là. On ne répare plus, on détoure.

## Le fond n'est pas uniforme

Le mur passe de 202 de luminance à hauteur de tête à 142 à hauteur d'épaule :
un dégradé net, plus une inclinaison gauche-droite. Le soustraire comme une
constante laisserait un halo en haut et rognerait en bas.

Il est donc **modélisé** : pour chaque ligne, la médiane des pixels de la marge
gauche et celle de la marge droite, interpolées entre les deux. Le mur étant
visible sur toute la hauteur des deux côtés, le modèle est mesuré, pas deviné.

## De l'écart à l'opacité

L'écart au fond suffit pour les cheveux et la peau — ils sont cent points plus
sombres que le mur. Il ne suffit pas partout : la chemise blanche n'est plus
claire que le mur que de vingt à cinquante points, et certains plis l'égalent.

D'où un **trimap** plutôt qu'un seuil unique :

- au-delà de l'écart franc, on est dans le sujet ;
- les trous que cela laisse dans la chemise sont comblés — le sujet est d'un
  seul tenant, un pli n'est pas un trou ;
- sous l'écart du bruit du mur, on est dehors ;
- entre les deux, l'opacité varie continûment. C'est là que vivent les mèches,
  et c'est la seule zone où une valeur intermédiaire a un sens.

## La décontamination

Une fois l'opacité connue, la couleur du sujet se retrouve exactement :

    C = α·F + (1−α)·B    donc    F = (C − (1−α)·B) / α

B est le fond **modélisé**, pas une constante — c'est ce qui rend l'opération
juste en haut comme en bas de l'image.
"""

import pathlib
import subprocess
import sys

import numpy as np
from PIL import Image
from scipy import ndimage

Image.MAX_IMAGE_PIXELS = None

BRUT = pathlib.Path.home() / "Downloads" / "IMG_3655.DNG"
CIBLE = pathlib.Path(__file__).resolve().parent.parent / "public" / "images"
TRAVAIL = pathlib.Path("/tmp/ont-portrait")

# On travaille à demi-résolution : la sortie la plus large fait 1024 px, et
# 2674 × 3566 laisse une marge confortable pour le rééchantillonnage final.
REDUCTION = 2

# Les seuils d'écart au fond, en distance RGB. **Mesurés**, pas choisis :
#
#   résidu du modèle sur le mur seul   médiane 4, 99,9e centile 37, max 39
#   écart sur le visage                10e centile 113, médiane 195
#
# BRUIT se pose donc juste au-dessus du pire résidu du mur, et FRANC bien en
# dessous du plus faible écart du sujet. Entre les deux vivent les mèches, et
# l'opacité y varie continûment.
#
# Le premier essai les avait fixés à 16 et 52, au jugé : la propagation
# n'atteignait pas le mur, et le détourage rendait la photographie entière.
BRUIT = 42.0
FRANC = 95.0

# Le cadrage final, en fractions de l'image redressée : tête et buste, et pas
# au-delà. Un cadrage plus bas expose l'ombre creuse sous l'épaule gauche —
# elle est dans la photographie, pas dans le détourage, mais sur un fond sombre
# elle se lit comme une déchirure. Les proportions sont celles de la version
# précédente, pour que le composant du site n'ait rien à changer.
CADRE_HAUT = 0.015
CADRE_BAS = 0.760
PROPORTIONS = 640 / 892

LARGEURS = [640, 1024]

# WebP plutôt que PNG. Le PNG est sans perte, donc il conserve chaque grain de
# la peau et chaque mèche : 1 665 Ko pour la version large. WebP à qualité 85
# en fait 152 — **onze fois moins** — sans différence visible sur un portrait
# de vingt rem, et en gardant la couche alpha, ce que JPEG ne sait pas faire.
#
# Aucun repli en PNG : tous les navigateurs lisent le WebP depuis 2020, et un
# repli qu'on ne teste jamais est un fichier mort dans le dépôt.
QUALITE = 85


def developpe(source: pathlib.Path) -> Image.Image:
    """Développe le brut et redresse l'image.

    `sips` est le développeur de macOS : il applique le profil colorimétrique
    et la balance des blancs inscrits dans le fichier, ce qu'un décodeur
    générique ferait moins bien. La photo est prise en portrait mais stockée
    couchée, tête à gauche — une rotation horaire la redresse.
    """
    TRAVAIL.mkdir(exist_ok=True)
    intermediaire = TRAVAIL / "brut.png"
    if not intermediaire.exists():
        subprocess.run(
            ["sips", "-s", "format", "png", str(source), "--out", str(intermediaire)],
            check=True,
            capture_output=True,
        )
    return Image.open(intermediaire).convert("RGB").transpose(Image.ROTATE_270)


def modele_de_fond(a: np.ndarray) -> np.ndarray:
    """Le mur, reconstruit sous le sujet.

    Pour chaque ligne : la médiane de la marge gauche, celle de la marge
    droite, et une interpolation entre les deux. La médiane plutôt que la
    moyenne, pour qu'une poussière sur le mur ne déplace pas le modèle.
    """
    _, largeur, _ = a.shape
    marge = max(8, largeur // 40)
    gauche = np.median(a[:, :marge], axis=1)
    droite = np.median(a[:, -marge:], axis=1)

    # Un lissage vertical : le mur ne change pas d'une ligne à l'autre, et une
    # médiane de marge peut sursauter là où le sujet frôle le bord.
    gauche = ndimage.uniform_filter1d(gauche, 51, axis=0, mode="nearest")
    droite = ndimage.uniform_filter1d(droite, 51, axis=0, mode="nearest")

    t = np.linspace(0.0, 1.0, largeur, dtype=np.float32)[None, :, None]
    return gauche[:, None, :] * (1.0 - t) + droite[:, None, :] * t


def opacite(ecart: np.ndarray) -> np.ndarray:
    """L'opacité, par trimap.

    ## On cherche le fond, pas le sujet

    Seuiller sur « ressemble au sujet » échoue là où le sujet ressemble au mur —
    et c'est le cas d'un pli de chemise blanche dans l'ombre. Le premier essai
    y perçait un trou en pleine épaule.

    On raisonne donc à l'envers. Le mur est **connexe au bord de l'image** :
    c'est vrai par construction d'une photographie de studio, et ce n'est vrai
    d'aucune partie du sujet. On propage donc depuis les bords à travers les
    pixels qui ressemblent au mur, et tout ce que la propagation n'atteint pas
    appartient au sujet — quelle que soit sa couleur.

    Un pli entouré de chemise n'est jamais atteint : il reste dans le sujet
    sans qu'on ait à le deviner.
    """
    douce = np.clip((ecart - BRUIT) / (FRANC - BRUIT), 0.0, 1.0)

    # La propagation depuis les bords, à travers ce qui ressemble au mur.
    candidat = ecart < BRUIT
    etiquettes, nombre = ndimage.label(candidat)
    if nombre:
        bords = np.concatenate(
            [etiquettes[0], etiquettes[-1], etiquettes[:, 0], etiquettes[:, -1]]
        )
        atteintes = set(np.unique(bords)) - {0}
        fond = np.isin(etiquettes, list(atteintes)) if atteintes else np.zeros_like(candidat)
    else:
        fond = np.zeros_like(candidat)

    # Une **ouverture par reconstruction** du fond. La propagation s'infiltre
    # dans le sujet par les passages étroits où la chemise a exactement la
    # valeur du mur — un pli dans l'ombre, à l'épaule — et y creuse une baie,
    # qui n'est pas un trou et que le remplissage ne rattrape donc pas.
    #
    # On érode le fond pour rompre ces filets, puis on le reconstruit dans ses
    # propres limites : le mur, large, se rétablit entièrement ; l'infiltration,
    # étroite, ne repousse pas.
    graine = ndimage.binary_erosion(fond, np.ones((15, 15)))
    fond = ndimage.binary_propagation(graine, mask=fond)

    sujet = ~fond
    sujet = ndimage.binary_closing(sujet, np.ones((9, 9)))
    sujet = ndimage.binary_fill_holes(sujet)

    # Une seule silhouette : les îlots restants sont des défauts du mur.
    etiquettes, nombre = ndimage.label(sujet)
    if nombre > 1:
        tailles = ndimage.sum(sujet, etiquettes, range(1, nombre + 1))
        sujet = etiquettes == (1 + int(np.argmax(tailles)))

    # L'intérieur est opaque, mais on recule du bord : la frontière appartient
    # à la bande douce, où les mèches se dessinent.
    interieur = ndimage.binary_erosion(sujet, np.ones((9, 9)))

    # Et hors de la silhouette élargie, rien n'appartient au sujet.
    dehors = ~ndimage.binary_dilation(sujet, np.ones((25, 25)))

    resultat = np.where(interieur, 1.0, douce)
    return np.where(dehors, 0.0, resultat)


def detoure(image: Image.Image) -> Image.Image:
    a = np.asarray(
        image.resize((image.width // REDUCTION, image.height // REDUCTION), Image.BOX),
        dtype=np.float32,
    )
    fond = modele_de_fond(a)
    alpha = opacite(np.linalg.norm(a - fond, axis=2))[..., None].astype(np.float32)

    sujet = np.where(alpha > 0, (a - (1.0 - alpha) * fond) / np.maximum(alpha, 1e-6), a)
    np.clip(sujet, 0.0, 255.0, out=sujet)

    return Image.fromarray(
        np.concatenate([sujet, alpha * 255.0], axis=2).round().astype(np.uint8), "RGBA"
    )


def cadre(image: Image.Image) -> Image.Image:
    """Tête et buste, aux proportions de la version précédente."""
    haut = round(image.height * CADRE_HAUT)
    bas = round(image.height * CADRE_BAS)
    largeur = round((bas - haut) * PROPORTIONS)

    # Centré sur le sujet et non sur l'image : il n'est pas exactement au
    # milieu du cadre d'origine.
    colonnes = np.asarray(image)[haut:bas, :, 3].sum(axis=0, dtype=np.float64)
    centre = int(np.average(np.arange(len(colonnes)), weights=colonnes + 1.0))
    gauche = max(0, min(image.width - largeur, centre - largeur // 2))

    return image.crop((gauche, haut, gauche + largeur, bas))


def reduire(image: Image.Image, largeur: int) -> Image.Image:
    """Réduit en prémultipliant, pour que le transparent ne bave pas.

    À alpha droit, la couleur des pixels transparents — qui ne devrait compter
    pour rien — entre dans la moyenne du rééchantillonnage et rebave dans le
    sujet. C'est la moitié d'un halo, évitée gratuitement.
    """
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
    """L'écart entre les pixels de bord et le sujet, dans la région des cheveux.

    C'est la mesure qui compte : un contour plus clair que les cheveux qu'il
    borde ne peut venir que du fond.
    """
    a = np.asarray(image).astype(np.float32)
    haut = a[: a.shape[0] // 3]
    al = haut[..., 3] / 255.0
    bord = (al > 0.05) & (al < 0.9)
    plein = al > 0.98
    if not (bord.any() and plein.any()):
        return
    c, s = haut[..., :3][bord].mean(), haut[..., :3][plein].mean()
    print(f"  {nom:16} bord {c:6.1f} | cheveux {s:6.1f} | écart {c - s:+6.1f} | {int(bord.sum())} px")


def main() -> None:
    source = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else BRUT
    if not source.exists():
        raise SystemExit(f"fichier brut introuvable : {source}")

    print("→ développement du brut")
    photo = developpe(source)
    print(f"  {photo.width} × {photo.height}")

    print("→ détourage")
    decoupe = cadre(detoure(photo))
    mesure(decoupe, "détouré")

    for largeur in LARGEURS:
        reduit = reduire(decoupe, largeur)
        fichier = CIBLE / f"portrait-{largeur}.webp"
        # `method=6` : l'encodage le plus lent et le plus efficace. Il tourne
        # une fois, à la main ; la seconde qu'il coûte ne coûte rien.
        reduit.save(fichier, "WEBP", quality=QUALITE, method=6)
        mesure(reduit, f"portrait-{largeur}")
        print(
            f"  → {fichier.name}  {reduit.size[0]}×{reduit.size[1]}  "
            f"{fichier.stat().st_size // 1024} Ko"
        )


if __name__ == "__main__":
    main()
