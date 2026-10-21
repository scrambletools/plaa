#!/usr/bin/env bash
# Builds plaa's package for the release of Debian, Ubuntu or Fedora this
# runs on, as root in a fresh container of it, which build.sh beside it
# starts: a .deb or an .rpm whose dependencies are what the programs
# link, read from them by dpkg-shlibdeps or rpmdeps, as FFmpeg's
# libraries change names from release to release, and the libraries plaa
# loads as it runs.
#
#   build-package.sh OUT
#
# The source is this script's tree, which may be read-only. Builds go to
# CARGO_TARGET_DIR, /target by default. VERSION_SUFFIX, such as
# ~debian13, names the release in the package's version; it defaults to
# one made from /etc/os-release. PLAA_CROSS, a Debian architecture such
# as riscv64, builds a .deb for it on this one, with that architecture's
# libraries installed beside this one's, where no computer of its kind
# builds; its programs run here through QEMU, as binfmt_misc lets them.
set -euo pipefail

out=${1:?usage: build-package.sh OUT}
cross=${PLAA_CROSS:-}
root=$(cd "$(dirname "$0")/../.." && pwd)
nfpm_version=${NFPM_VERSION:-2.43.0}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/target}

# shellcheck source=/dev/null
. /etc/os-release
case $ID in
debian | ubuntu)
    kind=deb
    suffix=${VERSION_SUFFIX:-~$ID$VERSION_ID}
    export DEBIAN_FRONTEND=noninteractive
    libraries=(libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev
        libswscale-dev libswresample-dev libpipewire-0.3-dev libasound2-dev
        libfontconfig1-dev libfreetype-dev libxkbcommon-dev libwayland-dev)
    tools=(build-essential clang libclang-dev pkg-config curl ca-certificates git file dpkg-dev)
    if [ -n "$cross" ]; then
        dpkg --add-architecture "$cross"
        if [ "$ID" = ubuntu ]; then
            # Ubuntu keeps architectures other than amd64 and i386 on its
            # ports archive.
            native=$(dpkg --print-architecture)
            sed -i "/^Types:/a Architectures: $native" /etc/apt/sources.list.d/ubuntu.sources
            suites="$VERSION_CODENAME $VERSION_CODENAME-updates $VERSION_CODENAME-security"
            printf 'Types: deb\nURIs: http://ports.ubuntu.com/ubuntu-ports\nSuites: %s\nComponents: main universe\nArchitectures: %s\nSigned-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg\n' \
                "$suites" "$cross" >/etc/apt/sources.list.d/ports.sources
        fi
        libraries=("${libraries[@]/%/:$cross}")
        tools+=("crossbuild-essential-$cross")
    fi
    apt-get update -q
    apt-get install -y --no-install-recommends "${tools[@]}" "${libraries[@]}"
    ;;
fedora)
    if [ -n "$cross" ]; then
        echo "build-package.sh: PLAA_CROSS builds only .deb packages" >&2
        exit 2
    fi
    kind=rpm
    suffix=${VERSION_SUFFIX:-.fc$VERSION_ID}
    dnf install -y --setopt=install_weak_deps=False \
        gcc gcc-c++ clang clang-devel pkgconf-pkg-config curl git file rpm-build \
        libavcodec-free-devel libavformat-free-devel libavutil-free-devel \
        libavfilter-free-devel libavdevice-free-devel libswscale-free-devel \
        libswresample-free-devel pipewire-devel alsa-lib-devel fontconfig-devel \
        freetype-devel libxkbcommon-devel wayland-devel
    ;;
*)
    echo "build-package.sh: no package for $ID" >&2
    exit 2
    ;;
esac

if ! command -v cargo >/dev/null; then
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
fi
# shellcheck source=/dev/null
[ -f "${CARGO_HOME:-$HOME/.cargo}/env" ] && . "${CARGO_HOME:-$HOME/.cargo}/env"
# The tree belongs to someone else, which git otherwise refuses to read
# for the version's commit.
git config --global --add safe.directory '*'

version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
case ${cross:-$(uname -m)} in
x86_64 | amd64) deb_arch=amd64 rpm_arch=x86_64 rust_arch=x86_64 ;;
aarch64 | arm64) deb_arch=arm64 rpm_arch=aarch64 rust_arch=aarch64 ;;
riscv64) deb_arch=riscv64 rpm_arch=riscv64 rust_arch=riscv64gc ;;
*) echo "build-package.sh: unknown architecture ${cross:-$(uname -m)}" >&2; exit 2 ;;
esac

