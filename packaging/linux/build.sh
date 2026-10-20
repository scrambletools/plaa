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
# named for the image, so a second build is quicker. PLAA_AVB_CHECKOUT,
# a checkout of scrambletools/avb, stands in for GitHub's, as for a
# revision not pushed yet.
set -euo pipefail

image=${1:?usage: build.sh IMAGE OUT}
out=$(mkdir -p "${2:?usage: build.sh IMAGE OUT}" && cd "$2" && pwd)
root=$(cd "$(dirname "$0")/../.." && pwd)
volume=plaa-build-$(tr -c 'a-z0-9\n' - <<<"$image")

options=(--rm --platform "linux/$(docker version -f '{{.Server.Arch}}')"
    -v "$root:/src:ro" -v "$out:/out" -v "$volume-cargo:/cargo" -v "$volume-target:/target"
    -e CARGO_HOME=/cargo -e RUSTUP_HOME=/cargo/rustup -e CARGO_TERM_COLOR=always)
if [ -n "${PLAA_AVB_CHECKOUT:-}" ]; then
    options+=(-v "$(cd "$PLAA_AVB_CHECKOUT" && pwd):/avb:ro" -e GIT_CONFIG_COUNT=2
        -e GIT_CONFIG_KEY_0=url.file:///avb.insteadOf
        -e GIT_CONFIG_VALUE_0=https://github.com/scrambletools/avb
        -e GIT_CONFIG_KEY_1=safe.directory -e GIT_CONFIG_VALUE_1='*'
        -e CARGO_NET_GIT_FETCH_WITH_CLI=true)
fi
# The packages belong to whoever built them, not to the container's root.
docker run "${options[@]}" "$image" bash -c \
    "/src/packaging/linux/build-package.sh /out && chown -R $(id -u):$(id -g) /out"
