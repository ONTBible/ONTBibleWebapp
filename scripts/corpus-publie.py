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
import os
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
# Les clés sont celles que les clients cherchent, **relevées chez eux** et non
# choisies ici : `shemot` et `recherche` apparaissent onze et quatre fois dans
# `app/` et `android/`, contre une seule pour `search`. Une clé bien nommée mais
# autrement ne rend rien, et ne rend rien *en silence* — le client cherche, ne
# trouve pas, et garde ce qu'il avait.
FICHIERS = {
    "plan": "corpus.json",
    "quotidien": "daily.json",
    "glossaire": "glossary.json",
    "occurrences": "occurrences.json",
    # Ajoutés le 8 septembre 2026, sur l'audit externe (A10). Sans eux, les
    # fiches de Shemot et l'index de recherche restaient ceux de l'installation
    # pendant que le texte évoluait : `gavriel`, `moshe`, `sinai`, `eliyahu`
    # étaient déjà signalés comme Shemot sans fiche distribuée.
    "shemot": "shemot.json",
    "recherche": "search.json",
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

    Le refus n'écarte que la publication du corpus : il rend `REFUS`, que le
    workflow traite comme un pas de côté annoncé, et les pages du site partent
    quand même — elles embarquent `dist/` à la compilation et ne dépendent pas
    de ce qu'on publie ici. Les liseuses gardent le corpus précédent.
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


# La version du **manifeste**, et elle ne se recopie de nulle part.
#
# ## Ce qu'elle n'est pas
#
# Elle a d'abord été prise ici pour le contrat des nœuds du pipeline, et le
# script s'est mis à publier `contrat` sous ce nom. C'était faux, et il a fallu
# lire la liseuse pour le voir — pas la deviner :
#
#     CorpusUpdater.swift:87   « La version du manifeste que ce code sait lire.
#                                **2 depuis 1.0.3**, où l'accentuation a changé
#                                de nom sur le fil. »
#
# Il y a **deux manifestes**, et on les confond parce qu'ils portent presque le
# même nom :
#
#     dist/manifest.json           écrit par le pipeline    schema 1, contrat 3
#     /corpus/manifeste.json       écrit par ce script      schema 2
#
# La liseuse récupère `https://ontbible.com/corpus/` — donc **le nôtre** — et
# compare **notre** `schema` au sien. Le `contrat` du pipeline ne l'atteint
# jamais. Publier 3 ici aurait fait refuser le corpus par toutes les liseuses
# installées, qui sont à 2 et qui ont raison de l'être.
#
# ## Ce qu'elle est
#
# Le numéro de compatibilité **du fil**, copropriété de ce script et des deux
# liseuses. Il se monte délibérément, dans le même lot qu'une version de
# liseuse qui sait lire la nouveauté — jamais en le recopiant d'ailleurs.
#
# C'est la différence qui a manqué toute la journée : une valeur dont l'unique
# devoir est de **suivre** se recopie ; une valeur **copropriétaire** de deux
# parties se garde par une mesure. Celle-ci est de la seconde espèce, et le
# défaut n'était pas qu'elle soit écrite à la main — c'est que personne ne la
# mesurait contre quoi que ce soit.
SCHEMA_DU_MANIFESTE = 2

# Ce que « schéma 2 » veut dire, en extension.
#
# **C'est la garde qui compte**, et c'est la seule qui protège vraiment. Un
# nombre n'est qu'une promesse ; la liste dit ce qu'il promet. Le jour où le
# pipeline émet un type de plus, le corpus le porte — que le pipeline l'ait
# déclaré ou non, et même s'il déclare mal.
#
# Relevé le 11 septembre 2026 sur `books/`, `glossary.json` et `shemot.json` :
# dix-sept types, et aucun `renvoi` ni `reference`. La variante `Renvoi` existe
# pourtant **déjà** dans le schéma du pipeline sur `dev` — donc le corpus
# pourrait en porter demain, sans que rien d'autre ne bouge.
#
# Cette liste et `SCHEMA_DU_MANIFESTE` se déplacent **ensemble, dans le même
# commit**, avec la version de liseuse qui sait lire le type nouveau. L'une
# sans l'autre ne veut rien dire.
TYPES_DE_NOEUDS_CONNUS = {
    # les blocs
    "verses", "heading", "list", "para", "quote", "table", "rule",
    # les nœuds en ligne
    "text", "term", "accentuation", "gloss", "em", "translit", "heb",
    "link", "break", "shem",
}

# Les fichiers où `t` désigne un **type de nœud**. Ailleurs il porte autre
# chose — dans `search.json` c'est le texte de l'extrait, et le parcourir ferait
# rendre des milliers de « types » qui sont des phrases.
PORTEURS_DE_NOEUDS = ("books", "glossary.json", "shemot.json")

# Le schéma de la liseuse **en vente**, que ce script ne peut pas relever seul :
# elle vit sur `app-store`, une branche que le dépôt voisin n'a pas forcément
# décochée. La CI la va chercher en profondeur 1 et la passe ici.
EN_VENTE = "ONT_SCHEMA_EN_VENTE"

# Les deux liseuses, à la source. Un nombre recopié ici serait un troisième
# endroit à tenir d'accord, c'est-à-dire un troisième endroit qui peut mentir.
LISEUSES = {
    "iOS": (
        "app/Packages/ONTData/Sources/ONTData/Remote/CorpusUpdater.swift",
        re.compile(r"static\s+let\s+schema\s*=\s*(\d+)"),
    ),
    "Android": (
        "android/ontdata/src/main/kotlin/com/labibleont/ont/data"
        "/remote/CorpusUpdater.kt",
        re.compile(r"const\s+val\s+SCHEMA\s*:\s*Int\s*=\s*(\d+)"),
    ),
}


def lire_le_contrat(manifeste: dict) -> int | None:
    """Rend le contrat des nœuds du pipeline, ou `None` s'il ne le dit pas.

    On le **recopie**, on ne l'écrit jamais : sa seule propriété utile est de
    suivre `CONTRAT_DES_NOEUDS`. Il est publié à côté de `schema`, sous son
    propre nom, et il ne s'y substitue pas — voir `SCHEMA_DU_MANIFESTE`.

    ## Pourquoi son absence n'est pas un refus

    Aucune liseuse ne le lit encore. Le publier est un gain pour demain ; ne
    pas le publier rend exactement l'état d'aujourd'hui. Refuser pour un champ
    que personne ne consulte arrêterait la chaîne sans protéger personne.

    Ce qui protège, c'est `verifier_les_types_emis` — qui regarde le corpus au
    lieu de croire ce qu'il déclare. Un `contrat` absent **n'est pas sûr**, et
    un `contrat` présent ne l'est pas davantage : c'est le contenu qui décide.
    """
    contrat = manifeste.get("contrat")
    if contrat is None:
        print(
            "  ⚠ dist/manifest.json ne porte pas de « contrat » — pipeline\n"
            "    antérieur au champ. Le manifeste publié n'en portera pas."
        )
        return None
    if not isinstance(contrat, int) or isinstance(contrat, bool) or contrat < 1:
        raise Refus(
            f"  dist/manifest.json porte « contrat » = {contrat!r}.\n"
            "  C'est un entier ou rien : une valeur d'une autre forme veut dire\n"
            "  que le champ a changé de sens, pas qu'il vaut zéro."
        )
    return contrat


def verifier_les_types_emis() -> None:
    """Refuse de publier un corpus qui porte un type de nœud inconnu.

    ## Pourquoi celle-ci, et pas la déclaration du pipeline

    Le premier correctif recopiait le `contrat` du pipeline et refusait s'il
    manquait. Il gardait une **attestation**, et une attestation absente se
    lisait comme un danger là où une attestation présente se lisait comme une
    garantie — deux fois faux.

    Ce qui met les liseuses en danger n'est pas ce que le pipeline déclare,
    c'est ce que le corpus **contient**. La variante `Renvoi` est déjà dans le
    schéma du pipeline sur `dev`, sans le champ `contrat` : une garde sur
    l'attestation aurait laissé passer exactement le cas qu'elle prétend
    couvrir.

    On regarde donc le corpus. C'est plus long à écrire, et c'est la seule
    mesure qui ne puisse pas mentir.

    ## Ce que le refus coûte, et pourquoi il est bon

    Une liseuse qui reçoit un type qu'elle ignore **lève**, puis retombe sur
    son bundle — sans un mot, et pour toujours. Refuser de publier laisse le
    corpus précédent en place, qu'elle sait lire, et rend le défaut bruyant du
    côté où quelqu'un regarde.

    Le remède n'est jamais d'allonger la liste seule : c'est de livrer une
    liseuse qui sait lire le type, de monter `SCHEMA_DU_MANIFESTE` avec elle,
    et d'ajouter le type ici dans le même commit.
    """
    inconnus: dict[str, int] = {}
    vus = 0

    def parcourir(x) -> None:
        nonlocal vus
        if isinstance(x, dict):
            t = x.get("t")
            # Un type de nœud est court et sans espace. Le même nom de clé
            # porte du texte ailleurs dans dist/ — d'où `PORTEURS_DE_NOEUDS`,
            # et cette seconde barrière par prudence.
            if isinstance(t, str) and len(t) <= 40 and " " not in t:
                vus += 1
                if t not in TYPES_DE_NOEUDS_CONNUS:
                    inconnus[t] = inconnus.get(t, 0) + 1
            for y in x.values():
                parcourir(y)
        elif isinstance(x, list):
            for y in x:
                parcourir(y)

    for nom in PORTEURS_DE_NOEUDS:
        chemin = SOURCE / nom
        fichiers = sorted(chemin.glob("*.json")) if chemin.is_dir() else [chemin]
        for fichier in fichiers:
            if fichier.exists():
                parcourir(json.loads(fichier.read_text()))

    # Le témoin positif. Un corpus dont on ne lit **aucun** nœud passerait cette
    # garde sans rien vérifier, et son silence se lirait comme un accord — un
    # chemin renommé chez le pipeline suffirait. Ne rien trouver n'est pas
    # trouver que tout va bien.
    if vus == 0:
        raise Refus(
            "  Aucun nœud lu dans "
            + ", ".join(PORTEURS_DE_NOEUDS)
            + ".\n  Cette garde ne sait plus ce qu'elle garde. Les fichiers du\n"
            "  pipeline ont dû changer de nom ou de forme."
        )

    if inconnus:
        detail = ", ".join(f"{t} ({n})" for t, n in sorted(inconnus.items()))
        raise Refus(
            f"  Le corpus porte des types de nœuds inconnus : {detail}.\n"
            f"  Les liseuses en schéma {SCHEMA_DU_MANIFESTE} lèveraient dessus,\n"
            "  puis retomberaient sur leur bundle — en silence, et pour\n"
            "  toujours.\n"
            "  Il faut livrer une liseuse qui sait les lire, monter\n"
            "  SCHEMA_DU_MANIFESTE avec elle, et les ajouter à\n"
            "  TYPES_DE_NOEUDS_CONNUS dans le même commit."
        )

    print(f"  {vus} nœuds lus, tous connus du schéma {SCHEMA_DU_MANIFESTE}")


def verifier_l_accord_des_liseuses() -> None:
    """Refuse de publier un manifeste que les liseuses ne savent pas lire.

    La comparaison est en **égalité stricte** des deux côtés — `guard ==` en
    Swift, `!=` en Kotlin. Un manifeste dont le `schema` n'est pas exactement
    le leur est refusé **en bloc**, avec le corpus derrière.

    Le plafond qui compte est celui de la version **en vente** : les liseuses
    du dossier cloné ne sont dans les mains de personne, la sienne est dans
    toutes. Une liseuse clonée en avance sur nous est donc un avertissement —
    elle annonce le prochain palier — tandis qu'une liseuse en vente en
    désaccord est un refus.

    L'ordre, et il n'est pas négociable : **le lecteur d'abord, ce qu'il lit
    ensuite.** On livre la liseuse qui sait lire, puis on publie.
    """
    racine = SOURCE.parent
    releves: dict[str, int] = {}
    manquants: list[str] = []

    for liseuse, (chemin, motif) in LISEUSES.items():
        fichier = racine / chemin
        if not fichier.exists():
            manquants.append(f"{liseuse} : {chemin} introuvable")
            continue
        trouve = motif.search(fichier.read_text())
        if trouve is None:
            manquants.append(
                f"{liseuse} : la constante a changé de forme dans {chemin}"
            )
            continue
        releves[liseuse] = int(trouve.group(1))

    en_vente = os.environ.get(EN_VENTE, "").strip()
    if en_vente:
        if not en_vente.isdigit():
            raise Refus(
                f"  {EN_VENTE} vaut « {en_vente} », qui n'est pas un entier.\n"
                "  La CI le relève dans CorpusUpdater.swift de `app-store` ;\n"
                "  une valeur mal formée veut dire que le relevé a glissé."
            )
        releves["la version en vente"] = int(en_vente)

    # Le témoin positif, encore : une garde qui ne trouve rien à comparer
    # passerait sans rien vérifier.
    if manquants:
        raise Refus(
            "  Impossible de relever ce que les liseuses acceptent :\n"
            + "".join(f"    {m}\n" for m in manquants)
            + "  Cette garde ne sait plus ce qu'elle garde, donc elle ne laisse\n"
            "  pas passer. Si la constante a bougé, c'est LISEUSES qu'il faut\n"
            "  remettre d'accord avec elle."
        )

    accord = ", ".join(f"{l} {v}" for l, v in sorted(releves.items()))
    print(f"  schéma {SCHEMA_DU_MANIFESTE} — relevé chez les liseuses : {accord}")

    # Une liseuse **en avance** n'est pas une panne : elle attend un palier
    # qu'on n'a pas encore publié, ce qui est l'ordre voulu. On le dit, et on
    # continue — c'est le cas normal entre la livraison d'une liseuse et la
    # publication qui la suit.
    en_avance = {l: v for l, v in releves.items() if v > SCHEMA_DU_MANIFESTE}
    if en_avance:
        detail = ", ".join(f"{l} lit {v}" for l, v in sorted(en_avance.items()))
        print(
            f"  ⚠ {detail} — en avance sur le schéma publié. Monter\n"
            "    SCHEMA_DU_MANIFESTE quand le fil aura effectivement changé."
        )

    en_retard = {l: v for l, v in releves.items() if v < SCHEMA_DU_MANIFESTE}
    if en_retard:
        detail = ", ".join(f"{l} lit {v}" for l, v in sorted(en_retard.items()))
        raise Refus(
            f"  Le manifeste publierait le schéma {SCHEMA_DU_MANIFESTE}, et"
            f" {detail}.\n"
            "  La comparaison est en égalité stricte : ces liseuses refuseraient\n"
            "  le manifeste entier, donc tout le corpus derrière — y compris les\n"
            "  corrections des livres qu'elles lisaient très bien.\n"
            "  L'ordre est : livrer la liseuse qui sait lire, puis publier."
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
    manifeste_du_pipeline = json.loads((SOURCE / "manifest.json").read_text())
    genere = manifeste_du_pipeline.get("generatedAt", "")
    verifier_la_date(genere)

    # Le contrat des nœuds, recopié et non écrit — puis confronté à ce que les
    # liseuses savent lire. Les deux gardes sont ici, avant la première copie,
    # pour la raison dite plus haut : échouer avant d'agir ne laisse rien à
    # rattraper.
    # Trois gardes, toutes avant la première copie. Elles ne se recouvrent
    # pas : l'une regarde ce que le corpus **contient**, l'autre ce que les
    # liseuses **acceptent**, la troisième ne fait que recopier une déclaration.
    verifier_les_types_emis()
    verifier_l_accord_des_liseuses()
    contrat = lire_le_contrat(manifeste_du_pipeline)

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
        # La version du fil, copropriété de ce script et des deux liseuses.
        # **Pas** le contrat des nœuds du pipeline — voir SCHEMA_DU_MANIFESTE.
        "schema": SCHEMA_DU_MANIFESTE,
        "genere": genere,
        "fichiers": fichiers,
        "livres": livres,
    }
    if contrat is not None:
        manifeste["contrat"] = contrat
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