binaries=$CARGO_TARGET_DIR/release
targeting=()
if [ -n "$cross" ]; then
    # The other architecture's compilers, libraries and headers, for Rust,
    # the C its crates build, pkg-config and bindgen alike.
    gnu=$(dpkg-architecture -a"$cross" -qDEB_HOST_GNU_TYPE 2>/dev/null)
    target=$rust_arch-unknown-linux-gnu
    rustup target add "$target"
    target_variable=$(tr 'a-z-' 'A-Z_' <<<"$target")
    export "CARGO_TARGET_${target_variable}_LINKER=$gnu-gcc"
    export "CC_${target//-/_}=$gnu-gcc" "CXX_${target//-/_}=$gnu-g++"
    export PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_LIBDIR=/usr/lib/$gnu/pkgconfig:/usr/share/pkgconfig
    export BINDGEN_EXTRA_CLANG_ARGS="--target=$gnu -I/usr/include/$gnu"
    # ffmpeg-sys-next compiles a check of FFmpeg's version for this
    # computer with the other architecture's include folder, whose C
    # library headers this one's compiler cannot take: it looks there
    # last, past its own.
    cat >/usr/local/bin/plaa-host-cc <<SCRIPT
#!/bin/bash
arguments=()
while [ \$# -gt 0 ]; do
    if [ "\$1" = -I ] && [ "\$2" = /usr/include/$gnu ]; then
        arguments+=(-idirafter "\$2")
        shift 2
        continue
    fi
    arguments+=("\$1")
    shift
done
exec cc "\${arguments[@]}"
SCRIPT
    chmod +x /usr/local/bin/plaa-host-cc
    host=$(rustc -vV | sed -n 's/^host: //p')
    export "CC_${host//-/_}=plaa-host-cc"
    export DEB_HOST_ARCH=$cross
    targeting=(--target "$target")
    binaries=$CARGO_TARGET_DIR/$target/release
fi

# The installed copy, with its own settings and cache, not a dev build.
PLAA_PRODUCTION=1 cargo build --release --locked --manifest-path "$root/Cargo.toml" \
    -p plaa -p plaa-avb --bin plaa --bin plaa-netraw "${targeting[@]}"
"$binaries/plaa" --version

work=$(mktemp -d)
"$root/scripts/dist-install.sh" "$binaries" "$work/stage" /usr
mkdir -p "$work/packaging/linux"
cp "$root/packaging/linux/postinstall.sh" "$work/packaging/linux/"
programs=("$work/stage/usr/bin/plaa" "$work/stage/usr/lib/plaa/plaa-netraw")

# What the programs link, as package names or as the libraries' names,
# and what plaa loads as it runs: libass, the window system's libraries
# and Vulkan, with Mesa's drivers, whose software one draws the picture
# where no graphics card can: without a Vulkan driver wgpu turns to
# OpenGL, which on some systems cannot draw in the window at all. setcap
# grants the helper its capability as the package installs.
config=$work/nfpm.yaml
cp "$root/packaging/nfpm.yaml" "$config"
list() { sed 's/^/        - /'; }
if [ $kind = deb ]; then
    mkdir -p "$work/shlibs/debian"
    printf 'Source: plaa\n\nPackage: plaa\nArchitecture: any\n' >"$work/shlibs/debian/control"
    linked=$(cd "$work/shlibs" && dpkg-shlibdeps -O "${programs[@]/#/-e}" |
        sed -n 's/^shlibs:Depends=//p' | sed 's/, /\n/g')
    {
        echo "overrides:"
        echo "  deb:"
        echo "    depends:"
        printf '%s\n' "$linked" libass9 libcap2-bin libvulkan1 mesa-vulkan-drivers libwayland-client0 \
            libxkbcommon0 | list
        echo "    recommends:"
        printf '%s\n' linuxptp | list
        echo "    suggests:"
        printf '%s\n' yt-dlp | list
    } >>"$config"
else
    linked=$(/usr/lib/rpm/rpmdeps --requires "${programs[@]}" | grep -v '^rtld(')
    {
        echo "overrides:"
        echo "  rpm:"
        echo "    depends:"
        printf '%s\n' "$linked" libass libcap vulkan-loader mesa-vulkan-drivers libwayland-client \
            libxkbcommon | list
        echo "    recommends:"
        printf '%s\n' linuxptp | list
        echo "    suggests:"
        printf '%s\n' yt-dlp | list
    } >>"$config"
fi
sed -n '/^overrides:/,$p' "$config"

if ! command -v nfpm >/dev/null; then
    case $(uname -m) in
    x86_64) nfpm_arch=x86_64 ;;
    aarch64) nfpm_arch=arm64 ;;
    *) nfpm_arch=$(uname -m) ;;
    esac
    curl -sSLf "https://github.com/goreleaser/nfpm/releases/download/v$nfpm_version/nfpm_${nfpm_version}_Linux_$nfpm_arch.tar.gz" |
        tar -xz -C /usr/local/bin nfpm
fi
mkdir -p "$out"
cd "$work"
if [ $kind = deb ]; then
    VERSION=$version RELEASE="1$suffix" ARCH=$deb_arch nfpm package -f "$config" -p deb -t "$out/"
else
    VERSION=$version RELEASE="1$suffix" ARCH=$rpm_arch nfpm package -f "$config" -p rpm -t "$out/"
fi
ls -l "$out"
