#!/usr/bin/env python3
"""Engendre `style/jetons.css` depuis le design system de l'app.

## Pourquoi engendrer plutôt que recopier

L'auteur a demandé le 21 septembre 2026 que la webapp soit **identique en tout
point** à l'app iOS. Les couleurs sont donc les siennes, et elles doivent le
rester — y compris le jour où elles bougent chez elle.

Quatre-vingts valeurs recopiées à la main, ce sont quatre-vingts occasions de se
tromper **et** une divergence garantie à la première retouche. La semaine du
11 au 18 septembre en a donné trois exemples, chacun coûteux :

- une couleur transmise pour la terre brûlée, `#A3704D`, **écartée deux jours
  plus tôt** — la valeur voyageait encore alors que la décision était morte ;
- un `contrat` annoncé à 4 quand le pipeline en portait 3 ;
- un contre-exemple, `avraham`, qui n'existait que dans une fixture de test.

> **Une valeur qui voyage dans un message se périme en silence ; un chemin se
> relit.** Ce script pousse la règle d'un cran : on ne relit même plus, on
> demande.

## Pourquoi du Swift et pas une analyse du source

Parce que **la moitié des rôles calculent**. `ONTColors` n'est pas une table de
constantes : c'est un enum de fonctions du thème, et `shem`, `renvoi`, `accent`,
`inkSoft`, `separator`, les six rôles d'état — vingt au total, dont onze passent
par `melange(_:vers:part:)` ou une opacité.

Un extracteur textuel aurait rendu les neuf constantes justes et les onze autres
fausses. Fausses **avec l'air d'être bonnes**, ce qui est le pire des deux. On
exécute donc leur code : `swift run` importe `ONTDesignSystem` et interroge
chaque fonction.

## Le témoin qui prouve l'extraction

Le thème **mystique est né ici** — la nuit d'aubergine du site, transposée dans
l'app en août. Il doit donc revenir identique, et c'est vérifié à chaque passage :
neuf jetons comparés à `style/main.css`. S'ils divergent, c'est l'extraction qui
est fausse, pas la palette.

Un contrôle qui ne peut pas rougir ne prouve rien ; celui-ci a un cas dont on
connaît la réponse d'avance.

## Ce que ce script n'est pas

Une étape de CI. Il demande Xcode, que les coureurs Ubuntu n'ont pas. La sortie
est donc **commitée**, et se régénère à la main quand l'app bouge :

    ./scripts/porter-les-jetons.py
"""

import pathlib
import subprocess
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
OUTIL = RACINE / "scripts" / "jetons-de-l-app"
SORTIE = RACINE / "style" / "jetons.css"

# ── Le témoin, et pourquoi il est gelé ici plutôt que lu ─────────────────────
#
# Les neuf couleurs que `style/main.css` portait en dur avant ce portage, avec
# le rôle qui les rend chez l'app. `mystique` est né dans ce dépôt en août et a
# été transposé là-bas : il doit donc revenir **identique**, et c'est le seul
# cas de toute la chaîne dont les deux dépôts connaissent la réponse d'avance.
#
# La première version les relisait dans `main.css`. Elle ne le peut plus, et il
# faut comprendre pourquoi avant de vouloir l'y ramener : depuis le câblage,
# `main.css` prend ses couleurs de `var(--ont-*)`, c'est-à-dire du fichier que
# ce script écrit. Le témoin aurait vérifié sa propre sortie — un contrôle qui
# ne peut plus rougir, exactement ce que l'en-tête dit ne rien prouver.
#
# Elles sont donc **figées**, et ce n'est pas la transcription que ce script
# combat. Une transcription est une valeur qu'on recopie et qui doit suivre sa
# source ; celle-ci est un **point fixe** qui ne doit jamais bouger — c'est sa
# raison d'être. Le jour où l'auteur retouche une de ces neuf teintes, le
# portage **doit** rougir : il faut alors savoir lequel des deux dépôts a
# décidé, et mettre les deux d'accord avant de publier quoi que ce soit.
#
# Relevées sur `style/main.css` au commit a19cdcd, le 21 septembre 2026.
TEMOIN = {
    "background": "#18090D",
    "ink": "#CFC5B9",
    "inkStrong": "#EDE3D6",
    "inkSoft": "#9D948B",
    "accent": "#CDBE83",
    "accentuation": "#D87994",
    "shem": "#BA8C6C",
    "renvoi": "#D08C43",
    "surface": "#261016",
}

