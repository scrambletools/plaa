#!/usr/bin/env bash
# Installs plaa-link@ for development builds, which no package set up, so
# a dev build keeps its stream's frames on one transmit queue as an
# installed plaa does: the unit in /usr/local/lib/systemd/system and the
# polkit rule letting the person at the computer run it in
# /etc/polkit-1/rules.d. It also takes out the ptp4l services an older
# dev-ptp4l.sh put in, which would keep gptpd off their interface. The
# unit shadows a package's, so remove it before installing plaa's
# package. Needs sudo. gptpd, which keeps the network time, comes from
# its own package or its repository's dev install.
#
#   scripts/dev-link.sh           installs it
#   scripts/dev-link.sh remove    removes it
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
link=$root/packaging/linux/link
units=/usr/local/lib/systemd/system
rule=/etc/polkit-1/rules.d/50-plaa-link.rules

# What dev-ptp4l.sh installed before gptpd.
remove_ptp4l() {
    local old=(plaa-ptp4l@.service plaa-ptp4l-lite@.service) found=
    for service in "${old[@]}"; do
        [ -e "$units/$service" ] && found=1
    done
    [ -n "$found" ] || [ -e /etc/polkit-1/rules.d/50-plaa-ptp4l.rules ] || return 0
    sudo systemctl stop 'plaa-ptp4l@*.service' 'plaa-ptp4l-lite@*.service' || true
    for service in "${old[@]}"; do
        sudo rm -f "$units/$service"
    done
    sudo rm -f /usr/local/share/plaa/ptp4l-gptp.cfg /usr/local/share/plaa/ptp4l-lite.cfg \
        /etc/polkit-1/rules.d/50-plaa-ptp4l.rules
    sudo rmdir /usr/local/share/plaa 2>/dev/null || true
    echo "Removed plaa's old ptp4l services for dev builds"
}

case ${1:-install} in
    install)
        remove_ptp4l
        for folder in /usr/lib/systemd/system /lib/systemd/system; do
            if [ -e "$folder/plaa-link@.service" ]; then
                echo "dev-link.sh: plaa's package installed it already, in $folder" >&2
                sudo systemctl daemon-reload
                exit 0
            fi
        done
        sudo install -Dm644 "$link/plaa-link@.service" "$units/plaa-link@.service"
        sudo install -Dm644 "$link/50-plaa-link.rules" "$rule"
        sudo systemctl daemon-reload
        echo "Installed plaa-link@ for dev builds; 'scripts/dev-link.sh remove' takes it out"
        ;;
    remove)
        remove_ptp4l
        sudo rm -f "$units/plaa-link@.service" "$rule"
        sudo systemctl daemon-reload
        echo "Removed plaa-link@ for dev builds"
        ;;
    *)
        echo "usage: dev-link.sh [install | remove]" >&2
        exit 2
        ;;
esac
