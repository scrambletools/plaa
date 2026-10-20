<img src="data/icons/hicolor/scalable/apps/io.github.scrambletools.plaa.svg" width="128" alt="The plaa icon: a play button on a screen, with sound coming from it">

# plaa

A fast, open source media player for Linux, Windows and macOS, similar to
QuickTime Player and IINA. Built in Rust with [iced](https://iced.rs) and
[FFmpeg](https://ffmpeg.org), sharing its look with
[prev](https://github.com/scrambletools/prev) and
[triib](https://github.com/scrambletools/triib) through
[scramble-ui](https://github.com/scrambletools/scramble-ui).

> **Status:** early development, not yet released. Files, streams and
> web videos play, with the controls floating over the picture, and
> files can be trimmed, split, turned and exported, and sound and the
> screen recorded; see the [development plan](docs/PLAN.md) for what
> comes next: recording a camera, casting, an AVB/Milan talker and
> listener, and an assistant.

## Features

### Available now

- **Plays nearly anything** FFmpeg reads: MP4, Matroska, WebM, MOV, AVI,
  MPEG-TS and the rest, with H.264, HEVC, AV1, VP9, ProRes and other
  video, and AAC, Opus, FLAC, MP3, AC-3 and other sound.
- **Streams** at addresses: HTTP, HLS, DASH, RTSP and SRT (Open Address,
  Ctrl+U, or an address on the command line), which buffer ahead and
  wait to read on rather than stutter over a slow network, marking on
  the timeline how far they have read. The addresses opened lately are
  offered again.
- **Web videos**, such as YouTube's and those of the many other sites
  [yt-dlp](https://github.com/yt-dlp/yt-dlp) reads, when it is
  installed: with their titles, channels, chapters and thumbnails, and
  playlists as plaa's own lists.
- **Quality** chosen from a stream's or a web video's, or sound alone,
  by default the best that fits the screen.
- **Hardware decoding** on the graphics card (VAAPI, CUDA or Vulkan on
  Linux, Direct3D 11 on Windows, VideoToolbox on macOS), 8K at 60 fps
  included, falling back to the processor.
- **Playlists**: files opened or dropped together, M3U and PLS lists,
  and in Settings, the other files in the opened file's folder; the next
  plays when one ends (Playlist, Ctrl+P; previous and next, P and N).
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
- **Seeking** by dragging the timeline, the picture following the
  handle exactly and only the way it goes; back and forward 5 and 10
  seconds, and frame by frame. Pointing at the timeline shows a picture
  from there, the time and the chapter.
- **Chapters** marked on the timeline, in a menu, and a key away
  (Page Up, Page Down).
- **A-B loop** (B, then B again where it should end) besides looping the
  file.
- **Speed** from a quarter to three times, keeping the sound's pitch.
- **Audio and subtitle tracks**, picked by preferred languages or from the
  menu: text subtitles (SubRip, WebVTT, MP4 text and more), ASS and SSA
  drawn as styled by libass when the system has it, Blu-ray, DVD and DVB
  picture subtitles, and subtitle files beside the video, as
  `Film.en.srt` or in a `Subs` folder.
- **Picture in picture** (Ctrl+Shift+P): a small window kept over others
  in a corner, where the system lets apps place windows.
- **Zoom in** on the picture with Ctrl and the wheel, at the pointer, up
  to 16 times, and drag it to look around; zooming all the way out fits
  it again.
- **The system's media controls**: media keys, and what plays with its
  cover in the system's panels, through MPRIS on Linux (and
  `playerctl`), the media flyout on Windows and Now Playing on macOS.
- **At home on each system**: the menu bar on macOS, title bars in
  plaa's colors on macOS and Windows, and files dropped on windows on
  Wayland as elsewhere.
- **The screen stays on** while video plays, and the computer awake
  while sound does.
- **Sound output** chosen in Settings, through the sound server on Linux
  (PipeWire or PulseAudio).
- **Trim** (Ctrl+T) between two handles over a filmstrip, the part kept
  playing over and over meanwhile; **split** (Ctrl+Y) into clips, which
  can be moved, removed and trimmed one by one; **rotate** and **flip**
  (Ctrl+Shift+L, R, H and V); with undo and redo. The edit plays as it
  will be, clip after clip without a gap.
- **Export** (Ctrl+E) to H.264, HEVC or AV1 with AAC sound, as edited,
  at the file's size or smaller: on the graphics card's encoder where
  one opens (NVENC, AMF, Quick Sync, VAAPI or VideoToolbox), else on the
  processor (x264, x265, SVT-AV1), keeping HDR video's colors in HEVC
  and AV1. Or the sound alone, as AAC, MP3 or Opus at a bit rate chosen,
  or lossless as FLAC, ALAC or WAV in 16 or 24 bits. Closing with
  changes not exported asks first.
- **Record sound** (Ctrl+Shift+N) from a microphone, or from what the
  computer plays where the system offers it (the sound server's
  monitors on Linux, every output on Windows and macOS), with a level
  meter,
  in any of the export's sound formats, to the Recordings folder; the
  window then plays the recording, ready to trim and export.
- **Record the screen** (Ctrl+Alt+N): a screen, a window or (on Linux)
  a region, as the system's picker offers on Linux and plaa lists them
  on Windows and macOS, with the pointer, and the sound
  of a microphone or of what the computer plays if chosen, as H.264,
  HEVC or AV1 on the graphics card's encoder where one opens, at 24,
  30, 60 or 120 pictures a second or every picture the screen sends;
  pictures dropped because the encoder fell behind show beside the
  size. On macOS plaa's own windows are left out of a screen's
  recording; elsewhere, recording a screen, the recording window gets
  out of the way, after saying how to stop: click the red dot plaa puts
  in the tray (on Linux), or open plaa again. A recording also ends
  when the screen is no longer shared or the window recorded closes.
- **Loop**, **resume where you left off**, and **save the frame** shown as
  a PNG.
- **Inspector** (Ctrl+I) with the file's format, streams, codecs, frame
  rate, how it decodes, and its tags.
- **Settings** (Ctrl+,): light or dark, the system accent color, the
  floating controls, the graphics card, resuming, streams' quality,
  track languages and folders, saved in `~/.config/plaa.toml` (`%APPDATA%\plaa\plaa.toml` on
  Windows, `~/Library/Application Support/plaa/plaa.toml` on macOS).
- **One window per file** and one running plaa: files opened again, from
  the file manager or the command line, go to the running one.

### Planned

Recording a camera, a visualizer, casting, the AVB/Milan
entity, the assistant and MCP, packages and translations. The [plan](docs/PLAN.md) has the
order.

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

ASS subtitles are drawn by libass when the system has it, as most Linux
systems do through FFmpeg (`brew install libass` on macOS); plaa loads
it as it runs, so building needs nothing more.

On macOS, install the Xcode Command Line Tools and `brew install ffmpeg
pkgconf`, and set `LIBCLANG_PATH` to their `usr/lib` folder
(`/Library/Developer/CommandLineTools/usr/lib`) when Xcode itself is not
installed. On Windows, install the Visual Studio C++ build tools and LLVM,
set `LIBCLANG_PATH` to LLVM's `bin` folder, unpack a shared FFmpeg build
(such as [BtbN's](https://github.com/BtbN/FFmpeg-Builds/releases)) and
set `FFMPEG_DIR` to it, with its `bin` folder on the `PATH`.

Builds made with plain `cargo build` are development builds: they keep
their own settings (`~/.config/plaa-dev.toml`) and data under
`plaa-dev`, show "(dev)" in window titles, and run alongside an installed
copy. Packages are built with `PLAA_PRODUCTION=1`.

`PLAA_FFMPEG_LOG=1` shows FFmpeg's own messages,
`cargo run --release -p plaa-media --example play -- <file>` plays a file
without a window and says what it showed, and the `export` example
exports one as the Export dialog does.

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
| Loop, A-B loop | Ctrl+L, B |
| Previous, next in the playlist | P, N, Ctrl+Left, Ctrl+Right |
| Previous, next chapter | Page Up, Page Down |
| Playlist | Ctrl+P |
| Picture in picture | Ctrl+Shift+P |
| Zoom in, out at the pointer | Ctrl+wheel up, down |
| Look around while zoomed | Drag the picture |
| Save frame as picture | S |
| Trim, split clip | Ctrl+T, Ctrl+Y |
| Trim from, to the playhead | I, O (while trimming) |
| Remove the chosen clip | Delete (while arranging clips) |
| Rotate left, right | Ctrl+Shift+L, Ctrl+Shift+R |
| Flip horizontally, vertically | Ctrl+Shift+H, Ctrl+Shift+V |
| Undo, redo | Ctrl+Z, Ctrl+Shift+Z |
| Export | Ctrl+E |
| Record sound | Ctrl+Shift+N (Space starts and stops) |
| Record the screen | Ctrl+Alt+N (Space starts and stops) |
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
normal-speed, loop, screenshot, subtitles, audio-track, to-start,
previous, next, playlist, previous-chapter, next-chapter, ab-loop,
picture-in-picture, trim, split, rotate-left, rotate-right,
flip-horizontal, flip-vertical, undo, redo, revert, export,
record-sound and record-screen. A shortcut the settings give one action is taken
from any other that had it by default.

## License

plaa is licensed under the [GNU General Public License v3.0 or
later](LICENSE). The fonts it uses come with scramble-ui under their own
licenses: Roboto Flex under the SIL Open Font License 1.1 and Material
Symbols under the Apache License 2.0.
