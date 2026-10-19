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

start-hint = Open a video or audio file, or a stream's address.
start-open = Open
start-open-address = Open Address

## Actions, as buttons, tooltips and menus name them

action-open = Open
action-open-address = Open Address
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
action-exit-full-screen = Exit Full Screen

## The floating controls

overlay-keep-shown = Keep controls docked
overlay-auto-hide = Float controls over the picture

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Subtitles
tracks-off = Off
# A track the file gives no name or language, by its number.
tracks-numbered = Track { $number }

## Inspector

info-title = Inspector
info-file = File
info-name = Name
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
address-note = A stream or file on the network, such as https://example.com/live.m3u8 or rtsp://camera.local/stream.
address-label = Address
address-open = Open

## Notices

notice-dialog-failed = The file dialog could not open: { $error }
notice-screenshot-saved = Saved “{ $name }”.
notice-screenshot-failed = The picture could not be saved: { $error }
notice-no-sound = Playing without sound: { $reason }

## Settings

settings-title = Settings
settings-tab-appearance = Appearance
settings-tab-playback = Playback
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

settings-hardware = Decode on the graphics card
settings-hardware-note = Uses less power and plays large videos smoothly. Takes effect for files opened afterwards.
settings-audio-device = Sound output
settings-audio-device-system = System default
settings-resume = Resume where you left off
settings-resume-note = Files start where they were when last closed.
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
