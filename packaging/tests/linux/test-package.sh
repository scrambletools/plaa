#!/usr/bin/env bash
# Installs a plaa package in a fresh container, as root, and checks it as
# someone installing it would meet it: its dependencies alone are enough,
# the files and capability it sets up, the program and the FFmpeg it
# finds, raw Ethernet as a regular user, the window under a headless
# Wayland compositor, a video played in it, and what removing it leaves
# behind. run.sh beside it starts the container.
#
#   test-package.sh PACKAGE [PREVIOUS]
#
# PACKAGE is a .deb, .rpm or Arch package. PREVIOUS, an older package of
# the same kind, goes in first, so PACKAGE upgrades it.
# PLAA_TEST_EMULATED=1, for an architecture running under QEMU, leaves
# out raw Ethernet and the window. PLAA_TEST_SHOTS names a directory for
# screenshots of the window.
set -euo pipefail

package=$(readlink -f "${1:?usage: test-package.sh PACKAGE [PREVIOUS]}")
previous=${2:+$(readlink -f "$2")}
emulated=${PLAA_TEST_EMULATED:-}
shots=${PLAA_TEST_SHOTS:-}
here=$(cd "$(dirname "$0")" && pwd)
id=run.plaa

section() { printf '\n== %s\n' "$*"; }
fail() {
    printf 'FAIL: %s\n' "$*" >&2
    exit 1
}

case $package in
*.deb) kind=deb ;;
*.rpm) kind=rpm ;;
*.pkg.tar.*) kind=arch ;;
*) fail "not a package: $package" ;;
esac

# Only what each package depends on, none of what it recommends, so a
# dependency left out of a package shows.
install_package() {
    case $kind in
    deb) apt-get install -y --no-install-recommends "$1" ;;
    rpm) dnf install -y --setopt=install_weak_deps=False "$1" ;;
    arch) pacman -U --noconfirm "$1" ;;
    esac
}
add_tools() {
    case $kind in
    deb) apt-get install -y --no-install-recommends "$@" ;;
    rpm) dnf install -y --setopt=install_weak_deps=False "$@" ;;
    arch) pacman -S --noconfirm --needed "$@" ;;
    esac
}
remove_package() {
    case $kind in
    deb) apt-get purge -y plaa ;;
    rpm) dnf remove -y plaa ;;
    arch) pacman -Rns --noconfirm plaa ;;
    esac
}
installed_version() {
    case $kind in
    deb) dpkg-query -W -f '${Version}' plaa ;;
    rpm) rpm -q --qf '%{VERSION}-%{RELEASE}' plaa ;;
    arch) pacman -Q plaa | cut -d' ' -f2 ;;
    esac
}
package_version() {
    case $kind in
    deb) dpkg-deb -f "$1" Version ;;
    rpm) rpm -qp --qf '%{VERSION}-%{RELEASE}' "$1" ;;
    arch) pacman -Qp "$1" | cut -d' ' -f2 ;;
    esac
}

section "$(. /etc/os-release && echo "$PRETTY_NAME"), $(uname -m)"
case $kind in
deb)
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    ;;
arch) pacman -Syu --noconfirm ;;
esac

if [ -n "$previous" ]; then
    section "Install $(basename "$previous"), to upgrade from"
    install_package "$previous"
fi

section "Install $(basename "$package")"
install_package "$package" 2>&1 | tee /tmp/install.log
want=$(package_version "$package")
[ "$(installed_version)" = "$want" ] || fail "installed $(installed_version), not $want"
version=${want%-*}

section Files
[ -x /usr/bin/plaa ] || fail "/usr/bin/plaa is missing"
for file in \
    /usr/lib/plaa/plaa-netraw \
    /usr/lib/plaa/avb-check \
    "/usr/share/applications/$id.desktop" \
    "/usr/share/metainfo/$id.metainfo.xml" \
    "/usr/share/icons/hicolor/512x512/apps/$id.png" \
    "/usr/share/icons/hicolor/scalable/apps/$id.svg" \
    /usr/share/licenses/plaa/LICENSE \
    /usr/share/licenses/plaa/THIRD-PARTY.md \
    /usr/share/licenses/plaa/fonts/OFL.txt \
    /usr/share/licenses/plaa/fonts/LICENSE-MaterialSymbols.txt \
    /usr/share/plaa/ptp4l-gptp.cfg \
    /usr/share/plaa/ptp4l-lite.cfg \
    "/usr/lib/systemd/system/plaa-ptp4l@.service" \
    "/usr/lib/systemd/system/plaa-ptp4l-lite@.service" \
    /usr/share/polkit-1/rules.d/50-plaa-ptp4l.rules; do
    [ -f "$file" ] || fail "$file is missing"
