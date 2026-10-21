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
language-name = Dansk

## Common

app-name = plaa
common-cancel = Annuller
common-close = Luk
common-more = Mere

## Command line

usage-help =
    Brug: plaa [FIL ELLER ADRESSE]...

    Afspil video- og lydfiler og streams på adresser som
    https://example.com/live.m3u8. Hver af dem åbnes i et vindue i den
    kørende plaa, som starter, hvis det er nødvendigt.

    Tilvalg:
      -h, --help        Vis denne hjælp
      -V, --version     Vis versionen
          --interfaces  Vis de kablede grænseflader, og om plaa kan sende
                        AVB-streams på dem
          --mcp         Kør MCP på stdin og stdout, så AI-agenter kan se,
                        hvad den kørende plaa afspiller, og styre den

## Start window

start-hint = Åbn en video- eller lydfil, en stream eller en webvideo.
start-open = Åbn
start-open-address = Åbn adresse

## Actions, as buttons, tooltips and menus name them

action-open = Åbn
action-open-address = Åbn adresse
open-file = Åbn fil…
open-address = Åbn adresse…
action-settings = Indstillinger
action-info = Inspektør
action-loop = Loop
action-screenshot = Gem enkeltbillede som billede
action-play = Afspil
action-pause = Pause
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } sekund tilbage
   *[other] { $seconds } sekunder tilbage
}
action-forward = { $seconds ->
    [one] { $seconds } sekund frem
   *[other] { $seconds } sekunder frem
}
action-mute = Slå lyd fra
action-unmute = Slå lyd til
action-speed = Hastighed
action-tracks = Lyd og undertekster
action-full-screen = Fuld skærm
action-playlist = Afspilningsliste
action-chapters = Kapitler
action-picture-in-picture = Billede i billede
action-previous = Forrige
action-next = Næste
action-exit-full-screen = Slå fuld skærm fra
action-quality = Kvalitet
action-visualizer = Visualisering
action-network = Netværkslyd
action-cast = Cast

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Bølgeform
visualizer-prism = Prisme
visualizer-cover = Kun omslag

## The floating controls

overlay-keep-shown = Fastgør betjeningsknapperne
overlay-auto-hide = Lad betjeningsknapperne svæve over billedet

## Audio and subtitle tracks

