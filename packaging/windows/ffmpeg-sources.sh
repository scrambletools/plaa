#!/usr/bin/env bash
# Collects the sources of the FFmpeg build the Windows packages carry, as
# its GPL asks: FFmpeg's at its commit, BtbN's build scripts at theirs,
# and each library those scripts build for the targets plaa ships,
# fetched as the scripts fetch them, each packed as a tarball.
#
#   FFMPEG_COMMIT=... FFMPEG_BUILDS_COMMIT=... \
#       packaging/windows/ffmpeg-sources.sh OUT
#
# FFMPEG_TARGETS (win64 winarm64) and FFMPEG_VARIANT (gpl-shared) name
# the builds. It needs git, svn, wget, curl, python3 and the autotools
# (autoconf, automake, libtool), as BtbN's download stage does: some
# scripts prepare their sources as they fetch them.
set -euo pipefail
shopt -s globstar

out=$(mkdir -p "${1:?usage: ffmpeg-sources.sh OUT}" && cd "$1" && pwd)
: "${FFMPEG_COMMIT:?}" "${FFMPEG_BUILDS_COMMIT:?}"
targets=${FFMPEG_TARGETS:-win64 winarm64}
variant=${FFMPEG_VARIANT:-gpl-shared}
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

fetch() { curl -sSLf --retry 3 -o "$1" "$2"; }
fetch "$out/ffmpeg-$FFMPEG_COMMIT.tar.gz" \
    "https://github.com/FFmpeg/FFmpeg/archive/$FFMPEG_COMMIT.tar.gz"
fetch "$out/ffmpeg-builds-$FFMPEG_BUILDS_COMMIT.tar.gz" \
    "https://github.com/BtbN/FFmpeg-Builds/archive/$FFMPEG_BUILDS_COMMIT.tar.gz"
mkdir "$work/builds" "$work/bin" "$work/src"
tar -xzf "$out/ffmpeg-builds-$FFMPEG_BUILDS_COMMIT.tar.gz" -C "$work/builds" --strip-components=1

# BtbN's download helpers, which its scripts' download commands call.
for helper in git-mini-clone retry-tool check-wget; do
    install -m755 "$work/builds/images/base/$helper.sh" "$work/bin/$helper"
done
export PATH="$work/bin:$PATH"
cd "$work/builds"

# Each script's download commands for each target it is enabled for, as
# generate.sh picks them, the same commands once.
commands() {
    (
        source util/vars.sh "$1" "$variant" >/dev/null
        source util/dl_functions.sh
        source "$2"
        ffbuild_enabled || exit 0
        ffbuild_dockerdl
    ) 2>/dev/null || true
}
declare -A fetched
for script in scripts.d/**/*.sh; do
    name=$(basename "$script" .sh)
    for target in $targets; do
        lines=$(commands "$target" "$script")
        [ -n "$lines" ] && [ -z "${fetched[$lines]:-}" ] || continue
        fetched[$lines]=1
        dir="$work/src/$name"
        [ -d "$dir" ] && dir="$work/src/$name-$target"
        mkdir -p "$dir"
        echo "== $name ($target)"
        # Some scripts prepare their sources after fetching them, as with
        # autoreconf; where that fails, what was fetched is still the
        # source.
        if ! (cd "$dir" && bash -e -c "$lines") >"$work/fetch.log" 2>&1; then
            if [ -z "$(ls -A "$dir")" ]; then
                cat "$work/fetch.log" >&2
                echo "ffmpeg-sources.sh: could not fetch $name" >&2
                exit 1
            fi
            echo "ffmpeg-sources.sh: $name fetched, but preparing it failed:" \
                "$(tail -1 "$work/fetch.log")" >&2
        fi
        tar -C "$work/src" --exclude=.git --exclude=.svn -czf "$out/$(basename "$dir").tar.gz" "$(basename "$dir")"
        rm -rf "$dir"
    done
done
du -sh "$out"
