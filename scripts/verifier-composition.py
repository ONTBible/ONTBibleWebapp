#!/usr/bin/env python3

"""Cherche, dans le site rendu, une ponctuation double qui peut tomber à la ligne.

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
PAGES = [
    "/fr",
    "/fr/le-pourquoi",
    "/fr/ce-que-l-ont-n-est-pas",
    "/fr/l-app",
    "/fr/assistance",
    "/fr/confidentialite",
    "/fr/conditions",
    "/fr/lire",
    "/fr/lire/bereshit",
    "/fr/lire/bereshit/bereshit-1",
    "/fr/lexique",
    "/fr/lexique/adam",
]

# L'espace ordinaire devant une ponctuation double, ou juste après un guillemet
# ouvrant. On garde les mots qui précèdent, pour que le message dise où
# regarder.
COUPURE = re.compile(r"\S{0,30} [;:!?»]|« ")


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
        print(f"\n{fautes} au total. Une espace insécable manque.", file=sys.stderr)
        raise SystemExit(1)

    print(f"{len(PAGES)} pages, aucune ponctuation double détachable.")


if __name__ == "__main__":
    main()
