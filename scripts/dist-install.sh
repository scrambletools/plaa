#!/usr/bin/env bash
# Installs a built plaa and plaa-netraw and their data files into a
# system layout, for packages: plaa, plaa-netraw beside it in plaa's
# library folder, the one program granted CAP_NET_RAW, the desktop entry,
# icons, AppStream metadata and licenses, the check the packages run as
# they install for an interface that can send AVB streams, and plaa's
# ptp4l service, its gPTP configuration and the polkit rule letting the
# person at the computer start it.
#
#   scripts/dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]
#
# BINARY_DIR holds both programs from a release build made with
# PLAA_PRODUCTION=1. DESTDIR is the staging root (empty for a direct
# install) and PREFIX defaults to /usr. Packages then grant plaa-netraw
# CAP_NET_RAW when installed; plaa holds no capability, its raw sockets
# coming from it.
set -euo pipefail

binaries=${1:?usage: dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]}
destdir=${2:-}
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/.." && pwd)
id=run.plaa
share="$destdir$prefix/share"

install -Dm755 "$binaries/plaa" "$destdir$prefix/bin/plaa"
install -Dm755 "$binaries/plaa-netraw" "$destdir$prefix/lib/plaa/plaa-netraw"
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
ptp4l=$root/packaging/linux/ptp4l
install -Dm644 "$ptp4l/ptp4l-gptp.cfg" "$share/plaa/ptp4l-gptp.cfg"
install -Dm644 "$ptp4l/plaa-ptp4l@.service" "$destdir$prefix/lib/systemd/system/plaa-ptp4l@.service"
install -Dm644 "$ptp4l/ptp4l-lite.cfg" "$share/plaa/ptp4l-lite.cfg"
install -Dm644 "$ptp4l/plaa-ptp4l-lite@.service" "$destdir$prefix/lib/systemd/system/plaa-ptp4l-lite@.service"
install -Dm644 "$ptp4l/50-plaa-ptp4l.rules" "$share/polkit-1/rules.d/50-plaa-ptp4l.rules"
