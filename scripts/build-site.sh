#!/usr/bin/env bash
# Assembles plaa's website (plaa.run) into OUT: the pages in site/, in
# every language the app speaks, plus the icon, from the repository.
#
#   scripts/build-site.sh [OUT]
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=${1:-$root/target/site}

rm -rf "$out"
mkdir -p "$out"
cp -r "$root"/site/. "$out/"
# The translations go into the pages rather than beside them.
rm -rf "$out/i18n"
python3 "$root/scripts/site-i18n.py" build "$out"
cp "$root/data/icons/hicolor/scalable/apps/run.plaa.svg" "$out/icon.svg"
echo "Built the site in $out"
