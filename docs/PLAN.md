# plaa: design and plan

plaa is a fast, open source media player for Linux, Windows and macOS,
similar to QuickTime Player and IINA. It plays nearly any video or audio
file or stream through FFmpeg, and shares its look and its stack with
[prev](https://github.com/scrambletools/prev) and
[triib](https://github.com/scrambletools/triib) through
[scramble-ui](https://github.com/scrambletools/scramble-ui). The
[README](../README.md) lists what it does today; this document covers how
it is built, what it will do, and in what order.

## Guiding priorities

1. Open source: plaa and every dependency are open source, checked in CI.
2. Performance: instant open, smooth playback of large videos on modest
   machines, low latency controls.
3. Small footprint: few dependencies, a lean binary and memory use.

QuickTime Player and IINA are the reference for features and behaviour,
unless something is not possible on one of the systems plaa runs on; the
look follows Material Design 3, as in prev.

## Decisions

| Area | Decision |
|---|---|
| License | GPL-3.0-or-later, so packages can bundle a full FFmpeg build, GPL parts included |
| Platforms | As prev: Linux (Wayland first, X11 supported), Windows 10 and 11, macOS 11 and later on Apple Silicon |
| Language | Rust, stable toolchain, edition 2024, minimum Rust 1.95 |
| GUI | iced 0.14 with scramble-ui's Material 3 components; the picture through a wgpu shader widget of plaa's own |
| Media engine | FFmpeg's libraries (libavformat, libavcodec, libavfilter, libswresample, libswscale) through `ffmpeg-next`, all inside `plaa-media` ([ADR 0001](decisions/0001-media-engine.md)) |
| Hardware decoding | Through FFmpeg: VAAPI, then CUDA, then Vulkan on Linux; D3D11VA, CUDA, then DXVA2 on Windows; VideoToolbox on macOS. Pictures are copied to memory and converted on the graphics card; the processor takes over when no device opens |
| Sound out | cpal: ALSA (and PipeWire through it) on Linux, WASAPI on Windows, Core Audio on macOS. The device's clock drives playback |
| Color | Y'CbCr to RGB in the shader, BT.601, BT.709 and BT.2020, limited and full range, 8 to 16 bits. PQ and HLG are tone mapped for standard screens, BT.2020 mapped to BT.709 |
| Subtitles | Text subtitles (SubRip, ASS, WebVTT, MP4 text and the rest FFmpeg reads) drawn by plaa. Picture subtitles and ASS styling come next |
| Windows | One window per file, one running instance; a start window when nothing is open |
| Overlay | For video, M3 floating toolbars over the picture by default, hiding once the pointer rests (2 s, adjustable) or leaves the window; Settings docks them. Sound always has them docked |
| Settings | `~/.config/plaa.toml` (`%APPDATA%\plaa\plaa.toml`, `~/Library/Application Support/plaa/plaa.toml`), with a `[keys]` table as prev's; where files were left off in the state folder |
| Interface languages | Fluent files in `i18n/`, English first; the 38 languages prev has once the interface settles |
| Assistant and MCP | As prev: one tool registry for MCP clients (`plaa --mcp`) and an assistant panel with the user's own model |
| AVB and Milan | plaa's own Milan entity, with triib's `atdecc`, `avb-mrp` and `avb-net` crates pinned by revision: a talker of what plays, and a listener to play or record a stream. Linux first, with ptp4l as triib runs it; audio first (AAF, AM824), IEEE 1722 CVF video later |
| Distribution | As prev, once the features settle: .deb, .rpm, AppImage, tarball, Flatpak, PKGBUILD, MSI and zip, disk image |

## Scope

Playback as QuickTime Player and IINA have it, and beyond them:

- trimming, splitting, rotating and exporting, as QuickTime's Edit menu;
- streams and web video: HTTP, HLS, DASH, RTSP and SRT through FFmpeg,
  and web pages through yt-dlp when it is installed;
- an audio visualizer for files without pictures;
- casting to Chromecast and DLNA/UPnP renderers;
- recording from a microphone, a camera, the screen and AVB streams;
- being an AVB/Milan talker and listener;
- the assistant panel and MCP server prev has.

Not in scope: a media library, DVD and Blu-ray menus, editing beyond
trimming and splitting, and Intel Macs.

## Milestones

1. **Playback core** (done): FFmpeg reading and decoding on threads,
   hardware decoding, sound with the device's clock, pictures drawn by the
   shader with HDR tone mapping, seeking, speed with the pitch kept, the
   floating and docked controls, audio and subtitle tracks, text
   subtitles, the inspector, saved frames, resuming, Settings, one
   running instance, CI on the three systems.
2. **Playback complete**: playlists (the folder's other files, and lists
   dropped in), chapters on the timeline, picture subtitles and ASS
   styling (libass), subtitle files beside the video, thumbnails over the
   timeline, A-B loops, picture in picture, choosing the output device,
   media keys and the system's now playing (MPRIS, SMTC, Now Playing),
   keeping the screen awake, drag and drop on Wayland (prev's
   smithay-clipboard patch), the macOS menu bar, title bar colors on
   Windows and macOS.
3. **Network**: yt-dlp, choosing a stream's quality, a buffering
   indicator, recent addresses.
4. **Edit and export**: trim, split, rotate and flip, then export to
   H.264, HEVC, AV1 or audio only, on hardware encoders where there are.
5. **Recording**: microphone, camera, screen (the ScreenCast portal and
   PipeWire, ScreenCaptureKit, Windows.Graphics.Capture), AVB streams.
6. **Visualizer**: waveform, spectrum and cover art for sound.
7. **AVB and Milan**: the entity, talker and listener.
8. **Casting**: Chromecast and DLNA/UPnP.
9. **Assistant and MCP**.
10. **Release**: packages, translations, signing, the website.

## Architecture

### Workspace

```
crates/
  plaa/          the app: iced windows, the picture's shader, controls,
                 Settings, dialogs, single instance
  plaa-media/    the engine: FFmpeg reading and decoding, sound out, the
                 clock, pictures and subtitles for the interface
  plaa-store/    settings, where files were left off, atomic writes
```

Later crates: `plaa-avb` (the Milan entity), `plaa-cast`, `plaa-assist`.

### Engine

A `Player` plays one source. Opening it starts a reading thread, which
opens the source with FFmpeg (an interrupt callback lets closing cut a
stalled network read short), reads its streams and starts a thread per
decoder:

- **Reading**: reads packets and hands each to its stream's queue,
  keeping a few seconds or a few megabytes ahead, and takes commands:
  seeks and track changes. Text subtitles are small enough to decode here.
- **Video**: decodes, on the graphics card when it can, copies hardware
  pictures to memory, converts the few formats the shader does not read
  with swscale, and queues six pictures ahead of the clock.
- **Audio**: decodes and runs a filter graph (`atempo` for speed with
  the pitch kept, then `aformat` for the device's rate, channels and
  float samples), queueing 400 ms ahead.
- **Output**: cpal's callback plays the queue and sets the clock to the
  media time of what it plays, heard after the device's latency.

The clock follows the sound while there is sound: it stalls while sound
is missing, after a seek or while a stream buffers, so pictures wait
rather than run ahead. Without sound, it starts from the first picture
after a seek and runs on the system's clock. Every seek bumps a serial
that packets, pictures and samples carry, so anything from before it is
dropped wherever it is. Exact seeks also drop what decodes before the
target; dragging the timeline seeks to keyframes and lands exactly when
let go.

### Pictures

The picture is a shader widget that asks the player for the picture due
each time it draws, and asks to be drawn again when the next one is due,
so playback runs without the window's interface being rebuilt for every
frame. Each picture's planes are uploaded once, by its id: 8 bit planes
as `R8` and `RG8` textures, 16 bit ones split into their two bytes
(filtering each byte still blends the values correctly, as the value is
linear in them). The shader turns Y'CbCr into RGB with the picture's
matrix, range and bit depth, tone maps PQ and HLG, maps BT.2020 into
BT.709, turns the picture by the stream's display matrix, and fits it
into the widget. Every window shares one pipeline, keyed by a surface
id per window.

### Interface

Windows follow prev: an iced daemon, one window per file, a start window
when nothing is open, Settings drawn over the window that asked, the
theme from the system's accent (Omarchy's on Omarchy) and light or dark,
and a close button where the window manager draws no title bar. The
overlay is scramble-ui's floating bars; the picture widget notes pointer
moves without rebuilding the window, and a tick every 100 ms while
something plays takes the status and hides the bars once the pointer
rests.

### Platform layer

| Area | Linux | Windows | macOS |
|---|---|---|---|
| Settings and data | XDG folders | `%APPDATA%\plaa`, `%LOCALAPPDATA%\plaa` | `~/Library/Application Support/plaa` |
| Single instance | Unix socket | named pipe | Unix socket |
| File dialogs | XDG desktop portal (ashpd) | rfd | rfd |
| Sound | cpal (ALSA, PipeWire) | cpal (WASAPI) | cpal (Core Audio) |
| Hardware decoding | VAAPI, CUDA, Vulkan | D3D11VA, CUDA, DXVA2 | VideoToolbox |
| FFmpeg | the system's | bundled | bundled |
| Theme | the settings portal, Omarchy | accent color, light or dark | accent color, light or dark |

## Dependency and license policy

- `cargo-deny` runs in CI with an allowlist of licenses compatible with
  GPL-3.0-or-later. The FFmpeg bindings are WTFPL, allowed by name.
- A new crate is added only after checking its license and maintenance,
  and whether an existing dependency already does the job.
- FFmpeg comes from the system on Linux. Packages for Windows and macOS
  bundle a build of it, whose source is offered with the release as the
  GPL asks.

## Performance

Targets, to be measured as prev's are.

| Metric | Target |
|---|---|
| Start to window | < 150 ms |
| Open a local file to its first picture | < 150 ms |
| 4K HEVC 10 bit at 60 fps, hardware decoding | no dropped frames |
| Audio and picture apart | < 20 ms |
| Idle memory, one 1080p video playing | < 200 MB |

## Testing and CI

- Unit tests in each crate; the shader is parsed and validated with naga,
  and its uniforms' layout checked against what the app writes.
- `crates/plaa-media/examples/play.rs` plays a file without a window and
  reports what it showed, for checking the engine by itself.
- Every push and pull request: `cargo fmt`, clippy with warnings as
  errors and the tests on Linux, Windows and macOS (FFmpeg from Ubuntu,
  Homebrew and BtbN's builds), the minimum Rust version, `cargo deny`,
  and the desktop entry and AppStream data.
- The interface is tested by hand, as prev's is.

## Risks

| Risk | Mitigation |
|---|---|
| `ffmpeg-next` falls behind FFmpeg releases | The calls it lacks are in one small module (`ffi.rs`); a bindgen crate of plaa's own can replace it |
| Copying hardware pictures to memory costs bandwidth at 4K and above | Measure; import DMA-BUF, D3D11 and IOSurface pictures into wgpu directly where it pays off |
| `atempo` holds a little sound, so speed changes are a fraction of a second late | Acceptable; the clock follows what is heard |
| Patented codecs in bundled FFmpeg builds | Linux uses the system's FFmpeg; the Windows and macOS builds are chosen with this in mind before release |
| Screen recording differs by desktop | The ScreenCast portal on Wayland, the system APIs elsewhere |
| AVB needs hardware timestamping and ptp4l | Follow triib: Linux first, the entity off where the interface cannot do it |
