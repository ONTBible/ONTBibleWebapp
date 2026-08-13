#!/usr/bin/env bash

# Ouvre une page du site dans le simulateur iOS et en capture l'écran.
#
#     ./scripts/sim.sh /fr                      → /tmp/ont-sim/fr.png
#     ./scripts/sim.sh /fr/l-auteur auteur.png
#
# ## Pourquoi cet outil compte
#
# `scripts/apercu.py` rend les pages avec QuickLook, ce qui suffit à juger une
# composition — mais **pas le responsive** : QuickLook rend à une fenêtre fixe
# puis réduit l'image, donc les requêtes média voient toujours un grand écran.
# Toutes les vérifications « mobile » faites avec lui étaient sans valeur, et
# une bande qui réapparaissait sur téléphone est passée à travers.
#
# Le simulateur, lui, est un vrai Safari sur un vrai iPhone : ce qu'il montre
# est ce que voit un lecteur. C'est la seule vérification qui vaille pour tout
# ce qui dépend de la largeur.
#
# Le serveur de développement doit tourner (`cargo leptos watch`). Le
# simulateur partage le réseau de l'hôte, donc `127.0.0.1:3000` lui répond.

set -euo pipefail

CHEMIN="${1:-/fr}"
SORTIE="${2:-/tmp/ont-sim/$(echo "${CHEMIN#/}" | tr '/' '-' | sed 's/^$/accueil/').png}"
SERVEUR="http://127.0.0.1:3000"

# On prend le simulateur nommé « Web » s'il existe, sinon le premier démarré :
# les autres portent l'app ONT, et y ouvrir Safari les sortirait de leur état.
APPAREIL=$(xcrun simctl list devices booted -j \
  | python3 -c '
import json, sys
appareils = [a for liste in json.load(sys.stdin)["devices"].values() for a in liste]
web = [a for a in appareils if a["name"] == "Web"]
choisi = (web or appareils)
if not choisi:
    raise SystemExit("aucun simulateur démarré")
print(choisi[0]["udid"])')

mkdir -p "$(dirname "$SORTIE")"

xcrun simctl openurl "$APPAREIL" "$SERVEUR$CHEMIN"
# Safari a besoin d'un instant pour charger et poser les fontes ; sans cette
# pause, la capture attrape la page en fonte de repli.
sleep 3
xcrun simctl io "$APPAREIL" screenshot "$SORTIE" >/dev/null 2>&1

printf '%s → %s\n' "$CHEMIN" "$SORTIE"
