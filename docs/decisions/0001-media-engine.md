# ADR 0001: Media engine

Date: 2026-10-18. Status: accepted.

## Decision

Read and decode with FFmpeg's libraries, called through the `ffmpeg-next`
crate, with plaa's own threads, clock and presentation in `plaa-media`.
Draw pictures with a wgpu shader in the iced window, and play sound
through cpal. plaa is GPL-3.0-or-later, so its packages may bundle a full
FFmpeg build.

## Options considered

| Engine | License | Outcome |
|---|---|---|
| FFmpeg's libraries, through `ffmpeg-next` | WTFPL bindings, LGPL or GPL FFmpeg | Chosen |
| libmpv | LGPL or GPL | Rejected: its render API draws with OpenGL or a software renderer, and wgpu's OpenGL backend cannot draw to iced's Wayland windows (prev found the same); mpv's own window would not be part of plaa's interface |
| GStreamer | LGPL | Rejected: plugin sets vary by system and distribution, large to bundle on Windows and macOS, and its sinks would need the same wgpu glue |
| FFmpeg through `rsmpeg` | MIT bindings | Close second: a sound design, but it followed FFmpeg 8 when FFmpeg 9 was the system's |
| FFmpeg through bindings of our own | ours | Kept as the way out if `ffmpeg-next` falls behind |

## Reasons

- FFmpeg reads the broadest set of formats, codecs, protocols and devices,
  which is what plaa is for, and is the same library on every system.
- Its hardware decoding (VAAPI, CUDA, Vulkan, D3D11VA, DXVA2,
  VideoToolbox) sits behind one interface.
- Its filters give speed with the pitch kept (`atempo`), resampling and
  channel mixing, and later trimming, exporting and the visualizer's
  analysis; its devices give recording.
- Drawing pictures in plaa's own shader keeps them inside the iced
  window, under the floating controls, with the colors converted and HDR
  tone mapped on the graphics card.
- `ffmpeg-next` builds its bindings against the FFmpeg it finds, so it
  works with the system's on Linux (FFmpeg 6.1 on Ubuntu 24.04 up to 9 on
  Arch) and a bundled one elsewhere.

## Consequences

- plaa owns synchronization, seeking, buffering and subtitles, as ffplay
  and mpv do on top of FFmpeg; `plaa-media` keeps all of it in one place.
- The few calls `ffmpeg-next` lacks (hardware devices, copying pictures
  from graphics memory, display matrices, attached pictures, channel
  layout names) are in `plaa-media/src/ffi.rs`, the only unsafe code.
- Pictures decoded on the graphics card are copied to memory and uploaded
  again; importing them directly into wgpu is a later optimization.
- ASS styling and picture subtitles need libass and a compositor of their
  own; text subtitles are drawn by iced until then.
