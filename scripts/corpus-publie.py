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
import datetime
import re
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


# Un horodatage ISO 8601 en UTC, à la seconde : « 2026-08-30T00:14:00Z ».
#
# La forme est **exacte** et non approchée, parce que l'app compare ces dates
# comme des chaînes. Un décalage écrit `+02:00` au lieu du `Z` se trierait avant
# un `T00:` du même jour ; une date sans secondes se trierait avant elle-même
# allongée. Deux estampilles bien formées mais de formes différentes s'ordonnent
# alors à l'envers, et l'app garderait le plus vieux des deux corpus en croyant
# garder le plus neuf.
DATE_ATTENDUE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")


# Le code que rend un **refus délibéré**, par opposition à une panne. L'appelant
# peut alors distinguer « je n'ai rien publié, et je sais pourquoi » de « je suis
# tombé ». Les deux valent 1 sans ça, et il faut choisir entre tout bloquer et
# ne rien voir.
REFUS = 2


class Refus(Exception):
    """Un refus délibéré de publier — pas une panne."""


def verifier_la_date(genere: str) -> None:
    """Refuse de publier un corpus que l'app ne saura pas dater.

    ## Ce que cette date empêche

    L'app lit son corpus **du disque quand il y est, du bundle sinon**, le disque
    l'emportant toujours — et le disque est rempli depuis ce qu'on publie ici.

    Tant que le publié est plus récent que l'embarqué, tout va bien. L'inverse
    arrive chaque fois qu'un build part avant un déploiement du site,
    c'est-à-dire à chaque livraison TestFlight : le corpus publié, plus vieux,
    **écrase** le corpus embarqué, plus neuf. Mesuré sur simulateur le 30 août
    2026 — bundle à 1913 occurrences de `shem`, disque à 217, et l'app recréait
    le disque au lancement en retéléchargeant l'ancien.

    `genere` est le champ qui permet à l'app de dire « ce que tu me proposes est
    plus vieux que ce que je porte, je garde le mien ». Il traverse toute la
    chaîne depuis le début, et il était **vide** depuis le début.

    ## Pourquoi refuser plutôt que publier quand même

    Ce script reportait déjà la date du pipeline, correctement et pour la bonne
    raison. Ce qu'il ne faisait pas, c'est **constater qu'elle manquait** : il
    lisait un champ absent, prenait la chaîne vide par défaut, et publiait un
    manifeste bien formé que rien ne pouvait dater. Une valeur par défaut qui
    remplace une mesure absente produit une sortie qu'on croit vérifiée.

    C'est la règle de `soumettre-aux-index.py` : ne pas répondre à une question
    qu'on ne peut pas trancher vaut mieux que rendre une réponse bien formée.

    Le refus bloque le déploiement du site tant que le pipeline n'écrit pas la
    date — et c'est voulu, puisque déployer dans cet état reconduit le défaut.
    """
    if not genere:
        raise Refus(
            "  dist/manifest.json ne porte pas de generatedAt.\n"
            "  Le corpus publié serait indatable, et l'app l'emploierait pour\n"
            "  écraser un corpus embarqué plus récent — silencieusement.\n"
            "  C'est `generated_at` dans le pipeline qu'il faut remplir, puis\n"
            "  régénérer dist/ ; le report est déjà fait ici."
        )
    if not DATE_ATTENDUE.match(genere):
        raise Refus(
            f"  generatedAt vaut « {genere} », que l'app ne saura pas comparer.\n"
            "  Elle trie ces dates comme des chaînes : il faut de l'ISO 8601 en\n"
            "  UTC à la seconde, « 2026-08-30T00:14:00Z ». Un décalage horaire\n"
            "  ou des secondes omises inversent l'ordre sans rien casser."
        )

    # ── La forme peut être juste et la valeur fausse ─────────────────────────
    #
    # `git log --date=format:%Y-%m-%dT%H:%M:%SZ` rend l'heure **locale** du
    # commit et lui colle un `Z`. Vingt signes, secondes présentes, `Z` final :
    # la forme est irréprochable, et la valeur ment de l'écart au méridien —
    # deux heures pour une machine à Paris en août. L'expression régulière
    # ci-dessus ne peut rien y voir, ni celle de l'app.
    #
    # La valeur se trahit ailleurs : une date écrite en heure locale à l'est de
    # Greenwich tombe **dans le futur** une fois lue comme de l'UTC. Un corpus
    # daté d'après l'instant où on le publie n'a aucun sens.
    #
    # Ce contrôle est **partiel, et il faut le dire** : il n'attrape rien à
    # l'ouest de Greenwich, où la même faute produit une date trop ancienne,
    # indiscernable d'un corpus simplement bâti la veille. Il attrape le cas
    # qui se présente — la machine de l'auteur et les coureurs GitHub — et pas
    # la faute en général.
    #
    # **Et il n'y a rien en amont qui la couvre.** Le seul endroit qui le
    # pourrait est le pipeline, parce que lui sait de quelle horloge la date
    # vient ; il tient la forme, pas la valeur. Ce qui rend la valeur juste
    # aujourd'hui est un `TZ=UTC` et un `--date=format-local:` dans une ligne
    # de shell — que rien n'éprouve. Le jour où quelqu'un les retire, ou revient
    # à `--date=format:` en trouvant l'autre obscur, le faux `Z` revient et
    # cette ligne-ci est la seule de toute la chaîne à le dire, à l'est de
    # Greenwich seulement.
    #
    # C'est donc un filet, pas une garantie. Ne pas le lire comme une garantie
    # parce qu'on suppose l'amont couvert : il ne l'est pas, et cette phrase
    # remplace une version antérieure qui l'affirmait à tort.
    #
    # La marge absorbe l'écart d'horloge entre la machine qui bâtit le corpus et
    # celle qui le publie. Elle est très inférieure à une heure, donc au plus
    # petit décalage horaire qui existe.
    MARGE = datetime.timedelta(minutes=5)
    date = datetime.datetime.strptime(genere, "%Y-%m-%dT%H:%M:%SZ").replace(
        tzinfo=datetime.timezone.utc
    )
    maintenant = datetime.datetime.now(datetime.timezone.utc)
    if date > maintenant + MARGE:
        avance = (date - maintenant).total_seconds() / 3600
        raise Refus(
            f"  generatedAt vaut « {genere} », soit {avance:.1f} h dans le futur.\n"
            "  La forme est juste, donc la valeur ne l'est pas : c'est ce que\n"
            "  produit une heure locale à laquelle on a collé un « Z ».\n"
            "  Côté pipeline, `--date=format:` rend l'heure locale du commit ;\n"
            "  c'est `--date=format-local:` avec TZ=UTC qu'il faut."
        )


