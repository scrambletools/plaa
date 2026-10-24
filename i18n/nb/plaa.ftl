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
language-name = Norsk bokmål

## Common

app-name = plaa
common-cancel = Avbryt
common-close = Lukk
common-more = Mer

## Command line

usage-help =
    Bruk: plaa [FIL ELLER ADRESSE]...

    Spill av video- og lydfiler, og strømmer på adresser som
    https://example.com/live.m3u8. Hver av dem åpnes i et vindu i
    plaa-instansen som kjører, og plaa startes ved behov.

    Valg:
      -h, --help        Vis denne hjelpen
      -V, --version     Vis versjonen
          --interfaces  Vis de kablede grensesnittene, og om plaa kan
                        sende AVB-strømmer på dem
          --mcp         Kjør MCP på stdin og stdout, så KI-agenter kan se hva
                        plaa-instansen som kjører, spiller av, og styre den

## Start window

start-hint = Åpne en video- eller lydfil, en strøm eller en nettvideo.
start-open = Åpne
start-open-address = Åpne adresse
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Koble til

## Actions, as buttons, tooltips and menus name them

action-open = Åpne
action-open-address = Åpne adresse
open-file = Åpne fil…
open-address = Åpne adresse…
action-settings = Innstillinger
action-info = Inspektør
action-loop = Gjenta
action-screenshot = Lagre bilderute som bilde
action-play = Spill av
action-pause = Pause
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } sekund tilbake
   *[other] { $seconds } sekunder tilbake
}
action-forward = { $seconds ->
    [one] { $seconds } sekund fremover
   *[other] { $seconds } sekunder fremover
}
action-mute = Slå av lyden
action-unmute = Slå på lyden
action-speed = Hastighet
action-tracks = Lyd og undertekster
action-full-screen = Fullskjerm
action-playlist = Spilleliste
action-chapters = Kapitler
action-picture-in-picture = Bilde-i-bilde
action-previous = Forrige
action-next = Neste
action-exit-full-screen = Avslutt fullskjerm
action-quality = Kvalitet
action-visualizer = Visualisering
action-network = Nettverkslyd
action-cast = Cast
action-connect = Koble til
action-assistant = Assistent

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Bølgeform
visualizer-prism = Prisme
visualizer-cover = Bare omslag

## The floating controls

overlay-keep-shown = Fest kontrollene
overlay-auto-hide = La kontrollene sveve over bildet

## Audio and subtitle tracks

