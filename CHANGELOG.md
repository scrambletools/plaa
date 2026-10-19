# Changelog

## Unreleased

The first build.

- Plays files and streams through FFmpeg, decoding video on the graphics
  card where it can, with sound in step through the output device's clock.
- Draws pictures with a shader of its own: BT.601, BT.709 and BT.2020,
  8 to 16 bits, HDR10 and HLG tone mapped, phone videos turned upright.
- Controls float over the picture and slide away while the pointer rests,
  or dock above and below it; sound files show their cover and tags.
- Seeking, skipping, frame steps, speed with the pitch kept, looping,
  audio and subtitle tracks, text subtitles, the inspector, saving the
  frame, resuming where a file was left.
- Settings, keyboard shortcuts that the settings file can change, one
  window per file and one running plaa.
