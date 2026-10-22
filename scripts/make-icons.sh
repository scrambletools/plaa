#!/usr/bin/env bash
# Renders plaa's icons from the scalable SVG the Linux packages install:
# the hicolor PNGs, the 1024-pixel PNG the macOS bundle takes, and the
# Windows .ico. Run it again whenever the SVG changes. Needs rsvg-convert
# (librsvg) and ImageMagick.
#
#   scripts/make-icons.sh
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
id=run.plaa
hicolor=$root/data/icons/hicolor
svg=$hicolor/scalable/apps/$id.svg
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

for size in 16 22 24 32 48 64 96 128 256 512; do
  mkdir -p "$hicolor/${size}x$size/apps"
  rsvg-convert -w "$size" -h "$size" "$svg" -o "$hicolor/${size}x$size/apps/$id.png"
done

rsvg-convert -w 1024 -h 1024 "$svg" -o "$root/data/icons/plaa-1024.png"

ico=()
for size in 16 20 24 32 40 48 64 96 128 256; do
  rsvg-convert -w "$size" -h "$size" "$svg" -o "$work/$size.png"
  ico+=("$work/$size.png")
done
mkdir -p "$root/packaging/windows"
magick "${ico[@]}" "$root/packaging/windows/plaa.ico"

echo "Rendered the icons from $svg"
