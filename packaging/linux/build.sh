#!/usr/bin/env bash
# Builds plaa's package for one Linux release in a fresh container of it,
# with Docker: a .deb for Debian and Ubuntu, an .rpm for Fedora, each
# against that release's own FFmpeg.
#
#   packaging/linux/build.sh IMAGE OUT
#   packaging/linux/build.sh debian:13 dist
#
# The container is of this computer's architecture, whatever the image's
# tag last pulled. Cargo's downloads and the build stay in Docker volumes
# named for the image, so a second build is quicker. PLAA_CROSS, such as
# riscv64, builds a .deb for that architecture instead. PLAA_DOCKER_CPUS,
# such as 4, limits the processors the build takes, leaving room on a
# computer keeping network time with ptp4l.
set -euo pipefail

image=${1:?usage: build.sh IMAGE OUT}
out=$(mkdir -p "${2:?usage: build.sh IMAGE OUT}" && cd "$2" && pwd)
root=$(cd "$(dirname "$0")/../.." && pwd)
volume=plaa-build-$(tr -c 'a-z0-9\n' - <<<"$image")

options=(--rm --platform "linux/$(docker version -f '{{.Server.Arch}}')"
    -v "$root:/src:ro" -v "$out:/out" -v "$volume-cargo:/cargo" -v "$volume-target:/target"
    -e CARGO_HOME=/cargo -e RUSTUP_HOME=/cargo/rustup -e CARGO_TERM_COLOR=always)
# A .deb for another architecture, built on this one's (build-package.sh).
if [ -n "${PLAA_CROSS:-}" ]; then
    options+=(-e "PLAA_CROSS=$PLAA_CROSS")
fi
if [ -n "${PLAA_DOCKER_CPUS:-}" ]; then
    options+=(--cpus "$PLAA_DOCKER_CPUS")
fi
# The packages belong to whoever built them, not to the container's root.
docker run "${options[@]}" "$image" bash -c \
    "/src/packaging/linux/build-package.sh /out && chown -R $(id -u):$(id -g) /out"