tracks-audio = Lyd
tracks-subtitles = Undertekster
tracks-off = Av
# A track the file gives no name or language, by its number.
tracks-numbered = Spor { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Bare lyd

## Inspector

info-title = Inspektør
playlist-title = Spilleliste
chapters-numbered = Kapittel { $number }
info-file = Fil
info-name = Navn
info-address = Adresse
info-folder = Mappe
info-size = Størrelse
info-format = Filformat
info-duration = Varighet
info-bit-rate = Bithastighet
info-video = Video
info-audio = Lyd
info-codec = Kodek
info-dimensions = Dimensjoner
info-frame-rate = Bildefrekvens
info-frames-per-second = { $rate } bilder per sekund
info-pixel-format = Pikselformat
info-decoding = Dekoding
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Grafikkort ({ $api })
info-decoding-software = Prosessor
info-sample-rate = Samplingsfrekvens
info-hertz = { $rate } Hz
info-channels = Kanaler
info-language = Språk
info-no-sound = Ingen lyd
info-tags = Tagger
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Sender
info-stream = Strøm
# The talker's listeners other than plaa.
info-other-listeners = Andre mottakere
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR-klasse
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Et nytt format kobler fra { $count ->
        [one] senderens andre mottaker.
       *[other] senderens { $count } andre mottakere.
    }

## Dialogs

dialog-open = Åpne
dialog-filter-media = Video og lyd
dialog-filter-all = Alle filer

address-title = Åpne adresse
address-note = En nettside med en video, for eksempel siden til en YouTube-video, eller en strøm eller fil på nettverket, for eksempel https://example.com/live.m3u8.
address-label = Adresse
address-open = Åpne
address-recent = Nylige
address-forget = Fjern fra nylige

## Notices

notice-dialog-failed = Kunne ikke åpne fildialogen: { $error }
notice-screenshot-saved = Lagret «{ $name }».
notice-screenshot-failed = Kunne ikke lagre bildet: { $error }
notice-no-sound = Spiller av uten lyd: { $reason }
notice-loop-from = Gjentar fra { $time }. Trykk igjen der det skal slutte.
notice-loop-between = Gjentar { $from } til { $to }.
notice-loop-off = Gjentakelse av.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Dette ser ut som en nettside. plaa spiller av videoer på nettsider med yt-dlp, som ikke er installert.
error-empty-list = Denne listen har ingenting å spille av.

## Settings

settings-title = Innstillinger
settings-tab-appearance = Utseende
settings-tab-playback = Avspilling
settings-tab-storage = Lagring
settings-tab-network = Nettverk
settings-tab-language = Språk
settings-tab-agents = Agenter
settings-tab-assistant = Assistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (utviklerversjon, { $build })
settings-choose = Velg…

settings-appearance-system = Følg systemet
settings-appearance-light = Lys
settings-appearance-dark = Mørk
settings-colors = Farger
settings-system-accent = Bruk aksentfargen fra systemet
settings-omarchy-note = Fargene bygger på aksentfargen i «{ $theme }».
settings-system-accent-note = Fargene bygger på systemets aksentfarge.
settings-system-accent-none = Systemet har ingen aksentfarge, så plaa bruker den som er valgt nedenfor.
settings-accent-chosen-note = Fargene bygger på fargen som er valgt nedenfor.
settings-controls = Kontroller
settings-overlay = Svevende kontroller
settings-overlay-note = Verktøylinjen og avspillingskontrollene svever over bildet eller omslaget og glir bort mens pekeren hviler eller er utenfor vinduet.
settings-overlay-delay = Skjul etter
settings-overlay-delay-note = Hvor lenge pekeren hviler over det som spilles, før kontrollene glir bort.
settings-seconds = { $seconds } s
settings-transparency = Gjennomsiktighet
settings-transparency-note = Hvor mye av bildet som synes gjennom de svevende kontrollene.
settings-percent = { $percent } %
settings-corner-radius = Hjørneradius
settings-corner-radius-note = For dialogbokser og de svevende kontrollene.
settings-pixels = { $pixels } px
settings-animations = Animasjoner
settings-animations-note = Glidende linjer og paneler, voksende dialogbokser og fjærende knapper.
settings-animations-reduced = Av så lenge systemet ber om redusert bevegelse.

settings-hardware = Bruk grafikkortet
settings-hardware-note = Dekoder video og koder eksporter på grafikkortet: bruker mindre strøm, spiller av store videoer jevnt og eksporterer raskere. Gjelder filer som åpnes etterpå.
settings-audio-device = Lydutgang
settings-audio-device-system = Systemstandard
settings-resume = Fortsett der du slapp
settings-resume-note = Filer starter der de var da de sist ble lukket.
settings-folder-playlists = Spill av de andre filene i mappen
settings-folder-playlists-note = Når du åpner én fil, listes de andre filene i mappen, og de spilles av etter den.
settings-streams = Strømmer og nettvideoer
settings-stream-quality = Kvalitet
settings-stream-quality-note = For strømmer og nettvideoer som finnes i flere kvaliteter. Gjelder det som åpnes etterpå.
settings-quality-auto = Automatisk, like høy som skjermen
settings-quality-best = Best
settings-web-pages = Spill av nettvideoer med yt-dlp
settings-web-pages-found = yt-dlp finner videoen på nettsteder som YouTube.
settings-web-pages-missing = yt-dlp er ikke installert. Installer det for å spille av videoene på nettsteder som YouTube.
settings-default-app = Standardapp
settings-default-app-label = Åpne filer med plaa
settings-default-app-note = Gjør plaa til appen som åpner video- og lydfiler.
settings-default-app-note-windows = I Windows kan du bare velge standardapper i systemets egne Innstillinger. Dette åpner plaas side der.
settings-default-app-note-macos = macOS ber deg bekrefte hver type: MP4, MOV, MKV, WebM, MP3, M4A, FLAC og WAV.
settings-default-app-status = { $set } av { $total } filtyper åpnes med plaa.
settings-default-app-button = Gjør til standard
settings-default-app-button-windows = Åpne Innstillinger
settings-default-app-no-entry = plaas skrivebordsfil er ikke installert, så systemet kan ikke åpne filer med plaa. Installer plaa fra en pakke.
settings-default-app-no-bundle = Åpne plaa fra plaa.app for å gjøre den til standard.
settings-default-app-failed = Kunne ikke gjøre plaa til standard: { $error }
settings-folders = Mapper
settings-sound-recordings = Lydopptak
settings-video-recordings = Videoopptak
settings-screenshots = Lagrede bilderuter

settings-interface-language = Språk i grensesnittet
# The first entry of the language list.
settings-language-system = Følg systemet
settings-track-languages = Språk for spor
settings-track-languages-note = Språkkoder som en eller fra, de mest ønskede først, atskilt med komma. plaa velger lyd og undertekster på disse språkene når en fil har dem.
settings-audio-languages = Lyd
settings-subtitle-languages = Undertekster

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-grensesnitt
settings-avb-interface-note = Den kablede porten der plaa finner AVB- og Milan-sendere og -mottakere. For å sende trengs en port med en PTP-maskinvareklokke, for eksempel den i en Intel I210.
settings-avb-off = Av
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (bare visning: ingen PTP-klokke)
settings-avb-off-mac = { $interface } (bare visning: AVB er av)
settings-avb-interface-note-mac = Den kablede porten der plaa finner AVB- og Milan-sendere og -mottakere. Sendingen går gjennom denne Macens egen AVB-entitet, som krever at AVB er slått på for porten i nettverksenhetsoversikten i Lyd/MIDI-oppsett.
settings-avb-talker-note-mac = plaa sender lyden fra vinduet som spilte sist, gjennom denne Macens egen AVB-entitet til mottakerne som er koblet til den.
settings-avb-talker = Send det som spilles
settings-avb-talker-note = plaa blir en Milan-sender og sender lyden fra vinduet som spilte sist, til mottakerne som er koblet til den.
settings-avb-name = Navn på nettverket
settings-avb-name-note = Kontrollprogrammer viser plaa som «{ $name }». Trykk på Enter for å gi det nytt navn.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa på { $computer }
avb-could-not-start = Kunne ikke bruke { $interface }: { $error }
network-title = Nettverkslyd
network-off = Slå på «Send det som spilles» i Innstillinger for å spille av på nettverket.
network-blocked = Sender ikke: { $reason }
network-waiting = Klar til å sende, venter på at en mottaker kobler til.
network-sending = Sender til mottakerne som er koblet til.
network-looking = Ser etter mottakere…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Starter ptp4l, som holder nettverkstiden plaas strøm sendes etter…
network-ptp-elsewhere = ptp4l kjører på et annet grensesnitt, så plaas strøm har ingen nettverkstid her, og mottakere kan kanskje ikke spille den av. Velg det grensesnittet i Innstillinger, eller kjør ptp4l på dette.
network-ptp-missing = ptp4l kjører ikke, så plaas strøm har ingen nettverkstid, og mottakere kan kanskje ikke spille den av. Installer linuxptp med plaas pakke, eller triib, som kjører det.
network-ptp-failed = ptp4l kjører ikke, så plaas strøm har ingen nettverkstid, og mottakere kan kanskje ikke spille den av. plaa kunne ikke starte det: { $reason }
network-no-listeners = Ingen mottaker på nettverket kan spille av lyden fra plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Spill av på { $listener }
network-stop-on = Slutt å spille av på { $listener }
network-choose-format = Velg et format for avspilling på { $listener }
network-failed = Kunne ikke koble til: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Oppdater triib for å spille av her.
network-choose = Velg en mottaker for å se hvordan den tar imot strømmen sin.
network-no-sources = Ingen sender på nettverket sender i et format plaa spiller av, eller kan bytte til et.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Slutt å lytte til { $talker }
network-listen-update-triib = Oppdater triib for å lytte her: plaa spiller bare av det som uansett når denne datamaskinen.
network-listen-lite = Porten kjører AVB Lite, som ikke reserverer strømmer: plaa spiller bare av det som uansett når denne datamaskinen.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Ser etter sendere…
# plaa listens through its entity, which exists only while it sends.
connect-off = Slå på «Send det som spilles» i Innstillinger for å lytte til sendere her.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa kan ikke lytte her: { $reason }
connect-sending = Sender i et format plaa spiller av
connect-changing = Bytter til et format plaa spiller av
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, i { $format }
connect-change-cuts-off = { $talker }, i { $format }, noe som kobler fra { $count ->
        [one] dens andre mottaker
       *[other] dens { $count } andre mottakere
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Ikke tilkoblet
network-from = Fra { $stream }
network-from-receiving = Fra { $stream }, mottar
network-from-waiting = Fra { $stream }, venter på senderen
network-from-failed = Fra { $stream }, senderens reservasjon mislyktes: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Medieklokke
network-locked = Låst til strømmen
network-not-locked = Ikke låst
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formater den kan ta imot fra plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Strøm-ID
# The MAC address the stream is sent to.
network-destination = Mål
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latens
network-microseconds = { $time } µs
network-frames = Mottatte rammer
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Rammer i feil rekkefølge
# How often the stream stopped coming while it was taken.
network-interrupted = Avbrudd
# Frames that came after, or long before, their time to be played.
network-late = Forsinkede rammer
network-early = For tidlige rammer
network-entity-id = Entitets-ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = utilstrekkelig båndbredde
msrp-failure-2 = utilstrekkelige ressurser i svitsjen
msrp-failure-3 = utilstrekkelig båndbredde for trafikklassen
msrp-failure-4 = strøm-ID-en brukes av en annen sender
msrp-failure-5 = måladressen er allerede i bruk
msrp-failure-6 = fortrengt av en strøm med høyere rang
msrp-failure-7 = den rapporterte latensen er endret
msrp-failure-8 = den utgående porten støtter ikke AVB
msrp-failure-9 = bruk en annen måladresse
msrp-failure-10 = tom for MSRP-ressurser
msrp-failure-11 = tom for MMRP-ressurser
msrp-failure-12 = kan ikke lagre måladressen
msrp-failure-13 = prioriteten er ikke en SR-klasseprioritet
msrp-failure-14 = rammene er for store for mediet
msrp-failure-15 = grensen for fan-in-porter er nådd
msrp-failure-16 = første verdi er endret for en registrert strøm
msrp-failure-17 = VLAN blokkert på den utgående porten
msrp-failure-18 = VLAN-tagging deaktivert på den utgående porten
msrp-failure-19 = SR-klasseprioritetene stemmer ikke overens
msrp-failure-unknown = ukjent årsak
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, ved svitsjen { $bridge }

## The macOS menu bar

menu-about = Om plaa
menu-settings = Innstillinger…
menu-services = Tjenester
menu-hide = Skjul plaa
menu-hide-others = Skjul andre
menu-show-all = Vis alle
menu-quit = Avslutt plaa
menu-file = Arkiv
menu-open = Åpne…
menu-open-address = Åpne adresse…
menu-close = Lukk vindu
menu-edit = Rediger
menu-cut = Klipp ut
menu-copy = Kopier
menu-paste = Lim inn
menu-select-all = Marker alt
menu-undo = Angre
menu-redo = Gjør om
menu-trim = Klipp til…
menu-split = Del klipp
menu-rotate-left = Roter mot venstre
menu-rotate-right = Roter mot høyre
menu-flip-horizontal = Speilvend vannrett
menu-flip-vertical = Speilvend loddrett
menu-revert = Tilbakestill til originalen
menu-export = Eksporter…
menu-view = Vis
menu-playback = Avspilling
menu-play-pause = Spill av eller pause
menu-to-start = Gå til starten
menu-previous-chapter = Forrige kapittel
menu-next-chapter = Neste kapittel
menu-faster = Raskere
menu-slower = Saktere
menu-normal-speed = Normal hastighet
menu-ab-loop = Gjenta mellom to punkter
menu-volume-up = Høyere
menu-volume-down = Lavere
menu-mute = Slå lyden av eller på
menu-next-subtitles = Neste undertekster
menu-next-audio = Neste lydspor
menu-window = Vindu
menu-minimize = Minimer
menu-zoom = Zoom
menu-bring-all-to-front = Legg alle fremst

## Editing

action-edit = Rediger
title-edited = { $name } (redigert)
edit-trim = Klipp til
edit-trim-apply = Klipp til
edit-split = Del klipp
edit-remove-clip = Fjern klipp
edit-done = Ferdig
edit-rotate-left = Roter mot venstre
edit-rotate-right = Roter mot høyre
edit-flip-horizontal = Speilvend vannrett
edit-flip-vertical = Speilvend loddrett
edit-undo = Angre
edit-redo = Gjør om
edit-revert = Tilbakestill til originalen
edit-export = Eksporter…

## Exporting

export-title = Eksporter
export-format = Filformat
export-size = Størrelse
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Bare lyd
export-size-original = { $lines }p, som den er
export-on-graphics-card = Kodes på grafikkortet ({ $api })
export-on-processor = Kodes på prosessoren, noe som tar lengre tid
export-no-encoder = Ingen koder for dette formatet kan åpnes her
export-checking = Ser etter en koder…
export-sound-format = Lydformat
export-quality = Kvalitet
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Komprimert; spilles av nesten overalt, også på Apples enheter
export-mp3-note = Komprimert; spilles av overalt
export-opus-note = Komprimert; den beste lyden for størrelsen, men ikke alle avspillere støtter det
export-flac-note = Tapsfritt: lyden nøyaktig som den er, i et åpent format
export-alac-note = Tapsfritt: lyden nøyaktig som den er, slik Apples enheter spiller den av
export-wav-note = Ukomprimert: lyden nøyaktig som den er, i de største filene
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Lagres med { $rate } kHz, lavere enn filens { $file } kHz
export-choose = Eksporter…
export-name = { $name } eksport
export-name-edited = { $name } redigert
export-progress = Eksporterer «{ $name }»
export-stop = Stopp eksporten
notice-exported = Eksporterte «{ $name }»
notice-export-stopped = Eksporten ble stoppet
notice-export-failed = Kunne ikke eksportere: { $error }
close-title = Vil du lukke uten å eksportere?
close-unexported = Endringene i «{ $name }» går tapt hvis du ikke eksporterer dem.
close-exporting = Eksporten av «{ $name }» stopper, og det som er skrevet så langt, går tapt.
close-anyway = Lukk
close-quit-anyway = Avslutt

## Recording

record-title = Lydopptak
record-file-name = Lydopptak
record-start = Ta opp
record-stop = Stopp opptaket
record-no-input = Ingen mikrofon
record-microphone = Mikrofon
record-default-microphone = Systemets standard
record-playing = Det datamaskinen spiller av
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanal
   *[other] { $count } kanaler
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, lagres med { $saved } kHz
action-record-sound = Ta opp lyd
open-record-sound = Nytt lydopptak
start-record-sound = Ta opp lyd
menu-record-sound = Nytt lydopptak
record-screen-title = Skjermopptak
record-screen-file-name = Skjermopptak
record-screen-whole = Skjerm
record-screen-number = Skjerm { $number }
record-screen-main = Skjerm { $number } (hovedskjerm)
record-screens = Skjermer
record-windows = Vinduer
record-screen-what = Hva som skal tas opp
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Hver bilderute
record-rate-tip = Bilder per sekund
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } hoppet over
record-screen-window = Vindu
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Deles ikke lenger
record-screen-again = Velg på nytt
record-screen-failed = Kunne ikke ta opp skjermen
record-sound = Lyd
record-no-sound = Ingen lyd
# How much louder the input is taken, for a quiet microphone.
record-gain = Forsterkning
record-gain-value = +{ $decibels } dB
record-level-automatically = Juster nivået automatisk
record-hide-title = plaa skjules mens det tar opp
record-hide-tray = Du stopper opptaket ved å klikke på den røde prikken plaa legger i systemstatusfeltet, eller ved å åpne plaa igjen.
record-hide-reopen = Åpne plaa igjen for å stoppe opptaket.
record-hide-start = Start opptaket
record-tray-title = plaa tar opp skjermen
record-tray-tip = Klikk for å stoppe opptaket
action-record-screen = Ta opp skjermen
open-record-screen = Nytt skjermopptak
start-record-screen = Ta opp skjermen
menu-record-screen = Nytt skjermopptak
record-camera-title = Kameraopptak
record-camera-file-name = Kameraopptak
record-camera = Kamera
record-camera-failed = Kunne ikke åpne kameraet
record-name-title = Gi opptaket et navn
record-name-note = Det er lagret som «{ $name }» i { $folder }.
record-name-label = Navn
record-name-keep = Behold
record-name-rename = Gi nytt navn
record-name-taken = Det finnes allerede en fil med navnet «{ $name }» der.
record-name-invalid = Et navn kan ikke inneholde noen av disse: / \ : * ? " < > |
record-name-failed = Kunne ikke gi nytt navn: { $error }
action-record-camera = Ta opp med kamera
open-record-camera = Nytt kameraopptak
start-record-camera = Ta opp med kamera
menu-record-camera = Nytt kameraopptak
notice-no-camera = Det finnes ikke noe kamera å ta opp med
notice-recorded = Lagret «{ $name }» i { $folder }
notice-record-failed = Kunne ikke ta opp: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Tillat ekstern styring
settings-allow-outside-control-note = KI-agenter som Claude Code kan se hva som spilles i plaa, og styre det via plaa --mcp. plaa spør før hver nye agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Tillatt: { $agents }
settings-forget-agents = Glem
settings-ask-before-note = Spør før en agent:
settings-ask-reading = Ser hva som spilles, eller bildet
settings-ask-playing = Styrer avspillingen, lyden eller fullskjerm
settings-ask-opening = Åpner en fil eller lukker et vindu
settings-ask-editing = Redigerer en fil eller eksporterer den
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vil du la { $agent } styre plaa?
agent-prompt-body = { $agent } ber om å bruke plaas eksterne styring til å se hva som spilles, og styre det. Du kan slå av ekstern styring i Innstillinger.
agent-prompt-allow = Tillat
agent-prompt-deny = Ikke tillat
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Vil du la { $agent } se hva som spilles her?
agent-ask-play = Vil du la { $agent } styre avspillingen her?
agent-ask-open = Vil du la { $agent } åpne eller lukke et vindu?
agent-ask-edit = Vil du la { $agent } redigere eller eksportere her?
agent-working = { $agent } jobber…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } ber om å bruke «{ $tool }». I Innstillinger velger du hva plaa spør om.

