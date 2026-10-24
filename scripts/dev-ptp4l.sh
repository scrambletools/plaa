#!/usr/bin/env bash
# Installs plaa's ptp4l services for development builds, which no package
# set up, so a dev build starts ptp4l where none runs on its interface,
# as an installed plaa does: plaa-ptp4l@ and plaa-ptp4l-lite@ in
# /usr/local/lib/systemd/system, their configurations in
# /usr/local/share/plaa, and the polkit rule letting the person at the
# computer start them in /etc/polkit-1/rules.d. The units shadow a
# package's, so remove them before installing plaa's package. Needs sudo,
# and ptp4l from linuxptp.
#
#   scripts/dev-ptp4l.sh           installs them
#   scripts/dev-ptp4l.sh remove    stops and removes them
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
ptp4l=$root/packaging/linux/ptp4l
units=/usr/local/lib/systemd/system
configs=/usr/local/share/plaa
rule=/etc/polkit-1/rules.d/50-plaa-ptp4l.rules
services=(plaa-ptp4l@.service plaa-ptp4l-lite@.service)

case ${1:-install} in
    install)
        for folder in /usr/lib/systemd/system /lib/systemd/system; do
            if [ -e "$folder/plaa-ptp4l@.service" ]; then
                echo "dev-ptp4l.sh: plaa's package installed them already, in $folder" >&2
                exit 0
            fi
        done
        if ! command -v ptp4l >/dev/null; then
            echo "dev-ptp4l.sh: ptp4l is missing; install linuxptp" >&2
            exit 1
        fi
        work=$(mktemp -d)
        trap 'rm -rf "$work"' EXIT
        for service in "${services[@]}"; do
            # The configurations from /usr/local, not the package's folder.
            sed "s,/usr/share/plaa/,$configs/," "$ptp4l/$service" >"$work/$service"
            sudo install -Dm644 "$work/$service" "$units/$service"
        done
        sudo install -Dm644 "$ptp4l/ptp4l-gptp.cfg" "$configs/ptp4l-gptp.cfg"
        sudo install -Dm644 "$ptp4l/ptp4l-lite.cfg" "$configs/ptp4l-lite.cfg"
        sudo install -Dm644 "$ptp4l/50-plaa-ptp4l.rules" "$rule"
        sudo systemctl daemon-reload
        echo "Installed plaa's ptp4l services for dev builds; 'scripts/dev-ptp4l.sh remove' takes them out"
        ;;
    remove)
        sudo systemctl stop 'plaa-ptp4l@*.service' 'plaa-ptp4l-lite@*.service' || true
        for service in "${services[@]}"; do
            sudo rm -f "$units/$service"
        done
        sudo rm -f "$configs/ptp4l-gptp.cfg" "$configs/ptp4l-lite.cfg" "$rule"
        sudo rmdir "$configs" 2>/dev/null || true
        sudo systemctl daemon-reload
        echo "Removed plaa's ptp4l services for dev builds"
        ;;
    *)
        echo "usage: dev-ptp4l.sh [install | remove]" >&2
        exit 2
        ;;
esac
