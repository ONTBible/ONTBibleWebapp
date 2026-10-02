#!/usr/bin/env python3

"""Deux choses que seul le **rendu** peut dire : une ponctuation détachable,
et une imbrication qui tue l'hydratation.

## Pourquoi il faut vérifier sur le rendu, et pas sur les sources

Le français demande une espace **insécable** devant `;` `:` `!` `?` `»` et
après `«`. Une espace ordinaire y est un point de coupure : le navigateur y
renvoie volontiers la ponctuation à la ligne suivante, et l'on obtient une
ligne qui commence par « ; ». Ça ne casse rien, ça ne lève aucune erreur, et
sur une mesure étroite — un téléphone — ça arrive tout le temps.

Le corpus est tenu par `design/verset.rs::composer`. Le reste ne l'était pas,
et trois sources distinctes lui échappaient :

* la prose du site, écrite en littéraux Rust ;
* les littéraux **coupés en deux** par une continuation de ligne, qu'aucune
  recherche sur les sources ne rapproche ;
* les chaînes nues du corpus — le rendu d'un intraduisible, l'extrait d'une
  occurrence — qui ne traversent pas l'arbre de nœuds.

Une seule vérification les voit toutes : celle qui lit la page telle qu'elle
part au lecteur. C'est ce que fait ce script.

    ./scripts/verifier-composition.py

Le serveur de développement doit tourner. Sortie non nulle s'il trouve quelque
chose, pour qu'il puisse servir de garde.
"""

import html
import re
import sys
import urllib.request

SERVEUR = "http://127.0.0.1:3000"

# Une page par forme de gabarit. Les inventorier toutes n'apporterait rien :
# deux fiches de lexique passent par le même composant.
# **Toutes les pages, et la liste a menti pendant une journée.**
#
# Elle nommait `/fr/lire/*`, qui **redirige** depuis le déménagement du
# 29 septembre 2026 : la garde contrôlait donc des renvois de trois lignes et
# les annonçait comme des pages saines. Et les cinq écrans écrits ce jour-là —
# Qahal, Chuqqot, Vous, la recherche, la prononciation — n'y étaient pas du
# tout.
#
# Une garde qui ne visite pas une page ne dit rien d'elle, et son décompte final
# — « 12 pages, aucune imbrication interdite » — se lit comme si elle avait tout
# vu. C'est la forme la plus coûteuse : elle rassure exactement là où elle ne
# regarde pas.
#
# **Un contrôle interdit désormais d'en ajouter une sans elle** : voir
# `chaque_route_du_site_est_controlee` plus bas.
# **Les deux arbres**, et pas un seul. Un défaut de composition peut vivre dans
# un chrome plutôt que dans le texte — l'en-tête du site et la barre latérale ne
# portent pas les mêmes chaînes —, et ne contrôler qu'un arbre laisserait la
# moitié des pages hors du relevé sans que le décompte final le dise.
HORS_ARBRE = [
    "/fr",
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
    "/fr/l-app",
    "/fr/assistance",
    "/fr/confidentialite",
    "/fr/conditions",
]

# Ce que chaque arbre porte, sous sa racine.
SOUS_UN_ARBRE = [
    "/qahal",
    "/bible",
    "/bible/partie/torah",
    "/bible/bereshit",
    "/bible/bereshit/bereshit-1",
    "/lexique",
    "/lexique/adam",
    "/lexique/prononciation",
    "/chuqqot",
    "/compte",
    "/compte/lecture",
    "/rechercher?q=ruach",
]

PAGES = HORS_ARBRE + [
    f"/fr/{arbre}{suite}" for arbre in ("liseuse", "webapp") for suite in SOUS_UN_ARBRE
]

# L'espace ordinaire devant une ponctuation double, ou juste après un guillemet
# ouvrant. On garde les mots qui précèdent, pour que le message dise où
# regarder.
COUPURE = re.compile(r"\S{0,30} [;:!?»]|« ")


