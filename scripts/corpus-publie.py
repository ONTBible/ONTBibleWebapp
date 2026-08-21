#!/usr/bin/env python3

"""Prépare le corpus pour la diffusion, à destination de l'app.

    ./scripts/corpus-publie.py            # écrit dans target/corpus/

## Pourquoi le corpus sort du bundle de l'app

Aujourd'hui l'app embarque `dist/` dans son paquet : corriger un verset demande
une compilation, un envoi à Apple, une revue, puis que chaque lecteur installe
la mise à jour. Une faute de frappe met des jours à disparaître, et davantage
chez qui n'a pas activé les mises à jour automatiques.

Publié ici, le corpus atteint les lecteurs en minutes. L'app garde son exemplaire
de bundle — il fait marcher une installation neuve, et il sert de filet — mais
elle lit d'abord ce qu'elle a téléchargé.

## Le nom porte l'empreinte, comme pour le site

Chaque fichier est nommé `<nom>.<empreinte>.json`. Un contenu nouveau a donc un
nom nouveau, ce qui autorise un cache d'un an sans risque : personne ne peut
recevoir une version périmée, puisque l'ancienne adresse ne désigne que
l'ancien contenu.

Le **manifeste** est la seule exception : nom fixe, cache court. C'est le point
d'entrée, celui que l'app interroge pour savoir s'il y a du nouveau. Un seul
fichier à revalider, quelques centaines d'octets.

## Livre par livre, pas d'un bloc

À soixante-dix livres le corpus pèsera une vingtaine de méga. Corriger un verset
de Bereshit ne doit pas en retélécharger vingt : chaque livre porte sa propre
empreinte, et l'app ne prend que ce qui a bougé.
"""

import hashlib
import json
import pathlib
import shutil
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
SOURCE = RACINE / ".." / "ONTBibleApp" / "dist"
SORTIE = RACINE / "target" / "corpus"

# Ce que l'app a besoin de recevoir, et sous quel nom public.
#
# `search.json` et `report.md` restent au vestiaire : le premier sert à la
# recherche du site, que l'app fait autrement ; le second est un rapport de
# construction destiné à un humain.
FICHIERS = {
    "plan": "corpus.json",
    "quotidien": "daily.json",
    "glossaire": "glossary.json",
    "occurrences": "occurrences.json",
}


def empreinte(octets: bytes) -> str:
    """Douze caractères de SHA-256, en base 16.

    Douze suffisent : la collision demanderait deux milliards de milliards de
    versions du même livre. Un nom de fichier reste lisible, et l'on peut le
    comparer à l'œil dans un journal.
    """
    return hashlib.sha256(octets).hexdigest()[:12]


def publier(nom: str, chemin: pathlib.Path, dossier: str = "") -> dict:
    octets = chemin.read_bytes()
    marque = empreinte(octets)
    relatif = f"{dossier}{nom}.{marque}.json"
    cible = SORTIE / relatif
    cible.parent.mkdir(parents=True, exist_ok=True)
    cible.write_bytes(octets)
    return {"chemin": relatif, "empreinte": marque, "octets": len(octets)}


def main() -> None:
    if not SOURCE.exists():
        raise SystemExit(
            f"{SOURCE} introuvable — le pipeline de ONTBibleApp doit avoir tourné"
        )

    # On repart de zéro : un fichier laissé d'une version antérieure serait
    # poussé sur S3 et n'en repartirait jamais, puisque son nom ne figure plus
    # dans aucun manifeste.
    if SORTIE.exists():
        shutil.rmtree(SORTIE)
    SORTIE.mkdir(parents=True)

    fichiers = {
        nom: publier(nom, SOURCE / source) for nom, source in FICHIERS.items()
    }

    livres = {
        chemin.stem: publier(chemin.stem, chemin, "livres/")
        for chemin in sorted((SOURCE / "books").glob("*.json"))
    }

    # La date vient du pipeline, pas de l'horloge de cette machine : c'est elle
    # qui date le corpus, et deux publications du même corpus doivent produire
    # le même manifeste.
    genere = json.loads((SOURCE / "manifest.json").read_text()).get("generatedAt", "")

    manifeste = {
        "schema": 2,
        "genere": genere,
        "fichiers": fichiers,
        "livres": livres,
    }
    (SORTIE / "manifeste.json").write_text(
        json.dumps(manifeste, ensure_ascii=False, separators=(",", ":"))
    )

    total = sum(f["octets"] for f in fichiers.values()) + sum(
        l["octets"] for l in livres.values()
    )
    print(f"  {len(livres)} livres, {len(fichiers)} fichiers, {total / 1024:.0f} Ko")
    print(f"  manifeste : {len(json.dumps(manifeste)) } octets")
    print(f"  → {SORTIE}")


if __name__ == "__main__":
    sys.exit(main())