done
# The services run ptp4l with the configurations the package brings.
for profile in gptp lite; do
    unit=plaa-ptp4l@.service
    [ "$profile" = lite ] && unit=plaa-ptp4l-lite@.service
    grep -q "^ExecStart=ptp4l -f /usr/share/plaa/ptp4l-$profile.cfg -i %i\$" \
        "/usr/lib/systemd/system/$unit" ||
        fail "$unit does not run ptp4l with plaa's configuration"
done
echo "all there"

# Only the helper holds the capability; plaa gets its raw sockets from
# it.
section Capabilities
capabilities=$(getcap /usr/lib/plaa/plaa-netraw)
echo "$capabilities"
grep -Eq 'cap_net_raw[+=]ep' <<<"$capabilities" || fail "/usr/lib/plaa/plaa-netraw lacks CAP_NET_RAW"
[ -z "$(getcap /usr/bin/plaa)" ] || fail "/usr/bin/plaa holds a capability"

# A container's interfaces have no PTP hardware clock, so installing must
# have said so.
section "No interface for AVB streams"
grep -q 'requires an AVB' /tmp/install.log ||
    fail "installing did not say that no interface can send AVB streams"
echo "said so as it installed"

section "Shared libraries"
for program in /usr/bin/plaa /usr/lib/plaa/plaa-netraw; do
    missing=$(ldd "$program" | grep 'not found' || true)
    [ -z "$missing" ] || fail "$program needs libraries the package does not bring: $missing"
done
echo "all found"

# Its version line names the FFmpeg it found, so FFmpeg's libraries load.
section "plaa, as a regular user"
command -v runuser >/dev/null || add_tools util-linux >/dev/null
useradd -m tester
as_tester() { runuser -u tester -- "$@"; }
printed=$(as_tester /usr/bin/plaa --version)
echo "$printed"
[[ $printed == "plaa $version ("*"), FFmpeg "[0-9]* ]] ||
    fail "plaa --version printed '$printed', not 'plaa $version (...), FFmpeg ...'"