# ── L'imbrication qui tue l'hydratation ─────────────────────────────────────
#
# Un `<p>` ne peut pas en contenir un autre : l'analyseur **referme** le
# premier en rencontrant le second. Le DOM du navigateur cesse alors de
# correspondre au HTML que le serveur a écrit, et l'hydratation de Leptos meurt
# — « the framework expected a text node », `Unrecoverable hydration error`.
#
# Ce qu'on voit alors : une page parfaite où **rien ne répond au doigt**. Le
# rendu du serveur est juste, le texte est là, un moteur l'indexerait. Seul
# manque tout ce qui vit.
#
# ## Pourquoi ça ne se voit pas en relisant
#
# Les deux `<p>` sont dans **deux fichiers différents** : un conteneur qui
# enveloppe son slot dans un `<p>`, et un composant passé en slot qui en rend
# un aussi. Chacun est irréprochable seul. C'est leur rencontre qui est
# invalide, et rien dans une signature ne la signale.
#
# Trouvé le 29 septembre 2026, et l'auteur l'a décrit comme « quand je switch
# de tab il se passe rien ».
#
# ## Les autres imbrications interdites de la même famille
#
# Un `<a>` dans un `<a>`, un `<button>` dans un `<button>`, un `<form>` dans un
# `<form>`. Toutes se réparent silencieusement par l'analyseur, donc toutes
# produisent la même panne.
INTERDITES = {
    "p": ("p", "div", "ul", "ol", "table", "section", "article", "h1", "h2", "h3"),
    "a": ("a",),
    "button": ("button", "a"),
    "form": ("form",),
}


def imbrications(document: str) -> list[str]:
    """Les balises qu'un analyseur refermerait, donc qui cassent l'hydratation."""
    document = re.sub(r"<(script|style)[^>]*>.*?</\1>", " ", document, flags=re.S)
    fautes = []
    pile: list[str] = []
    for balise in re.finditer(r"<(/?)([a-zA-Z][a-zA-Z0-9]*)\b[^>]*?(/?)>", document):
        fermante, nom, autofermante = balise.group(1), balise.group(2).lower(), balise.group(3)
        if fermante:
            if nom in pile:
                # On dépile jusqu'à elle : les balises que le HTML autorise à
                # laisser ouvertes ne doivent pas bloquer la pile.
                while pile and pile.pop() != nom:
                    pass
            continue
        if autofermante or nom in ("br", "hr", "img", "input", "meta", "link", "path", "source"):
            continue
        for parent, interdits in INTERDITES.items():
            if nom in interdits and parent in pile:
                fautes.append(f"<{nom}> dans <{parent}>")
        pile.append(nom)
    return fautes


# Les préfixes de la syntaxe de Leptos. Posés sur un **composant**, ils disent
# « passe ceci à l'élément que tu rends » ; posés sur un élément natif, Leptos
# n'a rien à transmettre et écrit le nom tel quel — le document se retrouve avec
# un attribut dont le préfixe fait partie du nom.
#
# Rien ne s'en plaint : le HTML reste valide, la page s'affiche, et seul le
# sélecteur qui cherchait l'attribut rend une liste vide. `attr:data-verset` a
# ainsi vécu dans tous les versets du corpus, et `suivre_la_lecture` ne relevait
# jamais la position de lecture.
#
# ==Une faute de syntaxe qui se compile devient une donnée fausse, et une donnée
# fausse ne lève rien.== Celle-ci ne se voit que dans le document servi : ni le
# compilateur, ni un test de rendu, ni l'œil ne la rencontrent.
PREFIXES_DE_LEPTOS = re.compile(r"<[^>]*?\s((?:attr|prop|on|class|style):[\w:-]+)=")


def prefixes_restes(document: str) -> list[str]:
    """Les préfixes de Leptos qu'on retrouve dans le document servi."""
    return sorted({m.group(1) for m in PREFIXES_DE_LEPTOS.finditer(document)})


# **Les pages qui portent du texte à lire, et elles seules.**
#
# `.liseuse` pose une taille de police que le curseur des réglages gouverne :
# tout ce qu'elle contient en hérite. Posée sur les douze pages du gabarit, elle
# faisait enfler les boutons de connexion et les cartes de « Vous » — c'est-à-
# dire l'interface, que ce curseur ne doit jamais toucher.
#
# La liste est écrite par **suffixe de chemin**, pour qu'elle vaille dans les
# deux arbres sans être écrite deux fois.
#
# ==Un réglage qui agrandit l'interface se remarque tout de suite ; un corpus qui
# n'a pas grandi se remarque aussi.== L'oubli dans ce sens se voit ; dans
# l'autre, il passe pour une mise en page — d'où cette garde, qui refuse les
# deux.
PORTENT_DU_CORPUS = (
    "/lexique/prononciation",
    "/rechercher",
)


