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
| Hardware decoding | Through FFmpeg: VAAPI, then CUDA, then Vulkan on Linux, where NVIDIA's VAAPI driver waits until last, as it copies pictures out several times slower than CUDA; D3D11VA, CUDA, then DXVA2 on Windows; VideoToolbox on macOS. AV1 through FFmpeg's own decoder on graphics cards, dav1d on the processor. Pictures are copied to reused memory and converted on the graphics card; the processor takes over when no device opens |
| Sound out | cpal: the sound server on Linux through the PulseAudio protocol, which PipeWire speaks too, asked for 50 ms of buffering, and ALSA without a server; WASAPI on Windows, Core Audio on macOS. The device's clock drives playback; Settings picks the device |
| Color | Y'CbCr to RGB in the shader, BT.601, BT.709 and BT.2020, limited and full range, 8 to 16 bits. PQ and HLG are tone mapped for standard screens, BT.2020 mapped to BT.709 |
| Subtitles | Text subtitles (SubRip, WebVTT, MP4 text and the rest FFmpeg reads) drawn by plaa; ASS and SSA drawn as styled by libass, loaded as plaa runs, with the file's fonts; Blu-ray, DVD and DVB picture subtitles. Both kinds of picture are laid over the video in its shader, and fit and shrink with it |
| Windows | One window per file, one running instance; a start window when nothing is open |
| Overlay | M3 floating toolbars over the picture, or a sound file's cover, by default, hiding once the pointer rests (2 s, adjustable) or leaves the window; the toolbar or Settings docks them |
| Editing | Not destructive: an edit is the file's clips in order and a turn and flip, played as it will be and kept until exported, as QuickTime Player keeps it until saved |
| Export | Re-encoded through FFmpeg: NVENC, AMF or Quick Sync where they open, VAAPI on Linux, VideoToolbox on macOS, else x264, x265 and SVT-AV1, in MP4 with AAC sound (Apple's encoder on macOS); sound alone as AAC, MP3 or Opus at a bit rate chosen, or FLAC, ALAC or WAV in 16 or 24 bits |
| Web pages | yt-dlp, run as a program of its own when it is installed, not bundled: it follows the sites' changes faster than plaa could; its JSON names the streams, which FFmpeg plays |
| Settings | `~/.config/plaa.toml` (`%APPDATA%\plaa\plaa.toml`, `~/Library/Application Support/plaa/plaa.toml`), with a `[keys]` table as prev's; where files were left off in the state folder |
| Interface languages | Fluent files in `i18n/`, English first, and the 38 languages prev has |
| Assistant and MCP | As prev: one tool registry for MCP clients (`plaa --mcp`) and an assistant panel with the user's own model |
| AVB and Milan | plaa's own Milan entity on Linux, and the Mac's own entity on macOS, with the `atdecc`, `avb-mrp`, `avb-net` and `avb-stream` crates of scrambletools/avb pinned by revision: a talker of what plays, and a listener to play or record a stream. ptp4l as triib runs it on Linux; audio first (AAF, AM824), IEEE 1722 CVF video later |
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
2. **Playback complete** (done): playlists (files opened or dropped
   together, M3U and PLS lists, the folder's other files), chapters on
   the timeline, subtitle files beside the video, thumbnails over the
   timeline, A-B loops, picture in picture, choosing the output device,
   the system's media controls (MPRIS, SMTC and Now Playing), keeping the
   screen awake, picture subtitles and ASS styling (libass), drag and
   drop on Wayland (prev's smithay-clipboard patch), the macOS menu bar,
   and title bars in plaa's colors on Windows and macOS.
3. **Network** (done): web pages through yt-dlp, their picture and
   sound read as two streams in step, and playlists; choosing a
   stream's quality, by default as tall as the screen; buffering ahead,
   waiting to read on, and a spinner while it waits; recent addresses.
4. **Edit and export** (done): trim, split and arrange clips, rotate and
   flip, with undo, the edit playing without gaps; export to H.264,
   HEVC or AV1, as edited and made smaller, on hardware encoders where
   they open, or the sound alone, compressed or lossless.
5. **Recording** (done): microphone, screen (the ScreenCast portal and
   PipeWire on Linux, Windows.Graphics.Capture on Windows,
   ScreenCaptureKit on macOS), camera (FFmpeg's device inputs); AVB
   streams with the entity, in milestone 7.
6. **Visualizer** (done): a spectrum of what is heard and the file's
   waveform under a sound file's cover, a scene in three dimensions
   filling the window, or the cover alone.
7. **AVB and Milan**: the entity and talker, and a controller to connect
   talkers and listeners (done); the listener and AVB Lite next.
8. **Casting**: Chromecast and DLNA/UPnP.
9. **Assistant and MCP**: the MCP server, `plaa --mcp`, with prev's
   consent prompts (done); the assistant panel next.
10. **Release**: packages (made, with installer tests on each system,
    `docs/RELEASING.md`), translations (done, 38 languages), signing,
    the website.

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
decoder. A web page is read by yt-dlp first, when it is installed: plaa
picks the best format the chosen quality allows from what yt-dlp ranks,
and when the site keeps sound apart, as YouTube does, opens it as a
second input, read in step with the picture's. Streams no decoder takes
are skipped, so an HLS stream fetches one of its qualities.

- **Reading**: reads packets and hands each to its stream's queue,
  keeping a second or so ahead of a file and up to 30 seconds ahead of a
  stream, and takes commands: seeks and track changes. Text subtitles
  are small enough to decode here.
- **Video**: decodes, on the graphics card when it can, copies hardware
  pictures into memory it reuses (at 8K, fresh memory for each picture
  costs the system more than the copy), converts the few formats the shader does not read
  with swscale, and queues six pictures ahead of the clock.
- **Audio**: decodes and runs a filter graph (`atempo` for speed with
  the pitch kept, then `aformat` for the device's rate, channels and
  float samples), queueing 400 ms ahead.
- **Output**: cpal's callback plays the queue and sets the clock to the
  media time of what it plays, heard after the device's latency.

The clock follows the sound while there is sound: it stalls while sound
is missing, after a seek or while a stream buffers, so pictures wait
rather than run ahead. With no sound device, the sound goes to a silent
output that takes it in time, so playback goes on as with one. Without
sound, it starts from the first picture after a seek and runs on the
system's clock. Once the source runs out, after a seek and when it
opens, the clock also waits for the source to read 2 seconds ahead, so a
slow network pauses playback once rather than every moment. Every seek
bumps a serial that packets, pictures and samples carry, so anything
from before it is dropped wherever it is. Exact seeks also drop what
decodes before the target.

An edit plays as it will be. Its clips, joined where one carries on
the next in the file, are parts the reader reads one after another,
seeking to the next once every stream playing has read past the end of
this one, by decoding time where packets have it. Packets carry their
part, so the decoders know where each picture and sound goes in the
edited time: a decoder drains itself at the end of a part, pictures
keep within it by their middle and sound to the sample, and the clock,
seeks and the position all count in the edited time, so the parts play
one after another without a gap.

Dragging the timeline holds the clock and seeks exactly, so the picture
only moves the way the handle does; seeking to keyframes instead would
show each keyframe before the frames after it, rocking back and forth.
Ahead of the picture, with no keyframe between in the file's index, the
target moves on and decoding carries on to it, which keeps up with the
handle; otherwise one seek at a time, the next once the last one's
picture shows, to the handle's latest position. Letting go lands on the
handle and lets the clock go on.

### Export

An export runs on a thread of its own, from its own reading of the file:
each part of the edit is read in turn, as in playback, its pictures and
sound kept within it and stamped with the edited time. Pictures, copied
out of graphics memory when decoded there, go through a filter graph
that flips, turns and sizes them for the encoder, and sound through one
that resamples it and cuts it into the encoder's frames, filling gaps
between parts. The encoder is the first of the system's that opens for
the format and size, so a card that cannot take 8K H.264 leaves it to
the processor; the dialog asks the same way, so it says which. Ten bit
pictures stay ten bits, but a card that encodes only eight, as AMD's
with older drivers, takes them in eight unless they are HDR. What it
writes goes to a hidden file beside the one chosen, renamed once done,
and removed if it is stopped.

### Recording

A recording window opens its input device at once, so its meter moves
before recording starts: the device's callback, on cpal's thread, notes
each channel's peak and, while recording, queues its samples. A thread
of the recording's own takes them through the same filter graph and
encoder as exporting sound alone, timestamped by the samples counted,
into a hidden file renamed once stopped; the window then plays it. The
sound server lists what each output plays as an input too, offered
apart from microphones.

Many microphones are quiet even at their full gain: a USB one here
peaks at −20 dB. The callback makes the samples louder by the gain
Settings keep for that input, up to 30 dB, held within full scale,
before noting peaks or queueing them, so the meter shows the level
recorded, every recording takes it, and the slider works while
recording. "Level automatically" puts FFmpeg's dynaudnorm first in the
sound graph (frames of 200 ms over about three seconds, up to twenty
times louder, peaks 3 dB below full as encoders overshoot), chosen as
a recording starts.

A screen recording on Linux asks the ScreenCast portal, whose picker is
the desktop's own, for a screen, window or region with the pointer in
it, then reads the stream it shares through PipeWire on a thread of its
own. A still screen sends no pictures, so while not recording one is
kept every quarter second, and the last one kept starts the recording.
Once Record is pressed, the encoder opens first (NVENC takes a fifth of
a second), then the pictures and the sound are taken from that moment:
each picture is copied out of PipeWire's buffer, timed in microseconds
as it came, made BT.709 limited range in the filter graph (encoders
that take RGB convert it as they please, NVENC by BT.601) and encoded
with the export's encoders, software ones at their quick presets. The
sound is the chosen input's, taken from its callback as a sound
recording takes it. The last picture is repeated where the recording
ends. Stopping sharing from the desktop ends the recording, saved as
it is; closing the window ends the portal's session, or the compositor
would keep copying the screen.

A screen sends as many pictures as it refreshes, 120 or more a second
on some, while it changes. A recording keeps at most the rate chosen
(60 by default), skipping the rest before they are copied: credit for
a picture builds with time, one is kept once there is three quarters
of a picture's, and up to a quarter more carries over. So a screen as
fast as the rate keeps every picture though they come a few
milliseconds early or late, a faster one keeps them evenly spaced, and
a still screen saves nothing up. Measured at 3840×1080 from a 120
picture source: NVENC keeps every picture at any rate; x264 drops none
at 60 and about one in fifteen at 120. Pictures dropped because the
encoder fell behind, with eight waiting, are counted and shown beside
the size.

Recording a screen or a region would show the recording window, so it
closes first, after a warning saying how to stop: no system lets a
Wayland window hide itself (Hyprland ignores minimizing), so the window
goes and the recorder lives on without one. The recording begins 0.8
seconds later, once the window has gone; of the pictures until then,
only the last is kept, to begin it. A red dot in the tray, a
StatusNotifierItem of plaa's own, stops it when clicked, and so does
opening plaa again, which tells the running plaa; the warning names
only the second where no tray is shown. Once saved, a new window plays
the recording. Other plaa windows stay, so a video plaa plays can be
recorded.

On Windows there is no picker of the system's that names what it picked
in a way FFmpeg takes, so plaa lists the screens, in the order Windows
lists them, which gfxcapture numbers them by, and the windows Alt+Tab
would show, but its own and minimized ones; the menu lists them again
each time it opens. FFmpeg's gfxcapture (Windows.Graphics.Capture, from
FFmpeg 8) captures either, with the pointer, on the graphics card;
hwdownload brings each picture into memory as BGRA for the same graph
and encoders as on Linux. Like PipeWire, it sends pictures only as the
screen changes, and looks for one a second at a time, so its thread
checks whether to stop between; a window recorded that closes ends the
stream, and so the recording. What an output plays is recorded through
its loopback, which sends nothing at all while nothing plays, so sound
that comes after a silence is timed by the clock rather than by the
samples counted, and the filter graph fills the silence; a recording
whose input fell quiet ends with silence up to where the clock does.

On macOS, ScreenCaptureKit lists the displays and windows (the windows
of other apps that are on screen, in the normal layer and named) and
captures one: a display with plaa's own application left out, as
QuickTime Player leaves itself out, so the recording window stays where
it is; or a window on its own. Pictures come as BGRA in sRGB, asked for
as the display may be P3, on a dispatch queue of plaa's, as often as
the display sends them; those saying nothing changed are skipped, and
each new one is copied out of its CVPixelBuffer into the same queue and
graph as elsewhere. Asking what there is to record fails until macOS
lets plaa record the screen, which the notice says. What an output
plays comes through cpal's process tap of it, which sends silence while
nothing plays.

A camera is read by FFmpeg's device input for the system, v4l2,
dshow or avfoundation, which decodes its pictures, MJPEG included, on
a thread of the capture's own, into the same queue, graph and encoders
as a screen's; its colors are made BT.709 video. FFmpeg lists the
cameras on Linux and Windows (one each where Linux has several nodes,
its doubled name tidied) and AVFoundation on macOS, where FFmpeg cannot.
v4l2 is asked for MJPEG at 1080p and 30 pictures a second first, as raw
pictures of that size come slowly over USB, then smaller, then as the
camera likes; avfoundation needs a rate the camera offers. Each picture
is also made a small RGBA preview, which the window shows mirrored, as
people expect to see themselves; the recording is as the camera sees.
`PLAA_TEST_CAMERA` adds FFmpeg's test picture as the first camera, for
machines without one.

### Visualizer

Under a sound file's cover, the toolbar's menu chooses a spectrum, the
waveform, or the cover alone, kept in Settings for every window. With
one showing, the cover gives way to it in a short window.

The output's callback keeps the last third of a second it was handed,
mixed to one channel before the volume, and when the newest of it is
heard. The spectrum takes the 4096 frames heard by now, leaving out
what the device holds yet, through a Hann window and a Fourier
transform of plaa's own (85 ms at 48 kHz, so the bass's pitches part),
into bars spaced evenly by octave from 40 Hz to 16 kHz, 70 dB deep,
tilted 3 dB an octave about 1 kHz as music's highs are quieter. Each
frame drawn asks for the next while there is sound or a bar is still
falling: bars rise to the sound in a frame or two and fall back a
little more than their height a second, so a paused spectrum comes to
rest and stops drawing.

The waveform is read from the file on a thread of its own, decoded from
start to end into 4096 stretches of its length, each its loudest sample
and root mean square, handed over every quarter second as it goes so it
draws from the left as it is read. Lines are drawn by the square root
of the loudness, so quiet sound is seen, the played part in the accent
color; an edit's time is turned into the file's for each line. Clicking
or dragging seeks as the timeline does. Streams have no waveform, as
they would be read twice over the network.

Prism fills the window as a video does, the bars floating over it and
hiding while the pointer rests, the cover, title, artist and album in
its corner. Each frame, 32 bands of the sound heard are eased as the
spectrum's are; their low, middle and high thirds, the overall level,
and beats (the bass rising 0.07 over its level of the last half second
or so, once until it falls back) drive a scene of signed distance
fields, ray marched in a shader of plaa's own with no textures: a
sphere's shell carved by a gyroid into lace that turns, swells with
the bass and opens with the highs, colored as a thin film by the angle
it is seen at; a white-hot core lighting it from within, its halo the
glow's integral along each ray worked out whole, so it has no seams;
a tipped, spinning crown of 32 shards, the bands up one side and down
the other, each glowing by its level; a grid of light below rippling
out with the bass; and a nebula of Kali's fractal with stars, folding
tighter with the bass, that the surfaces reflect. A beat punches the
lens, flashes the core and sends a ring of light out from the middle.
The scene's time runs faster as the sound is louder, so it drifts when
quiet and races when loud; paused, it comes to rest and stops drawing.
It is drawn at no more than 800,000 pixels into a picture of its own,
then stretched over the window with its colors parting toward the
edges, more on each beat, vignetted, and grained at the screen's own
pixels.

### Pictures

The picture is a shader widget that asks the player for the picture due
each time it draws, and asks to be drawn again when the next one is due,
so playback runs without the window's interface being rebuilt for every
frame. Each picture's planes are uploaded once, by its id: 8 bit planes
as `R8` and `RG8` textures, 16 bit ones split into their two bytes
(filtering each byte still blends the values correctly, as the value is
linear in them). They go through two staging buffers each window keeps
and reuses, as wgpu's own staging is fresh memory for every write, which
at 8K takes three times as long. The shader turns Y'CbCr into RGB with
the picture's matrix, range and bit depth, tone maps PQ and HLG, maps
BT.2020 into BT.709, turns the picture by the stream's display matrix,
and fits it into the widget. Where the picture is shrunk, each screen
pixel weighs the texels within its size of its centre on a Hermite
curve, as mpv's default downscaling does, two texels to a linear sample;
single samples would skip texels and make text and fine lines shimmer.
At its size or larger, one linear sample. Every window shares one
pipeline, keyed by a surface id per window.

Ctrl and the wheel zoom the picture at the pointer: the widget sends
each turn, with where the pointer is and how the picture fits, and the
window applies them in turn, so quick turns of a touchpad all count. A
zoom is a scale and the point of the picture in the widget's middle;
the shader moves and scales the picture's corners, and the widget's
edges cut it. Where the zoomed picture is larger than the widget it is
kept covering it, its edge stopping at the widget's, and where it is
smaller, in the middle; so near an edge the point under the pointer can
move toward it. Dragging a zoomed picture moves it with the pointer, as
far as it still covers the widget; the press is left to the window, so
a double click still fills the screen, and the overlay stays away while
dragging. A new file starts fitted.

### Interface

Windows follow prev: an iced daemon, one window per file, a start window
when nothing is open, Settings drawn over the window that asked, the
theme from the system's accent (Omarchy's on Omarchy) and light or dark,
and a close button where the window manager draws no title bar. The
overlay is scramble-ui's floating bars; the picture widget notes pointer
moves without rebuilding the window, and a tick every 100 ms while
something plays takes the status and hides the bars once the pointer
rests. The timeline, too, places its handle from the player's position
as it draws, and asks to draw again when the handle reaches the screen's
next pixel, so it moves a pixel at a time at any width.

### Platform layer

| Area | Linux | Windows | macOS |
|---|---|---|---|
| Settings and data | XDG folders | `%APPDATA%\plaa`, `%LOCALAPPDATA%\plaa` | `~/Library/Application Support/plaa` |
| Single instance | Unix socket | named pipe | Unix socket |
| File dialogs | XDG desktop portal (ashpd) | rfd | rfd |
| Sound | cpal (PulseAudio protocol to PipeWire or PulseAudio, else ALSA) | cpal (WASAPI) | cpal (Core Audio) |
| Media keys, now playing | MPRIS over D-Bus (zbus) | SMTC, on a window never shown (windows) | MPNowPlayingInfoCenter and MPRemoteCommandCenter (objc2) |
| Menus and title bar | the window manager's | the system title bar, dark or light as plaa is | a menu bar (muda), the title bar transparent over plaa's chrome |
| Drag and drop | winit on X11; prev's patched smithay-clipboard on Wayland | winit | winit |
| Screen kept on | the screen saver's D-Bus service, else logind's idle lock (keepawake) | execution state (keepawake) | power assertions (keepawake) |
| Hardware decoding | VAAPI, CUDA, Vulkan | D3D11VA, CUDA, DXVA2 | VideoToolbox |
| Hardware encoding | NVENC, VAAPI, Quick Sync | NVENC, AMF, Quick Sync | VideoToolbox |
| Sound in | cpal (the sound server through the PulseAudio protocol, which also offers what outputs play; ALSA without one) | cpal (WASAPI; what an output plays through its loopback) | cpal (Core Audio; what an output plays through a process tap, from macOS 14.2) |
| Camera in | FFmpeg's v4l2 input, cameras listed by it | FFmpeg's dshow input, cameras listed by it | FFmpeg's avfoundation input, cameras listed by AVFoundation (objc2) |
| Screen in | the ScreenCast portal (ashpd), its picture through PipeWire | Windows.Graphics.Capture through FFmpeg's gfxcapture, screens and windows listed by plaa (Win32) | ScreenCaptureKit (objc2), screens and windows listed by it, plaa's own windows left out |
| Tray, while recording hidden | StatusNotifierItem over D-Bus (zbus) | the notification area (planned) | none needed: plaa's windows are left out, so it does not hide |
| yt-dlp | on the path, or `~/.local/bin` | on the path, WinGet's or Scoop's | on the path, or Homebrew's |
| FFmpeg | the system's | bundled | bundled |
| Theme | the settings portal, Omarchy | accent color, light or dark | accent color, light or dark |

### AVB and Milan

plaa uses the AVB crates triib grew, which moved for it to their own
repository, scrambletools/avb: `atdecc` for ATDECC's controller and
entity state machines, `avb-mrp` for MSRP and MVRP, `avb-net` for raw
Ethernet, and `avb-stream` for AAF talkers paced on gPTP time. The
`plaa-avb` crate runs them on one thread for the interface Settings
choose, as triib's endpoint daemon does.

Its controller, on every system, reads each entity's names, streams,
formats and bindings, and binds a listener's input to a talker's output,
setting the input to the talker's format first where it lists it, as
Milan listeners only take a stream in their own format.

On Linux, plaa's entity has one stream output, "Playback", of two
channels at 48 kHz, AAF in 32 or 24 bits, six samples a frame (class A),
its entity ID the interface address with 0xff70 in the middle, beside
triib's 0xff00 and up. MAAP claims its multicast address; MSRP declares
it on VLAN 2 with class A and B's domains, as MVRP joins the VLAN; gPTP
comes from ptp4l through pmc, plaa starting its own ptp4l service where
none runs, and the stream's presentation times from
the interface's PTP hardware clock. When MSRP says a listener is ready,
the talker starts and stops again when none is. It sends what the media
window plays: plaa's audio output hands a copy of each chunk it gives
its device, in stereo after the volume, to a buffer the talker drains on
gPTP time, resampling between the two clocks by how full it keeps.

plaa holds no capability: a program holding one runs as one other
programs may not look into, and the desktop portal then refuses it file
dialogs and screen sharing. `plaa-netraw`, built beside it, is the one
file granted CAP_NET_RAW. plaa starts it once, and every raw socket the
avb crates open comes from it, passed over a Unix socket
(`avb_net::broker`), as triib's binaries get theirs too.

On a Mac, the talker is the Mac's own entity, which macOS runs once AVB
is on for the port in Audio MIDI Setup: macOS never shows its AVB stack
what a program writes to BPF, so the controller sends that entity its
commands through Apple's AudioVideoBridging framework
(`atdecc::blocking::Local`), which answers them, and the Network Audio
sheet offers the entity's first output as plaa's. While one of its
stream outputs is connected, macOS makes a Core Audio device of it; plaa
plays into its first two channels from the same buffer the Linux talker
drains, steering both sides to their clocks (`host.rs`).

Two MRP participants on one port withdraw each other's declarations, so
where triib's entity service runs on the interface, the entity reserves
its stream in the service's participant (`avb_mrp::sharing`, a socket in
the user's runtime folder): the service declares the stream and tells
plaa its listeners, and that it cannot while the port runs AVB Lite.
When the service goes, plaa runs its own participants a moment later,
and shares the service's again when it is back. Beside an older service,
which does not share it, or another program declaring MRP from this
computer's address, the entity steps aside. Tested against an ESP32
Milan listener through a MOTU AVB switch: the listener locked to the
stream, 8,000 frames a second, none late or early, with plaa on its own
participants and through triib's, the service stopped and started again
while it streamed.

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
- The packages, by the Release workflow: each installed, upgraded, run,
  made to play a clip and removed in a clean container, runner or Windows
  Server container (`packaging/tests`), as triib's are.
- The interface is tested by hand, as prev's is.

## Risks

| Risk | Mitigation |
|---|---|
| `ffmpeg-next` falls behind FFmpeg releases | The calls it lacks are in one small module (`ffi.rs`); a bindgen crate of plaa's own can replace it |
| libass missing where plaa runs | ASS shows as plain text then; packages for Windows and macOS bundle libass beside FFmpeg |
| Copying hardware pictures to memory costs bandwidth at 4K and above | Measured: with memory reused on the way out and on the way back in, an RTX 4080 SUPER copies 8K 10 bit pictures out at 137 a second through CUDA and uploads one in 8.5 ms, so 8K plays at 60 fps. Import DMA-BUF, D3D11 and IOSurface pictures into wgpu directly where it pays off |
| `atempo` holds a little sound, so speed changes are a fraction of a second late | Acceptable; the clock follows what is heard |
| Patented codecs in bundled FFmpeg builds | Linux uses the system's FFmpeg; the Windows and macOS builds are chosen with this in mind before release |
| Screen recording differs by desktop | The ScreenCast portal on Wayland, the system APIs elsewhere |
| macOS asks before an app hears the microphone, naming the app by its bundle | Until plaa is packaged, macOS asks for the terminal that started it; the package's Info.plist gives the reason (`NSMicrophoneUsageDescription`) |
| AVB needs hardware timestamping and ptp4l | Follow triib: Linux first, the entity off where the interface cannot do it |
