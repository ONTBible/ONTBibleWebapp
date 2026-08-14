#!/usr/bin/env bash
# Le mode `watch` régénère `target/site/pkg/ontbible.{css,js,wasm}` mais **pas**
# les copies empreintées — et c'est celles-là que le serveur écrit dans le HTML,
# puisqu'il démarre avec les empreintes de `target/debug/hash.txt`.
#
# Conséquence : en développement, la page servie porte les assets du dernier
# **redémarrage complet**, pas ceux du dernier enregistrement. On corrige un
# jeton, on recharge, rien ne bouge, et l'on croit que la correction est fausse.
#
# Ce script recopie les frais sur les empreintés. À lancer avant toute
# vérification visuelle.
set -euo pipefail
cd "$(dirname "$0")/.."
pkg=target/site/pkg
while read -r cle valeur; do
  ext="${cle%:}"
  [ -f "$pkg/ontbible.$ext" ] || continue
  cp "$pkg/ontbible.$ext" "$pkg/ontbible.$valeur.$ext"
  echo "ontbible.$ext → ontbible.$valeur.$ext"
done < target/debug/hash.txt
