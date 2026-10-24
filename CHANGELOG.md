# Changelog

## [Unreleased]

### Added

- A Make Default button in Settings, under Playback, makes plaa the
  system's default app for the video and sound files it plays, as
  prev's does for its files, with a dot showing whether none, some or
  all of them open with plaa. On macOS it asks for the common types,
  which macOS confirms one by one; on Windows it opens plaa's page in
  Default apps, which the installer now registers.
- `scripts/dev-ptp4l.sh` installs plaa's ptp4l services for dev builds,
  which then start ptp4l where none runs, as an installed plaa does.
- Agents and the assistant read any part of what plays, as edited: its
  pictures at any times (`get_frames`), how loud it is and where it is
  silent (`get_levels`), its subtitles' text (`get_subtitles`), and its
  sound for models that hear (`get_sound`).
- Agents and the assistant edit: trim, cut out a part, split, move
  clips, insert part of another file, play another file's sound or
  picture over a part, or silence or black, turn the picture, undo, and
  export (`get_edit`, `trim`, `cut`, `split`, `move_clip`, `insert`,
  `replace`, `turn_picture`, `undo`, `export`). Settings has a switch
  to be asked before an agent edits.
- Edits that mix files play as they will be, picture and sound each
  from its own file, and export the same way, other files' pictures
  fitted in the movie's.
- `apply_filter` and `remove_filters` put a window's picture or sound,
  all of it or a part, through FFmpeg filters, such as `hue=s=0` for
  black and white, checked before they start: only filters that change
  frames as they come, none given a file, device or address. They play
  at once and export; a filter over the whole movie that crops it
  exports at its size.
- Agents' calls that take more than two seconds report how far they
  have come, and the window shows it.
- `mix_in` mixes another file's sound in with what plays, such as music
  under a video, from a time of the edit, at a volume in decibels, with
  fades; it plays at once, exports, and is heard by `get_sound` and
  `get_levels`. `remove_mix` takes it out.

### Changed

- One panel at a time at a window's side: the inspector, the playlist,
  the network sheet and the assistant each close the one open before,
  and each runs the window's full height, beside the toolbars. A panel
  slides in only when none was open; one taking another's place shows
  at once. The assistant's slides in too.

## [0.1.0] - 2026-10-23

The first release.

- Plays files and streams through FFmpeg, decoding video on the graphics
  card where it can, 8K at 60 fps included, with sound in step through
  the output device's clock.
- Draws pictures with a shader of its own: BT.601, BT.709 and BT.2020,
  8 to 16 bits, HDR10 and HLG tone mapped, phone videos turned upright,
  and shrunk pictures filtered as mpv does, so text and fine lines stay
  free of moiré.
- Controls float over the picture or a sound file's cover and slide away
  while the pointer rests, or dock above and below; sound files show
  their cover and tags.
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
- Each recording named as it finishes, and saved to the folder Settings
  choose for its kind: sound to Music, screen and camera to Videos at
  first.
- A visualizer under a sound file's cover: the spectrum of what is
  heard, or the file's waveform, which seeks where it is clicked.
- plaa as an AVB talker on Linux: a Milan entity of its own sending what
  plays as a stereo AAF stream on gPTP time, reserved by its own MSRP or
  by triib's entity service where that runs, and a Network Audio sheet
  listing the listeners that can play it, with the chosen one's status,
  format, stream and counters under them, kept up to date as other
  controllers change them, by asking again where entities send no
  notifications. plaa and triib's entity service greet each other as
  they share the port's reservations, and listeners on this computer
  say when triib must be updated to play there. plaa's entity shows
  controllers its app icon, and Scramble Tools' logo as its vendor's,
  PNG memory objects read through Address Access, and counts its
  stream's starts, stops and frames, and its interface's link and
  grandmaster changes, for them. Started again, plaa sends
  to the address its stream had last time at once, while MAAP claims
  one anew, so listeners hear it again within a fraction of a second.
  Where no ptp4l runs, plaa starts its own, `plaa-ptp4l@<interface>`,
  which the packages bring, and Network Audio says when ptp4l keeps no
  time for its stream.
- plaa listens too: its entity's stream input plays the stream connected
  to it, with Milan's input counters, the talkers it can take listed by
  Connect.
- plaa sends and takes the formats the avb crates set for every
  endpoint: the AVB community's Basic and Extended Audio profiles'
  AAF, in SR class A and B at 48, 96 and 192 kHz, and AM824 as MOTU's,
  Apple's and PreSonus's endpoints send, of 2 channels or 8; and meets
  a talker or listener whose format takes up to as many channels.
- Connect, beside Open on the start screen and in the toolbar: a menu of
  the talkers plaa can listen to, those sending in a format it plays and
  those that can change to one, which it changes as it connects, saying
  which other listeners that cuts off.
- The stream plaa listens to plays in a window as a file plays, titled
  by its talker, with plaa's volume, the visualizer and the media keys:
  pausing stops listening and closing the window stops it too. The
  inspector shows the talker and the stream as it comes, its counters,
  and its format, which it changes to another plaa takes.
- AVB Lite where no bridge reserves streams, as on a switch without AVB:
  plaa falls back from its own MSRP, or follows triib's entity service,
  declaring by CVU SRP, sending each listener it admits, up to 5, a
  unicast copy of its stream, taking one sent here, and answering
  GET_LITE_STATUS and SET_LITE_CONFIG, while its ptp4l runs the AVB Lite
  PTP profile, `plaa-ptp4l-lite@<interface>`.
- On a Mac, plaa sends through the Mac's own AVB entity, turned on in
  Audio MIDI Setup, as triib uses it: its controller reaches the entity
  through Apple's AVB framework, and plaa plays into the Core Audio
  device macOS makes of it while a listener is connected.
- Prism, a visualizer in three dimensions filling the window, drawn on
  the graphics card from shapes alone, no pictures, moved by the
  spectrum and its beats.
- Packages: a .deb for each of Debian 13 and Ubuntu 24.04 and 26.04, an
  .rpm for Fedora 44, the AUR's PKGBUILD, an MSI and zip for Windows and
  an installer for macOS, the Windows and macOS ones carrying FFmpeg,
  each installed, upgraded, run, made to play a video and removed in a
  clean system by the Release workflow before a release goes out.
- `plaa --interfaces` says whether plaa may use raw Ethernet and which
  wired interfaces could send an AVB stream.
- The assistant's models and keys shared with the other Scramble Tools
  apps, such as prev, in `~/.config/scramble/assistant.toml`: set up in
  one, they are there in the others, and a change reaches them while
  they run.
