#!/usr/bin/env python3
"""Déclarer les adresses du site aux moteurs qui acceptent IndexNow.

## Pourquoi ce script existe

Le 25 août 2026, deux modèles de langage ont échoué à lire le corpus. ChatGPT a
répondu « le site ne s'ouvre pas correctement depuis mon outil » — faux, mesuré :
tous les agents reçoivent 200 et le même octet. Gemini a été plus franc — « ce
n'est pas indexé dans mes bases » — puis a **inventé** l'auteur du projet, le
remplaçant par un collectif anonyme.

Un modèle qui n'a pas les données ne dit pas qu'il ne sait pas : il comble. Tant
que le site n'est dans aucun index, la seule version de l'ONT qui circule est
celle qu'il invente.

## Ce que ce script couvre, et ce qu'il ne couvre pas

IndexNow n'exige **aucun compte** : on pose une clé à la racine du domaine, on
POSTe la liste des adresses, et les moteurs participants se la partagent.

- **Couvert** — Bing, Yandex, Naver, Seznam, Yep. Dont **Bing, qui alimente la
  recherche de ChatGPT** : c'est ce qui débloque le test que Gloire refait.
- **Pas couvert** — Google, donc Gemini. Google a refusé le protocole en 2021 et
  n'a pas changé d'avis. Son index passe par la Search Console, qui demande un
  compte : ça reste un geste humain, décrit au §8 septies.

## Ce qu'il refuse de faire

Il **ne soumet pas** si la clé n'est pas déjà servie en ligne. Bing rendrait 403,
et un 403 ne dit pas *lequel* des deux fichiers il n'a pas aimé — on chercherait
dans la liste d'adresses un défaut qui est dans le déploiement.

C'est la règle du journal appliquée : un instrument se valide sur un cas dont on
connaît la réponse, et refuse de répondre à une question qu'il ne peut pas
trancher.

    ./scripts/soumettre-aux-index.py            # les adresses du sitemap en ligne
    ./scripts/soumettre-aux-index.py --simuler  # tout vérifier, ne rien envoyer
"""

from __future__ import annotations

import json
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

SITE = "ontbible.com"
SITEMAP = f"https://{SITE}/sitemap.xml"
POINT_DE_DEPOT = "https://api.indexnow.org/indexnow"
PUBLIC = Path(__file__).resolve().parent.parent / "public"


def lire(url: str, delai: int = 30) -> bytes:
    requete = urllib.request.Request(url, headers={"User-Agent": "ONTBible/1.0"})
    with urllib.request.urlopen(requete, timeout=delai) as reponse:
        return reponse.read()


def cle_locale() -> str:
    """La clé, telle que le dépôt la porte.

    Elle est trouvée par sa **forme** — 32 hexadécimaux — et non par un nom
    inscrit ici : le nom du fichier *est* la clé, donc l'écrire en dur ferait
    deux endroits à tenir d'accord le jour d'une rotation.
    """
    candidats = [
        f for f in PUBLIC.glob("*.txt") if re.fullmatch(r"[0-9a-f]{32}", f.stem)
    ]
    if len(candidats) != 1:
        noms = ", ".join(sorted(f.name for f in candidats)) or "aucun"
        sys.exit(f"✗ il faut exactement un fichier de clé dans public/ — trouvé : {noms}")
    return candidats[0].read_text(encoding="utf-8").strip()


def cle_servie(cle: str) -> bool:
    """La clé est-elle joignable en ligne, et vaut-elle la nôtre ?

    Les deux moitiés comptent. Servir *une* clé ne suffit pas : Bing compare
    celle du fichier à celle du corps de la requête, et un déploiement en retard
    sert l'ancienne — cas où tout paraît en place et où le dépôt rend 403.
    """
    url = f"https://{SITE}/{cle}.txt"
    try:
        corps = lire(url, delai=20).decode("utf-8").strip()
    except urllib.error.HTTPError as erreur:
        print(f"  ✗ {url} → {erreur.code}")
        return False
    except OSError as erreur:
        print(f"  ✗ {url} → {erreur}")
        return False
    if corps != cle:
        print(f"  ✗ {url} sert une autre clé — déploiement en retard ?")
        return False
    print(f"  ✓ {url} sert la bonne clé")
    return True


def adresses() -> list[str]:
    plan = lire(SITEMAP).decode("utf-8")
    trouvees = re.findall(r"<loc>([^<]+)</loc>", plan)
    if not trouvees:
        sys.exit(f"✗ aucune adresse dans {SITEMAP} — le relevé est cassé")
    return trouvees


def deposer(cle: str, liste: list[str]) -> int:
    charge = json.dumps(
        {
            "host": SITE,
            "key": cle,
            "keyLocation": f"https://{SITE}/{cle}.txt",
            "urlList": liste,
        }
    ).encode("utf-8")
    requete = urllib.request.Request(
        POINT_DE_DEPOT,
        data=charge,
        headers={"Content-Type": "application/json; charset=utf-8"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(requete, timeout=60) as reponse:
            code = reponse.status
    except urllib.error.HTTPError as erreur:
        code = erreur.code

    # 200 accepté, 202 accepté et en attente de validation de la clé. Les deux
    # sont des succès : le second est même le cas normal d'une première fois.
    explication = {
        200: "accepté",
        202: "accepté, clé en cours de validation",
        400: "format invalide",
        403: "clé refusée — le fichier ne correspond pas",
        422: "des adresses ne relèvent pas de ce domaine",
        429: "trop de requêtes — réessayer plus tard",
    }.get(code, "réponse inattendue")
    print(f"\n  {POINT_DE_DEPOT} → {code} ({explication})")
    return 0 if code in (200, 202) else 1


def main() -> int:
    simuler = "--simuler" in sys.argv

    print(f"Déclarer {SITE} aux moteurs qui acceptent IndexNow\n")
    cle = cle_locale()
    print(f"  clé du dépôt : {cle}")

    if not cle_servie(cle):
        print(
            "\n✗ La clé n'est pas servie en ligne — rien n'a été envoyé.\n"
            "  Bing rendrait 403, et un 403 ne dit pas lequel des deux fichiers\n"
            "  il refuse : on chercherait le défaut dans la liste d'adresses.\n"
            "  Déployer d'abord, relancer ensuite."
        )
        return 1

    liste = adresses()
    print(f"  {len(liste)} adresses au sitemap")

    if simuler:
        print("\n  --simuler : tout est en place, rien n'a été envoyé.")
        for url in liste[:3]:
            print(f"    {url}")
        print(f"    … et {len(liste) - 3} autres")
        return 0

    return deposer(cle, liste)


if __name__ == "__main__":
    raise SystemExit(main())
