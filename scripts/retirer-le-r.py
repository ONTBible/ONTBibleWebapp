#!/usr/bin/env python3

"""Retire le ® du wordmark et du combination mark.

## Pourquoi ce script existe

Apposer ® sur une marque qui n'est pas déposée à l'INPI relève de l'article
L.716-9 du code de la propriété intellectuelle. Ce n'est pas un risque
commercial dont on pèserait la probabilité : c'est une infraction, et le
wordmark est sur **toutes** les pages du site.

Le dépôt est lancé. Tant que l'enregistrement n'est pas prononcé — plusieurs
mois — le sigle ne peut pas rester. Ce script le retire, et garde l'original
sous `*-avec-r.svg` pour le jour où il redeviendra légitime : il suffira alors
de remettre ces fichiers en place.

## Il mesure au lieu de supposer

Le ® n'est pas un élément séparé qu'on pourrait supprimer du balisage. Affinity
exporte la marque entière en **un seul tracé**, dont le ® n'est que quatre
sous-tracés parmi trente-deux — le cercle, son intérieur, la panse du R et son
délié.

On ne les désigne donc pas par leur numéro, qui changerait au prochain export.
On les trouve par la géométrie, et la règle tient en une phrase : **le ® est le
plus petit groupe de sous-tracés, pris depuis la droite, qui ne chevauche plus
rien de ce qui reste**. Aucune lettre n'est à droite du sigle — c'est ce qui le
définit comme exposant final.

Un premier essai triait par la **hauteur** : les quatre sous-tracés du ® sont
petits, les lettres sont grandes. Il ne marchait pas, et l'erreur est
instructive. Les contre-formes — l'intérieur d'un B, d'un e, d'un O — sont des
sous-tracés à part entière, et certaines font trente unités quand le cercle du
® en fait trente-cinq. Aucun seuil de hauteur ne sépare les deux, parce qu'ils
se recouvrent réellement.

La séparation horizontale, elle, est nette : le sigle commence à 15635 là où le
texte s'arrête à 15622. On prend donc le plus petit groupe séparable, et non le
plus grand — car en remontant vers la gauche, chaque lettre finit par être
« séparable » de celles qui la précèdent, et un groupe trop grand emporterait
des lettres. Une vérification de taille refuse en dernier recours tout groupe
dont la hauteur approche celle d'une capitale.

## Il est idempotent

Relancé sur un fichier déjà traité, il trouve la dernière lettre à la place du
sigle, la mesure contre ses voisines de ligne, et n'écrit pas. Vérifié sur trois
passes. À passer après `normaliser-svg.py`, sans réfléchir, à chaque nouvel
export.
"""

import re
import shutil
import statistics
import sys
from pathlib import Path

RACINE = Path(__file__).resolve().parent.parent / "public/images"
MARQUES = ["wordmark.svg", "combination-mark.svg"]


def sous_traces(d: str) -> list[str]:
    """Découpe l'attribut `d` en sous-tracés.

    Chaque sous-tracé commence par un `M` absolu — c'est la seule commande qui
    lève le crayon. Affinity n'émet que des `M` majuscules en tête de contour.
    """
    return ["M" + morceau for morceau in d.split("M") if morceau.strip()]


def boite(sous_trace: str) -> tuple[float, float, float, float]:
    """La boîte englobante, prise sur les points de contrôle.

    C'est une approximation — une courbe de Bézier peut sortir de l'enveloppe
    de ses points — mais elle est *conservatrice dans le bon sens* : elle
    surestime plutôt qu'elle ne sous-estime, donc elle ne fait jamais passer un
    tracé pour plus à gauche qu'il n'est.
    """
    nombres = [float(x) for x in re.findall(r"-?\d+\.?\d*", sous_trace)]
    xs, ys = nombres[0::2], nombres[1::2]
    return min(xs), min(ys), max(xs), max(ys)


