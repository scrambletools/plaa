#!/bin/bash
# Installs plaa's package on this Mac and checks it as someone installing
# it would meet it: Gatekeeper's verdict on a downloaded copy, libraries
# that only macOS itself provides or the app carries, what the package
# sets up, BPF access for the user, the window, a video played in it, and
# what the uninstall script leaves behind. For a Mac with nothing to
# lose, such as a CI runner: it replaces any installed plaa.app and
# removes it at the end, and needs sudo.
#
#   packaging/tests/macos/test-pkg.sh PACKAGE [PREVIOUS]
#
# PREVIOUS, an older package, goes in first, so PACKAGE upgrades it.
# PLAA_EXPECT_SIGNED=true also asks for Scramble Tools' Developer ID
# signatures and notarization.
set -euo pipefail

package=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
previous=${2:+$(cd "$(dirname "$2")" && pwd)/$(basename "$2")}
here=$(cd "$(dirname "$0")" && pwd)
id=io.github.scrambletools.plaa
app=/Applications/plaa.app
daemon=/Library/LaunchDaemons/$id.bpf.plist
support="/Library/Application Support/plaa"
console=$(stat -f%Su /dev/console)

section() { printf '\n== %s\n' "$*"; }
fail() {
    printf 'FAIL: %s\n' "$*" >&2
    exit 1
}
# Whether version $1 is at most $2, both as dotted numbers.
at_most() {
    awk -v a="$1" -v b="$2" 'BEGIN {
        n = split(a, x, "."); m = split(b, y, ".")
        for (i = 1; i <= (n > m ? n : m); i++) {
            if (x[i] + 0 < y[i] + 0) exit 0
            if (x[i] + 0 > y[i] + 0) exit 1
        }
        exit 0
    }'
}

section "macOS $(sw_vers -productVersion), $(uname -m)"
work=$(mktemp -d)
pkgutil --expand-full "$package" "$work/expanded"
want=$(xmllint --xpath 'string(//pkg-ref[@version]/@version)' "$work/expanded/Distribution")
bundle="$work/expanded/$id.pkg/Payload/Applications/plaa.app"
[ -d "$bundle" ] || fail "the package has no plaa.app"
echo "plaa $want"

section "Gatekeeper, for a downloaded copy"
if [ "${PLAA_EXPECT_SIGNED:-}" = true ]; then
    mkdir "$work/downloaded"
    cp "$package" "$work/downloaded/"
    copy="$work/downloaded/$(basename "$package")"
    xattr -w com.apple.quarantine "0081;$(printf %x "$(date +%s)");Safari;" "$copy"
    spctl --assess --type install --verbose=2 "$copy" || fail "Gatekeeper rejects the downloaded package"
    pkgutil --check-signature "$copy" | grep -q 'Notarization: trusted by the Apple notary service' ||
        fail "the package is not notarized"
else
    echo "this build is not signed, so skipped"
fi

