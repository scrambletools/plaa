# plaa's interface text in English, the source for every other language.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value plaa fills in, such
# as a time or a file name; keep it, and move it where the language needs
# it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# Sections follow the parts of the interface. Keys shared by several
# parts are under "Common".

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = English (US)

## Common

app-name = plaa
common-cancel = Cancel
common-close = Close
common-more = More

## Command line

usage-help =
    Usage: plaa [FILE OR ADDRESS]...

    Play video and audio files, and streams at addresses such as
    https://example.com/live.m3u8. Each opens in a window of the running
    plaa, which starts if needed.

    Options:
      -h, --help     Show this help
      -V, --version  Show the version

## Start window

start-hint = Open a video or audio file, a stream or a web video.
start-open = Open
start-open-address = Open Address

## Actions, as buttons, tooltips and menus name them

action-open = Open
action-open-address = Open Address
open-file = Open file…
open-address = Open address…
action-settings = Settings
action-info = Inspector
action-loop = Loop
action-screenshot = Save Frame as Picture
action-play = Play
action-pause = Pause
# $seconds is how far the button moves, such as 10.
action-back = Back { $seconds } seconds
action-forward = Forward { $seconds } seconds
action-mute = Mute
action-unmute = Unmute
action-speed = Speed
action-tracks = Audio and Subtitles
action-full-screen = Full Screen
action-playlist = Playlist
action-chapters = Chapters
action-picture-in-picture = Picture in Picture
action-previous = Previous
action-next = Next
action-exit-full-screen = Exit Full Screen
action-quality = Quality
action-visualizer = Visualizer
action-network = Network Audio

## What a sound file shows under its cover

visualizer-spectrum = Spectrum
visualizer-waveform = Waveform
visualizer-prism = Prism
visualizer-cover = Cover only

## The floating controls

