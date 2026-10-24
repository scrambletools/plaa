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
language-name = Svenska

## Common

app-name = plaa
common-cancel = Avbryt
common-close = Stäng
common-more = Mer

## Command line

usage-help =
    Användning: plaa [FIL ELLER ADRESS]...

    Spela upp video- och ljudfiler, och strömmar på adresser som
    https://example.com/live.m3u8. Var och en öppnas i ett fönster i den
    plaa-instans som körs, och plaa startas vid behov.

    Alternativ:
      -h, --help        Visa den här hjälpen
      -V, --version     Visa versionen
          --interfaces  Visa de trådbundna gränssnitten och om plaa kan
                        skicka AVB-strömmar på dem
          --mcp         Kör MCP på stdin och stdout, så att AI-agenter kan
                        se vad plaa-instansen som körs spelar upp och styra den

## Start window

start-hint = Öppna en video- eller ljudfil, en ström eller en webbvideo.
start-open = Öppna
start-open-address = Öppna adress
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Anslut

## Actions, as buttons, tooltips and menus name them

action-open = Öppna
action-open-address = Öppna adress
open-file = Öppna fil…
open-address = Öppna adress…
action-settings = Inställningar
action-info = Inspektör
action-loop = Upprepa
action-screenshot = Spara bildruta som bild
action-play = Spela upp
action-pause = Pausa
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Bakåt { $seconds } sekund
   *[other] Bakåt { $seconds } sekunder
}
action-forward = { $seconds ->
    [one] Framåt { $seconds } sekund
   *[other] Framåt { $seconds } sekunder
}
action-mute = Ljud av
action-unmute = Ljud på
action-speed = Hastighet
action-tracks = Ljud och undertexter
action-full-screen = Helskärm
action-playlist = Spellista
action-chapters = Kapitel
action-picture-in-picture = Bild-i-bild
action-previous = Föregående
action-next = Nästa
action-exit-full-screen = Avsluta helskärm
action-quality = Kvalitet
action-visualizer = Visualisering
action-network = Nätverksljud
action-cast = Casta
action-connect = Anslut
action-assistant = Assistent

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Vågform
visualizer-prism = Prisma
visualizer-cover = Endast omslag

## The floating controls

overlay-keep-shown = Fäst kontrollerna
overlay-auto-hide = Låt kontrollerna sväva över bilden

## Audio and subtitle tracks

