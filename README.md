<img src="data/icons/hicolor/scalable/apps/io.github.scrambletools.plaa.svg" width="128" alt="The plaa icon: a play button on a screen, with sound coming from it">

# plaa

A fast, open source media player for Linux, Windows and macOS, similar to
QuickTime Player and IINA. Built in Rust with [iced](https://iced.rs) and
[FFmpeg](https://ffmpeg.org), sharing its look with
[prev](https://github.com/scrambletools/prev) and
[triib](https://github.com/scrambletools/triib) through
[scramble-ui](https://github.com/scrambletools/scramble-ui).

> **Status:** early development, not yet released. Files and streams
> play, with the controls floating over the picture; see the
> [development plan](docs/PLAN.md) for what comes next: editing and
> export, recording, casting, an AVB/Milan talker and listener, and an
> assistant.

## Features

### Available now

- **Plays nearly anything** FFmpeg reads: MP4, Matroska, WebM, MOV, AVI,
  MPEG-TS and the rest, with H.264, HEVC, AV1, VP9, ProRes and other
  video, and AAC, Opus, FLAC, MP3, AC-3 and other sound.
- **Streams** at addresses: HTTP, HLS, DASH, RTSP and SRT (Open Address,
  Ctrl+U, or an address on the command line).
- **Hardware decoding** on the graphics card (VAAPI, CUDA or Vulkan on
  Linux, Direct3D 11 on Windows, VideoToolbox on macOS), falling back to
  the processor.
- **Pictures drawn on the graphics card**, with BT.601, BT.709 and
  BT.2020 colors, 8 to 16 bits, HDR10 and HLG tone mapped for ordinary
  screens, and phone videos turned upright.
- **Sound in step with the picture**, following the output device's own
  clock.
- **Controls floating over the picture**, as M3 floating toolbars that
  slide away once the pointer rests or leaves the window, and come back
  when it moves; Settings can dock them above and below instead. Sound
  files show their cover art, title, artist and album, with the controls
  docked.
- **Seeking** by dragging the timeline, to keyframes while dragging and
  exactly where it is let go; back and forward 5 and 10 seconds, and frame
  by frame.
- **Speed** from a quarter to three times, keeping the sound's pitch.
- **Audio and subtitle tracks**, picked by preferred languages or from the
  menu; text subtitles (SubRip, ASS, WebVTT, MP4 text and more).
- **Loop**, **resume where you left off**, and **save the frame** shown as
  a PNG.
- **Inspector** (Ctrl+I) with the file's format, streams, codecs, frame
  rate, how it decodes, and its tags.
- **Settings** (Ctrl+,): light or dark, the system accent color, the
  floating controls, hardware decoding, resuming, track languages and
  folders, saved in `~/.config/plaa.toml` (`%APPDATA%\plaa\plaa.toml` on
  Windows, `~/Library/Application Support/plaa/plaa.toml` on macOS).
- **One window per file** and one running plaa: files opened again, from
  the file manager or the command line, go to the running one.

### Planned

Playlists, chapters, picture subtitles and ASS styling, timeline
thumbnails, picture in picture, media keys, yt-dlp, trimming and export,
recording, a visualizer, casting, the AVB/Milan entity, the assistant and
MCP, packages and translations. The [plan](docs/PLAN.md) has the order.

## Building

plaa builds with Rust 1.95 or newer and FFmpeg's development files.

```sh
# Arch
sudo pacman -S clang pkgconf ffmpeg alsa-lib fontconfig freetype2 libxkbcommon wayland
# Debian and Ubuntu
sudo apt install clang libclang-dev pkg-config libasound2-dev libfontconfig1-dev \
  libfreetype-dev libxkbcommon-dev libwayland-dev libavcodec-dev libavformat-dev \
  libavutil-dev libavfilter-dev libswscale-dev libswresample-dev
# Fedora (FFmpeg from RPM Fusion)
sudo dnf install clang pkgconf alsa-lib-devel fontconfig-devel libxkbcommon-devel \
  wayland-devel ffmpeg-devel
```

Then build and run:

```sh
cargo build --release
./target/release/plaa video.mkv
```

On macOS, install the Xcode Command Line Tools and `brew install ffmpeg
pkgconf`. On Windows, install the Visual Studio C++ build tools and LLVM,
set `LIBCLANG_PATH` to LLVM's `bin` folder, unpack a shared FFmpeg build
(such as [BtbN's](https://github.com/BtbN/FFmpeg-Builds/releases)) and
set `FFMPEG_DIR` to it, with its `bin` folder on the `PATH`.

Builds made with plain `cargo build` are development builds: they keep
their own settings (`~/.config/plaa-dev.toml`) and data under
`plaa-dev`, show "(dev)" in window titles, and run alongside an installed
copy. Packages are built with `PLAA_PRODUCTION=1`.

`PLAA_FFMPEG_LOG=1` shows FFmpeg's own messages, and
`cargo run --release -p plaa-media --example play -- <file>` plays a file
without a window and says what it showed.

### Changing scramble-ui alongside

plaa pins scramble-ui by commit. To change both together,
`cargo --config .cargo/scramble-ui-local.toml build` builds against the
scramble-ui checkout beside this one (for clippy, put the option after the
subcommand: `cargo clippy --config .cargo/scramble-ui-local.toml`). The
file is git-ignored and writes local paths into `Cargo.lock`, so once
scramble-ui is pushed, move the pin in `Cargo.toml` to its new commit and
build once without the option before committing.

## Keyboard shortcuts

On macOS, use ⌘ in place of Ctrl.

| Action | Shortcut |
|---|---|
| Play or pause | Space, K |
| Back, forward 5 seconds | Left, Right |
| Back, forward 10 seconds | J, L, Shift+Left, Shift+Right |
| Previous, next frame | `,` `.` |
| Back to the start | Home |
| Volume up, down | Up, Down |
| Mute | M |
| Slower, faster, normal speed | `[`, `]`, Backspace |
| Next subtitles, next audio track | C, A |
| Full screen | F, F11 (Escape leaves it) |
| Loop | Ctrl+L |
| Save frame as picture | S |
| Open, open address | Ctrl+O, Ctrl+U |
| Inspector | Ctrl+I |
| Settings | Ctrl+, |
| Close window, quit | Ctrl+W, Ctrl+Q |

Shortcuts can be changed in the `[keys]` table of the settings file, as in
prev: each entry names an action and gives it a shortcut, a list of them,
or an empty list for none, and `linux`, `windows` and `macos` tables
apply only there.

```toml
[keys]
play-pause = ["Space", "P"]
screenshot = "Ctrl+Shift+S"
```

The actions are open, open-address, close-window, quit, settings, info,
play-pause, back, forward, back-more, forward-more, previous-frame,
next-frame, volume-up, volume-down, mute, full-screen, faster, slower,
normal-speed, loop, screenshot, subtitles, audio-track and to-start.

## License

plaa is licensed under the [GNU General Public License v3.0 or
later](LICENSE). The fonts it uses come with scramble-ui under their own
licenses: Roboto Flex under the SIL Open Font License 1.1 and Material
Symbols under the Apache License 2.0.
