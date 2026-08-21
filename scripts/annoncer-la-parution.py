#!/usr/bin/env python3
"""Annonce aux lecteurs ce qui vient de paraître.

    ./scripts/annoncer-la-parution.py --avant ancien-plan.json --apres target/corpus/...

Appelé par le déploiement, **avant** que le nouveau corpus n'écrase l'ancien sur
S3 : c'est le seul instant où les deux existent côte à côte, et donc où l'on
peut savoir ce qui a changé.

## Pourquoi le déploiement, et non l'app

L'app sait déjà repérer une parution toute seule — `NouveautesNotifications`
compare son état à celui du corpus qu'elle vient de télécharger. Mais elle ne
peut le faire qu'une fois réveillée, et iOS décide seul de l'horaire.

Pour prévenir **à l'instant**, il faut que quelqu'un pousse ; et celui qui sait
ce qui vient de paraître, c'est la chaîne qui publie.

## Ce qui compte comme une parution

Un slot qui cesse d'être vide, une unité qui n'existait pas, un lemme qui entre
au lexique. **Pas** un fichier dont l'empreinte a changé — une virgule corrigée
en produit un, et personne ne veut être réveillé pour ça.

Un chapitre qui passe de `brouillon` à `locked` n'en est pas une non plus : le
lecteur l'avait déjà, seule la mention disparaît.

## Le silence est un résultat

Le déploiement le plus fréquent ne publie rien de neuf — une fiche réécrite, un
correctif de mise en page. Ce script ne dit alors rien, et rend 0. Ce n'est pas
un échec : c'est le cas normal.
"""

import argparse
import json
import os
import pathlib
import sys
import urllib.error
import urllib.request


def unites(plan: dict) -> set[str]:
    """`livre:unité` pour tout ce que le plan porte de rédigé."""
    out = set()
    for corpus in plan.get("corpora", []) or plan.get("modes", []) or []:
        for mode in corpus.get("modes", [corpus]):
            for livre in mode.get("books", []):
                if livre.get("empty"):
                    continue
                for unite in (livre.get("chapters") or []) + (
                    [livre["intro"]] if livre.get("intro") else []
                ):
                    out.add(f"{livre['id']}:{unite['id']}")
    return out


def livres(plan: dict) -> dict[str, dict]:
    out = {}
    for corpus in plan.get("corpora", []) or plan.get("modes", []) or []:
        for mode in corpus.get("modes", [corpus]):
            for livre in mode.get("books", []):
                out[livre["id"]] = livre
    return out


def annonce(avant: dict, apres: dict) -> dict | None:
    """Ce qu'il faut dire, ou `None` s'il n'y a rien à dire."""
    neuves = unites(apres) - unites(avant)
    if not neuves:
        return None

    connus = {u.split(":", 1)[0] for u in unites(avant)}
    catalogue = livres(apres)
    # Le premier livre par ordre de slot : une annonce, jamais une par unité.
    par_livre: dict[str, int] = {}
    for u in neuves:
        par_livre[u.split(":", 1)[0]] = par_livre.get(u.split(":", 1)[0], 0) + 1

    livre_id = min(par_livre, key=lambda i: catalogue.get(i, {}).get("slot", 999))
    livre = catalogue.get(livre_id, {})
    titre = livre.get("title", livre_id)
    combien = par_livre[livre_id]

    if livre_id not in connus:
        corps = f"{livre.get('french', titre)} vient de paraître dans La Bible ONT."
    elif combien == 1:
        corps = f"Un nouveau chapitre de {titre} vient de paraître."
    else:
        corps = f"{combien} nouveaux chapitres de {titre} viennent de paraître."

    return {"titre": titre, "corps": corps, "livre": livre_id}


def main() -> int:
    a = argparse.ArgumentParser()
    a.add_argument("--avant", required=True, help="le plan publié jusqu'ici")
    a.add_argument("--apres", required=True, help="le plan qu'on s'apprête à publier")
    a.add_argument("--api", default=os.environ.get("ONT_API", ""))
    options = a.parse_args()

    avant_p = pathlib.Path(options.avant)
    # Premier déploiement, ou plan précédent introuvable : on se tait. Annoncer
    # tout le corpus comme une nouveauté serait pire que de ne rien dire.
    if not avant_p.exists() or not avant_p.stat().st_size:
        print("  aucun plan précédent — rien annoncé")
        return 0

    avant = json.loads(avant_p.read_text())
    apres = json.loads(pathlib.Path(options.apres).read_text())

    quoi = annonce(avant, apres)
    if quoi is None:
        print("  rien de neuf — aucune annonce")
        return 0

    print(f"  à annoncer : {quoi['corps']}")

    secret = os.environ.get("SECRET_DIFFUSION", "")
    if not secret or not options.api:
        print("  diffusion non configurée — annonce non envoyée")
        return 0

    requete = urllib.request.Request(
        f"{options.api.rstrip('/')}/diffuser",
        data=json.dumps(quoi).encode(),
        headers={"Content-Type": "application/json", "X-Secret-Diffusion": secret},
        method="POST",
    )
    try:
        with urllib.request.urlopen(requete, timeout=30) as r:
            print(f"  diffusé, code {r.status}")
    except urllib.error.HTTPError as e:
        # **Non fatal, et c'est délibéré.** Le corpus est publié : les lecteurs
        # l'auront. Faire échouer le déploiement pour une notification manquée
        # laisserait le site à moitié déployé pour un agrément perdu.
        print(f"::warning::diffusion refusée, code {e.code} — le corpus est publié quand même")
    except Exception as e:  # noqa: BLE001
        print(f"::warning::diffusion injoignable ({e}) — le corpus est publié quand même")
    return 0


if __name__ == "__main__":
    sys.exit(main())
