#!/bin/bash
# Keeps Homebrew's builds of FORMULAE, and of each formula they need to
# run, for CI on Intel Macs, where Homebrew has made no bottles since
# 7.0 and builds every one from source: it takes hours, so each set of
# versions is built once and cached.
#
#   packaging/macos/homebrew-cache.sh key FORMULAE...
#   packaging/macos/homebrew-cache.sh save DIR FORMULAE...
#   packaging/macos/homebrew-cache.sh restore DIR
#
# key prints a hash of the formulae's names and versions, after brew
# update, to name the cache by. save puts their installed kegs in DIR,
# with a list of them. restore puts the kegs from DIR into Homebrew's
# Cellar and links them as brew install would; brew install then finds
# them installed.
set -euo pipefail

# Each formula and each it needs to run, one per line: its name, version
# and whether it is keg-only.
formulae() {
    local names
    names=$( (printf '%s\n' "$@"; brew deps --union "$@") | sort -u)
    # shellcheck disable=SC2086
    brew info --json=v2 $names | jq -r '.formulae[] |
        [.name, (.versions.stable + (if .revision > 0 then "_\(.revision)" else "" end)), .keg_only] | @tsv'
}

command=${1:?usage: homebrew-cache.sh key|save|restore ...}
shift
case $command in
key)
    formulae "$@" | shasum -a 256 | cut -c1-16
    ;;
save)
    dir=${1:?usage: homebrew-cache.sh save DIR FORMULAE...}
    shift
    mkdir -p "$dir"
    cellar=$(brew --cellar)
    formulae "$@" >"$dir/formulae.tsv"
    kegs=()
    while IFS=$'\t' read -r name version _; do
        [ -d "$cellar/$name/$version" ] || {
            echo "$name $version is not installed" >&2
            exit 1
        }
        kegs+=("$name/$version")
    done <"$dir/formulae.tsv"
    tar -czf "$dir/kegs.tar.gz" -C "$cellar" "${kegs[@]}"
    echo "kept ${#kegs[@]} kegs, $(du -h "$dir/kegs.tar.gz" | cut -f1)"
    ;;
restore)
    dir=${1:?usage: homebrew-cache.sh restore DIR}
    prefix=$(brew --prefix)
    tar -xzf "$dir/kegs.tar.gz" -C "$(brew --cellar)"
    mkdir -p "$prefix/opt"
    while IFS=$'\t' read -r name version keg_only; do
        # Libraries name the others by their opt paths.
        ln -sfn "../Cellar/$name/$version" "$prefix/opt/$name"
        if [ "$keg_only" = false ]; then
            # The runner may have an older version of it linked.
            brew unlink "$name" >/dev/null 2>&1 || true
            brew link --overwrite "$name" >/dev/null
        fi
    done <"$dir/formulae.tsv"
    echo "restored $(wc -l <"$dir/formulae.tsv" | tr -d ' ') kegs"
    ;;
*)
    echo "unknown command: $command" >&2
    exit 2
    ;;
esac