tracks-audio = Ljud
tracks-subtitles = Undertexter
tracks-off = Av
# A track the file gives no name or language, by its number.
tracks-numbered = Spår { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Endast ljud

## Inspector

info-title = Inspektör
playlist-title = Spellista
chapters-numbered = Kapitel { $number }
info-file = Fil
info-name = Namn
info-address = Adress
info-folder = Mapp
info-size = Storlek
info-format = Filformat
info-duration = Längd
info-bit-rate = Bithastighet
info-video = Video
info-audio = Ljud
info-codec = Kodek
info-dimensions = Mått
info-frame-rate = Bildfrekvens
info-frames-per-second = { $rate } bildrutor per sekund
info-pixel-format = Pixelformat
info-decoding = Avkodning
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Grafikkort ({ $api })
info-decoding-software = Processor
info-sample-rate = Samplingsfrekvens
info-hertz = { $rate } Hz
info-channels = Kanaler
info-language = Språk
info-no-sound = Inget ljud
info-tags = Taggar
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Sändare
info-stream = Ström
# The talker's listeners other than plaa.
info-other-listeners = Andra mottagare
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR-klass
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Ett nytt format kopplar bort { $count ->
        [one] sändarens andra mottagare.
       *[other] sändarens { $count } andra mottagare.
    }

## Dialogs

dialog-open = Öppna
dialog-filter-media = Video och ljud
dialog-filter-all = Alla filer

address-title = Öppna adress
address-note = En webbsida med en video, till exempel sidan för en YouTube-video, eller en ström eller fil på nätverket, till exempel https://example.com/live.m3u8.
address-label = Adress
address-open = Öppna
address-recent = Senaste
address-forget = Ta bort från senaste

## Notices

notice-dialog-failed = Det gick inte att öppna fildialogen: { $error }
notice-screenshot-saved = ”{ $name }” har sparats.
notice-screenshot-failed = Det gick inte att spara bilden: { $error }
notice-no-sound = Spelar upp utan ljud: { $reason }
notice-loop-from = Upprepning från { $time }. Tryck igen där den ska sluta.
notice-loop-between = Upprepar { $from } till { $to }.
notice-loop-off = Upprepning av.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Det här ser ut som en webbsida. plaa spelar upp videor på webbsidor med yt-dlp, som inte är installerat.
error-empty-list = Den här listan har inget att spela upp.

## Settings

settings-title = Inställningar
settings-tab-appearance = Utseende
settings-tab-playback = Uppspelning
settings-tab-storage = Lagring
settings-tab-network = Nätverk
settings-tab-language = Språk
settings-tab-agents = Agenter
settings-tab-assistant = Assistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (utvecklingsversion, { $build })
settings-choose = Välj…

settings-appearance-system = Följ systemet
settings-appearance-light = Ljust
settings-appearance-dark = Mörkt
settings-colors = Färger
settings-system-accent = Använd systemets accentfärg
settings-omarchy-note = Färgerna bygger på accentfärgen i ”{ $theme }”.
settings-system-accent-note = Färgerna bygger på systemets accentfärg.
settings-system-accent-none = Systemet har ingen accentfärg, så plaa använder den som är vald nedan.
settings-accent-chosen-note = Färgerna bygger på färgen som är vald nedan.
settings-controls = Kontroller
settings-overlay = Svävande kontroller
settings-overlay-note = Verktygsfältet och uppspelningskontrollerna svävar över bilden eller omslaget och glider undan medan pekaren vilar eller är utanför fönstret.
settings-overlay-delay = Göm efter
settings-overlay-delay-note = Hur länge pekaren vilar över det som spelas innan kontrollerna glider undan.
settings-seconds = { $seconds } s
settings-transparency = Genomskinlighet
settings-transparency-note = Hur mycket av bilden som syns genom de svävande kontrollerna.
settings-percent = { $percent } %
settings-corner-radius = Hörnradie
settings-corner-radius-note = För dialogrutor och de svävande kontrollerna.
settings-pixels = { $pixels } px
settings-animations = Animeringar
settings-animations-note = Glidande fält och paneler, växande dialogrutor och fjädrande knappar.
settings-animations-reduced = Av så länge systemet begär minskad rörelse.

settings-hardware = Använd grafikkortet
settings-hardware-note = Avkodar video och kodar exporter på grafikkortet: drar mindre ström, spelar upp stora videor jämnt och exporterar snabbare. Gäller filer som öppnas efteråt.
settings-audio-device = Ljudutgång
settings-audio-device-system = Systemstandard
settings-resume = Fortsätt där du slutade
settings-resume-note = Filer börjar där de var när de senast stängdes.
settings-folder-playlists = Spela upp mappens övriga filer
settings-folder-playlists-note = När du öppnar en fil listas de andra filerna i samma mapp, och de spelas upp efter den.
settings-streams = Strömmar och webbvideor
settings-stream-quality = Kvalitet
settings-stream-quality-note = För strömmar och webbvideor som finns i flera kvaliteter. Gäller det som öppnas efteråt.
settings-quality-auto = Automatisk, lika hög som skärmen
settings-quality-best = Bäst
settings-web-pages = Spela upp webbvideor med yt-dlp
settings-web-pages-found = yt-dlp hittar videon på sidor som YouTubes.
settings-web-pages-missing = yt-dlp är inte installerat. Installera det för att spela upp videorna på sidor som YouTubes.
settings-default-app = Standardapp
settings-default-app-label = Öppna filer med plaa
settings-default-app-note = Gör plaa till appen som öppnar video- och ljudfiler.
settings-default-app-note-windows = I Windows kan du bara välja standardappar i systemets egna Inställningar. Det här öppnar plaas sida där.
settings-default-app-note-macos = macOS ber dig bekräfta varje typ: MP4, MOV, MKV, WebM, MP3, M4A, FLAC och WAV.
settings-default-app-status = { $set } av { $total } filtyper öppnas med plaa.
settings-default-app-button = Gör till standard
settings-default-app-button-windows = Öppna Inställningar
settings-default-app-no-entry = plaas skrivbordsfil är inte installerad, så systemet kan inte öppna filer med plaa. Installera plaa från ett paket.
settings-default-app-no-bundle = Öppna plaa från plaa.app för att göra den till standard.
settings-default-app-failed = Det gick inte att göra plaa till standard: { $error }
settings-folders = Mappar
settings-sound-recordings = Ljudinspelningar
settings-video-recordings = Videoinspelningar
settings-screenshots = Sparade bildrutor

settings-interface-language = Gränssnittets språk
# The first entry of the language list.
settings-language-system = Följ systemet
settings-track-languages = Spårens språk
settings-track-languages-note = Språkkoder som en eller fra, de mest önskade först, åtskilda med kommatecken. plaa väljer ljud och undertexter på de här språken när en fil har dem.
settings-audio-languages = Ljud
settings-subtitle-languages = Undertexter

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-gränssnitt
settings-avb-interface-note = Den trådbundna port där plaa hittar AVB- och Milan-sändare och -mottagare. För att skicka krävs en port med en PTP-hårdvaruklocka, till exempel den i en Intel I210.
settings-avb-off = Av
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (bara visning: ingen PTP-klocka)
settings-avb-off-mac = { $interface } (bara visning: AVB är av)
settings-avb-interface-note-mac = Den trådbundna port där plaa hittar AVB- och Milan-sändare och -mottagare. Ljudet skickas via den här Macens egen AVB-entitet, som kräver att AVB är påslaget för porten i nätverksenhetsläsaren i Ljud-MIDI-inställning.
settings-avb-talker-note-mac = plaa skickar ljudet från fönstret som spelade senast via den här Macens egen AVB-entitet till mottagarna som är anslutna till den.
settings-avb-talker = Skicka det som spelas
settings-avb-talker-note = plaa blir en Milan-sändare och skickar ljudet från fönstret som spelade senast till mottagarna som är anslutna till den.
settings-avb-name = Namn på nätverket
settings-avb-name-note = Styrprogram visar plaa som ”{ $name }”. Tryck på Enter för att byta namn.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa på { $computer }
avb-could-not-start = Det gick inte att använda { $interface }: { $error }
network-title = Nätverksljud
network-off = Slå på ”Skicka det som spelas” i Inställningar för att spela upp på nätverket.
network-blocked = Skickar inte: { $reason }
network-waiting = Redo att skicka, väntar på att en mottagare ansluter.
network-sending = Skickar till de anslutna mottagarna.
network-looking = Söker efter mottagare…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Startar ptp4l, som håller nätverkstiden som plaas ström skickas efter…
network-ptp-elsewhere = ptp4l körs på ett annat gränssnitt, så plaas ström har ingen nätverkstid här och mottagare kanske inte kan spela upp den. Välj det gränssnittet i Inställningar, eller kör ptp4l på det här.
network-ptp-missing = ptp4l körs inte, så plaas ström har ingen nätverkstid och mottagare kanske inte kan spela upp den. Installera linuxptp med plaas paket, eller triib, som kör det.
network-ptp-failed = ptp4l körs inte, så plaas ström har ingen nätverkstid och mottagare kanske inte kan spela upp den. plaa kunde inte starta det: { $reason }
network-no-listeners = Ingen mottagare på nätverket kan spela upp ljudet från plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Spela upp på { $listener }
network-stop-on = Sluta spela upp på { $listener }
network-choose-format = Välj vilket format som ska spelas upp på { $listener }
network-failed = Det gick inte att ansluta: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Uppdatera triib för att spela upp här.
network-choose = Välj en mottagare för att se hur den tar emot sin ström.
network-no-sources = Ingen sändare på nätverket skickar i ett format som plaa spelar upp, eller kan byta till ett.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Sluta lyssna på { $talker }
network-listen-update-triib = Uppdatera triib för att lyssna här: plaa spelar bara upp det som ändå når den här datorn.
network-listen-lite = Porten kör AVB Lite, som inte reserverar några strömmar: plaa spelar bara upp det som ändå når den här datorn.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Söker efter sändare…
# plaa listens through its entity, which exists only while it sends.
connect-off = Slå på ”Skicka det som spelas” i Inställningar för att lyssna på sändare här.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa kan inte lyssna här: { $reason }
connect-sending = Skickar i ett format som plaa spelar upp
connect-changing = Byter till ett format som plaa spelar upp
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, i { $format }
connect-change-cuts-off = { $talker }, i { $format }, vilket kopplar bort { $count ->
        [one] dess andra mottagare
       *[other] dess { $count } andra mottagare
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Inte ansluten
network-from = Från { $stream }
network-from-receiving = Från { $stream }, tar emot
network-from-waiting = Från { $stream }, väntar på sändaren
network-from-failed = Från { $stream }, sändarens reservation misslyckades: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Mediaklocka
network-locked = Låst till strömmen
network-not-locked = Inte låst
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Format den kan ta emot från plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Ström-ID
# The MAC address the stream is sent to.
network-destination = Mål
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latens
network-microseconds = { $time } µs
network-frames = Mottagna ramar
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Ramar i fel ordning
# How often the stream stopped coming while it was taken.
network-interrupted = Avbrott
# Frames that came after, or long before, their time to be played.
network-late = Försenade ramar
network-early = För tidiga ramar
network-entity-id = Entitets-ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = otillräcklig bandbredd
msrp-failure-2 = otillräckliga resurser i switchen
msrp-failure-3 = otillräcklig bandbredd för trafikklassen
msrp-failure-4 = ström-ID används av en annan sändare
msrp-failure-5 = måladressen används redan
msrp-failure-6 = undanträngd av en ström med högre rang
msrp-failure-7 = den rapporterade latensen har ändrats
msrp-failure-8 = den utgående porten saknar stöd för AVB
msrp-failure-9 = använd en annan måladress
msrp-failure-10 = slut på MSRP-resurser
msrp-failure-11 = slut på MMRP-resurser
msrp-failure-12 = kan inte lagra måladressen
msrp-failure-13 = prioriteten är inte en SR-klassprioritet
msrp-failure-14 = ramarna är för stora för mediet
msrp-failure-15 = gränsen för fan-in-portar har nåtts
msrp-failure-16 = första värdet har ändrats för en registrerad ström
msrp-failure-17 = VLAN blockerat på den utgående porten
msrp-failure-18 = VLAN-taggning avaktiverad på den utgående porten
msrp-failure-19 = SR-klassprioriteterna stämmer inte överens
msrp-failure-unknown = okänd orsak
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, i switchen { $bridge }

## The macOS menu bar

menu-about = Om plaa
menu-settings = Inställningar…
menu-services = Tjänster
menu-hide = Göm plaa
menu-hide-others = Göm övriga
menu-show-all = Visa alla
menu-quit = Avsluta plaa
menu-file = Arkiv
menu-open = Öppna…
menu-open-address = Öppna adress…
menu-close = Stäng fönster
menu-edit = Redigera
menu-cut = Klipp ut
menu-copy = Kopiera
menu-paste = Klistra in
menu-select-all = Markera allt
menu-undo = Ångra
menu-redo = Gör om
menu-trim = Beskär…
menu-split = Dela klipp
menu-rotate-left = Rotera åt vänster
menu-rotate-right = Rotera åt höger
menu-flip-horizontal = Vänd vågrätt
menu-flip-vertical = Vänd lodrätt
menu-revert = Återgå till originalet
menu-export = Exportera…
menu-view = Innehåll
menu-playback = Uppspelning
menu-play-pause = Spela upp eller pausa
menu-to-start = Gå till början
menu-previous-chapter = Föregående kapitel
menu-next-chapter = Nästa kapitel
menu-faster = Snabbare
menu-slower = Långsammare
menu-normal-speed = Normal hastighet
menu-ab-loop = Upprepa mellan två punkter
menu-volume-up = Högre
menu-volume-down = Lägre
menu-mute = Ljud av eller på
menu-next-subtitles = Nästa undertexter
menu-next-audio = Nästa ljudspår
menu-window = Fönster
menu-minimize = Minimera
menu-zoom = Zooma
menu-bring-all-to-front = Lägg alla överst

## Editing

action-edit = Redigera
title-edited = { $name } (redigerad)
edit-trim = Beskär
edit-trim-apply = Beskär
edit-split = Dela klipp
edit-remove-clip = Ta bort klipp
edit-done = Klar
edit-rotate-left = Rotera åt vänster
edit-rotate-right = Rotera åt höger
edit-flip-horizontal = Vänd vågrätt
edit-flip-vertical = Vänd lodrätt
edit-undo = Ångra
edit-redo = Gör om
edit-revert = Återgå till originalet
edit-export = Exportera…

## Exporting

export-title = Exportera
export-format = Filformat
export-size = Storlek
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Endast ljud
export-size-original = { $lines }p, som den är
export-on-graphics-card = Kodas på grafikkortet ({ $api })
export-on-processor = Kodas på processorn, vilket tar längre tid
export-no-encoder = Ingen kodare för det här formatet går att öppna här
export-checking = Söker efter en kodare…
export-sound-format = Ljudformat
export-quality = Kvalitet
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Komprimerat; spelas upp nästan överallt, även på Apples enheter
export-mp3-note = Komprimerat; spelas upp överallt
export-opus-note = Komprimerat; bästa ljudet för storleken, men alla spelare klarar det inte
export-flac-note = Förlustfritt: ljudet exakt som det är, i ett öppet format
export-alac-note = Förlustfritt: ljudet exakt som det är, så som Apples enheter spelar upp det
export-wav-note = Okomprimerat: ljudet exakt som det är, i de största filerna
export-kilobits = { $rate } kb/s
export-bits = { $bits } bitar
export-rate-lower = Sparas med { $rate } kHz, lägre än filens { $file } kHz
export-choose = Exportera…
export-name = { $name } export
export-name-edited = { $name } redigerad
export-progress = Exporterar ”{ $name }”
export-stop = Sluta exportera
notice-exported = ”{ $name }” har exporterats
notice-export-stopped = Exporten avbröts
notice-export-failed = Det gick inte att exportera: { $error }
close-title = Stänga utan att exportera?
close-unexported = Ändringarna i ”{ $name }” går förlorade om du inte exporterar dem.
close-exporting = Exporten av ”{ $name }” avbryts, och det som har skrivits hittills går förlorat.
close-anyway = Stäng
close-quit-anyway = Avsluta

## Recording

record-title = Ljudinspelning
record-file-name = Ljudinspelning
record-start = Spela in
record-stop = Sluta spela in
record-no-input = Ingen mikrofon
record-microphone = Mikrofon
record-default-microphone = Systemets standard
record-playing = Det datorn spelar upp
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanal
   *[other] { $count } kanaler
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, sparas med { $saved } kHz
action-record-sound = Spela in ljud
open-record-sound = Ny ljudinspelning
start-record-sound = Spela in ljud
menu-record-sound = Ny ljudinspelning
record-screen-title = Skärminspelning
record-screen-file-name = Skärminspelning
record-screen-whole = Skärm
record-screen-number = Skärm { $number }
record-screen-main = Skärm { $number } (huvudskärm)
record-screens = Skärmar
record-windows = Fönster
record-screen-what = Vad som ska spelas in
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Varje bildruta
record-rate-tip = Bildrutor per sekund
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } tappad
   *[other] { $size } · { $count } tappade
}
record-screen-window = Fönster
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Delas inte längre
record-screen-again = Välj igen
record-screen-failed = Det gick inte att spela in skärmen
record-sound = Ljud
record-no-sound = Inget ljud
# How much louder the input is taken, for a quiet microphone.
record-gain = Förstärkning
record-gain-value = +{ $decibels } dB
record-level-automatically = Justera nivån automatiskt
record-hide-title = plaa göms medan den spelar in
record-hide-tray = Om du vill sluta spela in klickar du på den röda pricken som plaa lägger i systemfältet, eller öppnar plaa igen.
record-hide-reopen = Öppna plaa igen för att sluta spela in.
record-hide-start = Börja spela in
record-tray-title = plaa spelar in skärmen
record-tray-tip = Klicka för att sluta spela in
action-record-screen = Spela in skärmen
open-record-screen = Ny skärminspelning
start-record-screen = Spela in skärmen
menu-record-screen = Ny skärminspelning
record-camera-title = Kamerainspelning
record-camera-file-name = Kamerainspelning
record-camera = Kamera
record-camera-failed = Det gick inte att öppna kameran
record-name-title = Namnge inspelningen
record-name-note = Den har sparats som ”{ $name }” i { $folder }.
record-name-label = Namn
record-name-keep = Behåll
record-name-rename = Byt namn
record-name-taken = Det finns redan en fil som heter ”{ $name }” där.
record-name-invalid = Ett namn får inte innehålla något av dessa tecken: / \ : * ? " < > |
record-name-failed = Det gick inte att byta namn: { $error }
action-record-camera = Spela in med kameran
open-record-camera = Ny kamerainspelning
start-record-camera = Spela in med kameran
menu-record-camera = Ny kamerainspelning
notice-no-camera = Det finns ingen kamera att spela in med
notice-recorded = ”{ $name }” har sparats i { $folder }
notice-record-failed = Det gick inte att spela in: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Tillåt extern styrning
settings-allow-outside-control-note = AI-agenter som Claude Code kan se vad som spelas i plaa och styra plaa via plaa --mcp. plaa frågar före varje ny agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Tillåtna: { $agents }
settings-forget-agents = Glöm
settings-ask-before-note = Fråga innan en agent:
settings-ask-reading = Ser vad som spelas, eller bilden
settings-ask-playing = Styr uppspelningen, ljudet eller helskärmsläget
settings-ask-opening = Öppnar en fil eller stänger ett fönster
settings-ask-editing = Redigerar en fil eller exporterar den
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vill du låta { $agent } styra plaa?
agent-prompt-body = { $agent } ber om att få använda plaas externa styrning för att se vad som spelas och styra det. Du kan stänga av extern styrning i Inställningar.
agent-prompt-allow = Tillåt
agent-prompt-deny = Tillåt inte
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Vill du låta { $agent } se vad som spelas här?
agent-ask-play = Vill du låta { $agent } styra uppspelningen här?
agent-ask-open = Vill du låta { $agent } öppna eller stänga ett fönster?
agent-ask-edit = Vill du låta { $agent } redigera eller exportera här?
agent-working = { $agent } arbetar…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } ber om att få använda ”{ $tool }”. I Inställningar väljer du vad plaa frågar om.