overlay-keep-shown = Keep controls docked
overlay-auto-hide = Float controls over the picture

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Subtitles
tracks-off = Off
# A track the file gives no name or language, by its number.
tracks-numbered = Track { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Audio only

## Inspector

info-title = Inspector
playlist-title = Playlist
chapters-numbered = Chapter { $number }
info-file = File
info-name = Name
info-address = Address
info-folder = Folder
info-size = Size
info-format = Format
info-duration = Duration
info-bit-rate = Bit rate
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Dimensions
info-frame-rate = Frame rate
info-frames-per-second = { $rate } frames per second
info-pixel-format = Pixel format
info-decoding = Decoding
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Graphics card ({ $api })
info-decoding-software = Processor
info-sample-rate = Sample rate
info-hertz = { $rate } Hz
info-channels = Channels
info-language = Language
info-no-sound = No sound
info-tags = Tags

## Dialogs

dialog-open = Open
dialog-filter-media = Video and audio
dialog-filter-all = All files

address-title = Open Address
address-note = A web page with a video, such as a YouTube video's, or a stream or file on the network, such as https://example.com/live.m3u8.
address-label = Address
address-open = Open
address-recent = Recent
address-forget = Remove from recent

## Notices

notice-dialog-failed = The file dialog could not open: { $error }
notice-screenshot-saved = Saved “{ $name }”.
notice-screenshot-failed = The picture could not be saved: { $error }
notice-no-sound = Playing without sound: { $reason }
notice-loop-from = Loop from { $time }. Press again where it should end.
notice-loop-between = Looping { $from } to { $to }.
notice-loop-off = Loop off.

## Errors, shown in place of what could not play

error-needs-yt-dlp = This looks like a web page. plaa plays the videos on web pages with yt-dlp, which is not installed.
error-empty-list = This list has nothing to play.

## Settings

settings-title = Settings
settings-tab-appearance = Appearance
settings-tab-playback = Playback
settings-tab-network = Network
settings-tab-language = Language
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (dev build, { $build })
settings-choose = Choose…

settings-appearance-system = Follow system
settings-appearance-light = Light
settings-appearance-dark = Dark
settings-colors = Colors
settings-system-accent = Use the system accent color
settings-omarchy-note = Colors are built from the accent of “{ $theme }”.
settings-system-accent-note = Colors are built from the system's accent color.
settings-system-accent-none = The system has no accent color, so plaa uses the one chosen below.
settings-accent-chosen-note = Colors are built from the color chosen below.
settings-controls = Controls
settings-overlay = Float controls over videos
settings-overlay-note = The toolbar and playback controls float over the picture and slide away while the pointer rests or is outside the window. Audio keeps them docked.
settings-overlay-delay = Hide after
settings-overlay-delay-note = How long the pointer rests over a playing video before the controls slide away.
settings-seconds = { $seconds } s
settings-transparency = Transparency
settings-transparency-note = How much of the picture shows through the floating controls.
settings-percent = { $percent }%
settings-corner-radius = Corner radius
settings-corner-radius-note = For dialogs and the floating controls.
settings-pixels = { $pixels } px
settings-animations = Animations
settings-animations-note = Sliding bars and panels, growing dialogs and springy buttons.
settings-animations-reduced = Off while the system asks for reduced motion.

settings-hardware = Use the graphics card
settings-hardware-note = Decodes video and encodes exports on it: uses less power, plays large videos smoothly and exports faster. Takes effect for files opened afterwards.
settings-audio-device = Sound output
settings-audio-device-system = System default
settings-resume = Resume where you left off
settings-resume-note = Files start where they were when last closed.
settings-folder-playlists = Play the folder's other files
settings-folder-playlists-note = Opening one file lists the others in its folder, which play after it.
settings-streams = Streams and web videos
settings-stream-quality = Quality
settings-stream-quality-note = For streams and web videos that come in several qualities. Takes effect for what opens afterwards.
settings-quality-auto = Automatic, as tall as the screen
settings-quality-best = Best
settings-web-pages = Play web videos with yt-dlp
settings-web-pages-found = yt-dlp finds the video on pages such as YouTube's.
settings-web-pages-missing = yt-dlp is not installed. Install it to play the videos on pages such as YouTube's.
settings-folders = Folders
settings-recordings = Recordings
settings-screenshots = Saved frames

settings-interface-language = Interface language
# The first entry of the language list.
settings-language-system = Follow system
settings-track-languages = Track languages
settings-track-languages-note = Language codes such as en or fra, most wanted first, separated by commas. plaa picks audio and subtitles in these languages when a file has them.
settings-audio-languages = Audio
settings-subtitle-languages = Subtitles

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB interface
settings-avb-interface-note = The wired port plaa finds AVB and Milan talkers and listeners on. Sending needs a port with a PTP hardware clock, such as an Intel I210's.
settings-avb-off = Off
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (lists only: no PTP clock)
settings-avb-talker = Send what plays
settings-avb-talker-note = plaa becomes a Milan talker, sending the sound of the window last played to the listeners connected to it.
settings-avb-name = Name on the network
settings-avb-name-note = Controllers show plaa as “{ $name }”. Press Enter to rename it.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa on { $computer }
avb-could-not-start = Could not use { $interface }: { $error }
network-title = Network Audio
network-plaa = plaa
network-off = Not sending. Turn on Send what plays in Settings to send the sound to listeners.
network-blocked = Not sending: { $reason }
network-waiting = Ready to send, waiting for a listener to connect.
network-sending = Sending to the listeners connected.
network-connections = Connections
network-looking = Looking for talkers and listeners…
network-not-connected = Not connected
# A stream on the network: its device's name and its own.
network-stream = { $entity }: { $stream }
network-unknown-talker = A talker not found ({ $talker })
network-failed = Could not connect: { $reason }

## The macOS menu bar

menu-about = About plaa
menu-settings = Settings…
menu-services = Services
menu-hide = Hide plaa
menu-hide-others = Hide Others
menu-show-all = Show All
menu-quit = Quit plaa
menu-file = File
menu-open = Open…
menu-open-address = Open Address…
menu-close = Close Window
menu-edit = Edit
menu-cut = Cut
menu-copy = Copy
menu-paste = Paste
menu-select-all = Select All
menu-undo = Undo
menu-redo = Redo
menu-trim = Trim…
menu-split = Split Clip
menu-rotate-left = Rotate Left
menu-rotate-right = Rotate Right
menu-flip-horizontal = Flip Horizontal
menu-flip-vertical = Flip Vertical
menu-revert = Revert to Original
menu-export = Export…
menu-view = View
menu-playback = Playback
menu-play-pause = Play or Pause
menu-to-start = Go to Start
menu-previous-chapter = Previous Chapter
menu-next-chapter = Next Chapter
menu-faster = Faster
menu-slower = Slower
menu-normal-speed = Normal Speed
menu-ab-loop = Loop Between Two Points
menu-volume-up = Louder
menu-volume-down = Quieter
menu-mute = Mute or Unmute
menu-next-subtitles = Next Subtitles
menu-next-audio = Next Audio Track
menu-window = Window
menu-minimize = Minimize
menu-zoom = Zoom
menu-bring-all-to-front = Bring All to Front

## Editing

action-edit = Edit
title-edited = { $name } (edited)
edit-trim = Trim
edit-trim-apply = Trim
edit-split = Split clip
edit-remove-clip = Remove clip
edit-done = Done
edit-rotate-left = Rotate left
edit-rotate-right = Rotate right
edit-flip-horizontal = Flip horizontally
edit-flip-vertical = Flip vertically
edit-undo = Undo
edit-redo = Redo
edit-revert = Revert to original
edit-export = Export…

## Exporting

export-title = Export
export-format = Format
export-size = Size
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Sound only
export-size-original = { $lines }p, as it is
export-on-graphics-card = Encoded on the graphics card ({ $api })
export-on-processor = Encoded on the processor, which takes longer
export-no-encoder = No encoder for this format opens here
export-checking = Looking for an encoder…
export-sound-format = Sound format
export-quality = Quality
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Compressed; plays almost everywhere, Apple’s devices included
export-mp3-note = Compressed; plays everywhere
export-opus-note = Compressed; the best sound for its size, though not every player takes it
export-flac-note = Lossless: the sound exactly as it is, in an open format
export-alac-note = Lossless: the sound exactly as it is, as Apple’s devices play it
export-wav-note = Uncompressed: the sound exactly as it is, in the largest files
export-kilobits = { $rate } kb/s
export-bits = { $bits }-bit
export-rate-lower = Saved at { $rate } kHz, below the file’s { $file } kHz
export-choose = Export…
export-name = { $name } export
export-name-edited = { $name } edited
export-progress = Exporting “{ $name }”
export-stop = Stop exporting
notice-exported = Exported “{ $name }”
notice-export-stopped = Export stopped
notice-export-failed = Couldn’t export: { $error }
close-title = Close without exporting?
close-unexported = The changes to “{ $name }” are lost unless you export them.
close-exporting = Exporting “{ $name }” stops, and what it wrote so far is lost.
close-anyway = Close
close-quit-anyway = Quit

## Recording

record-title = Sound recording
record-file-name = Sound recording
record-start = Record
record-stop = Stop recording
record-no-input = No microphone
record-microphone = Microphone
record-default-microphone = The system’s default
record-playing = What the computer plays
record-mono = mono
record-stereo = stereo
record-channels = { $count } channels
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, saved at { $saved } kHz
action-record-sound = Record sound
open-record-sound = New sound recording
start-record-sound = Record Sound
menu-record-sound = New Sound Recording
record-screen-title = Screen recording
record-screen-file-name = Screen recording
record-screen-whole = Screen
record-screen-number = Screen { $number }
record-screen-main = Screen { $number } (main)
record-screens = Screens
record-windows = Windows
record-screen-what = What to record
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Every picture
record-rate-tip = Pictures a second
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } dropped
record-screen-window = Window
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = No longer shared
record-screen-again = Choose again
record-screen-failed = Couldn’t record the screen
record-sound = Sound
record-no-sound = No sound
# How much louder the input is taken, for a quiet microphone.
record-gain = Louder
record-gain-value = +{ $decibels } dB
record-level-automatically = Level automatically
record-hide-title = plaa hides while it records
record-hide-tray = To stop recording, click the red dot plaa puts in the tray, or open plaa again.
record-hide-reopen = To stop recording, open plaa again.
record-hide-start = Start recording
record-tray-title = plaa is recording the screen
record-tray-tip = Click to stop recording
action-record-screen = Record screen
open-record-screen = New screen recording
start-record-screen = Record Screen
menu-record-screen = New Screen Recording
record-camera-title = Camera recording
record-camera-file-name = Camera recording
record-camera = Camera
record-camera-failed = Couldn’t open the camera
action-record-camera = Record camera
open-record-camera = New camera recording
start-record-camera = Record Camera
menu-record-camera = New Camera Recording
notice-no-camera = There is no camera to record
notice-recorded = Saved “{ $name }” in { $folder }
notice-record-failed = Couldn’t record: { $error }
