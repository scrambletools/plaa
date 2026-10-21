#!/bin/bash
# Builds plaa.app on macOS, carrying the FFmpeg and libass it uses, and
# with --pkg an installer package that also lets plaa reach the network
# for AVB. The app is signed ad hoc, which Apple Silicon needs to run it
# at all, unless an identity is given, as below. A release carries
# Scramble Tools' Developer ID signatures and is notarized
# (docs/RELEASING.md).
#
#   packaging/macos/bundle.sh [--pkg] [--out DIR]
#
# The libraries come from Homebrew (brew install ffmpeg libass): each one
# plaa links or loads, and each one those need, outside macOS itself,
# goes into Contents/Frameworks, found there through @rpath. Homebrew
# builds them for the macOS it runs on, so the app claims the newest
# macOS any of them needs; CI builds on the oldest macOS it supports.
# Contents/Resources/licenses/libraries.txt names each library's formula,
# version, license and source.
#
# plaa reaches the network through the BPF devices, which are root's
# alone at every start. The package installs a launch daemon that opens
# them to the access_bpf group at each start, as Wireshark's ChmodBPF and
# triib do with the same group, and adds the user installing it to the
# group. As it opens, the installer warns when the Mac has no AVB
# Ethernet port (distribution.xml).
#
# PLAA_PRODUCTION=1 makes a release build, as in CI; without it the app
# is the development build, with its own settings and cache.
#
# PLAA_SIGN_IDENTITY signs with that identity rather than ad hoc, from the
# keychain PLAA_SIGN_KEYCHAIN when given, unlocked with
# PLAA_SIGN_KEYCHAIN_PASSWORD. An ad hoc signature changes with every
# build, so macOS asks again for the microphone, the camera and screen
# recording; one that stays, such as a local certificate's, keeps what was
# allowed. PLAA_INSTALLER_IDENTITY signs the package, from the same
# keychain. PLAA_NOTARY_KEY, an App Store Connect API key's .p8 file, with
# PLAA_NOTARY_KEY_ID and PLAA_NOTARY_ISSUER, has the package notarized and
# the ticket stapled to it.
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
here="$root/packaging/macos"
out="$root/target/macos"
pkg=false
while [ $# -gt 0 ]; do
    case "$1" in
        --pkg) pkg=true ;;
        --out) out=$2; shift ;;
        *) echo "unknown option: $1" >&2; exit 2 ;;
    esac
    shift
done

version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
if [ -n "${PLAA_PRODUCTION:-}" ]; then
    name=plaa; id=io.github.scrambletools.plaa
else
    name=plaa-dev; id=io.github.scrambletools.plaa.dev
fi
brew=$(brew --prefix)

cargo build --release --locked -p plaa --manifest-path "$root/Cargo.toml"

app="$out/$name.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Frameworks" "$app/Contents/Resources/licenses/fonts"
cp "$root/target/release/plaa" "$app/Contents/MacOS/"
cp "$root/LICENSE" "$app/Contents/Resources/licenses/"
cp "$root"/packaging/licenses/* "$app/Contents/Resources/licenses/fonts/"

# Whether a library is macOS's own, which every Mac has.
system() {
    case $1 in
        /usr/lib/* | /System/*) return 0 ;;
        *) return 1 ;;
    esac
}
# The libraries a Mach-O file links, as their load commands name them.
linked() { otool -L "$1" | tail -n +2 | awk '{print $1}'; }
# Where a library a file links is, its @rpath and @loader_path resolved
# against the folder the file came from.
located() {
    local library=$1 from=$2
    case $library in
        @rpath/* | @loader_path/*)
            local leaf=${library#*/}
            for folder in "$from" "$brew/lib"; do
                [ -e "$folder/$leaf" ] && { echo "$folder/$leaf"; return; }
            done
            echo "bundle.sh: cannot find $library, linked from $from" >&2
            return 1
            ;;
        *) echo "$library" ;;
    esac
}

# Each library plaa links, and libass, which it loads as it runs; then
# each library those link, until none is left outside macOS.
frameworks="$app/Contents/Frameworks"
queue=()
while read -r library; do
    system "$library" || queue+=("$(located "$library" "$root/target/release")")
