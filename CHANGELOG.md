# Changelog

## Unreleased

The first build.

- Plays files and streams through FFmpeg, decoding video on the graphics
  card where it can, 8K at 60 fps included, with sound in step through
  the output device's clock.
- Draws pictures with a shader of its own: BT.601, BT.709 and BT.2020,
  8 to 16 bits, HDR10 and HLG tone mapped, phone videos turned upright,
  and shrunk pictures filtered as mpv does, so text and fine lines stay
  free of moiré.
- Controls float over the picture and slide away while the pointer rests,
  or dock above and below it; sound files show their cover and tags.
- A timeline that moves a pixel at a time at any window size, and
  shows the frame under its handle as it is dragged.
- Seeking, skipping, frame steps, speed with the pitch kept, looping,
  audio and subtitle tracks, text subtitles, the inspector, saving the
  frame, resuming where a file was left.
- Settings, keyboard shortcuts that the settings file can change, one
  window per file and one running plaa.
- Playlists of files opened or dropped together, M3U and PLS lists, and
  optionally the opened file's folder.
- Chapters on the timeline, pictures and chapter names over it where it
  is pointed at, and an A-B loop.
- Subtitle files beside the video, picture in picture, the sound output
  in Settings, MPRIS on Linux, and the screen kept on while video plays.
