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
      -h, --help        Show this help
      -V, --version     Show the version
          --interfaces  Show the wired interfaces, and whether plaa can
                        send AVB streams on them
          --mcp         Serve MCP on stdin and stdout, for AI agents to
                        see what the running plaa plays and control it

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
action-cast = Cast
action-assistant = Assistant

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
settings-tab-agents = Agents
settings-tab-assistant = Assistant
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
settings-overlay = Float controls
settings-overlay-note = The toolbar and playback controls float over the picture or the cover and slide away while the pointer rests or is outside the window.
settings-overlay-delay = Hide after
settings-overlay-delay-note = How long the pointer rests over what plays before the controls slide away.
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
settings-avb-off-mac = { $interface } (lists only: AVB is off)
settings-avb-interface-note-mac = The wired port plaa finds AVB and Milan talkers and listeners on. Sending goes through this Mac's own AVB entity, which needs AVB turned on for the port in Audio MIDI Setup's Network Device Browser.
settings-avb-talker-note-mac = plaa sends the sound of the window last played through this Mac's own AVB entity, to the listeners connected to it.
settings-avb-talker = Send what plays
settings-avb-talker-note = plaa becomes a Milan talker, sending the sound of the window last played to the listeners connected to it.
settings-avb-name = Name on the network
settings-avb-name-note = Controllers show plaa as “{ $name }”. Press Enter to rename it.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa on { $computer }
avb-could-not-start = Could not use { $interface }: { $error }
network-title = Network Audio
network-off = Turn on Send what plays in Settings to play on the network.
network-blocked = Not sending: { $reason }
network-waiting = Ready to send, waiting for a listener to connect.
network-sending = Sending to the listeners connected.
network-looking = Looking for listeners…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Starting ptp4l, which keeps the network time plaa's stream is sent by…
network-ptp-elsewhere = ptp4l runs on another interface, so plaa's stream keeps no network time here and listeners may not play it. Choose that interface in Settings, or run ptp4l on this one.
network-ptp-missing = ptp4l is not running, so plaa's stream keeps no network time and listeners may not play it. Install linuxptp with plaa's package, or triib, which runs it.
network-ptp-failed = ptp4l is not running, so plaa's stream keeps no network time and listeners may not play it. plaa could not start it: { $reason }
network-no-listeners = No listener on the network can play plaa's sound.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Play on { $listener }
network-stop-on = Stop playing on { $listener }
network-choose-format = Choose a format to play on { $listener } in
network-failed = Could not connect: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Update triib to play here.
network-choose = Choose a listener to see how it takes its stream.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Listen here
network-no-talkers = No talker on the network sends in a format plaa can play.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Listen to { $talker }
network-listen-stop = Stop listening to { $talker }
network-listen-frames = { $frames } frames received
network-listen-update-triib = Update triib to listen here: plaa plays only what reaches this computer anyway.
network-listen-lite = The port runs AVB Lite, which reserves no streams: plaa plays only what reaches this computer anyway.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Not connected
network-from = From { $stream }
network-from-receiving = From { $stream }, receiving
network-from-waiting = From { $stream }, waiting for the talker
network-from-failed = From { $stream }, the talker's reservation failed: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Media clock
network-locked = Locked to the stream
network-not-locked = Not locked
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formats it can take from plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream ID
# The MAC address the stream is sent to.
network-destination = Destination
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latency
network-microseconds = { $time } µs
network-frames = Frames received
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Frames out of sequence
# How often the stream stopped coming while it was taken.
network-interrupted = Interruptions
network-entity-id = Entity ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = insufficient bandwidth
msrp-failure-2 = insufficient bridge resources
msrp-failure-3 = insufficient bandwidth for the traffic class
msrp-failure-4 = stream ID in use by another talker
msrp-failure-5 = destination address already in use
msrp-failure-6 = pre-empted by a stream of higher rank
msrp-failure-7 = reported latency has changed
msrp-failure-8 = egress port is not AVB capable
msrp-failure-9 = use a different destination address
msrp-failure-10 = out of MSRP resources
msrp-failure-11 = out of MMRP resources
msrp-failure-12 = cannot store the destination address
msrp-failure-13 = priority is not an SR class priority
msrp-failure-14 = frames too large for the medium
msrp-failure-15 = fan-in port limit reached
msrp-failure-16 = first value changed for a registered stream
msrp-failure-17 = VLAN blocked on the egress port
msrp-failure-18 = VLAN tagging disabled on the egress port
msrp-failure-19 = SR class priority mismatch
msrp-failure-unknown = unknown reason
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, at the bridge { $bridge }

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

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Allow outside control
settings-allow-outside-control-note = AI agents such as Claude Code can see what plays in plaa and control it, through plaa --mcp. plaa asks before each new agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Allowed: { $agents }
settings-forget-agents = Forget
settings-ask-before-note = Ask before an agent:
settings-ask-reading = Sees what plays, or the picture
settings-ask-playing = Controls playback, the sound or full screen
settings-ask-opening = Opens a file or closes a window
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Allow { $agent } to control plaa?
agent-prompt-body = { $agent } asks to use plaa's outside control, to see what plays and control it. You can turn outside control off in Settings.
agent-prompt-allow = Allow
agent-prompt-deny = Don't allow
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Let { $agent } see what plays here?
agent-ask-play = Let { $agent } control playback here?
agent-ask-open = Let { $agent } open or close a window?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } asks to use “{ $tool }”. Settings chooses what plaa asks about.