done < <(linked "$app/Contents/MacOS/plaa")
queue+=("$brew/opt/libass/lib/libass.9.dylib")
copied=()
while [ ${#queue[@]} -gt 0 ]; do
    source=${queue[0]}
    queue=("${queue[@]:1}")
    leaf=$(basename "$source")
    [ -e "$frameworks/$leaf" ] && continue
    real=$(realpath "$source")
    cp "$real" "$frameworks/$leaf"
    chmod 644 "$frameworks/$leaf"
    copied+=("$real")
    while read -r library; do
        system "$library" && continue
        [ "$(basename "$library")" = "$leaf" ] && continue
        queue+=("$(located "$library" "$(dirname "$real")")")
    done < <(linked "$real")
done

# Each copied library and plaa name the others by @rpath, which finds
# them in Frameworks: from plaa through its own rpath, from each library
# through @loader_path, the folder it is in. Any other place to look,
# such as Homebrew's, goes, as it would find Homebrew's copies on a Mac
# that has them.
rpaths() { otool -l "$1" | awk '$1 == "cmd" && $2 == "LC_RPATH" {getline; getline; print $2}'; }
unlook() {
    local path
    while read -r path; do
        [ -n "$path" ] && install_name_tool -delete_rpath "$path" "$1" 2>/dev/null
    done < <(rpaths "$1")
}
point() {
    local file=$1 library
    while read -r library; do
        system "$library" && continue
        local leaf
        leaf=$(basename "$library")
        [ "$library" = "@rpath/$leaf" ] && continue
        install_name_tool -change "$library" "@rpath/$leaf" "$file" 2>/dev/null
    done < <(linked "$file")
}
for library in "$frameworks"/*.dylib; do
    install_name_tool -id "@rpath/$(basename "$library")" "$library" 2>/dev/null
    point "$library"
    unlook "$library"
    install_name_tool -add_rpath @loader_path "$library" 2>/dev/null
done
point "$app/Contents/MacOS/plaa"
unlook "$app/Contents/MacOS/plaa"
install_name_tool -add_rpath @executable_path/../Frameworks "$app/Contents/MacOS/plaa" 2>/dev/null

# The newest macOS any part needs, which the app claims.
minimum=11.0
newer() {
    awk -v a="$1" -v b="$2" 'BEGIN {
        n = split(a, x, "."); m = split(b, y, ".")
        for (i = 1; i <= (n > m ? n : m); i++) {
            if (x[i] + 0 > y[i] + 0) exit 0
            if (x[i] + 0 < y[i] + 0) exit 1
        }
        exit 1
    }'
}
for file in "$app/Contents/MacOS/plaa" "$frameworks"/*.dylib; do
    minos=$(vtool -show-build "$file" | awk '$1 == "minos" {print $2; exit}')
    if [ -n "$minos" ] && newer "$minos" "$minimum"; then
        minimum=$minos
    fi
done

# What each library is, from its Homebrew formula, with the formula's
# license files.
{
    echo "plaa.app carries these libraries, built by Homebrew (https://brew.sh)."
    echo "Each one's source is at the address given, and with plaa's releases."
    echo
    for real in "${copied[@]}"; do
        keg=${real%/lib/*}
        formula=$(basename "$(dirname "$keg")")
        info=$(brew info --json=v2 "$formula" 2>/dev/null || true)
        printf '%s %s (%s): %s\n    %s\n' "$(basename "$real")" "$(basename "$keg")" "$formula" \
            "$(plutil -extract formulae.0.license raw -o - - <<<"$info" 2>/dev/null || echo unknown)" \
            "$(plutil -extract formulae.0.urls.stable.url raw -o - - <<<"$info" 2>/dev/null || true)"
        mkdir -p "$app/Contents/Resources/licenses/$formula"
        find "$keg" -maxdepth 1 -type f \( -iname 'LICENSE*' -o -iname 'COPYING*' -o -iname 'COPYRIGHT*' \
            -o -iname 'NOTICE*' \) -exec cp {} "$app/Contents/Resources/licenses/$formula/" \;
    done
} >"$app/Contents/Resources/licenses/libraries.txt"

# The icon: each size an .iconset asks for, from the PNGs the Linux
# packages use and the 1024 one they are made from.
icons="$root/data/icons/hicolor"
png() { echo "$icons/$1x$1/apps/io.github.scrambletools.plaa.png"; }
iconset=$(mktemp -d)/plaa.iconset
mkdir -p "$iconset"
for size in 16 32 128 256 512; do
    cp "$(png $size)" "$iconset/icon_${size}x${size}.png"
done
cp "$(png 32)" "$iconset/icon_16x16@2x.png"
cp "$(png 64)" "$iconset/icon_32x32@2x.png"
cp "$(png 256)" "$iconset/icon_128x128@2x.png"
cp "$(png 512)" "$iconset/icon_256x256@2x.png"
cp "$root/data/icons/plaa-1024.png" "$iconset/icon_512x512@2x.png"
iconutil -c icns "$iconset" -o "$app/Contents/Resources/plaa.icns"

cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>$name</string>
    <key>CFBundleDisplayName</key><string>$name</string>
    <key>CFBundleIdentifier</key><string>$id</string>
    <key>CFBundleExecutable</key><string>plaa</string>
    <key>CFBundleIconFile</key><string>plaa</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>$version</string>
    <key>CFBundleVersion</key><string>$version</string>
    <key>LSMinimumSystemVersion</key><string>$minimum</string>
    <key>LSApplicationCategoryType</key><string>public.app-category.video</string>
    <key>NSHighResolutionCapable</key><true/>
    <key>NSMicrophoneUsageDescription</key><string>plaa records sound from a microphone when you ask it to record sound, the screen or the camera.</string>
    <key>NSCameraUsageDescription</key><string>plaa records the camera when you ask it to.</string>
    <key>NSAudioCaptureUsageDescription</key><string>plaa records what the Mac plays when you record the screen with its sound.</string>
    <key>NSHumanReadableCopyright</key><string>GPL-3.0-or-later</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key><string>Video</string>
            <key>CFBundleTypeRole</key><string>Viewer</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>public.movie</string>
                <string>org.matroska.mkv</string>
            </array>
        </dict>
        <dict>
            <key>CFBundleTypeName</key><string>Audio</string>
            <key>CFBundleTypeRole</key><string>Viewer</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>public.audio</string>
                <string>org.matroska.mka</string>
            </array>
        </dict>
        <dict>
            <key>CFBundleTypeName</key><string>Playlist</string>
            <key>CFBundleTypeRole</key><string>Viewer</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>public.m3u-playlist</string>
            </array>
        </dict>
    </array>
    <key>UTImportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key><string>org.matroska.mkv</string>
            <key>UTTypeDescription</key><string>Matroska video</string>
            <key>UTTypeConformsTo</key><array><string>public.movie</string></array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key><array><string>mkv</string><string>mk3d</string></array>
                <key>public.mime-type</key><array><string>video/x-matroska</string></array>
            </dict>
        </dict>
        <dict>
            <key>UTTypeIdentifier</key><string>org.matroska.mka</string>
            <key>UTTypeDescription</key><string>Matroska audio</string>
            <key>UTTypeConformsTo</key><array><string>public.audio</string></array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key><array><string>mka</string></array>
                <key>public.mime-type</key><array><string>audio/x-matroska</string></array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
PLIST
plutil -lint "$app/Contents/Info.plist" >/dev/null

# Extended attributes would go into the package as AppleDouble files.
xattr -cr "$app"
# The hardened runtime, which notarization asks for, every build alike;
# under it the microphone and the camera take entitlements.
sign=(--force --options runtime --sign "${PLAA_SIGN_IDENTITY:--}")
if [ -n "${PLAA_SIGN_KEYCHAIN:-}" ]; then
    security unlock-keychain -p "${PLAA_SIGN_KEYCHAIN_PASSWORD:-}" "$PLAA_SIGN_KEYCHAIN"
    sign+=(--keychain "$PLAA_SIGN_KEYCHAIN")
fi
# Notarization also asks for a secure timestamp, which any identity gets,
# named or given by its SHA-1 hash.
if [ -n "${PLAA_SIGN_IDENTITY:-}" ] && [ "$PLAA_SIGN_IDENTITY" != - ]; then
    sign+=(--timestamp)
fi
for library in "$frameworks"/*.dylib; do
    codesign "${sign[@]}" "$library"
done
# The hardened runtime loads only libraries signed by the app's own team,
# and an ad hoc signature names none, so an app signed ad hoc may load the
# libraries it carries, signed ad hoc too.
entitlements="$here/plaa.entitlements"
if [ "${PLAA_SIGN_IDENTITY:--}" = - ]; then
    entitlements=$(mktemp -d)/plaa.entitlements
    sed 's#^</dict>#    <key>com.apple.security.cs.disable-library-validation</key>\
    <true/>\
</dict>#' "$here/plaa.entitlements" >"$entitlements"
    plutil -lint "$entitlements" >/dev/null
fi
codesign "${sign[@]}" --entitlements "$entitlements" "$app"
codesign --verify --strict --deep "$app"
"$app/Contents/MacOS/plaa" --version
echo "$app, for macOS $minimum and later, with $(find "$frameworks" -name '*.dylib' | wc -l | tr -d ' ') libraries"

if $pkg; then
    payload=$(mktemp -d)
    mkdir -p "$payload/Applications" "$payload/Library/LaunchDaemons" \
        "$payload/Library/Application Support/plaa"
    cp -R "$app" "$payload/Applications/"
    cp "$here/io.github.scrambletools.plaa.bpf.plist" "$payload/Library/LaunchDaemons/"
    install -m 755 "$here/bpf-access" "$here/uninstall" "$payload/Library/Application Support/plaa/"
    scripts=$(mktemp -d)
    install -m 755 "$here/postinstall" "$scripts/postinstall"
    # The app goes to /Applications even when a copy is elsewhere.
    components=$(mktemp -d)/components.plist
    pkgbuild --analyze --root "$payload" "$components" >/dev/null
    plutil -replace 0.BundleIsRelocatable -bool false "$components"
    component=$(mktemp -d)
    pkgbuild --root "$payload" --component-plist "$components" --scripts "$scripts" \
        --identifier "$id" --version "$version" --install-location / "$component/$id.pkg" >/dev/null
    distribution=$(mktemp -d)/distribution.xml
    sed -e "s/@NAME@/$name/g" -e "s/@ID@/$id/g" -e "s/@VERSION@/$version/g" \
        -e "s/@ARCH@/$(uname -m)/g" -e "s/@PACKAGE@/$id.pkg/g" -e "s/@MINIMUM@/$minimum/g" \
        "$here/distribution.xml" >"$distribution"
    package="$out/$name-$version-$(uname -m).pkg"
    rm -f "$package"
    product=(--distribution "$distribution" --package-path "$component")
    if [ -n "${PLAA_INSTALLER_IDENTITY:-}" ]; then
        product+=(--sign "$PLAA_INSTALLER_IDENTITY" --timestamp)
        if [ -n "${PLAA_SIGN_KEYCHAIN:-}" ]; then
            product+=(--keychain "$PLAA_SIGN_KEYCHAIN")
        fi
    fi
    productbuild "${product[@]}" "$package" >/dev/null
    if [ -n "${PLAA_NOTARY_KEY:-}" ]; then
        notary=(--key "$PLAA_NOTARY_KEY" --key-id "${PLAA_NOTARY_KEY_ID:?}"
            --issuer "${PLAA_NOTARY_ISSUER:?}")
        result=$(xcrun notarytool submit "$package" "${notary[@]}" --wait --output-format json) || true
        status=$(plutil -extract status raw -o - - <<<"$result" 2>/dev/null || echo none)
        if [ "$status" != Accepted ]; then
            echo "notarization: $status" >&2
            echo "$result" >&2
            submission=$(plutil -extract id raw -o - - <<<"$result" 2>/dev/null || true)
            if [ -n "$submission" ]; then
                xcrun notarytool log "$submission" "${notary[@]}" >&2 || true
            fi
            exit 1
        fi
        # The ticket can take a moment to reach Apple's servers after the
        # package is accepted, so stapling is tried again before failing.
        for attempt in 1 2 3 4 5; do
            if staple=$(xcrun stapler staple "$package" 2>&1); then
                break
            fi
            if [ "$attempt" = 5 ]; then
                echo "$staple" >&2
                exit 1
            fi
            sleep 30
        done
    fi
    echo "$package"
fi
