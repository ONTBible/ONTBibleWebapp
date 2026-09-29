#!/usr/bin/env python3

"""Engendre `style/mouvement.css` depuis les ressorts de l'app.

## Pourquoi engendrer, et pas approcher à l'œil

`ONTMouvement` déclare quatre ressorts — `response` et `dampingFraction` — et
son en-tête dit ce qu'ils valent :

> Quatorze vues du Mac capturées côte à côte avec l'iPad : tout ce qui bougeait
> sur le Mac bougeait en `easeOut` de 0,12 à 0,18 s — une rampe qui freine et
> s'arrête net. Sur l'iPhone, la feuille monte, dépasse d'un rien, se pose.
> **La même app, deux tempéraments** — et c'est le mouvement, plus que les
> formes, qui faisait dire « rigide ».

Le web n'a pas de ressort. Il a `cubic-bezier`, qui ne dépasse pas sa cible sans
qu'on le force, et depuis peu **`linear()`**, qui prend une suite de points et
interpole entre eux. C'est le bon outil : un ressort **s'échantillonne**.

Approcher chaque courbe par une cubique choisie à l'œil donnerait quatre
mouvements qui ressemblent aux leurs sans en être. Ici on résout l'équation.

## La physique, et d'où viennent les formules

SwiftUI définit `spring(response:dampingFraction:)` par :

    ω₀ = 2π / response          la pulsation propre
    ζ  = dampingFraction        l'amortissement réduit

Sous-amorti (ζ < 1), la réponse indicielle est :

    x(t) = 1 − e^(−ζω₀t) · [ cos(ω_d t) + (ζω₀ / ω_d) · sin(ω_d t) ]
    ω_d  = ω₀ · √(1 − ζ²)

Les quatre ressorts de l'app sont tous sous-amortis — leurs amortissements
vont de 0,66 à 0,78, et l'en-tête dit pourquoi : *les amortissements restent
au-dessus de 0,7 : en dessous, une liseuse tremble. On veut une étoffe, pas un
jouet.* `pop` descend à 0,66 parce qu'il ne sert qu'à de petits éléments.

## La durée, qui n'est pas dans la déclaration

Un ressort n'a pas de fin : il tend vers sa cible. On coupe quand il est
**visuellement arrivé** — à moins d'un millième de la cible, et qu'il y reste.
C'est cette durée qui devient celle de la transition CSS.

## Ce que ce script n'est pas

Une étape de CI. Il lit le Swift du voisin, comme le portage des couleurs, et
sa sortie est commitée.

    ./scripts/porter-les-ressorts.py
"""

import math
import pathlib
import re
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
SOURCE = (
    RACINE.parent
    / "ONTBibleApp/app/Packages/ONTDesignSystem/Sources/ONTDesignSystem/Motion/ONTMouvement.swift"
)
SORTIE = RACINE / "style" / "mouvement.css"

# Les noms français des ressorts, et ce qu'ils servent. Le nom de gauche est
# celui de l'app — c'est lui qui se relit là-bas.
NOMS = {
    "ressort": ("ressort", "un état qui change, une sélection qui se pose"),
    "ressortVif": ("ressort-vif", "un survol, un petit témoin — ce qui répond sous le doigt"),
    "arrivee": ("arrivee", "une carte, une feuille — le dépassement se voit, un peu"),
    "pop": ("pop", "un petit élément qui apparaît — vivant plutôt que posé là"),
}

# Le pas de la cascade et sa borne, relus eux aussi.
CASCADE = re.compile(r"min\(indice,\s*(\d+)\)\s*\)\s*\*\s*([\d.]+)")


class Refus(Exception):
    """Un refus délibéré — pas une panne."""


def relever() -> tuple[dict[str, tuple[float, float]], int, float]:
    """Les quatre ressorts et la cascade, lus dans le Swift de l'app."""
    if not SOURCE.exists():
        raise Refus(
            f"  {SOURCE} est illisible.\n"
            "  Les trois dépôts se rangent côte à côte sous `~/ONTBible/`."
        )
    source = SOURCE.read_text()

    ressorts = {}
    for nom in NOMS:
        trouve = re.search(
            rf"let {nom} = Animation\.spring\(response: ([\d.]+), dampingFraction: ([\d.]+)\)",
            source,
        )
        if trouve is None:
            raise Refus(
                f"  `{nom}` a changé de forme dans ONTMouvement.swift.\n"
                "  Les relever à la main et remettre ce script d'accord."
            )
        ressorts[nom] = (float(trouve.group(1)), float(trouve.group(2)))

    cascade = CASCADE.search(source)
    if cascade is None:
        raise Refus("  la cascade a changé de forme dans ONTMouvement.swift")
    return ressorts, int(cascade.group(1)), float(cascade.group(2))


