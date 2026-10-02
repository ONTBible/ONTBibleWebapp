#!/usr/bin/env python3
"""Porte les symboles de l'interface depuis Phosphor Icons.

## Pourquoi pas les SF Symbols

L'app dessine sa navigation avec eux, et ils ne peuvent pas venir : leur licence
les réserve aux logiciels **tournant sur les plateformes Apple**. Un site web
n'en est pas un, quel que soit l'appareil qui l'ouvre. Ce n'est pas une réserve
de prudence, c'est le texte.

## Pourquoi Phosphor, et pas une autre

Cherché le 29 septembre 2026, à la demande de l'auteur. Iconoir, Remix, Lucide,
Heroicons, Tabler, Feather sont tous libres et tous **en contour seul**.

Phosphor est la seule de la liste qui porte **le remplissage et le contour sur
la même silhouette** — et c'est exactement ce dont la barre d'onglets a besoin :
l'app y emploie `book.closed.fill`, `person.2.fill`, `square.stack.3d.up.fill`.
Un onglet actif se dit par le plein, pas seulement par la couleur.

Elle dessine aussi en **aplats** et non en traits, comme les SF Symbols. Les
silhouettes que ce dépôt dessinait à la main étaient des traits de 1,6 : plus
maigres, et surtout dessinées au coup par coup — chacune avec sa propre
épaisseur et sa propre marge.

MIT, sans attribution exigée. La licence est quand même déposée dans le dépôt :
on garde ce qu'on nous prête, même quand on n'y est pas obligé.

## Ce que le script fait, et ce qu'il ne fait pas

Il **ne choisit pas** les correspondances : elles sont dans `SYMBOLES`, avec le
nom du symbole de l'app en regard, pour qu'on puisse relire le choix plutôt que
le deviner. Il télécharge, extrait le tracé, et écrit un module Rust.

À relancer à la main quand un symbole s'ajoute. Ce n'est pas une étape de CI :
elle demande le réseau, et les coureurs n'ont aucune raison de le dépenser pour
des tracés qui ne bougent jamais.
"""

import pathlib
import re
import urllib.request

BASE = "https://raw.githubusercontent.com/phosphor-icons/core/main"
IONIC = "https://raw.githubusercontent.com/ionic-team/ionicons/main/src/svg"
RACINE = pathlib.Path(__file__).resolve().parent.parent

# nom chez nous → (nom Phosphor, le symbole de l'app qu'il remplace)
SYMBOLES = {
    "qahal": ("users", "person.2.fill"),
    "livre": ("book-bookmark", "book.closed.fill"),
    "lexique": ("book-open-text", "character.book.closed.fill"),
    "strates": ("stack", "square.stack.3d.up.fill"),
    "compte": ("user-circle", "person.crop.circle.fill"),
    "loupe": ("magnifying-glass", "magnifyingglass"),
    "signet": ("bookmark-simple", "bookmark.fill"),
    "feuillets": ("book-open", "book.pages"),
    "onde": ("waveform", "waveform"),
    "taille": ("text-aa", "textformat.size"),
    # Les trois du pavé « À venir » du Qahal.
    "retenu": ("heart-straight", "heart.text.square"),
    "echanges": ("chats-circle", "bubble.left.and.text.bubble.right"),
}

# Les marques des boutons de connexion — Ionicons, MIT, grille 512.
#
# **Pas Phosphor** : elle n'a pas de logos de marques, et c'est normal — une
# famille d'icônes d'interface n'a pas à porter les marques d'autrui.
#
# Le choix vient de l'app, arbitré par l'auteur le 29 septembre 2026 : elle
# portait trois provenances pour trois boutons — `apple.logo`, `g.circle.fill`
# (un G générique d'Apple, pas celui de Google) et trois chevrons `</>` pour
# GitHub. Trois graisses, trois grilles.
#
# **La réserve est levée et il faut dire comment.** Google exige son mark tel
# qu'il le fournit, en quatre couleurs ; la session iOS a mesuré qu'aucune
# variante monochrome officielle n'existe. L'auteur a vu les deux et demandé le
# monochrome, pour que les trois marques prennent la teinte de leur bouton au
# lieu d'y poser la leur. C'est une décision prise en connaissant l'écart, pas
# un oubli — et le calcul est plus simple ici que chez elle : un site n'est
# relu par personne.
MARQUES = ("apple", "google", "github")

# Ceux dont on a besoin **aussi** en plein — la barre d'onglets et la barre
# latérale, où l'actif se dit par le remplissage.
PLEINS = {"qahal", "livre", "lexique", "strates", "compte", "signet"}


