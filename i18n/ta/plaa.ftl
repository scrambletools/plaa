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
language-name = தமிழ்

## Common

app-name = plaa
common-cancel = ரத்துசெய்
common-close = மூடு
common-more = மேலும்

## Command line

usage-help =
    பயன்பாடு: plaa [FILE OR ADDRESS]...

    வீடியோ, ஆடியோ கோப்புகளையும், https://example.com/live.m3u8 போன்ற
    முகவரிகளில் உள்ள ஸ்ட்ரீம்களையும் இயக்கும். ஒவ்வொன்றும் இயங்கும்
    plaa-வின் ஒரு சாளரத்தில் திறக்கும்; தேவைப்பட்டால் plaa தானாகத் தொடங்கும்.

    விருப்பங்கள்:
      -h, --help        இந்த உதவியைக் காட்டு
      -V, --version     பதிப்பைக் காட்டு
          --interfaces  வயர்டு interfaces-ஐயும், அவற்றில் plaa AVB streams-ஐ
                        அனுப்ப முடியுமா என்பதையும் காட்டு
          --mcp         stdin, stdout வழியாக MCP-ஐ வழங்கும், இதனால் AI முகவர்கள்
                        இயங்கும் plaa என்ன இயக்குகிறது என்பதைப் பார்த்து
                        அதைக் கட்டுப்படுத்தலாம்

## Start window

start-hint = வீடியோ அல்லது ஆடியோ கோப்பு, ஸ்ட்ரீம் அல்லது இணைய வீடியோவைத் திறக்கவும்.
start-open = திற
start-open-address = முகவரியைத் திற

## Actions, as buttons, tooltips and menus name them

action-open = திற
action-open-address = முகவரியைத் திற
open-file = கோப்பைத் திற…
open-address = முகவரியைத் திற…
action-settings = அமைப்புகள்
action-info = ஆய்வி
action-loop = சுழற்சி
action-screenshot = சட்டகத்தைப் படமாகச் சேமி
action-play = இயக்கு
action-pause = இடைநிறுத்து
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } வினாடி பின்னால்
   *[other] { $seconds } வினாடிகள் பின்னால்
}
action-forward = { $seconds ->
    [one] { $seconds } வினாடி முன்னால்
   *[other] { $seconds } வினாடிகள் முன்னால்
}
action-mute = ஒலியடக்கு
action-unmute = ஒலியை மீட்டமை
action-speed = வேகம்
action-tracks = ஆடியோவும் வசனங்களும்
action-full-screen = முழுத்திரை
action-playlist = பிளேலிஸ்ட்
action-chapters = அத்தியாயங்கள்
action-picture-in-picture = படத்திற்குள் படம்
action-previous = முந்தையது
action-next = அடுத்தது
action-exit-full-screen = முழுத்திரையிலிருந்து வெளியேறு
action-quality = தரம்
action-visualizer = காட்சிப்படுத்தி
action-network = நெட்வொர்க் ஆடியோ
action-cast = அனுப்பு
action-assistant = உதவியாளர்

## What a sound file shows under its cover

visualizer-spectrum = ஸ்பெக்ட்ரம்
visualizer-waveform = அலைவடிவம்
visualizer-prism = பட்டகம்
visualizer-cover = அட்டைப்படம் மட்டும்

## The floating controls

overlay-keep-shown = கட்டுப்பாடுகளை நிலையாக வை
overlay-auto-hide = கட்டுப்பாடுகளைப் படத்தின் மேல் மிதக்கவிடு

## Audio and subtitle tracks

