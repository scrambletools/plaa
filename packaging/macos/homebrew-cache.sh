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
    # One not installed, such as one macOS provides, is left out: brew
    # install builds it again after a restore, which only costs time.
    kegs=()
    : >"$dir/formulae.tsv"
    while IFS=$'\t' read -r name version keg_only; do
        if [ -d "$cellar/$name/$version" ]; then
            kegs+=("$name/$version")
            printf '%s\t%s\t%s\n' "$name" "$version" "$keg_only" >>"$dir/formulae.tsv"
        else
            echo "::warning::$name $version is not installed, so not kept"
        fi
    done < <(formulae "$@")
    [ ${#kegs[@]} -gt 0 ] || {
        echo "none of the formulae is installed" >&2
        exit 1
    }
    tar -czf "$dir/kegs.tar.gz" -C "$cellar" "${kegs[@]}"
    echo "kept ${#kegs[@]} kegs, $(du -h "$dir/kegs.tar.gz" | cut -f1)"
    ;;
restore)
    dir=${1:?usage: homebrew-cache.sh restore DIR}
    prefix=$(brew --prefix)
    tar -xzf "$dir/kegs.tar.gz" -C "$(brew --cellar)"
    mkdir -p "$prefix/opt"
    while IFS=$'\t' read -r name version keg_only; do
        # The runner may have an older version of it linked. brew unlink
        # unlinks the version the opt path names, so this comes first.
        if [ "$keg_only" = false ]; then
            brew unlink "$name" >/dev/null 2>&1 || true
        fi
        # Libraries name the others by their opt paths.
        ln -sfn "../Cellar/$name/$version" "$prefix/opt/$name"
        if [ "$keg_only" = false ]; then
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
