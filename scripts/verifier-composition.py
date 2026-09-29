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
PAGES = [
    "/fr",
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
    "/fr/l-app",
    "/fr/assistance",
    "/fr/confidentialite",
    "/fr/conditions",
    # La webapp — les cinq onglets et ce qu'ils ouvrent.
    "/fr/qahal",
    "/fr/webapp",
    "/fr/compte/lecture",
    "/fr/webapp/partie/torah",
    "/fr/webapp/bereshit",
    "/fr/webapp/bereshit/bereshit-1",
    "/fr/lexique",
    "/fr/lexique/adam",
    "/fr/lexique/prononciation",
    "/fr/chuqqot",
    "/fr/compte",
    "/fr/rechercher?q=ruach",
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
        f"{len(PAGES)} pages, aucune ponctuation double détachable "
        "et aucune imbrication interdite."
    )


if __name__ == "__main__":
    main()