def main() -> None:
    if not SOURCE.exists():
        raise SystemExit(
            f"{SOURCE} introuvable — le pipeline de ONTBibleApp doit avoir tourné"
        )

    # On repart de zéro : un fichier laissé d'une version antérieure serait
    # poussé sur S3 et n'en repartirait jamais, puisque son nom ne figure plus
    # dans aucun manifeste.
    # ── Avant d'écrire quoi que ce soit ─────────────────────────────────────
    #
    # La date se lit et se vérifie **en premier**, alors qu'elle ne sert qu'au
    # manifeste, écrit en dernier. Placée à son point d'usage, la garde refusait
    # après avoir copié huit fichiers : le dossier restait à moitié publié, sans
    # manifeste, et un `aws s3 sync` lancé à la main dessus aurait posé un corpus
    # sans son point d'entrée.
    #
    # `deployer.sh` porte `set -euo pipefail` et n'y serait pas allé — mais une
    # garde ne doit pas dépendre du soin de celui qui l'appelle. Échouer avant
    # d'agir ne laisse rien à rattraper.
    genere = json.loads((SOURCE / "manifest.json").read_text()).get("generatedAt", "")
    verifier_la_date(genere)

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

    # `genere` vient du pipeline et non de l'horloge de cette machine : c'est lui
    # qui date le corpus, et deux publications du même corpus doivent produire le
    # même manifeste. Lu et vérifié en tête de cette fonction.

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
    try:
        sys.exit(main())
    except Refus as refus:
        print(refus, file=sys.stderr)
        sys.exit(REFUS)