tracks-audio = Lyd
tracks-subtitles = Undertekster
tracks-off = Fra
# A track the file gives no name or language, by its number.
tracks-numbered = Spor { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Kun lyd

## Inspector

info-title = Inspektør
playlist-title = Afspilningsliste
chapters-numbered = Kapitel { $number }
info-file = Fil
info-name = Navn
info-address = Adresse
info-folder = Mappe
info-size = Størrelse
info-format = Format
info-duration = Varighed
info-bit-rate = Bithastighed
info-video = Video
info-audio = Lyd
info-codec = Codec
info-dimensions = Dimensioner
info-frame-rate = Billedhastighed
info-frames-per-second = { $rate } billeder pr. sekund
info-pixel-format = Pixelformat
info-decoding = Afkodning
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Grafikkort ({ $api })
info-decoding-software = Processor
info-sample-rate = Samplingfrekvens
info-hertz = { $rate } Hz
info-channels = Kanaler
info-language = Sprog
info-no-sound = Ingen lyd
info-tags = Tags

## Dialogs

dialog-open = Åbn
dialog-filter-media = Video og lyd
dialog-filter-all = Alle filer

address-title = Åbn adresse
address-note = En webside med en video, f.eks. siden til en YouTube-video, eller en stream eller fil på netværket, f.eks. https://example.com/live.m3u8.
address-label = Adresse
address-open = Åbn
address-recent = Seneste
address-forget = Fjern fra seneste

## Notices

notice-dialog-failed = Fildialogen kunne ikke åbnes: { $error }
notice-screenshot-saved = “{ $name }” er gemt.
notice-screenshot-failed = Billedet kunne ikke gemmes: { $error }
notice-no-sound = Afspiller uden lyd: { $reason }
notice-loop-from = Loop fra { $time }. Tryk igen, hvor det skal slutte.
notice-loop-between = Afspiller { $from } til { $to } i loop.
notice-loop-off = Loop slået fra.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Det ligner en webside. plaa afspiller videoer på websider med yt-dlp, som ikke er installeret.
error-empty-list = Denne liste har intet at afspille.

## Settings

settings-title = Indstillinger
settings-tab-appearance = Udseende
settings-tab-playback = Afspilning
settings-tab-network = Netværk
settings-tab-language = Sprog
settings-tab-agents = Agenter
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (udviklingsversion, { $build })
settings-choose = Vælg…

settings-appearance-system = Følg systemet
settings-appearance-light = Lys
settings-appearance-dark = Mørk
settings-colors = Farver
settings-system-accent = Brug systemets accentfarve
settings-omarchy-note = Farverne er dannet ud fra accentfarven i “{ $theme }”.
settings-system-accent-note = Farverne er dannet ud fra systemets accentfarve.
settings-system-accent-none = Systemet har ingen accentfarve, så plaa bruger den, der er valgt nedenfor.
settings-accent-chosen-note = Farverne er dannet ud fra den farve, der er valgt nedenfor.
settings-controls = Betjening
settings-overlay = Svævende betjening
settings-overlay-note = Værktøjslinjen og afspilningsknapperne svæver over billedet eller omslaget og glider væk, mens markøren hviler eller er uden for vinduet.
settings-overlay-delay = Skjul efter
settings-overlay-delay-note = Hvor længe markøren hviler over det, der afspilles, før betjeningsknapperne glider væk.
settings-seconds = { $seconds } s
settings-transparency = Gennemsigtighed
settings-transparency-note = Hvor meget af billedet der skinner igennem de svævende betjeningsknapper.
settings-percent = { $percent }%
settings-corner-radius = Hjørneradius
settings-corner-radius-note = Til dialoger og de svævende betjeningsknapper.
settings-pixels = { $pixels } px
settings-animations = Animationer
settings-animations-note = Glidende linjer og paneler, voksende dialoger og fjedrende knapper.
settings-animations-reduced = Slået fra, mens systemet beder om reduceret bevægelse.

settings-hardware = Brug grafikkortet
settings-hardware-note = Afkoder video og koder eksporter på grafikkortet: bruger mindre strøm, afspiller store videoer jævnt og eksporterer hurtigere. Gælder for filer, der åbnes bagefter.
settings-audio-device = Lydudgang
settings-audio-device-system = Systemstandard
settings-resume = Fortsæt, hvor du slap
settings-resume-note = Filer starter, hvor de var, da de sidst blev lukket.
settings-folder-playlists = Afspil mappens andre filer
settings-folder-playlists-note = Når du åbner en fil, kommer de andre filer i mappen på listen og afspilles bagefter.
settings-streams = Streams og webvideoer
settings-stream-quality = Kvalitet
settings-stream-quality-note = Til streams og webvideoer, der findes i flere kvaliteter. Gælder for det, der åbnes bagefter.
settings-quality-auto = Automatisk, lige så høj som skærmen
settings-quality-best = Bedst
settings-web-pages = Afspil webvideoer med yt-dlp
settings-web-pages-found = yt-dlp finder videoen på sider som YouTubes.
settings-web-pages-missing = yt-dlp er ikke installeret. Installer det for at afspille videoerne på sider som YouTubes.
settings-folders = Mapper
settings-recordings = Optagelser
settings-screenshots = Gemte billeder

settings-interface-language = Sprog i brugerfladen
# The first entry of the language list.
settings-language-system = Følg systemet
settings-track-languages = Sprog for spor
settings-track-languages-note = Sprogkoder som en eller fra, de mest ønskede først, adskilt med kommaer. plaa vælger lyd og undertekster på disse sprog, når en fil har dem.
settings-audio-languages = Lyd
settings-subtitle-languages = Undertekster

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-grænseflade
settings-avb-interface-note = Den kablede port, hvor plaa finder AVB- og Milan-sendere og -modtagere. Afsendelse kræver en port med et PTP-hardwareur, som den i en Intel I210.
settings-avb-off = Fra
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (kun visning: intet PTP-ur)
settings-avb-off-mac = { $interface } (kun visning: AVB er slået fra)
settings-avb-interface-note-mac = Den kablede port, hvor plaa finder AVB- og Milan-sendere og -modtagere. Afsendelse går gennem denne Macs egen AVB-entitet, som kræver, at AVB er slået til for porten i netværksenhedsoversigten i Lyd-MIDI-opsætning.
settings-avb-talker-note-mac = plaa sender lyden fra det vindue, der sidst afspillede, gennem denne Macs egen AVB-entitet til de modtagere, der er forbundet til den.
settings-avb-talker = Send det, der afspilles
settings-avb-talker-note = plaa bliver en Milan-sender og sender lyden fra det vindue, der sidst afspillede, til de modtagere, der er forbundet til den.
settings-avb-name = Navn på netværket
settings-avb-name-note = Controllere viser plaa som “{ $name }”. Tryk på Enter for at omdøbe den.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa på { $computer }
avb-could-not-start = { $interface } kunne ikke bruges: { $error }
network-title = Netværkslyd
network-off = Slå “Send det, der afspilles” til i Indstillinger for at afspille på netværket.
network-blocked = Sender ikke: { $reason }
network-waiting = Klar til at sende, venter på, at en modtager forbinder.
network-sending = Sender til de forbundne modtagere.
network-looking = Søger efter modtagere…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Starter ptp4l, som holder den netværkstid, plaas stream sendes efter…
network-ptp-elsewhere = ptp4l kører på en anden grænseflade, så plaas stream har ingen netværkstid her, og modtagere kan muligvis ikke afspille den. Vælg den grænseflade i Indstillinger, eller kør ptp4l på denne.
network-ptp-missing = ptp4l kører ikke, så plaas stream har ingen netværkstid, og modtagere kan muligvis ikke afspille den. Installer linuxptp med plaas pakke, eller triib, som kører det.
network-ptp-failed = ptp4l kører ikke, så plaas stream har ingen netværkstid, og modtagere kan muligvis ikke afspille den. plaa kunne ikke starte det: { $reason }
network-no-listeners = Ingen modtager på netværket kan afspille lyden fra plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Afspil på { $listener }
network-stop-on = Stop afspilning på { $listener }
network-choose-format = Vælg et format til afspilning på { $listener }
network-failed = Kunne ikke forbinde: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Opdater triib for at afspille her.
network-choose = Vælg en modtager for at se, hvordan den tager imod sin stream.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Lyt her
network-no-talkers = Ingen sender på netværket sender i et format, som plaa kan afspille.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Lyt til { $talker }
network-listen-stop = Stop med at lytte til { $talker }
network-listen-frames = Modtagne rammer: { $frames }
network-listen-update-triib = Opdater triib for at lytte her: plaa afspiller kun det, der alligevel når frem til denne computer.
network-listen-lite = Porten kører AVB Lite, som ikke reserverer streams: plaa afspiller kun det, der alligevel når frem til denne computer.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Ikke forbundet
network-from = Fra { $stream }
network-from-receiving = Fra { $stream }, modtager
network-from-waiting = Fra { $stream }, venter på senderen
network-from-failed = Fra { $stream }, senderens reservation mislykkedes: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Medieur
network-locked = Låst til streamen
network-not-locked = Ikke låst
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formater, den kan tage imod fra plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream-id
# The MAC address the stream is sent to.
network-destination = Destination
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latens
network-microseconds = { $time } µs
network-frames = Modtagne rammer
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Rammer uden for rækkefølge
# How often the stream stopped coming while it was taken.
network-interrupted = Afbrydelser
network-entity-id = Entitets-id

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = utilstrækkelig båndbredde
msrp-failure-2 = utilstrækkelige ressourcer i switchen
msrp-failure-3 = utilstrækkelig båndbredde til trafikklassen
msrp-failure-4 = stream-id bruges af en anden sender
msrp-failure-5 = destinationsadressen er allerede i brug
msrp-failure-6 = fortrængt af en stream med højere rang
msrp-failure-7 = den rapporterede latens er ændret
msrp-failure-8 = den udgående port understøtter ikke AVB
msrp-failure-9 = brug en anden destinationsadresse
msrp-failure-10 = ikke flere MSRP-ressourcer
msrp-failure-11 = ikke flere MMRP-ressourcer
msrp-failure-12 = destinationsadressen kan ikke gemmes
msrp-failure-13 = prioriteten er ikke en SR-klasseprioritet
msrp-failure-14 = rammerne er for store til mediet
msrp-failure-15 = grænsen for fan-in-porte er nået
msrp-failure-16 = første værdi ændret for en registreret stream
msrp-failure-17 = VLAN blokeret på den udgående port
msrp-failure-18 = VLAN-tagging slået fra på den udgående port
msrp-failure-19 = SR-klasseprioriteterne passer ikke sammen
msrp-failure-unknown = ukendt årsag
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, ved switchen { $bridge }

## The macOS menu bar

menu-about = Om plaa
menu-settings = Indstillinger…
menu-services = Tjenester
menu-hide = Skjul plaa
menu-hide-others = Skjul andre
menu-show-all = Vis alle
menu-quit = Slut plaa
menu-file = Arkiv
menu-open = Åbn…
menu-open-address = Åbn adresse…
menu-close = Luk vindue
menu-edit = Rediger
menu-cut = Klip
menu-copy = Kopier
menu-paste = Indsæt
menu-select-all = Vælg alt
menu-undo = Fortryd
menu-redo = Gentag
menu-trim = Beskær…
menu-split = Opdel klip
menu-rotate-left = Roter mod venstre
menu-rotate-right = Roter mod højre
menu-flip-horizontal = Spejlvend vandret
menu-flip-vertical = Spejlvend lodret
menu-revert = Gendan original
menu-export = Eksporter…
menu-view = Oversigt
menu-playback = Afspilning
menu-play-pause = Afspil eller sæt på pause
menu-to-start = Gå til start
menu-previous-chapter = Forrige kapitel
menu-next-chapter = Næste kapitel
menu-faster = Hurtigere
menu-slower = Langsommere
menu-normal-speed = Normal hastighed
menu-ab-loop = Loop mellem to punkter
menu-volume-up = Højere
menu-volume-down = Lavere
menu-mute = Slå lyd fra eller til
menu-next-subtitles = Næste undertekster
menu-next-audio = Næste lydspor
menu-window = Vindue
menu-minimize = Minimer
menu-zoom = Zoom
menu-bring-all-to-front = Anbring alle forrest

## Editing

action-edit = Rediger
title-edited = { $name } (redigeret)
edit-trim = Beskær
edit-trim-apply = Beskær
edit-split = Opdel klip
edit-remove-clip = Fjern klip
edit-done = Færdig
edit-rotate-left = Roter mod venstre
edit-rotate-right = Roter mod højre
edit-flip-horizontal = Spejlvend vandret
edit-flip-vertical = Spejlvend lodret
edit-undo = Fortryd
edit-redo = Gentag
edit-revert = Gendan original
edit-export = Eksporter…

## Exporting

export-title = Eksporter
export-format = Format
export-size = Størrelse
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Kun lyd
export-size-original = { $lines }p, som den er
export-on-graphics-card = Kodes på grafikkortet ({ $api })
export-on-processor = Kodes på processoren, hvilket tager længere tid
export-no-encoder = Ingen koder til dette format kan åbnes her
export-checking = Søger efter en koder…
export-sound-format = Lydformat
export-quality = Kvalitet
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Komprimeret; afspilles næsten overalt, også på Apples enheder
export-mp3-note = Komprimeret; afspilles overalt
export-opus-note = Komprimeret; den bedste lyd i forhold til størrelsen, men ikke alle afspillere kan afspille det
export-flac-note = Tabsfri: lyden præcis som den er, i et åbent format
export-alac-note = Tabsfri: lyden præcis som den er, sådan som Apples enheder afspiller den
export-wav-note = Ukomprimeret: lyden præcis som den er, i de største filer
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Gemmes med { $rate } kHz, under filens { $file } kHz
export-choose = Eksporter…
export-name = { $name } eksport
export-name-edited = { $name } redigeret
export-progress = Eksporterer “{ $name }”
export-stop = Stop eksporten
notice-exported = “{ $name }” er eksporteret
notice-export-stopped = Eksporten blev stoppet
notice-export-failed = Kunne ikke eksportere: { $error }
close-title = Luk uden at eksportere?
close-unexported = Ændringerne i “{ $name }” går tabt, medmindre du eksporterer dem.
close-exporting = Eksporten af “{ $name }” stopper, og det, der er skrevet indtil nu, går tabt.
close-anyway = Luk
close-quit-anyway = Afslut

## Recording

record-title = Lydoptagelse
record-file-name = Lydoptagelse
record-start = Optag
record-stop = Stop optagelsen
record-no-input = Ingen mikrofon
record-microphone = Mikrofon
record-default-microphone = Systemets standard
record-playing = Det, computeren afspiller
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanal
   *[other] { $count } kanaler
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, gemmes med { $saved } kHz
action-record-sound = Optag lyd
open-record-sound = Ny lydoptagelse
start-record-sound = Optag lyd
menu-record-sound = Ny lydoptagelse
record-screen-title = Skærmoptagelse
record-screen-file-name = Skærmoptagelse
record-screen-whole = Skærm
record-screen-number = Skærm { $number }
record-screen-main = Skærm { $number } (hovedskærm)
record-screens = Skærme
record-windows = Vinduer
record-screen-what = Hvad der skal optages
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Hvert billede
record-rate-tip = Billeder pr. sekund
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } tabt
   *[other] { $size } · { $count } tabte
}
record-screen-window = Vindue
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Deles ikke længere
record-screen-again = Vælg igen
record-screen-failed = Skærmen kunne ikke optages
record-sound = Lyd
record-no-sound = Ingen lyd
# How much louder the input is taken, for a quiet microphone.
record-gain = Forstærkning
record-gain-value = +{ $decibels } dB
record-level-automatically = Juster niveauet automatisk
record-hide-title = plaa skjules under optagelsen
record-hide-tray = Du stopper optagelsen ved at klikke på den røde prik, som plaa sætter i systembakken, eller ved at åbne plaa igen.
record-hide-reopen = Åbn plaa igen for at stoppe optagelsen.
record-hide-start = Start optagelsen
record-tray-title = plaa optager skærmen
record-tray-tip = Klik for at stoppe optagelsen
action-record-screen = Optag skærmen
open-record-screen = Ny skærmoptagelse
start-record-screen = Optag skærmen
menu-record-screen = Ny skærmoptagelse
record-camera-title = Kameraoptagelse
record-camera-file-name = Kameraoptagelse
record-camera = Kamera
record-camera-failed = Kameraet kunne ikke åbnes
action-record-camera = Optag med kamera
open-record-camera = Ny kameraoptagelse
start-record-camera = Optag med kamera
menu-record-camera = Ny kameraoptagelse
notice-no-camera = Der er intet kamera at optage med
notice-recorded = “{ $name }” er gemt i { $folder }
notice-record-failed = Kunne ikke optage: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Tillad ekstern styring
settings-allow-outside-control-note = AI-agenter som Claude Code kan se, hvad der afspilles i plaa, og styre det via plaa --mcp. plaa spørger før hver ny agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Tilladt: { $agents }
settings-forget-agents = Glem
settings-ask-before-note = Spørg, før en agent:
settings-ask-reading = Ser, hvad der afspilles, eller billedet
settings-ask-playing = Styrer afspilningen, lyden eller fuld skærm
settings-ask-opening = Åbner en fil eller lukker et vindue
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vil du give { $agent } lov til at styre plaa?
agent-prompt-body = { $agent } beder om at bruge plaas eksterne styring til at se, hvad der afspilles, og styre det. Du kan slå ekstern styring fra i Indstillinger.
agent-prompt-allow = Tillad
agent-prompt-deny = Tillad ikke
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Vil du lade { $agent } se, hvad der afspilles her?
agent-ask-play = Vil du lade { $agent } styre afspilningen her?
agent-ask-open = Vil du lade { $agent } åbne eller lukke et vindue?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } beder om at bruge “{ $tool }”. I Indstillinger vælger du, hvad plaa spørger om.

## Casting to a TV or speaker on the network

cast-looking = Søger efter tv'er og højttalere…
cast-none = Der blev ikke fundet noget tv eller nogen højttaler, der afspiller via DLNA.
# $renderer is the TV's or speaker's name.
cast-stop = Stop afspilning på { $renderer }
notice-casting = Afspiller på { $renderer }.
notice-cast-failed = Kunne ikke afspille på { $renderer }: { $reason }
cast-web-page = Websider kan ikke castes endnu: åbn i stedet videoens egen adresse.