def porte_du_corpus(page: str) -> bool:
    """Un passage, une fiche, la prononciation ou une recherche."""
    # **Sans la chaîne de requête.** `/fr/webapp/rechercher?q=ruach` ne finit pas
    # par `/rechercher`, et la garde signalait une page de corpus comme une page
    # d'interface — en décrivant très bien un défaut qui n'existait pas.
    page = page.split("?", 1)[0]
    if page.endswith(PORTENT_DU_CORPUS):
        return True
    # Un passage : `/fr/<arbre>/bible/<livre>/<unité>`. Une fiche :
    # `/fr/<arbre>/lexique/<lemme>`, la feuille de prononciation mise à part.
    morceaux = [m for m in page.split("/") if m]
    # `/fr/<arbre>/bible/partie/<id>` a la même longueur qu'un passage et n'en
    # est pas un : c'est la liste des livres d'une partie.
    if len(morceaux) == 5 and morceaux[2] == "bible" and morceaux[3] != "partie":
        return True
    return len(morceaux) == 4 and morceaux[2] == "lexique"


def texte_de(page: str) -> str:
    document = urllib.request.urlopen(SERVEUR + page).read().decode()
    # Le script d'hydratation porte du JSON sérialisé, qui n'est pas de la
    # prose : le lire ferait des signalements que personne ne peut corriger.
    document = re.sub(r"<(script|style)[^>]*>.*?</\1>", " ", document, flags=re.S)
    return html.unescape(re.sub(r"<[^>]+>", "", document))


def main() -> None:
    fautes = 0
    for page in PAGES:
        try:
            document = urllib.request.urlopen(SERVEUR + page).read().decode()
            attendu = porte_du_corpus(page)
            # **La classe, et non le mot.** Un premier jet cherchait la chaîne
            # « liseuse » : elle est dans le canonique de chaque page
            # (`ontbible.com/fr/liseuse/…`) et dans le script d'avant-rendu, qui
            # porte les deux racines d'arbres. La garde signalait alors toutes
            # les pages de la webapp.
            #
            # ==Un nom qui sert aussi d'adresse ne se cherche pas comme un
            # mot.==
            # **Le conteneur du gabarit, et non la classe partout.** Le
            # sélecteur de fonte en pose une sur chacun de ses boutons, pour que
            # chaque ligne du menu se compose dans la fonte qu'elle propose : la
            # classe y est légitime, et la chercher sans sa balise faisait
            # rougir la page des réglages.
            #
            # `PageDeLecture` pose un `<div>` ; le sélecteur, des `<button>`.
            trouvee = re.search(r'<div class="liseuse(?:\s|")', document) is not None
            if attendu != trouvee:
                fautes += 1
                if trouvee:
                    print(f"{page} — `.liseuse` posée sur une page d'interface")
                    print("    le curseur de taille y ferait enfler les boutons et les cartes")
                else:
                    print(f"{page} — `.liseuse` absente d'une page de corpus")
                    print("    le curseur de taille n'y agrandirait rien")
            prefixes = prefixes_restes(document)
            if prefixes:
                fautes += len(prefixes)
                print(f"{page} — {len(prefixes)} préfixe(s) de Leptos écrit(s) tel(s) quel(s)")
                for prefixe in prefixes:
                    print(f"    {prefixe}  — un élément natif ne prend pas ce préfixe")
            nichees = imbrications(document)
            if nichees:
                fautes += len(nichees)
                print(f"{page} — {len(nichees)} imbrication(s) qui casse(nt) l'hydratation")
                for faute in dict.fromkeys(nichees):
                    print(f"    {faute}")
            trouve = [m.group(0) for m in COUPURE.finditer(texte_de(page))]
        except OSError as erreur:
            print(f"  {page} — injoignable ({erreur})", file=sys.stderr)
            fautes += 1
            continue

        if trouve:
            fautes += len(trouve)
            print(f"{page} — {len(trouve)} coupure(s) possible(s)")
            for extrait in trouve:
                print(f"    …{extrait}")

    if fautes:
        print(f"\n{fautes} au total.", file=sys.stderr)
        raise SystemExit(1)

    print(
        f"{len(PAGES)} pages, aucune ponctuation double détachable, "
        "aucune imbrication interdite, aucun préfixe de Leptos servi "
        "et `.liseuse` exactement sur les pages de corpus."
    )


if __name__ == "__main__":
    main()
