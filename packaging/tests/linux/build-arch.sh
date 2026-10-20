#!/usr/bin/env bash
# Builds the Arch package from packaging/arch/plaa, as someone using the
# PKGBUILD would, but from SOURCE, an archive of this tree, rather than
# the tag's, in a fresh Arch container. run.sh then tests the package.
#
#   git archive --prefix=plaa-0.1.0/ -o plaa-0.1.0.tar.gz HEAD
#   docker run --rm -v "$PWD:/work" -w /work archlinux:latest \
#       packaging/tests/linux/build-arch.sh plaa-0.1.0.tar.gz dist-arch
#
# The archive's name and folder must carry the PKGBUILD's pkgver. A
# checkout of scrambletools/avb mounted at /avb stands in for GitHub's,
# as for a revision not pushed yet.
set -euo pipefail

source_archive=$(readlink -f "${1:?usage: build-arch.sh SOURCE OUT}")
out=${2:?usage: build-arch.sh SOURCE OUT}
recipe=$(cd "$(dirname "$0")/../../arch/plaa" && pwd)

pacman -Syu --noconfirm --needed base-devel
# makepkg runs as a regular user, so the dependencies go in first.
# shellcheck source=/dev/null disable=SC2154
(. "$recipe/PKGBUILD" && pacman -S --noconfirm --needed "${depends[@]}" "${makedepends[@]}")
useradd -m builder
if [ -d /avb ]; then
    export GIT_CONFIG_COUNT=2 GIT_CONFIG_KEY_0=url.file:///avb.insteadOf
    export GIT_CONFIG_VALUE_0=https://github.com/scrambletools/avb
    export GIT_CONFIG_KEY_1=safe.directory GIT_CONFIG_VALUE_1='*'
    export CARGO_NET_GIT_FETCH_WITH_CLI=true
fi
build=$(mktemp -d)
cp "$recipe/PKGBUILD" "$recipe/plaa.install" "$source_archive" "$build/"
chown -R builder "$build"
mkdir -p "$out"
(cd "$build" && runuser -u builder -- makepkg --nocheck --noconfirm)
cp "$build"/plaa-[0-9]*.pkg.tar.* "$out/"
ls -l "$out"
