#!/usr/bin/env bash

# Ouvre une page du site dans le simulateur iOS et en capture l'écran.
#
#     ./scripts/sim.sh /fr                      → /tmp/ont-sim/fr.png
#     ./scripts/sim.sh /fr/l-auteur auteur.png
#     SIM="Web iPadOS" ./scripts/sim.sh /fr     → la même page en large
#
# ## Deux appareils, parce qu'il y a deux mises en page
#
# **« Web »** est un iPhone : c'est le format responsif, sous `lg`.
# **« Web iPadOS »** est un iPad, et c'est le plus proche d'un bureau qu'on
# ait — au-delà de 1024 px, donc là où la barre latérale remplace les onglets
# et où le site compose sur deux colonnes.
#
# L'auteur a créé le second le 2 octobre 2026, et il manquait : toutes les
# mesures de cet outil s'étaient faites sur un écran de téléphone, donc **sous
# `lg`**, en concluant chaque fois sur le site entier. ==Un banc qui ne peut pas
# prendre la taille du cas mesuré ne mesure que l'autre.==
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

# On prend le simulateur que `SIM` nomme — « Web » par défaut —, sinon le
# premier démarré : les autres portent l'app ONT, et y ouvrir Safari les
# sortirait de leur état.
APPAREIL=$(SIM="${SIM:-Web}" xcrun simctl list devices booted -j \
  | python3 -c '
import json, os, sys
appareils = [a for liste in json.load(sys.stdin)["devices"].values() for a in liste]
voulu = os.environ.get("SIM", "Web")
nomme = [a for a in appareils if a["name"] == voulu]
if not nomme and voulu != "Web":
    raise SystemExit(f"aucun simulateur démarré nommé « {voulu} »")
choisi = nomme or appareils
if not choisi:
    raise SystemExit("aucun simulateur démarré")
print(choisi[0]["udid"])')

mkdir -p "$(dirname "$SORTIE")"

# Safari est fermé avant chaque capture. Sans ça, un menu resté ouvert d'une
# session précédente — le panneau « aA », une feuille de partage — se retrouve
# au milieu de la capture, et on croit avoir photographié la page.
xcrun simctl terminate "$APPAREIL" com.apple.mobilesafari >/dev/null 2>&1 || true
sleep 1

xcrun simctl openurl "$APPAREIL" "$SERVEUR$CHEMIN"

# On attend que la page soit posée. Six secondes parce que Safari démarre à
# froid — on vient de le fermer — et qu'il lui faut ensuite charger le WASM et
# les fontes. À trois secondes, une capture sur deux était noire.
sleep "${ATTENTE:-6}"
xcrun simctl io "$APPAREIL" screenshot "$SORTIE" >/dev/null 2>&1

# Une capture presque entièrement noire est une page qui n'a pas fini de
# charger, pas une page sombre : le site n'a aucun écran vide.
python3 - "$SORTIE" <<'PY'
import sys
from PIL import Image
import numpy as np
a = np.asarray(Image.open(sys.argv[1]).convert("L"))
if a.mean() < 8:
    print("  ⚠ capture quasi noire — la page n'avait pas fini de charger", file=sys.stderr)
PY

printf '%s → %s\n' "$CHEMIN" "$SORTIE"
