#!/usr/bin/env bash
# Builds an Arch package plaa depends on that Arch does not carry,
# avb-endpointd's, from the PKGBUILD and install file its release
# carries, in a fresh Arch container, for test-package.sh to put in
# before plaa's. The PKGBUILD fetches its own source.
#
#   docker run --rm -v "$PWD:/work" -w /work archlinux:latest \
#       packaging/tests/linux/build-arch-first.sh RECIPE OUT
#
# RECIPE holds the PKGBUILD and its install file; OUT gets the package.
set -euo pipefail

recipe=$(readlink -f "${1:?usage: build-arch-first.sh RECIPE OUT}")
out=${2:?usage: build-arch-first.sh RECIPE OUT}

pacman -Syu --noconfirm --needed base-devel
# makepkg runs as a regular user, so the dependencies go in first; gptpd,
# which Arch does not carry either, the build does not need.
# shellcheck source=/dev/null disable=SC2154
(. "$recipe/PKGBUILD" &&
    printf '%s\n' "${depends[@]}" "${makedepends[@]}" | grep -vx gptpd |
    xargs pacman -S --noconfirm --needed)
id builder >/dev/null 2>&1 || useradd -m builder
build=$(mktemp -d)
cp "$recipe"/* "$build/"
chown -R builder "$build"
mkdir -p "$out"
(cd "$build" && runuser -u builder -- makepkg --nocheck --noconfirm --nodeps)
cp "$build"/*.pkg.tar.* "$out/"
ls -l "$out"
