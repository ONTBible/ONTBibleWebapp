#!/usr/bin/env bash
#
# Engendre `.cargo/config.toml` pour que le linker de LLVM remplace celui
# d'Apple, en développement local.
#
#   ./scripts/linker-local.sh
#
# ## Pourquoi
#
# Le site embarque tout `dist/` par `include_str!` (§8 bis) — livres, plan,
# glossaire et occurrences, près de deux mégaoctets de données statiques dans
# un seul crate. Passé un seuil, `ld` d'Apple ne rend pas une erreur : il
# **plante**, dans `ld::FixupFromRelocs::arm64_b26`, ou refuse avec
# « cannot encode offset of relocations; object file too large ».
#
# Ce n'est pas notre code, et ce n'est pas nouveau : le §8 quater note déjà que
# `cargo leptos build --release` casse ce même linker. Le défaut était masqué
# en développement par la compilation incrémentale — il ne se découvre qu'au
# premier `cargo clean`, et l'on croit alors avoir cassé quelque chose.
#
# ## Pourquoi un script, et pas un fichier committé
#
# Le chemin de `ld64.lld` contient le nom de la toolchain installée. L'écrire
# en dur dans un fichier versionné le rendrait faux sur toute autre machine —
# et faux **en silence**, puisque cargo dirait seulement « linker introuvable ».
# Le script le résout à l'exécution ; `.cargo/config.toml` est ignoré par git.
#
# Ne touche ni la CI (Ubuntu, linker GNU) ni le déploiement (`cargo lambda` et
# zig, qui croise-compile vers Linux).

set -euo pipefail
cd "$(dirname "$0")/.."

CIBLE="$(rustc -vV | awk '/^host: / {print $2}')"
LLD="$(rustc --print sysroot)/lib/rustlib/${CIBLE}/bin/gcc-ld/ld64.lld"

if [[ ! -x "$LLD" ]]; then
  echo "ld64.lld introuvable — attendu à $LLD" >&2
  echo "il est livré avec la toolchain Rust ; vérifier « rustup component list »" >&2
  exit 1
fi

mkdir -p .cargo
cat > .cargo/config.toml <<TOML
# Engendré par scripts/linker-local.sh — ne pas éditer à la main.
#
# Le linker d'Apple plante sur le corpus embarqué ; celui de LLVM assemble le
# même binaire. Réglage de poste de travail : ni la CI ni le déploiement n'y
# passent.
[target.${CIBLE}]
rustflags = ["-C", "link-arg=-fuse-ld=${LLD}"]
TOML

echo "linker → $LLD"