tracks-audio = ஆடியோ
tracks-subtitles = வசனங்கள்
tracks-off = முடக்கம்
# A track the file gives no name or language, by its number.
tracks-numbered = டிராக் { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = ஆடியோ மட்டும்

## Inspector

info-title = ஆய்வி
playlist-title = பிளேலிஸ்ட்
chapters-numbered = அத்தியாயம் { $number }
info-file = கோப்பு
info-name = பெயர்
info-address = முகவரி
info-folder = கோப்புறை
info-size = அளவு
info-format = வடிவமைப்பு
info-duration = கால அளவு
info-bit-rate = பிட் வீதம்
info-video = வீடியோ
info-audio = ஆடியோ
info-codec = கோடெக்
info-dimensions = பரிமாணங்கள்
info-frame-rate = சட்டக வீதம்
info-frames-per-second = வினாடிக்கு { $rate } சட்டகங்கள்
info-pixel-format = பிக்சல் வடிவமைப்பு
info-decoding = குறிவிலக்கம்
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = கிராஃபிக்ஸ் கார்டு ({ $api })
info-decoding-software = செயலி
info-sample-rate = மாதிரி வீதம்
info-hertz = { $rate } Hz
info-channels = சேனல்கள்
info-language = மொழி
info-no-sound = ஒலி இல்லை
info-tags = குறிச்சொற்கள்

## Dialogs

dialog-open = திற
dialog-filter-media = வீடியோவும் ஆடியோவும்
dialog-filter-all = எல்லாக் கோப்புகளும்

address-title = முகவரியைத் திற
address-note = வீடியோ உள்ள இணையப் பக்கம் (எ.கா. ஒரு YouTube வீடியோவின் பக்கம்), அல்லது நெட்வொர்க்கில் உள்ள ஸ்ட்ரீம் அல்லது கோப்பு (எ.கா. https://example.com/live.m3u8).
address-label = முகவரி
address-open = திற
address-recent = சமீபத்தியவை
address-forget = சமீபத்தியவற்றிலிருந்து அகற்று

## Notices

notice-dialog-failed = கோப்பு உரையாடலைத் திறக்க முடியவில்லை: { $error }
notice-screenshot-saved = “{ $name }” சேமிக்கப்பட்டது.
notice-screenshot-failed = படத்தைச் சேமிக்க முடியவில்லை: { $error }
notice-no-sound = ஒலி இல்லாமல் இயங்குகிறது: { $reason }
notice-loop-from = { $time } முதல் சுழற்சி. அது முடிய வேண்டிய இடத்தில் மீண்டும் அழுத்தவும்.
notice-loop-between = { $from } முதல் { $to } வரை சுழற்சியில் இயங்குகிறது.
notice-loop-off = சுழற்சி முடக்கப்பட்டது.

## Errors, shown in place of what could not play

error-needs-yt-dlp = இது ஓர் இணையப் பக்கம் போலத் தெரிகிறது. இணையப் பக்கங்களில் உள்ள வீடியோக்களை plaa, yt-dlp மூலம் இயக்குகிறது; அது நிறுவப்படவில்லை.
error-empty-list = இந்தப் பட்டியலில் இயக்க எதுவும் இல்லை.

## Settings

settings-title = அமைப்புகள்
settings-tab-appearance = தோற்றம்
settings-tab-playback = இயக்கம்
settings-tab-network = நெட்வொர்க்
settings-tab-language = மொழி
settings-tab-agents = முகவர்கள்
settings-tab-assistant = உதவியாளர்
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (மேம்பாட்டுப் பதிப்பு, { $build })
settings-choose = தேர்வுசெய்…

settings-appearance-system = கணினியைப் பின்பற்று
settings-appearance-light = வெளிர்
settings-appearance-dark = அடர்
settings-colors = வண்ணங்கள்
settings-system-accent = கணினியின் அக்சென்ட் வண்ணத்தைப் பயன்படுத்து
settings-omarchy-note = வண்ணங்கள் “{ $theme }” இன் அக்சென்ட்டிலிருந்து உருவாக்கப்படுகின்றன.
settings-system-accent-note = வண்ணங்கள் கணினியின் அக்சென்ட் வண்ணத்திலிருந்து உருவாக்கப்படுகின்றன.
settings-system-accent-none = கணினியில் அக்சென்ட் வண்ணம் இல்லை, எனவே plaa கீழே தேர்ந்தெடுக்கப்பட்ட வண்ணத்தைப் பயன்படுத்துகிறது.
settings-accent-chosen-note = வண்ணங்கள் கீழே தேர்ந்தெடுக்கப்பட்ட வண்ணத்திலிருந்து உருவாக்கப்படுகின்றன.
settings-controls = கட்டுப்பாடுகள்
settings-overlay = கட்டுப்பாடுகளை மிதக்கவிடு
settings-overlay-note = கருவிப்பட்டியும் இயக்கக் கட்டுப்பாடுகளும் படம் அல்லது அட்டைப்படத்தின் மேல் மிதக்கும்; சுட்டி அசையாமல் இருக்கும்போதோ சாளரத்திற்கு வெளியே இருக்கும்போதோ நகர்ந்து மறையும்.
settings-overlay-delay = இதற்குப் பின் மறை
settings-overlay-delay-note = கட்டுப்பாடுகள் நகர்ந்து மறைவதற்கு முன், இயங்குவதன் மேல் சுட்டி எவ்வளவு நேரம் அசையாமல் இருக்க வேண்டும்.
settings-seconds = { $seconds } வி.
settings-transparency = ஒளிபுகுதன்மை
settings-transparency-note = மிதக்கும் கட்டுப்பாடுகளின் வழியே படம் எவ்வளவு தெரியும்.
settings-percent = { $percent }%
settings-corner-radius = மூலை ஆரம்
settings-corner-radius-note = உரையாடல்களுக்கும் மிதக்கும் கட்டுப்பாடுகளுக்கும்.
settings-pixels = { $pixels } px
settings-animations = அனிமேஷன்கள்
settings-animations-note = நகரும் பட்டிகளும் பலகங்களும், விரியும் உரையாடல்களும், துள்ளும் பொத்தான்களும்.
settings-animations-reduced = கணினி குறைந்த இயக்கத்தைக் கோரும்போது முடக்கப்படும்.

settings-hardware = கிராஃபிக்ஸ் கார்டைப் பயன்படுத்து
settings-hardware-note = வீடியோவை அதில் குறிவிலக்கி, ஏற்றுமதிகளை அதில் குறியாக்கும்: குறைந்த மின்சாரம் செலவாகும், பெரிய வீடியோக்கள் சீராக இயங்கும், ஏற்றுமதி வேகமாக முடியும். இதன் பிறகு திறக்கும் கோப்புகளுக்குப் பொருந்தும்.
settings-audio-device = ஒலி வெளியீடு
settings-audio-device-system = கணினி இயல்புநிலை
settings-resume = விட்ட இடத்திலிருந்து தொடர்
settings-resume-note = கோப்புகள் கடைசியாக மூடப்பட்டபோது இருந்த இடத்திலிருந்து தொடங்கும்.
settings-folder-playlists = கோப்புறையின் மற்ற கோப்புகளையும் இயக்கு
settings-folder-playlists-note = ஒரு கோப்பைத் திறந்தால், அதன் கோப்புறையில் உள்ள மற்றவையும் பட்டியலிடப்பட்டு, அதன் பிறகு இயங்கும்.
settings-streams = ஸ்ட்ரீம்களும் இணைய வீடியோக்களும்
settings-stream-quality = தரம்
settings-stream-quality-note = பல தரங்களில் கிடைக்கும் ஸ்ட்ரீம்களுக்கும் இணைய வீடியோக்களுக்கும். இதன் பிறகு திறப்பவற்றுக்குப் பொருந்தும்.
settings-quality-auto = தானியங்கு, திரையின் உயரத்திற்கு ஏற்ப
settings-quality-best = மிகச் சிறந்தது
settings-web-pages = இணைய வீடியோக்களை yt-dlp மூலம் இயக்கு
settings-web-pages-found = YouTube போன்ற பக்கங்களில் உள்ள வீடியோவை yt-dlp கண்டறியும்.
settings-web-pages-missing = yt-dlp நிறுவப்படவில்லை. YouTube போன்ற பக்கங்களில் உள்ள வீடியோக்களை இயக்க அதை நிறுவவும்.
settings-folders = கோப்புறைகள்
settings-recordings = பதிவுகள்
settings-screenshots = சேமித்த சட்டகங்கள்

settings-interface-language = இடைமுக மொழி
# The first entry of the language list.
settings-language-system = கணினியைப் பின்பற்று
settings-track-languages = டிராக் மொழிகள்
settings-track-languages-note = en அல்லது fra போன்ற மொழிக் குறியீடுகள், அதிகம் விரும்புவது முதலில், காற்புள்ளிகளால் பிரிக்கப்பட்டவை. கோப்பில் இந்த மொழிகளில் ஆடியோவும் வசனங்களும் இருந்தால் plaa அவற்றைத் தேர்ந்தெடுக்கும்.
settings-audio-languages = ஆடியோ
settings-subtitle-languages = வசனங்கள்

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB interface
settings-avb-interface-note = plaa, AVB மற்றும் Milan Talkers-ஐயும் Listeners-ஐயும் கண்டறியும் வயர்டு port. அனுப்புவதற்கு PTP hardware clock உள்ள port தேவை, எ.கா. Intel I210-இன் port.
settings-avb-off = முடக்கம்
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (பட்டியலிட மட்டும்: PTP clock இல்லை)
settings-avb-off-mac = { $interface } (பட்டியலிட மட்டும்: AVB முடக்கத்தில் உள்ளது)
settings-avb-interface-note-mac = plaa, AVB மற்றும் Milan Talkers-ஐயும் Listeners-ஐயும் கண்டறியும் வயர்டு port. அனுப்புதல் இந்த Mac-இன் சொந்த AVB entity வழியாக நடக்கும்; அதற்கு Audio MIDI Setup-இன் Network Device Browser-இல் இந்த port-க்கு AVB இயக்கப்பட்டிருக்க வேண்டும்.
settings-avb-talker-note-mac = கடைசியாக இயக்கிய சாளரத்தின் ஒலியை plaa இந்த Mac-இன் சொந்த AVB entity வழியாக, அதனுடன் இணைக்கப்பட்ட Listeners-க்கு அனுப்பும்.
settings-avb-talker = இயங்குவதை அனுப்பு
settings-avb-talker-note = plaa ஒரு Milan Talker ஆகி, கடைசியாக இயக்கிய சாளரத்தின் ஒலியை அதனுடன் இணைக்கப்பட்ட Listeners-க்கு அனுப்பும்.
settings-avb-name = நெட்வொர்க்கில் பெயர்
settings-avb-name-note = Controllers plaa-வை “{ $name }” எனக் காட்டும். பெயரை மாற்ற Enter-ஐ அழுத்தவும்.
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } இல் plaa
avb-could-not-start = { $interface }-ஐப் பயன்படுத்த முடியவில்லை: { $error }
network-title = நெட்வொர்க் ஆடியோ
network-off = நெட்வொர்க்கில் இயக்க, அமைப்புகளில் “இயங்குவதை அனுப்பு” என்பதை இயக்கவும்.
network-blocked = அனுப்பவில்லை: { $reason }
network-waiting = அனுப்பத் தயார், ஒரு Listener இணைவதற்காகக் காத்திருக்கிறது.
network-sending = இணைக்கப்பட்ட Listeners-க்கு அனுப்புகிறது.
network-looking = Listeners-ஐத் தேடுகிறது…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l-ஐத் தொடங்குகிறது; plaa-வின் stream அனுப்பப்படும் நெட்வொர்க் நேரத்தை அதுவே பராமரிக்கிறது…
network-ptp-elsewhere = ptp4l வேறொரு interface-இல் இயங்குகிறது, எனவே இங்கே plaa-வின் stream-க்கு நெட்வொர்க் நேரம் கிடைக்காது, Listeners அதை இயக்காமல் போகலாம். அமைப்புகளில் அந்த interface-ஐத் தேர்ந்தெடுக்கவும், அல்லது இதில் ptp4l-ஐ இயக்கவும்.
network-ptp-missing = ptp4l இயங்கவில்லை, எனவே plaa-வின் stream-க்கு நெட்வொர்க் நேரம் கிடைக்காது, Listeners அதை இயக்காமல் போகலாம். plaa-வின் தொகுப்புடன் linuxptp-ஐ நிறுவவும், அல்லது அதை இயக்கும் triib-ஐ நிறுவவும்.
network-ptp-failed = ptp4l இயங்கவில்லை, எனவே plaa-வின் stream-க்கு நெட்வொர்க் நேரம் கிடைக்காது, Listeners அதை இயக்காமல் போகலாம். plaa-வால் அதைத் தொடங்க முடியவில்லை: { $reason }
network-no-listeners = நெட்வொர்க்கில் உள்ள எந்த Listener-ஆலும் plaa-வின் ஒலியை இயக்க முடியாது.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener }-இல் இயக்கு
network-stop-on = { $listener }-இல் இயக்குவதை நிறுத்து
network-choose-format = { $listener }-இல் இயக்குவதற்கான format-ஐத் தேர்ந்தெடு
network-failed = இணைக்க முடியவில்லை: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = இங்கே இயக்க triib-ஐப் புதுப்பிக்கவும்.
network-choose = ஒரு Listener தன் stream-ஐ எப்படிப் பெறுகிறது என்பதைப் பார்க்க, அதைத் தேர்ந்தெடுக்கவும்.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = இங்கே கேள்
network-no-talkers = plaa இயக்கக்கூடிய format-இல் அனுப்பும் Talker எதுவும் நெட்வொர்க்கில் இல்லை.
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker }-ஐக் கேள்
network-listen-stop = { $talker }-ஐக் கேட்பதை நிறுத்து
network-listen-frames = பெற்ற frames: { $frames }
network-listen-update-triib = இங்கே கேட்க triib-ஐப் புதுப்பிக்கவும்: எப்படியும் இந்தக் கணினியை வந்தடைவதை மட்டுமே plaa இயக்கும்.
network-listen-lite = இந்த port AVB Lite-இல் இயங்குகிறது, அது எந்த stream-ஐயும் reserve செய்வதில்லை: எப்படியும் இந்தக் கணினியை வந்தடைவதை மட்டுமே plaa இயக்கும்.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = நிலை
network-not-connected = இணைக்கப்படவில்லை
network-from = { $stream }-இலிருந்து
network-from-receiving = { $stream }-இலிருந்து, பெறுகிறது
network-from-waiting = { $stream }-இலிருந்து, Talker-க்காகக் காத்திருக்கிறது
network-from-failed = { $stream }-இலிருந்து, Talker-இன் reservation தோல்வி: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Media clock
network-locked = Stream-உடன் lock ஆனது
network-not-locked = Lock ஆகவில்லை
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = plaa-விடமிருந்து இது பெறக்கூடிய formats
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream ID
# The MAC address the stream is sent to.
network-destination = இலக்கு
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latency
network-microseconds = { $time } µs
network-frames = பெற்ற frames
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = வரிசை தவறிய frames
# How often the stream stopped coming while it was taken.
network-interrupted = தடங்கல்கள்
network-entity-id = Entity ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = போதுமான bandwidth இல்லை
msrp-failure-2 = bridge-இல் போதுமான வளங்கள் இல்லை
msrp-failure-3 = traffic class-க்குப் போதுமான bandwidth இல்லை
msrp-failure-4 = stream ID-ஐ வேறொரு Talker பயன்படுத்துகிறது
msrp-failure-5 = destination address ஏற்கனவே பயன்பாட்டில் உள்ளது
msrp-failure-6 = அதிக rank கொண்ட stream இதை முந்தியது
msrp-failure-7 = தெரிவிக்கப்பட்ட latency மாறிவிட்டது
msrp-failure-8 = egress port-இல் AVB ஆதரவு இல்லை
msrp-failure-9 = வேறு destination address-ஐப் பயன்படுத்து
msrp-failure-10 = MSRP வளங்கள் தீர்ந்துவிட்டன
msrp-failure-11 = MMRP வளங்கள் தீர்ந்துவிட்டன
msrp-failure-12 = destination address-ஐச் சேமிக்க முடியாது
msrp-failure-13 = முன்னுரிமை SR class முன்னுரிமை அல்ல
msrp-failure-14 = frames ஊடகத்துக்கு மிகப் பெரியவை
msrp-failure-15 = fan-in port வரம்பை எட்டியது
msrp-failure-16 = பதிவுசெய்யப்பட்ட stream-இன் முதல் மதிப்பு மாறியது
msrp-failure-17 = egress port-இல் VLAN தடுக்கப்பட்டுள்ளது
msrp-failure-18 = egress port-இல் VLAN tagging முடக்கப்பட்டுள்ளது
msrp-failure-19 = SR class முன்னுரிமை பொருந்தவில்லை
msrp-failure-unknown = தெரியாத காரணம்
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, bridge { $bridge }-இல்

