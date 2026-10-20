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
- Blu-ray, DVD and DVB picture subtitles, and ASS subtitles drawn as
  styled by libass.
- Now Playing on macOS and the media flyout on Windows, besides MPRIS,
  and files dropped on windows on Wayland.
- The menu bar on macOS, and title bars in plaa's colors on macOS and
  Windows.
- Streams buffer ahead, wait to read on rather than stutter, and fetch
  only the quality that plays; a spinner shows while they wait.
- Web videos and playlists through yt-dlp, a quality menu, recent
  addresses, and AV1 decoded on the graphics card.
- Zooming in on the picture with Ctrl and the wheel, at the pointer,
  and dragging it to look around.
- Trimming, splitting and arranging clips, rotating and flipping, with
  undo, the edit playing clip after clip without a gap.
- Exporting to H.264, HEVC, AV1 or sound alone, as edited and made
  smaller, on the graphics card's encoder where one opens; sound alone
  as AAC, MP3, Opus, FLAC, ALAC or WAV, at a quality chosen.
- Recording sound from a microphone, or what the computer plays, with a
  level meter, played back once saved.
- Recording the screen, a window or a region on Linux, through the
  ScreenCast portal and PipeWire, with a microphone's sound or what the
  computer plays, encoded on the graphics card where it can. Recording
  a screen, plaa's recording window hides, and a red dot in the tray,
  or opening plaa again, stops it.
- Recording the screen or a window on macOS, through ScreenCaptureKit,
  plaa's own windows left out, and what the computer plays there.
- Recordings take a quiet input up to 30 dB louder, kept for each input,
  and can even out the sound's level automatically.
- Recording a camera, its picture shown mirrored as it records, through
  FFmpeg's device inputs: V4L2 on Linux, DirectShow on Windows and
  AVFoundation on macOS.
- Screen recordings at 24, 30, 60 or 120 pictures a second, or every
  picture the screen sends, and the pictures dropped shown as they
  record.
- Recording the screen or a window on Windows, through Windows.Graphics.
  Capture, and recording what the computer plays there through each
  output's loopback.
- A visualizer under a sound file's cover: the spectrum of what is
  heard, or the file's waveform, which seeks where it is clicked.
