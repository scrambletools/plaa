#!/usr/bin/env bash
# Installs a built plaa and its data files into a system layout, for
# packages: plaa, the desktop entry, icons, AppStream metadata and
# licenses, and the check the packages run as they install for an
# interface that can send AVB streams. AVB comes from avb-endpointd, a
# package of its own: the daemon, avb-netraw, which opens raw sockets for
# plaa's controller, avb-link@, and gptpd with it.
#
#   scripts/dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]
#
# BINARY_DIR holds plaa from a release build made with PLAA_PRODUCTION=1.
# DESTDIR is the staging root (empty for a direct install) and PREFIX
# defaults to /usr. plaa holds no capability.
set -euo pipefail

binaries=${1:?usage: dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]}
destdir=${2:-}
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/.." && pwd)
id=run.plaa
share="$destdir$prefix/share"

install -Dm755 "$binaries/plaa" "$destdir$prefix/bin/plaa"
install -Dm755 "$root/packaging/linux/avb-check" "$destdir$prefix/lib/plaa/avb-check"
install -Dm644 "$root/data/$id.desktop" "$share/applications/$id.desktop"
install -Dm644 "$root/data/$id.metainfo.xml" "$share/metainfo/$id.metainfo.xml"
(cd "$root/data/icons" && find hicolor -type f) | while read -r icon; do
    install -Dm644 "$root/data/icons/$icon" "$share/icons/$icon"
done
install -Dm644 "$root/LICENSE" "$share/licenses/plaa/LICENSE"
install -Dm644 "$root/docs/THIRD-PARTY.md" "$share/licenses/plaa/THIRD-PARTY.md"
for font_license in OFL.txt LICENSE-MaterialSymbols.txt; do
    install -Dm644 "$root/packaging/licenses/$font_license" \
        "$share/licenses/plaa/fonts/$font_license"
done
install -Dm644 "$root/README.md" "$share/doc/plaa/README.md"