## Casting to a TV or speaker on the network

cast-looking = Looking for TVs and speakers…
cast-none = No TV or speaker that plays DLNA was found.
# $renderer is the TV's or speaker's name.
cast-stop = Stop playing on { $renderer }
notice-casting = Playing on { $renderer }.
notice-cast-failed = Could not play on { $renderer }: { $reason }
cast-web-page = Web pages cannot be cast yet: open the video's own address instead.

## The assistant panel, and its models in Settings

assistant-add-another = Add a model…
assistant-add-model = The assistant needs a model: a cloud model with its API key, or a local model.
assistant-ask = Ask about what plays
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = The chat was getting too long for { $model } to read at once, so its earlier part was summarized. The assistant may look again at what it saw.
assistant-compacting = Summarizing the earlier chat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } on this computer
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } at { $host }
assistant-new-chat = New chat
assistant-no-model = Add a model in Settings first.
assistant-open-settings = Add a model
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = The conversation no longer fits in what { $model } can read at once. Start a new chat, or choose a model that can read more.
assistant-problem-credit = { $provider } says the account has no credit. A new account needs some bought on { $provider }'s site before its key works; then try again.
assistant-problem-key = { $provider } turned down the API key. Check it in Settings.
assistant-problem-model = { $provider } has no model named { $model }. Check its name in Settings.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } did not answer: { $message }
assistant-problem-rate = { $provider } asks to slow down. Try again in a moment.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } asks to slow down: { $message }
assistant-problem-refused = { $model } declined to answer.
assistant-problem-unavailable = { $provider } is busy or having trouble. Try again in a moment.
assistant-problem-unreachable = plaa could not reach { $provider }. Check your connection, or that the server is running.
assistant-running = Running…
assistant-send = Send
assistant-shortened = The chat was too long for { $model } to read at once, so earlier tool results were left out. The assistant may look again at what it saw.
assistant-stop = Stop
assistant-stopped = Stopped.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Now talking to { $model }.
assistant-thinking = Thinking…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Thoughts
assistant-title = Assistant
settings-assistant-add = Add a model
settings-assistant-add-button = Add
# $model is the model's name.
settings-assistant-added = { $model } answered and is added.
settings-assistant-added-tag = Added
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Address, such as { $example }
settings-assistant-change-key = Change key
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-compatible server
# The menu of how much a local model reads at once.
settings-assistant-context = Context
settings-assistant-context-note = More lets the assistant keep more of a chat in mind, but the model takes more memory and may answer more slowly.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Open the page
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } is running at { $address }.
settings-assistant-found-none = { $provider } has no models yet. Download one with it, then look again.
settings-assistant-get-key = Get an API key
settings-assistant-get-server = Get { $provider }
settings-assistant-in-use = In use
settings-assistant-key = API key
# $error is what the keychain said.
settings-assistant-key-failed = The key could not be kept in the keychain: { $error }
settings-assistant-key-kept = Your { $provider } key is kept in the system's keychain.
settings-assistant-key-needed = This model needs an API key.
settings-assistant-key-refused = { $provider } turned down this key. Check that it was copied whole, from the right account.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Make a key on { $provider }'s site and paste it here.
settings-assistant-look-again = Look again
settings-assistant-looking = Looking for models…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, such as { $example }
settings-assistant-no-server = plaa found no { $provider } running on this computer. Start it, or give its address below.
settings-assistant-no-tools = Can't use tools, which the assistant needs
settings-assistant-none = No models yet. Add one below: a local model, such as Ollama's, stays on this computer; a cloud model needs an API key from its provider.
settings-assistant-note = The models the assistant panel can talk to. Keys are kept in the system's keychain.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Use another address
settings-assistant-recommended = Recommended
settings-assistant-remove = Remove
settings-assistant-sees = Sees pictures
settings-assistant-trying = Trying…
settings-assistant-use = Use
settings-assistant-use-key = Continue
settings-assistant-uses-tools = Uses tools