THEMES = ["parchemin", "clair", "sombre", "mystique"]

# Le thème sur lequel le site **ouvre**, et il n'est pas celui de l'app.
#
# L'app ouvre sur `parchemin` : c'est un lecteur, et un lecteur s'adapte à qui
# le tient. Le site est une **édition** — sa nuit d'aubergine est une décision,
# pas un défaut qu'on propose de corriger (CLAUDE.md §8 bis). Les trois autres
# restent atteignables ; c'est celui-ci qui se sert quand personne n'a choisi.
DEFAUT = "mystique"

# ── Le rôle que l'app n'a pas ────────────────────────────────────────────────
#
# Le site porte **trois** niveaux de surface, l'app en porte deux :
#
#     site   nuit  →  surface       →  surface-haute
#     app    background  →  surface
#
# Le troisième est celui des cartes, des feuilles et du bouton de réglages —
# huit composants s'en servent. Le laisser littéral donnerait une carte
# aubergine posée sur du parchemin ; le confondre avec `surface` effacerait
# l'élévation que ces huit composants dessinent.
#
# Il se **déduit** donc, d'un pas de plus dans la direction que l'app donne
# déjà : `surface` s'écarte de `background`, `surface-haute` s'en écarte
# d'autant encore. La direction s'inverse toute seule selon le thème — une
# carte monte vers le clair sur du parchemin, vers le clair aussi sur une nuit,
# parce que c'est ce que `surface` fait chez l'app dans les deux cas.
#
# **Le prix est mesuré, et il est d'un point.** Sur `mystique`, la déduction
# rend #34171F là où la rampe du site, dérivée à teinte constante 343°, portait
# #35151E. L'écart est sous le seuil de perception et du même ordre que celui
# que la rampe s'accorde déjà à elle-même — sa propre vérification retombe sur
# #411B26 pour une marque à #421B26. On préfère une règle qui vaut pour les
# quatre thèmes à une valeur juste pour un seul.
DEDUIT = "surfaceHaute"


def deduire_la_surface_haute(palette: dict[str, str]) -> str:
    """Un pas de plus que `surface`, dans la direction que l'app a choisie."""
    fond = palette["background"]
    surface = palette["surface"]

    def canal(debut: int) -> str:
        f = int(fond[debut : debut + 2], 16)
        s = int(surface[debut : debut + 2], 16)
        return f"{max(0, min(255, 2 * s - f)):02X}"

    return "#" + "".join(canal(d) for d in (1, 3, 5))


class Refus(Exception):
    """Un refus délibéré — pas une panne."""


def interroger() -> dict[str, dict[str, str]]:
    """Demande à l'app ses couleurs, thème par thème."""
    rendu = subprocess.run(
        ["swift", "run", "-c", "release", "extraire"],
        cwd=OUTIL,
        capture_output=True,
        text=True,
    )
    if rendu.returncode != 0:
        raise Refus(
            "  `swift run` a échoué dans scripts/jetons-de-l-app.\n"
            "  Ce script demande Xcode et le dépôt de l'app à côté — voir\n"
            "  l'en-tête. La sortie de swift :\n"
            + "".join(f"    {l}\n" for l in rendu.stderr.splitlines()[-6:])
        )

    jetons: dict[str, dict[str, str]] = {t: {} for t in THEMES}
    for ligne in rendu.stdout.splitlines():
        if ligne.count("\t") != 2:
            continue
        theme, role, valeur = ligne.split("\t")
        if theme in jetons:
            jetons[theme][role] = valeur.upper()

    # Le témoin positif : un relevé vide passerait toutes les vérifications
    # suivantes, et son silence se lirait comme un accord.
    manquants = [t for t in THEMES if len(jetons[t]) < 15]
    if manquants:
        raise Refus(
            f"  Thèmes incomplets ou vides : {manquants}.\n"
            "  L'outil a rendu moins de quinze rôles pour l'un d'eux — la liste\n"
            "  de `main.swift` a dû diverger de `ONTColors`."
        )
    return jetons