## The macOS menu bar

menu-about = plaa பற்றி
menu-settings = அமைப்புகள்…
menu-services = சேவைகள்
menu-hide = plaa-வை மறை
menu-hide-others = மற்றவற்றை மறை
menu-show-all = அனைத்தையும் காட்டு
menu-quit = plaa-விலிருந்து வெளியேறு
menu-file = கோப்பு
menu-open = திற…
menu-open-address = முகவரியைத் திற…
menu-close = சாளரத்தை மூடு
menu-edit = திருத்து
menu-cut = வெட்டு
menu-copy = நகலெடு
menu-paste = ஒட்டு
menu-select-all = அனைத்தையும் தேர்ந்தெடு
menu-undo = செயல்தவிர்
menu-redo = மீண்டும் செய்
menu-trim = நறுக்கு…
menu-split = கிளிப்பைப் பிரி
menu-rotate-left = இடப்புறம் சுழற்று
menu-rotate-right = வலப்புறம் சுழற்று
menu-flip-horizontal = கிடைமட்டமாகப் புரட்டு
menu-flip-vertical = செங்குத்தாகப் புரட்டு
menu-revert = அசல் நிலைக்கு மீட்டமை
menu-export = ஏற்றுமதி செய்…
menu-view = காட்சி
menu-playback = இயக்கம்
menu-play-pause = இயக்கு அல்லது இடைநிறுத்து
menu-to-start = தொடக்கத்திற்குச் செல்
menu-previous-chapter = முந்தைய அத்தியாயம்
menu-next-chapter = அடுத்த அத்தியாயம்
menu-faster = வேகமாக
menu-slower = மெதுவாக
menu-normal-speed = இயல்பான வேகம்
menu-ab-loop = இரண்டு புள்ளிகளுக்கு இடையே சுழற்சி
menu-volume-up = ஒலியைக் கூட்டு
menu-volume-down = ஒலியைக் குறை
menu-mute = ஒலியடக்கு அல்லது ஒலியை மீட்டமை
menu-next-subtitles = அடுத்த வசனங்கள்
menu-next-audio = அடுத்த ஆடியோ டிராக்
menu-window = சாளரம்
menu-minimize = குறுக்கு
menu-zoom = சாளரத்தைப் பெரிதாக்கு
menu-bring-all-to-front = அனைத்தையும் முன்னால் கொண்டுவா