## Casting to a TV or speaker on the network

cast-looking = Söker efter tv-apparater och högtalare…
cast-none = Ingen tv, högtalare eller Chromecast hittades.
# $renderer is the TV's or speaker's name.
cast-stop = Sluta spela upp på { $renderer }
notice-casting = Spelar upp på { $renderer }.
notice-cast-failed = Det gick inte att spela upp på { $renderer }: { $reason }
cast-web-page = Webbsidor kan inte castas än: öppna videons egen adress i stället.

## The assistant panel, and its models in Settings

assistant-add-another = Lägg till en modell…
assistant-add-model = Assistenten behöver en modell: en molnmodell med sin API-nyckel, eller en lokal modell.
assistant-ask = Fråga om det som spelas
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Chatten höll på att bli för lång för att { $model } ska kunna läsa den på en gång, så den tidigare delen sammanfattades. Assistenten kanske tittar igen på det den såg.
assistant-compacting = Sammanfattar den tidigare delen av chatten
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } på den här datorn
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } på { $host }
assistant-new-chat = Ny chatt
assistant-no-model = Lägg först till en modell i Inställningar.
assistant-open-settings = Lägg till en modell
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Samtalet ryms inte längre i det som { $model } kan läsa på en gång. Starta en ny chatt, eller välj en modell som kan läsa mer.
assistant-problem-credit = { $provider } säger att kontot saknar saldo. För ett nytt konto måste saldot först fyllas på via webbplatsen för { $provider } innan nyckeln fungerar; försök sedan igen.
assistant-problem-key = { $provider } avvisade API-nyckeln. Kontrollera den i Inställningar.
assistant-problem-model = { $provider } har ingen modell som heter { $model }. Kontrollera namnet i Inställningar.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } svarade inte: { $message }
assistant-problem-rate = { $provider } ber dig sakta ner. Försök igen om en stund.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } ber dig sakta ner: { $message }
assistant-problem-refused = { $model } avböjde att svara.
assistant-problem-unavailable = { $provider } är upptagen eller har problem. Försök igen om en stund.
assistant-problem-unreachable = plaa kunde inte nå { $provider }. Kontrollera anslutningen, eller att servern körs.
assistant-running = Kör…
assistant-send = Skicka
assistant-shortened = Chatten var för lång för att { $model } ska kunna läsa den på en gång, så tidigare verktygsresultat utelämnades. Assistenten kanske tittar igen på det den såg.
assistant-stop = Stoppa
assistant-stopped = Stoppad.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Pratar nu med { $model }.
assistant-thinking = Tänker…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Tankar
assistant-title = Assistent
settings-assistant-add = Lägg till en modell
settings-assistant-add-button = Lägg till
# $model is the model's name.
settings-assistant-added = { $model } svarade och har lagts till.
settings-assistant-added-tag = Tillagd
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adress, till exempel { $example }
settings-assistant-change-key = Byt nyckel
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-kompatibel server
# The menu of how much a local model reads at once.
settings-assistant-context = Kontext
settings-assistant-context-note = Mer låter assistenten komma ihåg mer av en chatt, men modellen tar mer minne och kan svara långsammare.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Öppna sidan
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } körs på { $address }.
settings-assistant-found-none = { $provider } har inga modeller än. Hämta en med den och sök sedan igen.
settings-assistant-get-key = Skaffa en API-nyckel
settings-assistant-get-server = Hämta { $provider }
settings-assistant-in-use = Används
settings-assistant-key = API-nyckel
# $error is what the keychain said.
settings-assistant-key-failed = Nyckeln kunde inte sparas i nyckelringen: { $error }
settings-assistant-key-kept = Din nyckel för { $provider } förvaras i systemets nyckelring.
settings-assistant-key-needed = Den här modellen behöver en API-nyckel.
settings-assistant-key-refused = { $provider } avvisade den här nyckeln. Kontrollera att den kopierades i sin helhet, från rätt konto.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Skapa en nyckel på webbplatsen för { $provider } och klistra in den här.
settings-assistant-look-again = Sök igen
settings-assistant-looking = Söker efter modeller…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modell, till exempel { $example }
settings-assistant-no-server = plaa hittade ingen { $provider } som körs på den här datorn. Starta den, eller ange dess adress nedan.
settings-assistant-no-tools = Kan inte använda verktyg, vilket assistenten behöver
settings-assistant-none = Inga modeller än. Lägg till en nedan: en lokal modell, som Ollamas, stannar på den här datorn; en molnmodell behöver en API-nyckel från leverantören.
settings-assistant-note = Modellerna som assistenten kan prata med. Nycklar förvaras i systemets nyckelring.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Använd en annan adress
settings-assistant-recommended = Rekommenderas
settings-assistant-remove = Ta bort
settings-assistant-sees = Ser bilder
settings-assistant-trying = Provar…
settings-assistant-use = Använd
settings-assistant-use-key = Fortsätt
settings-assistant-uses-tools = Använder verktyg
