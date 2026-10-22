# Releasing plaa

Everything a release ships is built from this repository:

| Path | What |
|---|---|
| `data/io.github.scrambletools.plaa.desktop` | Desktop entry |
| `data/io.github.scrambletools.plaa.metainfo.xml` | AppStream metadata, with the release history |
| `data/icons/hicolor/scalable/apps/io.github.scrambletools.plaa.svg` | The app icon, which `scripts/make-icons.sh` renders into the hicolor PNGs every package picks up, `data/icons/plaa-1024.png` for macOS and `packaging/windows/plaa.ico` |
| `packaging/licenses/` | The licenses of the fonts plaa carries, from scramble-ui |
| `scripts/dist-install.sh` | Installs a build of plaa and plaa-netraw and the files above into a system layout; the Linux packages use it |
| `packaging/linux/` | Debian and RPM packages, each built by `build.sh` in a container of the release it is for and made with nfpm (`packaging/nfpm.yaml`); they grant `plaa-netraw` `CAP_NET_RAW` as they install and run `avb-check` |
| `packaging/arch/plaa/` | AUR package, whose install script does the same |
| `packaging/windows/` | Windows MSI (WiX 5) and zip, made by `build.ps1`, carrying FFmpeg's DLLs and libass's, which `libass.ps1` builds with vcpkg; `ffmpeg-sources.sh` gathers the sources of the FFmpeg build |
| `packaging/macos/` | plaa.app, carrying FFmpeg and libass, and its installer package, made by `bundle.sh --pkg` around `distribution.xml`, signed and notarized as [macOS signing](#macos-signing) says, with the BPF access helper |
| `.github/workflows/release.yml` | Builds all of them for a tag and drafts the GitHub release once the installer tests pass |
| `packaging/tests/` | The installer tests: each package installed, upgraded, run, made to play a video and removed in a clean system, as [Installer tests](#installer-tests) says |
| `site/`, `.github/workflows/site.yml` | The website, plaa.run, published to GitHub Pages whenever it changes on main |
| `BUILD`, `.gitattributes` | The commit, which `git archive` and GitHub's source archives write into `BUILD` for the version line of a build without `.git`, as the AUR's |

The Linux packages are built for x86_64 and ARM64 on GitHub's runners of
that architecture, the Windows ones for x64 and ARM64; the macOS package
is built for Apple Silicon only. GitHub has no RISC-V runners, so the
RISC-V .debs, for Debian 13 and Ubuntu 24.04 and 26.04, are built on
x86_64 against each release's riscv64 libraries installed beside its own
(`PLAA_CROSS=riscv64 packaging/linux/build.sh IMAGE OUT`), and tested
under QEMU, which leaves out raw Ethernet and the window. Fedora has no
RISC-V release to build for.

Packages are built with `PLAA_PRODUCTION=1`, which makes the installed
copy: settings in `plaa.toml` (`~/.config` on Linux, `%APPDATA%\plaa` on
Windows, `~/Library/Application Support/plaa` on macOS), state and cache
under `plaa`, app id `io.github.scrambletools.plaa`. Builds without it
are development builds, which keep their own.

## What each system needs

- **Linux:** FFmpeg from the system, whose libraries change names from
  release to release, so each release of Debian, Ubuntu and Fedora gets
  a package of its own, built in a container of it:
  `plaa_X.Y.Z-1~debian13`, `~ubuntu24.04`, `~ubuntu26.04` and
  `plaa-X.Y.Z-1.fc44`. Each depends on what the programs link, as
  `dpkg-shlibdeps` and `rpmdeps` read them, so Fedora's package takes
  Fedora's FFmpeg or RPM Fusion's alike, and on what plaa loads as it
  runs: libass, the window system's libraries, and Vulkan with Mesa's
  drivers, whose software driver draws the picture where no graphics
  card can (without a Vulkan driver, wgpu turns to OpenGL, which panics
  configuring the window under some systems, as on Ubuntu 26.04 without
  a GPU); the Arch package only suggests a Vulkan driver, as pacman
  would otherwise pick NVIDIA's for anyone without one, and Arch's
  OpenGL draws where there is none. AVB needs `CAP_NET_RAW`, held only
  by `/usr/lib/plaa/plaa-netraw`, which opens raw sockets for plaa; the
  packages set it as they install (`setcap`, from libcap), recommend
  linuxptp, whose ptp4l the AVB talker needs running gPTP, and say when
  no wired interface has a PTP hardware clock. They bring plaa's ptp4l
  service, `plaa-ptp4l@.service`, its gPTP configuration in
  `/usr/share/plaa`, and a polkit rule letting the person at the
  computer start it, as plaa does where no ptp4l runs; it conflicts with
  triib's units, which serve plaa as well. There is no plain
  archive, AppImage or Flatpak, as none can carry the capability, and an
  archive would hold one release's FFmpeg names.
- **macOS:** FFmpeg and libass, from Homebrew, inside plaa.app, built on
  the oldest macOS supported, as Homebrew builds them for the macOS it
  runs on; the app claims the newest macOS any of its parts needs, and
  the installer refuses an older one. `Contents/Resources/licenses/
  libraries.txt` names each library, its version, license and source,
  and the release carries those sources. For AVB, plaa needs the BPF
  devices, `/dev/bpf*`, which are root's alone at every start: the
  installer package puts `plaa.app` in /Applications, adds the user
  installing it to the `access_bpf` group, and installs the launch daemon
  `io.github.scrambletools.plaa.bpf`, which opens the devices to that
  group at every start, as triib's and Wireshark's ChmodBPF do with the
  same group. `sudo "/Library/Application Support/plaa/uninstall"`
  removes it all. As it opens, the installer warns when the Mac has no
  AVB Ethernet port.
- **Windows:** FFmpeg's DLLs, BtbN's shared GPL build
  (https://github.com/BtbN/FFmpeg-Builds), beside plaa.exe, with
  FFmpeg's license, and libass with the libraries it needs (FreeType,
  HarfBuzz, FriBidi and theirs), built as DLLs by vcpkg at the commit
  the workflow pins, each with its license in `licenses`. They use the
  Visual C++ runtime, whose DLLs a clean Windows lacks, so those go
  beside them too, keeping Microsoft's signatures. The MSI installs for
  the current user, with no administrator prompt, adds plaa to the
  Start menu, and offers it in Open With for video and audio files
  without taking them over.

The release offers the corresponding source of everything GPL it
carries: the macOS job attaches the sources Homebrew built from; the
Windows jobs attach the source archives vcpkg built from
(`*-windows-library-sources.zip`) and, for FFmpeg, FFmpeg's source at
the commit of the build, BtbN's scripts at theirs and each library
they build for x64 and ARM64, fetched as the scripts fetch them
(`*-windows-ffmpeg-sources.tar`).

The FFmpeg build is pinned by its tag, version and SHA-256 in
`release.yml` and `ci.yml`. BtbN keeps daily builds for 14 days and
each month's last build for two years, so pin a month's last build. To
move to a newer one, take its tag from BtbN's releases, the FFmpeg
version from its file names, FFmpeg's commit from that version's hash,
and the scripts' commit as the one BtbN tagged; then update the
SHA-256 of both zips. Before the first public release, check the
codecs the macOS and Windows builds carry against patents
(`docs/PLAN.md`, Risks).

## Steps

1. Update the version in `Cargo.toml`, the workspace version and the
   versions of the `plaa-*` crates under `[workspace.dependencies]`, and
   run `cargo check` so `Cargo.lock` follows; update `pkgver` in
   `packaging/arch/plaa/PKGBUILD` and add a `<release>` to the metainfo.
2. Write the version's section in `CHANGELOG.md`, as `## [X.Y.Z]`; the
   release notes are taken from it, or made by GitHub when there is none.
   Changes go under `## Unreleased` as they land, which the release
   renames to `## [X.Y.Z] - YYYY-MM-DD`.
3. Try the packaging without releasing: run the Release workflow by hand
   (Actions, Release, Run workflow), which builds every package as an
   artifact and runs the installer tests on them, or with "Packages to
   build" only one system's, such as
   `gh workflow run release.yml -f only=windows`.
4. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`. The workflow
   checks that the tag matches `Cargo.toml`, builds the packages and
   drafts a release with them, `SHA256SUMS`, and a `PKGBUILD` with the
   source checksum filled in, beside its `plaa.install`. Check the draft
   and publish it.
5. AUR: in a clone of `ssh://aur@aur.archlinux.org/plaa.git`, replace
   `PKGBUILD` with the one from the release and copy
   `packaging/arch/plaa/plaa.install`, run
   `makepkg --printsrcinfo >.SRCINFO`, build it once with `makepkg`,
   commit and push.

## Installer tests

GitHub's runners have far more installed than the computers plaa goes
to, which can hide what a package leaves out. So the release workflow
tests each package in a clean system, as someone installing it would
meet it, and drafts the release only once every test passes. Each test
installs the last release's package first, when there is an older one,
so the new one upgrades it, and removes plaa at the end. Each also plays
`packaging/tests/media/test-clip.webm`, a few seconds of VP9 and Opus,
which every FFmpeg decodes, Fedora's without patented codecs too, and
looks for the yellow, cyan and magenta bars of its test pattern in the
window.

| Package | Where | What |
|---|---|---|
| .deb, .rpm | Fresh containers of Debian 13, Ubuntu 24.04 and 26.04 and Fedora 44, x86_64 and ARM64, and of the Debian and Ubuntu releases for RISC-V under QEMU, each with its own package | `packaging/tests/linux/test-package.sh`: installs with the package's dependencies only, then checks its files, `CAP_NET_RAW`, the AVB check's warning, that every shared library is found, the version line and the FFmpeg it names as a regular user, raw Ethernet as that user (`plaa --interfaces`), and the window opening, drawing and playing the clip under a headless sway, with no sound device; then that removing leaves nothing |
| Arch | A fresh Arch container | `build-arch.sh` builds the PKGBUILD from this tree, then the same test |
| MSI | GitHub's Windows Server 2025 and Windows 11 ARM runners | `packaging/tests/windows/test-msi.ps1`: checks every DLL plaa, FFmpeg and libass import against those Windows itself provides, then installs and checks the files, the entry in Apps, the Start menu shortcut, Open With, the version line and the FFmpeg and libass it names, the window and the clip played; then that uninstalling leaves nothing |
| MSI, x64 | A Windows Server 2022 container, which has nothing installed | The same, without the window and the clip |
| .pkg | Fresh macOS 14, 15 and 26 runners | `packaging/tests/macos/test-pkg.sh`: Gatekeeper's verdict on a downloaded copy, that plaa and every library it carries link only macOS's own libraries and each other, look nowhere outside the app, and run on the oldest macOS the app claims, then installs and checks the app, the version line, the BPF launch daemon and access for the user, the window and the clip played; then that the uninstall script leaves nothing |

To run one by hand:

- Linux, with Docker: `packaging/linux/build.sh debian:13 dist`, then
  `packaging/tests/linux/run.sh debian:13
  dist/plaa_X.Y.Z-1~debian13_amd64.deb [previous.deb]`.
  `PLAA_AVB_CHECKOUT=../avb` builds against a local avb checkout, as for
  a revision not pushed yet.
- Windows, on a computer without plaa installed, as it installs and
  removes it: `powershell -ExecutionPolicy Bypass -File
  packaging\tests\windows\test-msi.ps1 -Msi plaa-X.Y.Z-x64.msi`. For a
  fully clean Windows 11, run it in Windows Sandbox.
- macOS, on a Mac without plaa installed, with sudo:
  `packaging/tests/macos/test-pkg.sh plaa-X.Y.Z-arm64.pkg`.

When a new DLL fails the Windows check, link it in or install it beside
plaa; add it to the script's list only when every Windows plaa supports
provides it.

## Windows code signing

Until releases are signed, Windows shows SmartScreen's warning for the
MSI and the programs. The workflow signs plaa.exe, the DLLs and the MSI
with Microsoft's Azure Artifact Signing once the repository variables
`AZURE_CLIENT_ID`, `AZURE_TENANT_ID`, `AZURE_SUBSCRIPTION_ID`,
`AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT` and
`AZURE_CERTIFICATE_PROFILE` are set, signing in with GitHub's OpenID
Connect token as the environment `release`, as triib's does
(`docs/RELEASING.md` there): the signing account, identity validation
and certificate profile can be shared, and plaa needs an app
registration of its own, with a federated credential for this
repository's `release` environment.

## macOS signing

As for triib: releases carry Scramble Tools' Developer ID signatures and
are notarized, so Gatekeeper opens a downloaded copy without asking, and
macOS keeps the microphone, camera and screen recording permissions
across updates. `bundle.sh` signs every library the app carries, then the
app, with the hardened runtime and the microphone and camera entitlements
(`packaging/macos/plaa.entitlements`); under the hardened runtime the app
loads only libraries signed by its own team, so a build signed ad hoc,
which names no team, also takes
`com.apple.security.cs.disable-library-validation`. It signs with the
identity in `PLAA_SIGN_IDENTITY` where it is set, else ad hoc, from the
keychain in `PLAA_SIGN_KEYCHAIN`, unlocked with
`PLAA_SIGN_KEYCHAIN_PASSWORD`, and the package with
`PLAA_INSTALLER_IDENTITY`; given `PLAA_NOTARY_KEY`, `PLAA_NOTARY_KEY_ID`
and `PLAA_NOTARY_ISSUER` it has the package notarized and staples the
ticket.

The release workflow's macOS job runs in the environment `release` and
reads the same variables and secrets as triib's: `MACOS_APP_IDENTITY`,
`MACOS_INSTALLER_IDENTITY`, `APPLE_API_KEY_ID` and `APPLE_API_ISSUER`;
`MACOS_APP_P12`, `MACOS_INSTALLER_P12`, `MACOS_P12_PASSWORD` and
`APPLE_API_KEY`. Until `MACOS_APP_IDENTITY` is set the job signs ad hoc;
once it is, it checks that the app and every library are signed by it,
the package by `MACOS_INSTALLER_IDENTITY`, that the package is notarized
with its ticket stapled, and that Gatekeeper accepts both.