## Editing

action-edit = திருத்து
title-edited = { $name } (திருத்தப்பட்டது)
edit-trim = நறுக்கு
edit-trim-apply = நறுக்கு
edit-split = கிளிப்பைப் பிரி
edit-remove-clip = கிளிப்பை அகற்று
edit-done = முடிந்தது
edit-rotate-left = இடப்புறம் சுழற்று
edit-rotate-right = வலப்புறம் சுழற்று
edit-flip-horizontal = கிடைமட்டமாகப் புரட்டு
edit-flip-vertical = செங்குத்தாகப் புரட்டு
edit-undo = செயல்தவிர்
edit-redo = மீண்டும் செய்
edit-revert = அசல் நிலைக்கு மீட்டமை
edit-export = ஏற்றுமதி செய்…

## Exporting

export-title = ஏற்றுமதி செய்
export-format = வடிவமைப்பு
export-size = அளவு
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = ஒலி மட்டும்
export-size-original = { $lines }p, உள்ளபடியே
export-on-graphics-card = கிராஃபிக்ஸ் கார்டில் குறியாக்கப்படும் ({ $api })
export-on-processor = செயலியில் குறியாக்கப்படும், இதற்கு அதிக நேரமாகும்
export-no-encoder = இந்த வடிவமைப்புக்கான எந்தக் குறியாக்கியும் இங்கே திறக்கவில்லை
export-checking = குறியாக்கியைத் தேடுகிறது…
export-sound-format = ஒலி வடிவமைப்பு
export-quality = தரம்
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = சுருக்கப்பட்டது; Apple-இன் சாதனங்கள் உட்பட கிட்டத்தட்ட எல்லா இடங்களிலும் இயங்கும்
export-mp3-note = சுருக்கப்பட்டது; எல்லா இடங்களிலும் இயங்கும்
export-opus-note = சுருக்கப்பட்டது; அதன் அளவுக்கு மிகச் சிறந்த ஒலி, ஆனால் எல்லா பிளேயர்களும் இதை ஏற்பதில்லை
export-flac-note = இழப்பற்றது: ஒலி உள்ளபடியே, திறந்த வடிவமைப்பில்
export-alac-note = இழப்பற்றது: ஒலி உள்ளபடியே, Apple-இன் சாதனங்கள் இயக்கும் வடிவில்
export-wav-note = சுருக்கப்படாதது: ஒலி உள்ளபடியே, மிகப் பெரிய கோப்புகளில்
export-kilobits = { $rate } kb/s
export-bits = { $bits }-பிட்
export-rate-lower = { $rate } kHz இல் சேமிக்கப்படும், கோப்பின் { $file } kHz-ஐ விடக் குறைவு
export-choose = ஏற்றுமதி செய்…
export-name = { $name } ஏற்றுமதி
export-name-edited = { $name } திருத்தப்பட்டது
export-progress = “{ $name }” ஏற்றுமதி செய்யப்படுகிறது
export-stop = ஏற்றுமதியை நிறுத்து
notice-exported = “{ $name }” ஏற்றுமதி செய்யப்பட்டது
notice-export-stopped = ஏற்றுமதி நிறுத்தப்பட்டது
notice-export-failed = ஏற்றுமதி செய்ய முடியவில்லை: { $error }
close-title = ஏற்றுமதி செய்யாமல் மூடவா?
close-unexported = “{ $name }” இல் செய்த மாற்றங்களை ஏற்றுமதி செய்யாவிட்டால் அவை இழக்கப்படும்.
close-exporting = “{ $name }” ஏற்றுமதி நின்றுவிடும், இதுவரை எழுதியவை இழக்கப்படும்.
close-anyway = மூடு
close-quit-anyway = வெளியேறு