## Casting to a TV or speaker on the network

cast-looking = Ser etter TV-er og høyttalere…
cast-none = Fant ingen TV, høyttaler eller Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Slutt å spille av på { $renderer }
notice-casting = Spiller av på { $renderer }.
notice-cast-failed = Kunne ikke spille av på { $renderer }: { $reason }
cast-web-page = Nettsider kan ikke castes ennå: åpne videoens egen adresse i stedet.

## The assistant panel, and its models in Settings

assistant-add-another = Legg til en modell…
assistant-add-model = Assistenten trenger en modell: en skymodell med API-nøkkelen sin, eller en lokal modell.
assistant-ask = Spør om det som spilles
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Chatten begynte å bli for lang til at { $model } kan lese den på én gang, så den tidligere delen ble oppsummert. Assistenten ser kanskje på nytt på det den så.
assistant-compacting = Oppsummerer den tidligere delen av chatten
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } på denne datamaskinen
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } på { $host }
assistant-new-chat = Ny chat
assistant-no-model = Legg til en modell i Innstillinger først.
assistant-open-settings = Legg til en modell
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Samtalen får ikke lenger plass i det { $model } kan lese på én gang. Start en ny chat, eller velg en modell som kan lese mer.
assistant-problem-credit = { $provider } sier at kontoen ikke har noen saldo. For en ny konto må saldoen først fylles på via nettstedet til { $provider } før nøkkelen virker; prøv så igjen.
assistant-problem-key = { $provider } avviste API-nøkkelen. Sjekk den i Innstillinger.
assistant-problem-model = { $provider } har ingen modell som heter { $model }. Sjekk navnet i Innstillinger.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } svarte ikke: { $message }
assistant-problem-rate = { $provider } ber deg om å ta det roligere. Prøv igjen om litt.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } ber deg om å ta det roligere: { $message }
assistant-problem-refused = { $model } avslo å svare.
assistant-problem-unavailable = { $provider } er opptatt eller har problemer. Prøv igjen om litt.
assistant-problem-unreachable = plaa fikk ikke kontakt med { $provider }. Sjekk tilkoblingen, eller at serveren kjører.
assistant-running = Kjører…
assistant-send = Send
assistant-shortened = Chatten var for lang til at { $model } kan lese den på én gang, så tidligere verktøyresultater ble utelatt. Assistenten ser kanskje på nytt på det den så.
assistant-stop = Stopp
assistant-stopped = Stoppet.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Snakker nå med { $model }.
assistant-thinking = Tenker…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Tanker
assistant-title = Assistent
settings-assistant-add = Legg til en modell
settings-assistant-add-button = Legg til
# $model is the model's name.
settings-assistant-added = { $model } svarte og er lagt til.
settings-assistant-added-tag = Lagt til
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adresse, for eksempel { $example }
settings-assistant-change-key = Endre nøkkel
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-kompatibel server
# The menu of how much a local model reads at once.
settings-assistant-context = Kontekst
settings-assistant-context-note = Mer lar assistenten huske mer av en chat, men modellen bruker mer minne og kan svare tregere.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Åpne siden
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } kjører på { $address }.
settings-assistant-found-none = { $provider } har ingen modeller ennå. Last ned en med den, og søk så igjen.
settings-assistant-get-key = Skaff en API-nøkkel
settings-assistant-get-server = Last ned { $provider }
settings-assistant-in-use = I bruk
settings-assistant-key = API-nøkkel
# $error is what the keychain said.
settings-assistant-key-failed = Nøkkelen kunne ikke lagres i nøkkelringen: { $error }
settings-assistant-key-kept = Nøkkelen din for { $provider } oppbevares i systemets nøkkelring.
settings-assistant-key-needed = Denne modellen trenger en API-nøkkel.
settings-assistant-key-refused = { $provider } avviste denne nøkkelen. Sjekk at den ble kopiert i sin helhet, fra riktig konto.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Lag en nøkkel på nettstedet til { $provider }, og lim den inn her.
settings-assistant-look-again = Søk igjen
settings-assistant-looking = Ser etter modeller…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modell, for eksempel { $example }
settings-assistant-no-server = plaa fant ingen { $provider } som kjører på denne datamaskinen. Start den, eller oppgi adressen nedenfor.
settings-assistant-no-tools = Kan ikke bruke verktøy, som assistenten trenger
settings-assistant-none = Ingen modeller ennå. Legg til en nedenfor: en lokal modell, som den fra Ollama, blir på denne datamaskinen; en skymodell trenger en API-nøkkel fra leverandøren.
settings-assistant-note = Modellene assistenten kan snakke med. Nøkler oppbevares i systemets nøkkelring.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Bruk en annen adresse
settings-assistant-recommended = Anbefalt
settings-assistant-remove = Fjern
settings-assistant-sees = Ser bilder
settings-assistant-trying = Prøver…
settings-assistant-use = Bruk
settings-assistant-use-key = Fortsett
settings-assistant-uses-tools = Bruker verktøy