# Any library outside macOS and the app, such as one from Homebrew on the
# build machine, would be missing on another Mac, and an rpath outside
# the app could find it there and hide that; and every part must run on
# the oldest macOS the app claims.
section "Libraries and the oldest macOS"
minimum=$(plutil -extract LSMinimumSystemVersion raw "$bundle/Contents/Info.plist")
carried=$(ls "$bundle/Contents/Frameworks")
problems=()
while IFS= read -r -d '' binary; do
    file -b "$binary" | grep -q Mach-O || continue
    name=${binary#"$bundle/"}
    while read -r library; do
        case $library in
        /usr/lib/* | /System/Library/*) ;;
        @rpath/*) grep -qx "${library#@rpath/}" <<<"$carried" ||
            problems+=("$name links $library, which the app does not carry") ;;
        *) problems+=("$name links $library, which macOS does not have") ;;
        esac
    done < <(otool -L "$binary" | tail -n +2 | awk '{print $1}')
    while read -r path; do
        case $path in
        @loader_path | @executable_path/../Frameworks) ;;
        *) problems+=("$name looks for libraries in $path") ;;
        esac
    done < <(otool -l "$binary" | awk '$1 == "cmd" && $2 == "LC_RPATH" {getline; getline; print $2}')
    lipo -archs "$binary" | grep -qw arm64 || problems+=("$name has no arm64 code")
    minos=$(vtool -show-build "$binary" | awk '$1 == "minos" {print $2; exit}')
    at_most "$minos" "$minimum" || problems+=("$name needs macOS $minos, the app claims $minimum")
done < <(find "$bundle/Contents/MacOS" "$bundle/Contents/Frameworks" -type f -print0)
[ ${#problems[@]} -eq 0 ] || fail "$(printf '\n  %s' "${problems[@]}")"
echo "plaa and $(wc -l <<<"$carried" | tr -d ' ') libraries, for macOS $minimum and later"
at_most "$minimum" "$(sw_vers -productVersion)" || {
    echo "this Mac is older than the app needs; the installer must refuse it"
    if sudo installer -pkg "$package" -target / >/dev/null 2>&1; then
        fail "the installer took a macOS older than $minimum"
    fi
    section "Passed: $(basename "$package"), refused here"
    exit 0
}

if [ -n "$previous" ]; then
    section "Install $(basename "$previous"), to upgrade from"
    sudo installer -pkg "$previous" -target /
fi

section "Install $(basename "$package")"
sudo installer -pkg "$package" -target / || {
    tail -n 60 /var/log/install.log
    fail "the installer failed"
}
installed=$(pkgutil --pkg-info "$id" | awk '$1 == "version:" {print $2}')
[ "$installed" = "$want" ] || fail "installed $installed, not $want"

# Its version line names the FFmpeg it found, so the libraries the app
# carries load.
section "What it installed"
[ -d "$app" ] || fail "$app is missing"
codesign --verify --strict --deep "$app" || fail "the installed app's signature does not hold"
[ "$(plutil -extract CFBundleShortVersionString raw "$app/Contents/Info.plist")" = "$want" ] ||
    fail "the installed app is not $want"
printed=$("$app/Contents/MacOS/plaa" --version)
echo "$printed"
[[ $printed == "plaa $want ("*"), FFmpeg "[0-9]* ]] || fail "plaa --version printed '$printed'"
sudo launchctl print "system/$id.bpf" >/dev/null || fail "the BPF launch daemon is not loaded"
# The daemon runs as it loads, a moment after the installer is done.
for _ in $(seq 40); do
    [ "$(stat -f '%Sg' /dev/bpf0)" = access_bpf ] && break
    sleep 0.5
done
for device in /dev/bpf*; do
    [ "$(stat -f '%Sg' "$device")" = access_bpf ] || fail "$device is not in access_bpf"
    mode=$(stat -f '%Lp' "$device")
    [ $((8#$mode & 8#060)) -eq $((8#060)) ] || fail "$device is not open to access_bpf"
done
dseditgroup -o checkmember -m "$console" access_bpf || fail "$console did not join access_bpf"

# sudo -u takes the groups from the directory, as a new login would.
section "BPF, as $console"
listing=$(sudo -u "$console" "$app/Contents/MacOS/plaa" --interfaces)
echo "$listing"
grep -qx 'raw Ethernet: ready' <<<"$listing" || fail "plaa has no BPF access as $console"

# The window server names each window's owner and number without any
# permission; a window's picture, through screencapture, needs screen
# recording, which CI's runners allow.
section "The window"
xcrun swiftc -O -o "$work/windows" "$here/windows.swift"
plaa_windows() { "$work/windows" list | awk -F '\t' '$2 == "plaa" {print $1}'; }
quit_plaa() {
    pkill -x plaa || true
    for _ in $(seq 20); do
        pgrep -x plaa >/dev/null || return 0
        sleep 0.5
    done
}
sudo -u "$console" open "$app"
shown=
for _ in $(seq 60); do
    [ -n "$(plaa_windows)" ] && shown=1 && break
    sleep 0.5
done
[ -n "$shown" ] || fail "plaa's window did not open"
sleep 5
pgrep -x plaa >/dev/null || fail "plaa quit after opening its window"
echo "the window opened"
quit_plaa

# A clip of VP9 and Opus, its test pattern's yellow, cyan and magenta
# bars in the window once its pictures are drawn.
section "A video, played"
cp "$here/../media/test-clip.webm" "$work/test-clip.webm"
chmod 644 "$work/test-clip.webm"
chmod 755 "$work"
sudo -u "$console" open -n "$app" --args "$work/test-clip.webm"
seen=0
for _ in $(seq 40); do
    sleep 0.5
    window=$(plaa_windows | head -1)
    [ -n "$window" ] || continue
    screencapture -x -o -l "$window" "$work/video.png" 2>/dev/null || continue
    seen=$("$work/windows" bars "$work/video.png")
    [ "$seen" -ge 100 ] && break
done
pgrep -x plaa >/dev/null || fail "plaa quit while playing"
[ "$seen" -ge 100 ] || fail "the video's pictures did not show; the window is in $work/video.png"
echo "it played, each bar $((seen / 100)).$((seen % 100 / 10))% of the window"
quit_plaa

section "Uninstall"
sudo "$support/uninstall"
leftovers=()
for path in "$app" "$daemon" "$support"; do
    [ -e "$path" ] || [ -L "$path" ] && leftovers+=("$path")
done
sudo launchctl print "system/$id.bpf" >/dev/null 2>&1 && leftovers+=("the launch daemon")
pkgutil --pkgs | grep -qx "$id" && leftovers+=("the package receipt")
[ ${#leftovers[@]} -eq 0 ] || fail "uninstalling left ${leftovers[*]}"
echo "nothing left"

section "Passed: $(basename "$package")"