if [ -z "$emulated" ]; then
    section "Raw Ethernet, as a regular user"
    listing=$(as_tester /usr/bin/plaa --interfaces)
    echo "$listing"
    grep -qx 'raw Ethernet: ready' <<<"$listing" || fail "plaa has no raw Ethernet as a regular user"

    # sway, as its IPC names the windows open, and grim for the picture.
    section "The window, under a headless Wayland compositor"
    case $kind in
    deb) add_tools sway grim procps >/dev/null ;;
    rpm) add_tools sway grim procps-ng >/dev/null ;;
    arch) add_tools sway grim procps-ng >/dev/null ;;
    esac
    # Arch's sway carries CAP_SYS_NICE, which a container cannot grant,
    # so it would not start at all.
    setcap -r "$(command -v sway)" 2>/dev/null || true
    runtime=/tmp/runtime-tester
    install -d -m 700 -o tester -g tester "$runtime"
    printf 'output HEADLESS-1 resolution 1280x800\n' >/tmp/sway.conf
    session=(env XDG_RUNTIME_DIR="$runtime" WLR_BACKENDS=headless WLR_RENDERER=pixman
        WLR_LIBINPUT_NO_DEVICES=1)
    # Older sway will not start where NVIDIA's driver is loaded, which a
    # container sees of its host.
    as_tester "${session[@]}" sway --unsupported-gpu -c /tmp/sway.conf >/tmp/sway.log 2>&1 &
    for _ in $(seq 50); do
        display=$(cd "$runtime" && ls wayland-[0-9] 2>/dev/null | head -1 || true)
        ipc=$(ls "$runtime"/sway-ipc.*.sock 2>/dev/null | head -1 || true)
        [ -n "$display" ] && [ -n "$ipc" ] && break
        sleep 0.2
    done
    [ -n "$display" ] && [ -n "$ipc" ] || {
        cat /tmp/sway.log
        fail "sway did not start"
    }
    session+=(WAYLAND_DISPLAY="$display" SWAYSOCK="$ipc")
    # Waits for a window named $1 to open, while plaa runs.
    window_named() {
        for _ in $(seq 60); do
            as_tester "${session[@]}" swaymsg -t get_tree | grep -q "\"name\": \"$1\"" && return 0
            pgrep -x -u tester plaa >/dev/null || return 1
            sleep 0.5
        done
        return 1
    }
    # The number of pixels of each of yellow, cyan and magenta in a
    # picture of the screen, the smallest of the three, in hundredths of
    # the picture.
    bars() {
        as_tester "${session[@]}" grim -t ppm /tmp/screen.ppm
        tail -c +"$(($(head -n 3 /tmp/screen.ppm | wc -c) + 1))" /tmp/screen.ppm | od -An -v -tu1 -w3 |
            awk '{ if ($1 > 180 && $2 > 180 && $3 < 80) yellow++
                   else if ($1 < 80 && $2 > 180 && $3 > 180) cyan++
                   else if ($1 > 180 && $2 < 80 && $3 > 180) magenta++ }
                 END { least = yellow; if (cyan < least) least = cyan
                       if (magenta < least) least = magenta; print int(least * 10000 / NR) }'
    }
    shot() {
        [ -n "$shots" ] || return 0
        as_tester "${session[@]}" grim -t png - \
            >"$shots/$(. /etc/os-release && echo "$ID-${VERSION_ID:-rolling}")-$(uname -m)-$kind-$1.png"
    }

    as_tester "${session[@]}" plaa >/tmp/plaa.log 2>&1 &
    window_named plaa || {
        cat /tmp/plaa.log
        fail "plaa's window did not open"
    }
    # Running a few seconds on, and drawing more than a blank window.
    sleep 5
    pgrep -x -u tester plaa >/dev/null || {
        cat /tmp/plaa.log
        fail "plaa quit after opening its window"
    }
    as_tester "${session[@]}" grim -t ppm /tmp/window.ppm
    shot start
    values=$(tail -c +100 /tmp/window.ppm | od -An -v -tu1 | tr -s ' ' '\n' | sort -u | wc -l)
    [ "$values" -gt 32 ] || fail "plaa's window looks blank"
    echo "the window opened and drew"

    # A clip of VP9 and Opus, which every release's FFmpeg decodes, its
    # test pattern's yellow, cyan and magenta bars on screen once its
    # pictures are drawn. The running plaa opens it, as a file manager
    # would, in a window of its own; the container has no sound device.
    section "A video, played"
    install -m 644 "$here/../media/test-clip.webm" /tmp/test-clip.webm
    as_tester "${session[@]}" plaa /tmp/test-clip.webm >>/tmp/plaa.log 2>&1 &
    window_named test-clip.webm || {
        cat /tmp/plaa.log
        fail "the video's window did not open"
    }
    seen=0
    for _ in $(seq 20); do
        seen=$(bars)
        [ "$seen" -ge 100 ] && break
        sleep 0.5
    done
    shot video
    [ "$seen" -ge 100 ] || {
        cat /tmp/plaa.log
        fail "the video's pictures did not show"
    }
    pgrep -x -u tester plaa >/dev/null || fail "plaa quit while playing"
    echo "it played, each bar $((seen / 100)).$((seen % 100 / 10))% of the screen"
    pkill -x -u tester plaa || true
    pkill -x -u tester sway || true
fi

section Remove
remove_package
leftovers=()
for path in /usr/bin/plaa /usr/lib/plaa "/usr/share/applications/$id.desktop" \
    "/usr/share/metainfo/$id.metainfo.xml" /usr/share/licenses/plaa /usr/share/plaa \
    "/usr/lib/systemd/system/plaa-ptp4l@.service" "/usr/lib/systemd/system/plaa-ptp4l-lite@.service" \
    /usr/share/polkit-1/rules.d/50-plaa-ptp4l.rules; do
    [ -e "$path" ] && leftovers+=("$path")
done
[ ${#leftovers[@]} -eq 0 ] || fail "removing left ${leftovers[*]}"
echo "nothing left"

section "Passed: $(basename "$package")"
