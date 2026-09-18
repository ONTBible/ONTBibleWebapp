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
# ## Et depuis macOS 27, il faut aussi lui donner un SDK qu'il sache lire
#
# Les SDK de macOS 27 déclarent une architecture que `ld64.lld` ne connaît pas :
#
#     targets: [ …, arm64e.x1-macos, arm64e.x1-maccatalyst ]
#     rust-lld: could not load TAPI file … CoreFoundation.tbd: malformed file
#                                          4:54: error: unknown architecture
#
# Les deux moitiés se referment l'une sur l'autre, et c'est ce qui rend le
# diagnostic trompeur : sans ce script, le linker d'Apple **plante** sur l'objet
# géant ; avec lui mais sans le SDK, `rust-lld` **refuse** le SDK. On croit alors
# que le script ne sert plus, et l'on essaie de s'en passer — ce qui ramène au
# premier plantage, sous une autre trace.
#
# Le SDK n'est pas écrit en dur : on prend **le plus récent qui ne porte pas le
# jeton**, mesuré dans le fichier lui-même. Le jour où Rust embarquera un `lld`
# qui lit les SDK de macOS 27, cette moitié pourra tomber — et son retrait se
# signalera par l'erreur ci-dessus.
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

# Le plus récent SDK que `ld64.lld` sache lire — celui dont le `CoreFoundation.tbd`
# ne déclare pas `arm64e.x1`. On regarde le fichier, on ne devine pas d'après le
# numéro de version.
SDK=""
for candidat in $(ls -d \
      /Library/Developer/CommandLineTools/SDKs/MacOSX*.sdk \
      /Applications/Xcode*.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX*.sdk \
      2>/dev/null | sort -rV); do
  TBD="$candidat/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation.tbd"
  [[ -f "$TBD" ]] || continue
  if ! grep -q "arm64e\.x1" "$TBD"; then
    SDK="$candidat"
    break
  fi
done

if [[ -z "$SDK" ]]; then
  echo "aucun SDK lisible par ld64.lld — tous déclarent arm64e.x1" >&2
  echo "installer les Command Line Tools d'une version antérieure, ou vérifier" >&2
  echo "si la toolchain Rust embarque désormais un lld qui les lit" >&2
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

# Le SDK que ce linker sait lire. \`force = false\` : une session qui pose son
# propre \`SDKROOT\` garde le sien — les sessions iOS compilent contre la bêta.
[env]
SDKROOT = { value = "${SDK}", force = false }
TOML

echo "linker → $LLD"
echo "SDK    → $SDK"