def tracer(nom: str, plein: bool) -> str:
    """Le `d` du tracé, depuis le SVG de Phosphor."""
    variante = "fill" if plein else "regular"
    fichier = f"{nom}-fill" if plein else nom
    url = f"{BASE}/assets/{variante}/{fichier}.svg"
    with urllib.request.urlopen(url, timeout=20) as reponse:
        svg = reponse.read().decode()

    # **Plusieurs `<path>` possibles**, et le premier relevé le croyait unique.
    # `stack-fill` en porte trois — une feuille par strate. Coller le premier
    # aurait rendu une seule feuille, c'est-à-dire un symbole *plausible* et
    # faux : celui qu'on ne regarde pas deux fois.
    #
    # Ils se concatènent dans un seul `d` sans rien perdre : chaque sous-tracé
    # commence par son propre `M`, et c'est ce qui en fait des sous-tracés.
    # La règle de remplissage par défaut (`nonzero`) les compose comme le SVG
    # d'origine, puisqu'aucun ne porte de `fill-rule` à lui.
    chemins = re.findall(r'<path[^>]*\bd="([^"]+)"', svg)
    if not chemins:
        raise SystemExit(f"{url} ne porte aucun tracé")
    if 'fill-rule' in svg or 'clip-rule' in svg:
        raise SystemExit(f"{url} porte une règle de remplissage que la fusion perdrait")
    if 'viewBox="0 0 256 256"' not in svg:
        raise SystemExit(f"{url} n'est pas sur la grille 256 — la table le suppose")
    return " ".join(chemins)


def main() -> None:
    lignes = []
    for chez_nous, (chez_eux, sf) in sorted(SYMBOLES.items()):
        contour = tracer(chez_eux, plein=False)
        lignes.append(f'        // {sf}  ←  phosphor/{chez_eux}\n')
        lignes.append(f'        ("{chez_nous}", false) => "{contour}",\n')
        if chez_nous in PLEINS:
            plein = tracer(chez_eux, plein=True)
            lignes.append(f'        ("{chez_nous}", true) => "{plein}",\n')
        print(f"  {chez_nous:11} ← {chez_eux:18} ({sf})")

    for marque in MARQUES:
        url = f"{IONIC}/logo-{marque}.svg"
        with urllib.request.urlopen(url, timeout=20) as reponse:
            svg = reponse.read().decode()
        chemins = re.findall(r'<path[^>]*\bd="([^"]+)"', svg)
        if not chemins:
            raise SystemExit(f"{url} ne porte aucun tracé")
        # Ionicons dessine sur 512, Phosphor sur 256. On ne convertit pas : le
        # rendu porte la grille dans son `viewBox`, et deux familles n'ont
        # aucune raison de partager une échelle.
        if 'viewBox="0 0 512 512"' not in svg:
            raise SystemExit(f"{url} n'est pas sur la grille 512")
        lignes.append(f'        // logo-{marque}  ←  ionicons (MIT)\n')
        lignes.append(f'        ("marque-{marque}", false) => "{" ".join(chemins)}",\n')
        print(f"  marque-{marque:8} ← ionicons/logo-{marque}")

    sortie = RACINE / "src" / "interface" / "design" / "symboles.rs"
    sortie.write_text(
        '//! Les symboles de l\'interface — **engendré, ne pas modifier à la main**.\n'
        "//!\n"
        "//! `scripts/porter-les-symboles.py` les tire de Phosphor Icons (MIT), et son\n"
        "//! en-tête dit pourquoi ce n'est pas les SF Symbols de l'app : leur licence\n"
        "//! les réserve aux logiciels tournant sur les plateformes Apple.\n"
        "//!\n"
        "//! Chaque tracé porte en commentaire le symbole de l'app auquel il répond, pour\n"
        "//! que la correspondance se **relise** au lieu de se deviner.\n"
        "//!\n"
        "//! Grille **256 × 256** et tracés en **aplat**, comme les SF Symbols — et non\n"
        "//! en traits, comme les silhouettes que ce module dessinait à la main.\n"
        "//!\n"
        "//! **Sauf les trois marques**, qui viennent d'Ionicons (MIT) et vivent sur une\n"
        "//! grille de 512. Elles ne sont pas converties : le rendu porte sa grille dans\n"
        "//! son `viewBox`, et deux familles n'ont aucune raison de partager une échelle.\n"
        "\n"
        "/// Le tracé d'un symbole, plein ou en contour.\n"
        "///\n"
        "/// `plein` n'est pas un ornement : c'est ainsi que la barre d'onglets de l'app\n"
        "/// dit l'onglet actif — `book.closed.fill` contre `book.closed`. La couleur\n"
        "/// seule ne suffit pas, un lecteur daltonien ne verrait rien.\n"
        "///\n"
        "/// Un nom inconnu rend le contour du compte plutôt que rien : un symbole absent\n"
        "/// laisserait un trou dans une barre, ce qui se lit comme une panne.\n"
        "pub fn trace(nom: &str, plein: bool) -> &'static str {\n"
        "    match (nom, plein) {\n"
        + "".join(lignes)
        + '        _ => trace("compte", plein),\n'
        "    }\n"
        "}\n"
    )
    print(f"→ {sortie.relative_to(RACINE)}")

    for nom, url in (
        ("Phosphor-MIT.txt", f"{BASE}/LICENSE"),
        ("Ionicons-MIT.txt", "https://raw.githubusercontent.com/ionic-team/ionicons/main/LICENSE"),
    ):
        licence = RACINE / "public" / "fontes" / nom
        with urllib.request.urlopen(url, timeout=20) as reponse:
            licence.write_bytes(reponse.read())
        print(f"→ {licence.relative_to(RACINE)}")


if __name__ == "__main__":
    main()