def retirer(chemin: Path) -> bool:
    """Écrit la marque sans son ®. Rend vrai si le fichier a changé."""
    source = chemin.read_text()
    correspondance = re.search(r'(\sd=")([^"]+)(")', source)
    if not correspondance:
        raise SystemExit(f"{chemin.name} : aucun attribut `d`.")

    traces = sous_traces(correspondance.group(2))
    boites = [boite(t) for t in traces]
    hauteur_totale = max(b[3] for b in boites) - min(b[1] for b in boites)

    largeur_totale = max(b[2] for b in boites) - min(b[0] for b in boites)

    # Les filets sont mis de côté avant tout.
    #
    # Le combination mark en porte un, qui sépare la montagne du texte et
    # traverse **toute** la largeur. Tant qu'il compte, rien n'est jamais
    # entièrement à droite de lui, et la recherche ne trouve rien — c'est ce
    # qui est arrivé au premier essai, sur une marque qui porte pourtant le
    # sigle en évidence.
    #
    # Un filet se reconnaît sans ambiguïté : très large, et presque sans
    # hauteur. Aucune lettre n'a cette forme.
    def est_filet(b: tuple[float, float, float, float]) -> bool:
        return (b[2] - b[0]) > largeur_totale * 0.6 and (b[3] - b[1]) < hauteur_totale * 0.05

    glyphes = [i for i, b in enumerate(boites) if not est_filet(boites[i])]

    # Du plus à droite au plus à gauche.
    ordre = sorted(glyphes, key=lambda i: boites[i][0], reverse=True)

    sigle: list[int] = []
    for k in range(1, len(ordre)):
        groupe, reste = ordre[:k], ordre[k:]
        if min(boites[i][0] for i in groupe) > max(boites[i][2] for i in reste):
            sigle = groupe
            break

    # Un groupe séparable n'est pas encore un ®. Sur une marque déjà nettoyée,
    # c'est la dernière lettre — séparable, elle aussi, une fois le sigle parti.
    #
    # On mesure donc le candidat contre **les glyphes qui partagent sa ligne**,
    # et non contre la hauteur de la marque entière. Un premier essai le faisait,
    # et il a mangé deux tracés de trop : le combination mark contient une
    # montagne, qui écrase toutes les proportions et fait passer une lettre pour
    # un exposant.
    #
    # Rapporté à sa ligne, le sigle mesure 37 % dans les deux marques — 35
    # unités contre 95 sur le wordmark, 73 contre 198 sur le combination mark.
    # Une lettre, elle, en mesure cent. Le seuil à 60 % laisse donc une marge
    # confortable des deux côtés, et c'est ce qui rend le script rejouable : la
    # seconde passe trouve la dernière lettre, la mesure, et s'arrête.
    if sigle:
        haut = min(boites[i][1] for i in sigle)
        bas = max(boites[i][3] for i in sigle)
        ligne = [
            boites[i][3] - boites[i][1]
            for i in glyphes
            if i not in set(sigle) and boites[i][1] < bas and boites[i][3] > haut
        ]
        if not ligne or (bas - haut) > statistics.median(ligne) * 0.6:
            sigle = []

    if not sigle:
        print(f"  {chemin.name} : déjà sans ®")
        return False

    garde = [t for i, t in enumerate(traces) if i not in set(sigle)]
    retires = len(sigle)

    # L'original, pour le jour de l'enregistrement.
    original = chemin.with_name(chemin.stem + "-avec-r.svg")
    if not original.exists():
        shutil.copy2(chemin, original)
        print(f"  {original.name} : original conservé")

    chemin.write_text(
        source[: correspondance.start(2)] + "".join(garde) + source[correspondance.end(2) :]
    )
    print(f"  {chemin.name} : {retires} sous-tracés retirés sur {len(traces)}")
    return True


def main() -> None:
    for nom in MARQUES:
        chemin = RACINE / nom
        if not chemin.exists():
            raise SystemExit(f"{nom} est introuvable dans {RACINE}.")
        retirer(chemin)


if __name__ == "__main__":
    sys.exit(main())
