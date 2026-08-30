#!/usr/bin/env python3

"""Compare ce que le site déclare disponible à ce que le backend sert vraiment.

    ./scripts/sonder-les-fournisseurs.py

## Pourquoi ce script existe

Le 30 août 2026, le bouton « Continuer avec GitHub » était en production et ne
pouvait pas marcher : le backend rendait `503 fournisseur non configuré` sur
l'origine `webapp`. Un lecteur partait chez GitHub, autorisait, revenait, et
tombait sur une erreur où il ne pouvait rien faire.

**Aucune garde du site ne pouvait le voir.** `compte_public::disponible` décide
ce que la page affiche, `compte::identifiant_client` ce que la route sait faire,
et un test tient les deux d'accord — mais les deux vivent ici. La configuration
du backend n'est visible d'aucun des deux, et c'est elle qui décide.

D'où la règle : **une garde du site ne peut pas voir la configuration d'un
service qu'il appelle ; seule une sonde contre le déployé mesure ce qui tourne,
tout le reste mesure ce qu'on a écrit.**

Et la règle pratique qui en découle, celle que ce script rend exécutable :
**la sonde fait partie de l'allumage, pas de la vérification d'après.**

## Le témoin positif, et il n'est pas décoratif

Chaque relevé porte `google` en origine `webapp`, dont on connaît la réponse :
il est configuré, donc il doit rendre `401`. Sans lui, deux `503` se lisent « le
chemin webapp est cassé » ; avec lui, ils se lisent « le chemin fonctionne, ce
sont ces deux-là qui manquent ».

C'est la leçon d'un défaut voisin du même jour : une mesure qui **confirme ce
qu'on espère** doit être éprouvée par un cas dont on connaît la réponse. Un
`grep` sur la page servie n'avait rien trouvé — ce qui était exactement ce qu'on
voulait lire — alors qu'il ne pouvait rien trouver du tout, butant sur les
marqueurs d'hydratation de Leptos.

## Lire le résultat

    401 « connexion refusée »        la requête est allée jusqu'au fournisseur,
                                     qui a refusé notre code bidon → CONFIGURÉ
    503 « fournisseur non configuré » elle n'est jamais partie     → ABSENT

L'origine ne passe **pas** par l'en-tête `Origin` : c'est un champ du corps JSON.
Une sonde qui l'omet retombe sur `app` — le défaut qui protège les apps déjà
installées — et mesure six fois le mauvais chemin. Une session s'y est fait
prendre le jour même et concluait que tout allait bien.
"""

import json
import pathlib
import re
import sys
import urllib.error
import urllib.request

BACKEND = "https://api.ontbible.com"
RETOUR = "https://ontbible.com/fr/compte/retour"
RACINE = pathlib.Path(__file__).resolve().parent.parent

# Le fournisseur dont on connaît la réponse. Il ne teste pas Google : il teste
# **la sonde**, en prouvant que le chemin `webapp` atteint bien un fournisseur.
TEMOIN = "google"


def declares() -> dict[str, bool]:
    """Ce que le site déclare, relevé dans la source plutôt que redit ici."""
    source = (RACINE / "src/interface/compte_public.rs").read_text(encoding="utf-8")
    bloc = source.split("pub fn disponible")[1].split("\n}")[0]
    trouves = dict(
        (m.group(1).lower(), m.group(2) == "true")
        for m in re.finditer(r"Fournisseur::(\w+) => (true|false)", bloc)
    )
    if not trouves:
        sys.exit("  impossible de relever `disponible` — la forme du match a changé")
    return trouves


def sonder(fournisseur: str, origine: str) -> tuple[int, str]:
    corps = json.dumps(
        {
            "code": "sonde-invalide",
            "redirect_uri": RETOUR,
            "code_verifier": "x",
            "origine": origine,
        }
    ).encode()
    requete = urllib.request.Request(
        f"{BACKEND}/auth/{fournisseur}",
        data=corps,
        headers={"content-type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(requete, timeout=25) as r:
            return r.status, r.read().decode()
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode()
    except OSError as e:
        sys.exit(f"  le backend est injoignable : {e}")


def configure(code: int) -> bool:
    """403 et 401 disent tous deux que la requête est partie ; 503 qu'elle n'a pas."""
    return code != 503


def main() -> int:
    attendus = declares()

    code, _ = sonder(TEMOIN, "webapp")
    if not configure(code):
        sys.exit(
            f"  le témoin {TEMOIN} rend {code} sur l'origine webapp.\n"
            "  Ce n'est pas un fournisseur qui manque, c'est le chemin webapp\n"
            "  lui-même qui ne répond plus — ou la sonde qui vise à côté.\n"
            "  Ne rien conclure des autres relevés tant que ceci n'est pas réglé."
        )
    print(f"  témoin {TEMOIN} : {code} — le chemin webapp atteint le fournisseur\n")

    ecarts = []
    for nom in sorted(attendus):
        code, corps = sonder(nom, "webapp")
        servi = configure(code)
        etat = "servi" if servi else "ABSENT du backend"
        allume = "allumé" if attendus[nom] else "éteint"
        marque = " " if servi == attendus[nom] else "✗"
        print(f"  {marque} {nom:8} {code}  {etat:18} · le site le dit {allume}")
        if servi and not attendus[nom]:
            ecarts.append(
                f"  {nom} est servi par le backend et éteint sur le site — "
                "il peut être rallumé"
            )
        elif not servi and attendus[nom]:
            ecarts.append(
                f"  {nom} est allumé sur le site et ABSENT du backend — "
                "un lecteur qui clique tombe sur une erreur"
            )

    if ecarts:
        print()
        print("\n".join(ecarts))
        return 1
    print("\n  le site et le backend disent la même chose")
    return 0


if __name__ == "__main__":
    sys.exit(main())