def reponse(t: float, omega: float, zeta: float) -> float:
    """La réponse indicielle d'un ressort sous-amorti, à l'instant t."""
    omega_d = omega * math.sqrt(1 - zeta * zeta)
    return 1 - math.exp(-zeta * omega * t) * (
        math.cos(omega_d * t) + (zeta * omega / omega_d) * math.sin(omega_d * t)
    )


def echantillonner(response: float, zeta: float, pas: int = 60) -> tuple[str, float]:
    """Rend la liste `linear()` et la durée jusqu'à l'arrivée visuelle."""
    omega = 2 * math.pi / response

    # La fin : le premier instant où l'écart reste sous un millième. On avance
    # au millième de seconde plutôt que de résoudre — l'enveloppe est
    # exponentielle, donc n'importe quelle borne suffit et celle-ci se lit.
    duree, t = None, 0.0
    while t < 6.0:
        t += 0.001
        if all(abs(reponse(t + d, omega, zeta) - 1) < 0.001 for d in (0.0, 0.05, 0.1)):
            duree = t
            break
    if duree is None:
        raise Refus(f"  le ressort ({response}, {zeta}) ne se pose pas en six secondes")

    points = [reponse(i / pas * duree, omega, zeta) for i in range(pas + 1)]
    points[0], points[-1] = 0.0, 1.0
    return ", ".join(f"{p:.4f}" for p in points), duree


def composer(ressorts, borne: int, pas_cascade: float) -> str:
    lignes = [
        "/* Engendré par scripts/porter-les-ressorts.py — ne pas éditer à la main.",
        " *",
        " * Les quatre ressorts de `ONTMouvement`, résolus et échantillonnés. Le web",
        " * n'a pas de ressort ; `linear()` prend une suite de points, et un ressort",
        " * s'échantillonne. Approcher chaque courbe par une cubique choisie à l'œil",
        " * donnerait quatre mouvements qui ressemblent aux leurs sans en être.",
        " *",
        " * Chaque ressort donne **deux** jetons : sa courbe et sa durée. Un ressort",
        " * n'a pas de fin — il tend vers sa cible —, donc la durée est l'instant où",
        " * il est visuellement arrivé : à moins d'un millième, et il y reste.",
        " */",
        "",
        ":root {",
    ]
    for nom, (response, zeta) in ressorts.items():
        css, duree = NOMS[nom][0], None
        courbe, duree = echantillonner(response, zeta)
        lignes.append(f"  /* {NOMS[nom][1]} — response {response}, amortissement {zeta} */")
        lignes.append(f"  --{css}: linear({courbe});")
        lignes.append(f"  --{css}-duree: {duree * 1000:.0f}ms;")
        lignes.append("")
    lignes.append("  /* La cascade — le pas entre deux éléments qui se suivent,")
    lignes.append(f"     borné au {borne}ᵉ : une grille de soixante-dix cases n'a pas à se")
    lignes.append("     déplier pendant deux secondes. */")
    lignes.append(f"  --cascade-pas: {pas_cascade * 1000:.0f}ms;")
    lignes.append(f"  --cascade-borne: {borne};")
    lignes.append("}")
    lignes.append("")
    return "\n".join(lignes)


def main() -> None:
    ressorts, borne, pas = relever()
    SORTIE.write_text(composer(ressorts, borne, pas))
    for nom, (response, zeta) in ressorts.items():
        _, duree = echantillonner(response, zeta)
        print(f"  {NOMS[nom][0]:<12} response {response}  ζ {zeta}  →  {duree * 1000:.0f} ms")
    print(f"  cascade      {pas * 1000:.0f} ms, bornée au {borne}ᵉ")
    print(f"→ {SORTIE.relative_to(RACINE)}")


if __name__ == "__main__":
    try:
        main()
    except Refus as refus:
        print(f"\nPortage refusé :\n{refus}", file=sys.stderr)
        sys.exit(2)