## Recording

record-title = ஒலிப்பதிவு
record-file-name = ஒலிப்பதிவு
record-start = பதிவுசெய்
record-stop = பதிவை நிறுத்து
record-no-input = மைக்ரோஃபோன் இல்லை
record-microphone = மைக்ரோஃபோன்
record-default-microphone = கணினியின் இயல்புநிலை
record-playing = கணினி இயக்குவது
record-mono = மோனோ
record-stereo = ஸ்டீரியோ
record-channels = { $count } சேனல்கள்
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, { $saved } kHz இல் சேமிக்கப்படும்
action-record-sound = ஒலியைப் பதிவுசெய்
open-record-sound = புதிய ஒலிப்பதிவு
start-record-sound = ஒலியைப் பதிவுசெய்
menu-record-sound = புதிய ஒலிப்பதிவு
record-screen-title = திரைப் பதிவு
record-screen-file-name = திரைப் பதிவு
record-screen-whole = திரை
record-screen-number = திரை { $number }
record-screen-main = திரை { $number } (முதன்மை)
record-screens = திரைகள்
record-windows = சாளரங்கள்
record-screen-what = எதைப் பதிவுசெய்வது
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = ஒவ்வொரு படமும்
record-rate-tip = வினாடிக்குப் படங்கள்
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } விடுபட்டது
   *[other] { $size } · { $count } விடுபட்டன
}
record-screen-window = சாளரம்
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = இனி பகிரப்படவில்லை
record-screen-again = மீண்டும் தேர்வுசெய்
record-screen-failed = திரையைப் பதிவுசெய்ய முடியவில்லை
record-sound = ஒலி
record-no-sound = ஒலி இல்லை
# How much louder the input is taken, for a quiet microphone.
record-gain = ஒலிப் பெருக்கம்
record-gain-value = +{ $decibels } dB
record-level-automatically = ஒலி அளவைத் தானாகச் சீராக்கு
record-hide-title = பதிவுசெய்யும்போது plaa மறைந்திருக்கும்
record-hide-tray = பதிவை நிறுத்த, plaa ட்ரேயில் வைக்கும் சிவப்புப் புள்ளியைக் கிளிக் செய்யவும், அல்லது plaa-வை மீண்டும் திறக்கவும்.
record-hide-reopen = பதிவை நிறுத்த, plaa-வை மீண்டும் திறக்கவும்.
record-hide-start = பதிவைத் தொடங்கு
record-tray-title = plaa திரையைப் பதிவுசெய்கிறது
record-tray-tip = பதிவை நிறுத்தக் கிளிக் செய்யவும்
action-record-screen = திரையைப் பதிவுசெய்
open-record-screen = புதிய திரைப் பதிவு
start-record-screen = திரையைப் பதிவுசெய்
menu-record-screen = புதிய திரைப் பதிவு
record-camera-title = கேமரா பதிவு
record-camera-file-name = கேமரா பதிவு
record-camera = கேமரா
record-camera-failed = கேமராவைத் திறக்க முடியவில்லை
action-record-camera = கேமராவில் பதிவுசெய்
open-record-camera = புதிய கேமரா பதிவு
start-record-camera = கேமராவில் பதிவுசெய்
menu-record-camera = புதிய கேமரா பதிவு
notice-no-camera = பதிவுசெய்யக் கேமரா இல்லை
notice-recorded = “{ $name }” { $folder } இல் சேமிக்கப்பட்டது
notice-record-failed = பதிவுசெய்ய முடியவில்லை: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = வெளிக் கட்டுப்பாட்டை அனுமதி
settings-allow-outside-control-note = Claude Code போன்ற AI முகவர்கள் plaa --mcp மூலம் plaa-வில் இயங்குவதைப் பார்க்கவும் அதைக் கட்டுப்படுத்தவும் முடியும். ஒவ்வொரு புதிய முகவருக்கும் முன் plaa கேட்கும்.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = அனுமதிக்கப்பட்டவை: { $agents }
settings-forget-agents = மறந்துவிடு
settings-ask-before-note = முகவர் இவற்றைச் செய்யும் முன் கேள்:
settings-ask-reading = இயங்குவதை அல்லது படத்தைப் பார்த்தல்
settings-ask-playing = இயக்கம், ஒலி அல்லது முழுத்திரையைக் கட்டுப்படுத்துதல்
settings-ask-opening = கோப்பைத் திறத்தல் அல்லது சாளரத்தை மூடுதல்
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = plaa-வைக் கட்டுப்படுத்த { $agent } முகவரை அனுமதிக்கவா?
agent-prompt-body = இயங்குவதைப் பார்க்கவும் அதைக் கட்டுப்படுத்தவும் plaa-வின் வெளிக் கட்டுப்பாட்டைப் பயன்படுத்த { $agent } கேட்கிறது. அமைப்புகளில் வெளிக் கட்டுப்பாட்டை முடக்கலாம்.
agent-prompt-allow = அனுமதி
agent-prompt-deny = அனுமதிக்காதே
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = இங்கே இயங்குவதைப் பார்க்க { $agent } முகவரை அனுமதிக்கவா?
agent-ask-play = இங்கே இயக்கத்தைக் கட்டுப்படுத்த { $agent } முகவரை அனுமதிக்கவா?
agent-ask-open = சாளரத்தைத் திறக்க அல்லது மூட { $agent } முகவரை அனுமதிக்கவா?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = “{ $tool }” என்பதைப் பயன்படுத்த { $agent } கேட்கிறது. plaa எவற்றைப் பற்றிக் கேட்கும் என்பதை அமைப்புகளில் தேர்ந்தெடுக்கலாம்.

