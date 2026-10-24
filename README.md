<img src="data/icons/hicolor/scalable/apps/run.plaa.svg" width="128" alt="The plaa icon: a play button on a disc of scan lines in sunset colors">

# plaa

A fast, open source media player for Linux, Windows and macOS, similar to
QuickTime Player and IINA. Built in Rust with [iced](https://iced.rs) and
[FFmpeg](https://ffmpeg.org), sharing its look with
[prev](https://github.com/scrambletools/prev) and
[triib](https://github.com/scrambletools/triib) through
[scramble-ui](https://github.com/scrambletools/scramble-ui).

Download it from [plaa.run](https://plaa.run#download) or the
[release page](https://github.com/scrambletools/plaa/releases/latest).

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
  when it moves, over a sound file's cover as over a picture; the
  toolbar or Settings can dock them above and below instead. Sound files
  show their cover art, title, artist and album.
- **A visualizer** under a sound file's cover: the spectrum of what is
  heard, as bars that rise with it and fall back slowly, or the file's
  waveform from start to end, the part played in the accent color,
  which a click or drag seeks along. Or Prism, a scene in three
  dimensions filling the window, the controls floating over it as over
  a video: a lace orb in thin-film colors around a core that swells on
  each beat, ringed by glass shards rising with the spectrum, over a
  rippling grid of light, in a nebula. The toolbar chooses between
  them, or the cover alone.
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
  will be, clip after clip without a gap. Agents and the assistant can
  also put in parts of other files and play another file's sound or
  picture over part of it, such as music under a video; the edit plays
  and exports those too.
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
  meter, a quiet input made up to 30 dB louder (kept for each input) and
  its level evened out automatically if chosen, in any of the export's
  sound formats, to a folder of its own, Music unless Settings choose
  another; once it stops, plaa asks what to name it, and the window then
  plays the recording, ready to trim and export.
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
- **Record a camera** (Ctrl+Alt+M), as QuickTime Player's New Movie
  Recording: the camera's picture shows, mirrored, as it records, with
  the sound of a microphone if chosen, in the screen recording's formats
  and rates; the recording itself is as the camera sees. Screen and
  camera recordings go to a folder apart from sound's, Videos (Movies
  on macOS) unless Settings choose another, and are named the same way.
- **Send what plays over AVB** (Milan): on Linux plaa becomes a talker
  entity of its own, named as Settings say, with a stereo stream on gPTP
  time in the formats the avb crates set for every endpoint: the AVB
  community's Basic and Extended Audio profiles' AAF, in class A or B at
  48, 96 or 192 kHz, and AM824 as MOTU's, Apple's and PreSonus's
  endpoints send, of 2 channels or 8. Any Milan controller can connect
  it, and it sends the sound of the window last played once a listener
  is ready; on a Mac it sends through the Mac's own AVB entity, as triib
  does.
- **Network Audio**: the listeners on the interface Settings choose that
  can play what plays, by name, each with one button: play there, stop,
  or the formats both share, the listener's format following plaa's.
  The one chosen shows how it takes its stream: from which talker, its
  media clock lock, format, stream ID, destination, latency and counters,
  kept up to date as other controllers, such as triib's, change them.
- **Connect**, beside Open: the talkers plaa can listen to, those sending
  in a format it plays, then those that can change to one, changed as
  plaa connects, with the other listeners that would cut off. The
  stream plays in a window as a file plays, and its inspector shows the
  talker and the stream as it comes, and changes its format.
- **AVB Lite** where the switch has no AVB: plaa finds that no bridge
  reserves its streams and falls back, as triib's entity service does,
  sending each listener, up to five, a copy of its own, and playing a
  stream sent to this computer, on its own or through triib's service.
- **Cast** to a Chromecast, or a TV, speaker or program that plays DLNA
  (UPnP AV), found on the network as the Cast menu opens: plaa serves
  the file to it, or hands it a stream's address, and its controls and
  timeline play, pause and seek there until casting stops, when the
  window goes on from where it got to.
- **Loop**, **resume where you left off**, and **save the frame** shown as
  a PNG.
- **Inspector** (Ctrl+I) with the file's format, streams, codecs, frame
  rate, how it decodes, and its tags.
- **Settings** (Ctrl+,): light or dark, the system accent color, the
  floating controls, the graphics card, resuming, streams' quality,
  track languages and folders, saved in `~/.config/plaa.toml` (`%APPDATA%\plaa\plaa.toml` on
  Windows, `~/Library/Application Support/plaa/plaa.toml` on macOS).
- **38 languages**, the system's or the one chosen in Settings, right to
  left for Arabic, Hebrew, Persian and Urdu.
- **One window per file** and one running plaa: files opened again, from
  the file manager or the command line, go to the running one.
- **An assistant beside what plays**: ask about it in a chat panel, and
  the assistant looks with plaa's own tools, at the status, the file's
  streams, its pictures at any time, how loud it is and its subtitles'
  text, and plays, pauses, seeks, changes the sound, or edits it for
  you. Use a local model that stays on your computer, such as
  Ollama's, or a cloud model from Anthropic, OpenAI or Google with your
  own key, kept in the system's keychain. The models and keys are shared
  with the other Scramble Tools apps, such as prev: set up in one, they
  are there in the others, kept in `~/.config/scramble/assistant.toml`
  (`%APPDATA%\scramble\` on Windows, `~/Library/Application Support/scramble/`
  on macOS); each app chooses which one it uses.
- **AI agents**: `plaa --mcp` serves plaa's tools over MCP, so Claude
  Code or a local model in an MCP client can see what plays in the plaa
  you are using and read any part of it, and play, pause, seek, loop,
  change the volume and tracks, open files and close windows, edit and
  export, asking you first. plaa bundles
  no model and listens on no network. See
  [Using plaa with AI agents](#using-plaa-with-ai-agents).

### Planned

Video over AVB, as IEEE 1722 CVF streams, and an icon in Windows'
notification area while a screen recording runs hidden. The
[plan](docs/PLAN.md) says how plaa is built and what it will do.

## Packages

Each release has packages for Debian 13, Ubuntu 24.04 and 26.04,
Fedora 44 and Arch (x86_64 and ARM64, and RISC-V for Debian and Ubuntu),
Windows (an MSI and a zip, x64 and ARM64) and macOS (Apple Silicon,
macOS 14 and later), each installed, upgraded, run, made to play a video
and removed in a clean system before the release goes out. On Linux
the packages use the system's FFmpeg, each built for its release; on
Windows and macOS they carry their own. Download them from
[plaa.run](https://plaa.run#download), which says how to install each,
or the [release page](https://github.com/scrambletools/plaa/releases/latest).
[Releasing](docs/RELEASING.md) says how they are made and checked, and
[THIRD-PARTY.md](docs/THIRD-PARTY.md) names what they are built from.

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
`plaa --interfaces` says whether plaa may use raw Ethernet and which
wired interfaces could send an AVB stream.

`PLAA_FFMPEG_LOG=1` shows FFmpeg's own messages, `PLAA_TEST_CAMERA=1`
offers FFmpeg's test picture as a camera, first, for trying camera
recording without one,
`cargo run --release -p plaa-media --example play -- <file>` plays a file
without a window and says what it showed, and the `export` example
exports one as the Export dialog does. Both take `--insert`, `--sound`
and `--picture` to try edits that mix files, and `--picture-filter` and
`--sound-filter` to try filters.

### AVB on Linux

Raw Ethernet needs `CAP_NET_RAW`, which plaa itself never holds, so the
desktop's portals keep answering it: a tiny helper built beside it,
`plaa-netraw`, holds the capability, opens the raw sockets and passes
them to plaa. Grant it after each build with `sudo setcap cap_net_raw+ep
target/release/plaa-netraw` (packages will). Sending
also needs ptp4l running gPTP on the interface, which must have a PTP
hardware clock, as triib's units run it. Where none runs, plaa starts
its own, `plaa-ptp4l@<interface>`, which its packages install with a
polkit rule letting the person at the computer start it, and which
gives way to triib's; Network Audio says when ptp4l keeps no time for
plaa's stream, and why. For dev builds, `scripts/dev-ptp4l.sh` installs
the services and the rule under `/usr/local` and `/etc`, and
`scripts/dev-ptp4l.sh remove` takes them out before a package goes in. Where triib's entity service
runs on the interface, plaa reserves its stream in the service's MRP
participant, which the service shares, as two on one port withdraw each
other's declarations; beside an older service, which does not share it,
plaa's entity steps aside. Without the service, plaa runs MSRP and MVRP
itself, and where no bridge reserves its streams it falls back to AVB
Lite, moving ptp4l to the AVB Lite PTP profile,
`plaa-ptp4l-lite@<interface>`, until the link goes down. Meanwhile it
holds an interface that filters VLANs in hardware, as the I210 does,
promiscuous, so it hears CVU SRP's commands in the media VLAN.

The AVB crates come from [scrambletools/avb](https://github.com/scrambletools/avb),
pinned by commit. To change them alongside, `cargo --config
.cargo/avb-local.toml build` builds against a checkout beside this one,
from a git-ignored file with `[patch."https://github.com/scrambletools/avb"]`
path entries for `atdecc`, `avb-lite`, `avb-mrp`, `avb-net` and
`avb-stream`.

### AVB on a Mac

On a Mac, plaa's talker is the Mac's own AVB entity, which macOS runs
once AVB is on for the port in Audio MIDI Setup's Network Device
Browser. plaa's controller reaches it through Apple's AudioVideoBridging
framework, as triib does, and while one of its stream outputs is
connected, macOS makes a Core Audio device of it, which plaa plays what
it plays into, in its first two channels. Finding the other entities
takes the BPF devices, which the installer opens to the user, as
triib's does.

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
| Record a camera | Ctrl+Alt+M (Space starts and stops) |
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
record-sound, record-screen and record-camera. A shortcut the settings give one action is taken
from any other that had it by default.

## Using plaa with AI agents

Add plaa to an agent as an MCP server that runs `plaa --mcp`. For Claude
Code:

```sh
claude mcp add plaa -- plaa --mcp
```

Where `plaa` is not on the path, give its whole path:
`/Applications/plaa.app/Contents/MacOS/plaa` on macOS, or
`%LOCALAPPDATA%\Programs\plaa\plaa.exe` on Windows. MCP clients for
local models take the same command, often as
`{ "mcpServers": { "plaa": { "command": "plaa", "args": ["--mcp"] } } }`.
`plaa --mcp` starts plaa when it is not running, and relays the agent's
calls to it over a socket only you can open (a named pipe on Windows).

The first time an agent connects, plaa asks whether to let it in. The
Agents tab of Settings turns outside control off, lists the agents
allowed, and has a switch for each kind of tool (seeing what plays,
controlling playback, opening files or closing windows, and editing or
exporting) that makes plaa ask before an agent's call runs. What the
agent sees, pictures and sound among it, goes to its model.

Times are in seconds of what plays: the edited movie, once the file is
edited. Every edit an agent makes shows in the window as it is made,
and undo takes it back. A call that takes more than two seconds, such
as reading many pictures, measuring a long film's sound or exporting,
tells the agent how far it has come, and shows it over the window.

`apply_filter` takes FFmpeg filter graphs as `-vf` and `-af` do, such
as `hue=s=0` for black and white, `eq=contrast=1.2`, `crop=ih:ih`,
`drawtext=text='Title'`, `highpass=f=80,loudnorm` or `afftdn`, for the
whole movie or a part. plaa checks each graph before any of its filters
starts: only filters that change each frame as it comes are allowed,
none given a file, folder, address or model, so a graph can neither
read nor write files, reach a device or the network, nor change when
things play. The filters play at once, export, and show in what
`get_frames` and `get_sound` give.

| Tool | What it does |
|---|---|
| `list_windows` | plaa's windows: their numbers, titles, what they play and how |
| `get_status` | A window's state, position, length, speed, volume, loop and tracks chosen |
| `get_info` | The file's format, tags, chapters and streams, and the window's list |
| `get_frame` | The picture shown, as a PNG at most 1280 pixels across |
| `play`, `pause`, `seek`, `set_speed` | Playback |
| `set_volume`, `set_loop`, `choose_tracks` | The sound, looping, and the sound track and subtitles |
| `next_item`, `previous_item` | Moving through a window's list |
| `set_fullscreen`, `focus_window` | The window |
| `open`, `close_window` | Opening a file or address, and closing a window |
| `get_frames` | Pictures at any times, or spread over a part, as JPEGs, as edited |
| `get_levels` | How loud each step is, and where it is silent |
| `get_subtitles` | The subtitles' lines and their times, as they show in the edit |
| `get_sound` | Up to a minute of sound as a WAV, for models that hear |
| `get_edit` | The clips, the files they play, what replaces their picture or sound |
| `trim`, `cut`, `split`, `move_clip` | Keeping a part, taking one out, splitting and ordering clips |
| `insert` | Putting in part of another file, or of the same one |
| `replace` | Another file's sound or picture over a part, or silence, or black |
| `mix_in`, `remove_mix` | Another file's sound mixed in with what plays, at a volume, with fades |
| `apply_filter`, `remove_filters` | FFmpeg filters on the picture or sound, all of it or a part, checked first |
| `turn_picture`, `undo` | Turning or flipping the picture; undo, redo and undoing all |
| `export` | Writing the edit to a new file, as the Export dialog does |

## License

plaa is licensed under the [GNU General Public License v3.0 or
later](LICENSE). The fonts it uses come with scramble-ui under their own
licenses: Roboto Flex under the SIL Open Font License 1.1 and Material
Symbols under the Apache License 2.0.