def verifier_le_temoin(jetons: dict[str, dict[str, str]]) -> None:
    """Mystique est né ici : il doit revenir identique."""
    ecarts = [
        f"{role} : site {attendu}, app {jetons['mystique'].get(role)}"
        for role, attendu in TEMOIN.items()
        if jetons["mystique"].get(role) != attendu
    ]
    if ecarts:
        raise Refus(
            "  Le thème mystique ne correspond plus à la palette du site :\n"
            + "".join(f"    {e}\n" for e in ecarts)
            + "  Il est né ici et transposé là-bas — un écart veut dire que l'un\n"
            "  des deux a bougé sans l'autre, et il faut savoir lequel avant de\n"
            "  publier quoi que ce soit."
        )
    print(f"  témoin : {len(TEMOIN)}/{len(TEMOIN)} jetons de mystique identiques au site")


def composer(jetons: dict[str, dict[str, str]]) -> str:
    """Un jeu de variables par thème, sous un sélecteur d'attribut."""
    roles = sorted(jetons["parchemin"])
    lignes = [
        "/* Engendré par scripts/porter-les-jetons.py — ne pas éditer à la main.",
        " *",
        " * Les couleurs viennent de `ONTColors` du design system de l'app, et non",
        " * d'une transcription : le script exécute leur code et lit ce qu'il rend.",
        " * La moitié des rôles calculent — mélanges, opacités, dérivations —, donc",
        " * une lecture du source aurait rendu la moitié des valeurs fausses.",
        " *",
        " * Le thème `mystique` est né dans ce dépôt et a été transposé dans l'app.",
        " * Il revient donc identique, et c'est le témoin qui prouve l'extraction.",
        " */",
        "",
    ]
    # ── Le défaut passe en tête, et ce n'est pas cosmétique ──────────────────
    #
    # Il porte `:root` en plus de son attribut, pour que la page se peigne même
    # sans JavaScript — le serveur n'écrit aucun `data-theme`.
    #
    # Or `:root` et `[data-theme='…']` ont **la même spécificité** : à égalité,
    # c'est l'ordre de la feuille qui tranche. Écrit en dernier, le bloc du
    # défaut gagnait donc contre les trois autres *sur l'élément racine*, et le
    # site restait sur sa nuit quel que soit le thème demandé.
    #
    # Le défaut s'est vu exactement comme le §8 sexies l'annonce : quatre
    # aperçus sortis **identiques à l'octet près**. Sans cette comparaison on
    # serait allé chercher la panne dans le signal, dans l'hydratation ou dans
    # l'attribut — trois endroits où il n'y avait rien.
    #
    # En tête, le bloc du défaut est celui qu'on écrase, et les trois autres
    # l'écrasent. Sur une racine qui porte déjà le défaut, les deux blocs qui
    # s'appliquent sont le même : l'ordre n'y change rien.
    ordre = [DEFAUT] + [t for t in THEMES if t != DEFAUT]
    for theme in ordre:
        selecteur = f":root, [data-theme='{theme}']" if theme == DEFAUT \
            else f"[data-theme='{theme}']"
        lignes.append(f"{selecteur} {{")
        for role in roles:
            lignes.append(f"  --ont-{role}: {jetons[theme][role]};")
        lignes.append("}")
        lignes.append("")
    return "\n".join(lignes)


def main() -> None:
    jetons = interroger()
    verifier_le_temoin(jetons)
    for palette in jetons.values():
        palette[DEDUIT] = deduire_la_surface_haute(palette)
    SORTIE.write_text(composer(jetons))
    roles = len(jetons["parchemin"])
    print(f"  {roles} rôles × {len(THEMES)} thèmes = {roles * len(THEMES)} jetons")
    print(f"  → {SORTIE.relative_to(RACINE)}")


if __name__ == "__main__":
    try:
        main()
    except Refus as refus:
        print(f"\nPortage refusé :\n{refus}", file=sys.stderr)
        sys.exit(2)