## Casting to a TV or speaker on the network

cast-looking = டிவிகளையும் ஸ்பீக்கர்களையும் தேடுகிறது…
cast-none = டிவி, ஸ்பீக்கர் அல்லது Chromecast எதுவும் கிடைக்கவில்லை.
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer }-இல் இயக்குவதை நிறுத்து
notice-casting = { $renderer }-இல் இயங்குகிறது.
notice-cast-failed = { $renderer }-இல் இயக்க முடியவில்லை: { $reason }
cast-web-page = இணையப் பக்கங்களை இன்னும் அனுப்ப முடியாது: அதற்குப் பதிலாக வீடியோவின் சொந்த முகவரியைத் திறக்கவும்.

## The assistant panel, and its models in Settings

assistant-add-another = மாடலைச் சேர்…
assistant-add-model = உதவியாளருக்கு ஒரு மாடல் தேவை: API விசையுடன் கூடிய மேகக்கணி மாடல், அல்லது உள்ளூர் மாடல்.
assistant-ask = இயங்குவதைப் பற்றிக் கேளுங்கள்
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = அரட்டை { $model } ஒரே முறையில் படிக்க முடியாத அளவுக்கு நீளமாகிக்கொண்டிருந்தது, எனவே அதன் முந்தைய பகுதி சுருக்கப்பட்டது. உதவியாளர் தான் பார்த்தவற்றை மீண்டும் பார்க்கலாம்.
assistant-compacting = முந்தைய அரட்டை சுருக்கப்படுகிறது
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = இந்தக் கணினியில் { $provider }
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $host } இல் { $provider }
assistant-new-chat = புதிய அரட்டை
assistant-no-model = முதலில் அமைப்புகளில் ஒரு மாடலைச் சேர்க்கவும்.
assistant-open-settings = மாடலைச் சேர்
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = { $model } ஒரே நேரத்தில் படிக்கக்கூடிய அளவை இந்த உரையாடல் தாண்டிவிட்டது. புதிய அரட்டையைத் தொடங்கவும், அல்லது அதிகம் படிக்கக்கூடிய மாடலைத் தேர்ந்தெடுக்கவும்.
assistant-problem-credit = கணக்கில் கிரெடிட் இல்லை என்று { $provider } கூறுகிறது. புதிய கணக்கின் விசை செயல்பட, முதலில் { $provider } தளத்தில் சிறிது கிரெடிட் வாங்க வேண்டும்; பிறகு மீண்டும் முயலவும்.
assistant-problem-key = { $provider } API விசையை ஏற்கவில்லை. அமைப்புகளில் அதைச் சரிபார்க்கவும்.
assistant-problem-model = { $model } என்ற பெயரில் { $provider } இல் மாடல் எதுவும் இல்லை. அமைப்புகளில் அதன் பெயரைச் சரிபார்க்கவும்.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } பதிலளிக்கவில்லை: { $message }
assistant-problem-rate = { $provider } வேகத்தைக் குறைக்கச் சொல்கிறது. சிறிது நேரத்தில் மீண்டும் முயலவும்.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } வேகத்தைக் குறைக்கச் சொல்கிறது: { $message }
assistant-problem-refused = { $model } பதிலளிக்க மறுத்துவிட்டது.
assistant-problem-unavailable = { $provider } பரபரப்பாக உள்ளது அல்லது சிக்கலில் உள்ளது. சிறிது நேரத்தில் மீண்டும் முயலவும்.
assistant-problem-unreachable = plaa-ஆல் { $provider } ஐ அடைய முடியவில்லை. உங்கள் இணைப்பைச் சரிபார்க்கவும், அல்லது சேவையகம் இயங்குகிறதா எனப் பார்க்கவும்.
assistant-running = இயங்குகிறது…
assistant-send = அனுப்பு
assistant-shortened = அரட்டை { $model } ஒரே முறையில் படிக்க முடியாத அளவுக்கு நீளமாக இருந்தது, எனவே முந்தைய கருவி முடிவுகள் விடப்பட்டன. உதவியாளர் தான் பார்த்தவற்றை மீண்டும் பார்க்கலாம்.
assistant-stop = நிறுத்து
assistant-stopped = நிறுத்தப்பட்டது.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = இப்போது { $model } உடன் பேசுகிறது.
assistant-thinking = சிந்திக்கிறது…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = சிந்தனைகள்
assistant-title = உதவியாளர்
settings-assistant-add = மாடலைச் சேர்
settings-assistant-add-button = சேர்
# $model is the model's name.
settings-assistant-added = { $model } பதிலளித்தது, சேர்க்கப்பட்டது.
settings-assistant-added-tag = சேர்க்கப்பட்டது
# $example is an address, such as http://localhost:11434.
settings-assistant-address = முகவரி, எ.கா. { $example }
settings-assistant-change-key = விசையை மாற்று
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI உடன் இணக்கமான சேவையகம்
# The menu of how much a local model reads at once.
settings-assistant-context = சூழல்
settings-assistant-context-note = அதிகமாக்கினால், உதவியாளர் ஒரு அரட்டையின் அதிகப் பகுதியை நினைவில் வைத்திருக்கலாம்; ஆனால் மாடல் அதிக நினைவகத்தை எடுக்கும், மெதுவாகப் பதிலளிக்கவும் கூடும்.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K டோக்கன்கள்
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = பக்கத்தைத் திற
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } { $address } இல் இயங்குகிறது.
settings-assistant-found-none = { $provider } இல் இன்னும் மாடல்கள் இல்லை. அதைக் கொண்டு ஒன்றைப் பதிவிறக்கி, மீண்டும் தேடவும்.
settings-assistant-get-key = API விசையைப் பெறு
settings-assistant-get-server = { $provider } ஐப் பெறு
settings-assistant-in-use = பயன்பாட்டில்
settings-assistant-key = API விசை
# $error is what the keychain said.
settings-assistant-key-failed = விசையைக் கீசெயினில் வைக்க முடியவில்லை: { $error }
settings-assistant-key-kept = உங்கள் { $provider } விசை கணினியின் கீசெயினில் வைக்கப்பட்டுள்ளது.
settings-assistant-key-needed = இந்த மாடலுக்கு API விசை தேவை.
settings-assistant-key-refused = { $provider } இந்த விசையை ஏற்கவில்லை. சரியான கணக்கிலிருந்து முழுமையாக நகலெடுக்கப்பட்டதா எனச் சரிபார்க்கவும்.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = { $provider } தளத்தில் ஒரு விசையை உருவாக்கி இங்கே ஒட்டவும்.
settings-assistant-look-again = மீண்டும் தேடு
settings-assistant-looking = மாடல்களைத் தேடுகிறது…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = மாடல், எ.கா. { $example }
settings-assistant-no-server = இந்தக் கணினியில் இயங்கும் { $provider } எதையும் plaa கண்டறியவில்லை. அதைத் தொடங்கவும், அல்லது கீழே அதன் முகவரியைக் கொடுக்கவும்.
settings-assistant-no-tools = கருவிகளைப் பயன்படுத்த முடியாது; உதவியாளருக்கு அவை தேவை
settings-assistant-none = இன்னும் மாடல்கள் இல்லை. கீழே ஒன்றைச் சேர்க்கவும்: Ollama போன்றவற்றின் உள்ளூர் மாடல் இந்தக் கணினியிலேயே இருக்கும்; மேகக்கணி மாடலுக்கு அதன் வழங்குநரிடமிருந்து API விசை தேவை.
settings-assistant-note = உதவியாளர் பலகம் பேசக்கூடிய மாடல்கள். விசைகள் கணினியின் கீசெயினில் வைக்கப்படுகின்றன.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = வேறு முகவரியைப் பயன்படுத்து
settings-assistant-recommended = பரிந்துரைக்கப்பட்டது
settings-assistant-remove = அகற்று
settings-assistant-sees = படங்களைப் பார்க்கும்
settings-assistant-trying = முயல்கிறது…
settings-assistant-use = பயன்படுத்து
settings-assistant-use-key = தொடர்
settings-assistant-uses-tools = கருவிகளைப் பயன்படுத்தும்
