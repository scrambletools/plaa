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
language-name = Nederlands

## Common

app-name = plaa
common-cancel = Annuleren
common-close = Sluiten
common-more = Meer

## Command line

usage-help =
    Gebruik: plaa [BESTAND OF ADRES]...

    Speel video- en audiobestanden af, en streams op adressen zoals
    https://example.com/live.m3u8. Elk ervan opent in een venster van plaa
    als dat al draait; anders wordt plaa eerst gestart.

    Opties:
      -h, --help        Deze hulp tonen
      -V, --version     De versie tonen
          --interfaces  De bekabelde interfaces tonen, en of plaa er
                        AVB-streams over kan versturen
          --mcp         MCP aanbieden via stdin en stdout, zodat AI-agents
                        kunnen zien wat de draaiende plaa afspeelt en hem
                        kunnen besturen

## Start window

start-hint = Open een video- of audiobestand, een stream of een webvideo.
start-open = Openen
start-open-address = Adres openen
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Verbinden

## Actions, as buttons, tooltips and menus name them

action-open = Openen
action-open-address = Adres openen
open-file = Bestand openen…
open-address = Adres openen…
action-settings = Instellingen
action-info = Infovenster
action-loop = Herhalen
action-screenshot = Beeld opslaan als afbeelding
action-play = Afspelen
action-pause = Pauzeren
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } seconde terug
   *[other] { $seconds } seconden terug
}
action-forward = { $seconds ->
    [one] { $seconds } seconde vooruit
   *[other] { $seconds } seconden vooruit
}
action-mute = Geluid uit
action-unmute = Geluid aan
action-speed = Snelheid
action-tracks = Audio en ondertiteling
action-full-screen = Schermvullende weergave
action-playlist = Afspeellijst
action-chapters = Hoofdstukken
action-picture-in-picture = Beeld-in-beeld
action-previous = Vorige
action-next = Volgende
action-exit-full-screen = Stop schermvullende weergave
action-quality = Kwaliteit
action-visualizer = Visualisatie
action-network = Netwerkaudio
action-cast = Casten
action-connect = Verbinden
action-assistant = Assistent

## What a sound file shows under its cover

visualizer-spectrum = Spectrum
visualizer-waveform = Golfvorm
visualizer-prism = Prisma
visualizer-cover = Alleen albumhoes

## The floating controls

overlay-keep-shown = Bediening vastzetten
overlay-auto-hide = Bediening over het beeld laten zweven

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Ondertiteling
tracks-off = Uit
# A track the file gives no name or language, by its number.
tracks-numbered = Spoor { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Alleen audio

## Inspector

info-title = Infovenster
playlist-title = Afspeellijst
chapters-numbered = Hoofdstuk { $number }
info-file = Bestand
info-name = Naam
info-address = Adres
info-folder = Map
info-size = Grootte
info-format = Indeling
info-duration = Duur
info-bit-rate = Bitsnelheid
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Afmetingen
info-frame-rate = Beeldsnelheid
info-frames-per-second = { $rate } beelden per seconde
info-pixel-format = Pixelindeling
info-decoding = Decodering
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Videokaart ({ $api })
info-decoding-software = Processor
info-sample-rate = Samplefrequentie
info-hertz = { $rate } Hz
info-channels = Kanalen
info-language = Taal
info-no-sound = Geen geluid
info-tags = Tags
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Zender
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = Andere ontvangers
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR-klasse
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Een nieuw formaat koppelt { $count ->
        [one] de andere ontvanger van de zender los.
       *[other] de { $count } andere ontvangers van de zender los.
    }

## Dialogs

dialog-open = Openen
dialog-filter-media = Video en audio
dialog-filter-all = Alle bestanden

address-title = Adres openen
address-note = Een webpagina met een video, zoals die van een YouTube-video, of een stream of bestand op het netwerk, zoals https://example.com/live.m3u8.
address-label = Adres
address-open = Openen
address-recent = Recent
address-forget = Verwijderen uit recent

## Notices

notice-dialog-failed = Kan het bestandsvenster niet openen: { $error }
notice-screenshot-saved = ‘{ $name }’ opgeslagen.
notice-screenshot-failed = Kan de afbeelding niet opslaan: { $error }
notice-no-sound = Afspelen zonder geluid: { $reason }
notice-loop-from = Herhalen vanaf { $time }. Druk nog eens waar het moet eindigen.
notice-loop-between = { $from } tot { $to } wordt herhaald.
notice-loop-off = Herhalen uit.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Dit lijkt een webpagina. plaa speelt video's op webpagina's af met yt-dlp, maar dat is niet geïnstalleerd.
error-empty-list = Deze lijst bevat niets om af te spelen.

## Settings

settings-title = Instellingen
settings-tab-appearance = Weergave
settings-tab-playback = Afspelen
settings-tab-network = Netwerk
settings-tab-language = Taal
settings-tab-agents = Agents
settings-tab-assistant = Assistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (ontwikkelversie, { $build })
settings-choose = Kiezen…

settings-appearance-system = Systeem volgen
settings-appearance-light = Licht
settings-appearance-dark = Donker
settings-colors = Kleuren
settings-system-accent = Accentkleur van het systeem gebruiken
settings-omarchy-note = De kleuren zijn afgeleid van het accent van ‘{ $theme }’.
settings-system-accent-note = De kleuren zijn afgeleid van de accentkleur van het systeem.
settings-system-accent-none = Het systeem heeft geen accentkleur, dus plaa gebruikt de hieronder gekozen kleur.
settings-accent-chosen-note = De kleuren zijn afgeleid van de hieronder gekozen kleur.
settings-controls = Bediening
settings-overlay = Zwevende bediening
settings-overlay-note = De knoppenbalk en de afspeelknoppen zweven boven het beeld of de albumhoes en schuiven weg zolang de aanwijzer stilstaat of buiten het venster is.
settings-overlay-delay = Verbergen na
settings-overlay-delay-note = Hoe lang de aanwijzer stil boven het afspelende beeld staat voordat de bediening wegschuift.
settings-seconds = { $seconds } s
settings-transparency = Transparantie
settings-transparency-note = Hoeveel van het beeld door de zwevende bediening schijnt.
settings-percent = { $percent }%
settings-corner-radius = Hoekradius
settings-corner-radius-note = Voor dialoogvensters en de zwevende bediening.
settings-pixels = { $pixels } px
settings-animations = Animaties
settings-animations-note = Schuivende balken en panelen, groeiende dialoogvensters en verende knoppen.
settings-animations-reduced = Uit zolang het systeem om minder beweging vraagt.

settings-hardware = Videokaart gebruiken
settings-hardware-note = Decodeert video en codeert exports op de videokaart: verbruikt minder stroom, speelt grote video's soepel af en exporteert sneller. Geldt voor bestanden die je daarna opent.
settings-audio-device = Geluidsuitvoer
settings-audio-device-system = Systeemstandaard
settings-resume = Verdergaan waar je was gebleven
settings-resume-note = Bestanden beginnen waar ze waren toen ze het laatst werden gesloten.
settings-folder-playlists = De andere bestanden in de map afspelen
settings-folder-playlists-note = Als je één bestand opent, komen de andere bestanden in de map in de lijst en worden ze daarna afgespeeld.
settings-streams = Streams en webvideo's
settings-stream-quality = Kwaliteit
settings-stream-quality-note = Voor streams en webvideo's die in meerdere kwaliteiten beschikbaar zijn. Geldt voor wat je daarna opent.
settings-quality-auto = Automatisch, zo hoog als het scherm
settings-quality-best = Beste
settings-web-pages = Webvideo's afspelen met yt-dlp
settings-web-pages-found = yt-dlp vindt de video op pagina's zoals die van YouTube.
settings-web-pages-missing = yt-dlp is niet geïnstalleerd. Installeer het om de video's af te spelen op pagina's zoals die van YouTube.
settings-folders = Mappen
settings-recordings = Opnamen
settings-screenshots = Opgeslagen beelden

settings-interface-language = Taal van de interface
# The first entry of the language list.
settings-language-system = Systeem volgen
settings-track-languages = Talen van sporen
settings-track-languages-note = Taalcodes zoals en of fra, de meest gewenste eerst, gescheiden door komma's. plaa kiest audio en ondertiteling in deze talen als een bestand ze heeft.
settings-audio-languages = Audio
settings-subtitle-languages = Ondertiteling

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-interface
settings-avb-interface-note = De bekabelde poort waarop plaa AVB- en Milan-zenders en -ontvangers vindt. Versturen vereist een poort met een PTP-hardwareklok, zoals die van een Intel I210.
settings-avb-off = Uit
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (alleen bekijken: geen PTP-klok)
settings-avb-off-mac = { $interface } (alleen bekijken: AVB staat uit)
settings-avb-interface-note-mac = De bekabelde poort waarop plaa AVB- en Milan-zenders en -ontvangers vindt. Versturen gaat via de eigen AVB-entiteit van deze Mac. Daarvoor moet AVB voor de poort aanstaan in de netwerkapparaatbrowser van Audio-MIDI-configuratie.
settings-avb-talker-note-mac = plaa verstuurt het geluid van het venster dat het laatst speelde via de eigen AVB-entiteit van deze Mac naar de ontvangers die erop zijn aangesloten.
settings-avb-talker = Versturen wat er speelt
settings-avb-talker-note = plaa wordt een Milan-zender en verstuurt het geluid van het venster dat het laatst speelde naar de ontvangers die erop zijn aangesloten.
settings-avb-name = Naam op het netwerk
settings-avb-name-note = Controllers tonen plaa als ‘{ $name }’. Druk op Enter om de naam te wijzigen.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa op { $computer }
avb-could-not-start = Kan { $interface } niet gebruiken: { $error }
network-title = Netwerkaudio
network-off = Zet ‘Versturen wat er speelt’ aan in Instellingen om op het netwerk af te spelen.
network-blocked = Er wordt niet verstuurd: { $reason }
network-waiting = Klaar om te versturen, wacht tot een ontvanger verbinding maakt.
network-sending = Verstuurt naar de aangesloten ontvangers.
network-looking = Ontvangers zoeken…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l wordt gestart, dat de netwerktijd bijhoudt waarop de stream van plaa wordt verstuurd…
network-ptp-elsewhere = ptp4l draait op een andere interface, dus de stream van plaa heeft hier geen netwerktijd en ontvangers kunnen hem misschien niet afspelen. Kies die interface in Instellingen, of start ptp4l op deze.
network-ptp-missing = ptp4l draait niet, dus de stream van plaa heeft geen netwerktijd en ontvangers kunnen hem misschien niet afspelen. Installeer linuxptp met het pakket van plaa, of triib, dat het uitvoert.
network-ptp-failed = ptp4l draait niet, dus de stream van plaa heeft geen netwerktijd en ontvangers kunnen hem misschien niet afspelen. plaa kon het niet starten: { $reason }
network-no-listeners = Geen enkele ontvanger op het netwerk kan het geluid van plaa afspelen.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Afspelen op { $listener }
network-stop-on = Stoppen met afspelen op { $listener }
network-choose-format = Kies een formaat om op { $listener } af te spelen
network-failed = Kan geen verbinding maken: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Werk triib bij om hier af te spelen.
network-choose = Kies een ontvanger om te zien hoe die zijn stream ontvangt.
network-no-sources = Geen enkele zender op het netwerk verstuurt in een formaat dat plaa afspeelt, of kan naar zo’n formaat wisselen.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Stoppen met luisteren naar { $talker }
network-listen-update-triib = Werk triib bij om hier te luisteren: plaa speelt alleen af wat deze computer hoe dan ook bereikt.
network-listen-lite = De poort gebruikt AVB Lite, dat geen streams reserveert: plaa speelt alleen af wat deze computer hoe dan ook bereikt.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Zenders zoeken…
# plaa listens through its entity, which exists only while it sends.
connect-off = Zet ‘Versturen wat er speelt’ aan in Instellingen om hier naar zenders te luisteren.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa kan hier niet luisteren: { $reason }
connect-sending = Versturen in een formaat dat plaa afspeelt
connect-changing = Wisselen naar een formaat dat plaa afspeelt
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, in { $format }
connect-change-cuts-off = { $talker }, in { $format }, waarbij { $count ->
        [one] zijn andere ontvanger wordt losgekoppeld
       *[other] zijn { $count } andere ontvangers worden losgekoppeld
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Niet verbonden
network-from = Van { $stream }
network-from-receiving = Van { $stream }, ontvangt
network-from-waiting = Van { $stream }, wacht op de zender
network-from-failed = Van { $stream }, de reservering van de zender is mislukt: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Mediaklok
network-locked = Vergrendeld op de stream
network-not-locked = Niet vergrendeld
network-format = Formaat
# The formats both the listener and plaa's stream can be in.
network-formats = Formaten die hij van plaa kan ontvangen
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream-ID
# The MAC address the stream is sent to.
network-destination = Bestemming
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latentie
network-microseconds = { $time } µs
network-frames = Ontvangen frames
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Frames buiten volgorde
# How often the stream stopped coming while it was taken.
network-interrupted = Onderbrekingen
# Frames that came after, or long before, their time to be played.
network-late = Te late frames
network-early = Te vroege frames
network-entity-id = Entiteits-ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = onvoldoende bandbreedte
msrp-failure-2 = onvoldoende middelen in de switch
msrp-failure-3 = onvoldoende bandbreedte voor de verkeersklasse
msrp-failure-4 = stream-ID in gebruik door een andere zender
msrp-failure-5 = bestemmingsadres al in gebruik
msrp-failure-6 = verdrongen door een stream met een hogere rang
msrp-failure-7 = gemelde latentie is gewijzigd
msrp-failure-8 = uitgaande poort ondersteunt geen AVB
msrp-failure-9 = gebruik een ander bestemmingsadres
msrp-failure-10 = geen MSRP-middelen meer
msrp-failure-11 = geen MMRP-middelen meer
msrp-failure-12 = kan het bestemmingsadres niet opslaan
msrp-failure-13 = prioriteit is geen SR-klasseprioriteit
msrp-failure-14 = frames te groot voor het medium
msrp-failure-15 = limiet voor fan-in-poorten bereikt
msrp-failure-16 = eerste waarde gewijzigd voor een geregistreerde stream
msrp-failure-17 = VLAN geblokkeerd op de uitgaande poort
msrp-failure-18 = VLAN-tagging uitgeschakeld op de uitgaande poort
msrp-failure-19 = SR-klasseprioriteiten komen niet overeen
msrp-failure-unknown = onbekende reden
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, bij switch { $bridge }

## The macOS menu bar

menu-about = Over plaa
menu-settings = Instellingen…
menu-services = Voorzieningen
menu-hide = Verberg plaa
menu-hide-others = Verberg andere
menu-show-all = Toon alles
menu-quit = Stop plaa
menu-file = Archief
menu-open = Open…
menu-open-address = Open adres…
menu-close = Sluit venster
menu-edit = Wijzig
menu-cut = Knip
menu-copy = Kopieer
menu-paste = Plak
menu-select-all = Selecteer alles
menu-undo = Herstel
menu-redo = Opnieuw
menu-trim = Knip bij…
menu-split = Splits clip
menu-rotate-left = Roteer linksom
menu-rotate-right = Roteer rechtsom
menu-flip-horizontal = Spiegel horizontaal
menu-flip-vertical = Spiegel verticaal
menu-revert = Zet terug naar origineel
menu-export = Exporteer…
menu-view = Weergave
menu-playback = Afspelen
menu-play-pause = Speel af of pauzeer
menu-to-start = Ga naar begin
menu-previous-chapter = Vorig hoofdstuk
menu-next-chapter = Volgend hoofdstuk
menu-faster = Sneller
menu-slower = Langzamer
menu-normal-speed = Normale snelheid
menu-ab-loop = Herhaal tussen twee punten
menu-volume-up = Harder
menu-volume-down = Zachter
menu-mute = Geluid uit of aan
menu-next-subtitles = Volgende ondertiteling
menu-next-audio = Volgend audiospoor
menu-window = Venster
menu-minimize = Minimaliseer
menu-zoom = Zoom
menu-bring-all-to-front = Breng alles naar voren

## Editing

action-edit = Bewerken
title-edited = { $name } (bewerkt)
edit-trim = Bijknippen
edit-trim-apply = Bijknippen
edit-split = Clip splitsen
edit-remove-clip = Clip verwijderen
edit-done = Gereed
edit-rotate-left = Linksom draaien
edit-rotate-right = Rechtsom draaien
edit-flip-horizontal = Horizontaal spiegelen
edit-flip-vertical = Verticaal spiegelen
edit-undo = Ongedaan maken
edit-redo = Opnieuw
edit-revert = Terugzetten naar origineel
edit-export = Exporteren…

## Exporting

export-title = Exporteren
export-format = Indeling
export-size = Grootte
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Alleen geluid
export-size-original = { $lines }p, zoals het is
export-on-graphics-card = Gecodeerd op de videokaart ({ $api })
export-on-processor = Gecodeerd op de processor, wat langer duurt
export-no-encoder = Er opent hier geen encoder voor deze indeling
export-checking = Encoder zoeken…
export-sound-format = Geluidsindeling
export-quality = Kwaliteit
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Gecomprimeerd; speelt bijna overal af, ook op apparaten van Apple
export-mp3-note = Gecomprimeerd; speelt overal af
export-opus-note = Gecomprimeerd; het beste geluid voor de grootte, al speelt niet elke speler het af
export-flac-note = Verliesvrij: het geluid precies zoals het is, in een open indeling
export-alac-note = Verliesvrij: het geluid precies zoals het is, zoals apparaten van Apple het afspelen
export-wav-note = Ongecomprimeerd: het geluid precies zoals het is, in de grootste bestanden
export-kilobits = { $rate } kb/s
export-bits = { $bits }-bits
export-rate-lower = Opgeslagen op { $rate } kHz, lager dan de { $file } kHz van het bestand
export-choose = Exporteren…
export-name = { $name } export
export-name-edited = { $name } bewerkt
export-progress = ‘{ $name }’ wordt geëxporteerd
export-stop = Exporteren stoppen
notice-exported = ‘{ $name }’ geëxporteerd
notice-export-stopped = Exporteren gestopt
notice-export-failed = Kan niet exporteren: { $error }
close-title = Sluiten zonder te exporteren?
close-unexported = De wijzigingen in ‘{ $name }’ gaan verloren tenzij je ze exporteert.
close-exporting = Het exporteren van ‘{ $name }’ stopt, en wat er tot nu toe is geschreven, gaat verloren.
close-anyway = Sluiten
close-quit-anyway = Stoppen

## Recording

record-title = Geluidsopname
record-file-name = Geluidsopname
record-start = Opnemen
record-stop = Opname stoppen
record-no-input = Geen microfoon
record-microphone = Microfoon
record-default-microphone = Standaard van het systeem
record-playing = Wat de computer afspeelt
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanaal
   *[other] { $count } kanalen
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, opgeslagen op { $saved } kHz
action-record-sound = Geluid opnemen
open-record-sound = Nieuwe geluidsopname
start-record-sound = Geluid opnemen
menu-record-sound = Nieuwe geluidsopname
record-screen-title = Schermopname
record-screen-file-name = Schermopname
record-screen-whole = Scherm
record-screen-number = Scherm { $number }
record-screen-main = Scherm { $number } (hoofdscherm)
record-screens = Schermen
record-windows = Vensters
record-screen-what = Wat opnemen
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Elk beeld
record-rate-tip = Beelden per seconde
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } overgeslagen
record-screen-window = Venster
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Niet meer gedeeld
record-screen-again = Opnieuw kiezen
record-screen-failed = Kan het scherm niet opnemen
record-sound = Geluid
record-no-sound = Geen geluid
# How much louder the input is taken, for a quiet microphone.
record-gain = Versterking
record-gain-value = +{ $decibels } dB
record-level-automatically = Niveau automatisch regelen
record-hide-title = plaa wordt verborgen tijdens het opnemen
record-hide-tray = Om de opname te stoppen klik je op de rode stip die plaa in het systeemvak zet, of open je plaa opnieuw.
record-hide-reopen = Open plaa opnieuw om de opname te stoppen.
record-hide-start = Opname starten
record-tray-title = plaa neemt het scherm op
record-tray-tip = Klik om de opname te stoppen
action-record-screen = Scherm opnemen
open-record-screen = Nieuwe schermopname
start-record-screen = Scherm opnemen
menu-record-screen = Nieuwe schermopname
record-camera-title = Camera-opname
record-camera-file-name = Camera-opname
record-camera = Camera
record-camera-failed = Kan de camera niet openen
action-record-camera = Opnemen met camera
open-record-camera = Nieuwe camera-opname
start-record-camera = Opnemen met camera
menu-record-camera = Nieuwe camera-opname
notice-no-camera = Er is geen camera om mee op te nemen
notice-recorded = ‘{ $name }’ opgeslagen in { $folder }
notice-record-failed = Kan niet opnemen: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Besturing van buitenaf toestaan
settings-allow-outside-control-note = AI-agents zoals Claude Code kunnen via plaa --mcp zien wat er in plaa speelt en plaa besturen. plaa vraagt het eerst bij elke nieuwe agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Toegestaan: { $agents }
settings-forget-agents = Vergeten
settings-ask-before-note = Eerst vragen voordat een agent dit doet:
settings-ask-reading = Zien wat er speelt, of het beeld
settings-ask-playing = Het afspelen, het geluid of de schermvullende weergave besturen
settings-ask-opening = Een bestand openen of een venster sluiten
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent } toestaan plaa te besturen?
agent-prompt-body = { $agent } vraagt om de besturing van buitenaf van plaa te gebruiken, om te zien wat er speelt en het te besturen. Je kunt besturing van buitenaf uitzetten in Instellingen.
agent-prompt-allow = Toestaan
agent-prompt-deny = Niet toestaan
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent } toestaan te zien wat hier speelt?
agent-ask-play = { $agent } toestaan het afspelen hier te besturen?
agent-ask-open = { $agent } toestaan een venster te openen of te sluiten?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } vraagt om ‘{ $tool }’ te gebruiken. In Instellingen kies je waarnaar plaa vraagt.

## Casting to a TV or speaker on the network

cast-looking = Tv's en speakers zoeken…
cast-none = Er is geen tv, speaker of Chromecast gevonden.
# $renderer is the TV's or speaker's name.
cast-stop = Stoppen met afspelen op { $renderer }
notice-casting = Afspelen op { $renderer }.
notice-cast-failed = Kan niet afspelen op { $renderer }: { $reason }
cast-web-page = Webpagina's kunnen nog niet worden gecast: open in plaats daarvan het eigen adres van de video.

## The assistant panel, and its models in Settings

assistant-add-another = Model toevoegen…
assistant-add-model = De assistent heeft een model nodig: een cloudmodel met de bijbehorende API-sleutel, of een lokaal model.
assistant-ask = Vraag iets over wat er speelt
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = De chat werd te lang voor { $model } om in één keer te lezen, dus het eerdere deel is samengevat. De assistent kijkt misschien nog eens naar wat hij heeft gezien.
assistant-compacting = Het eerdere deel van de chat wordt samengevat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } op deze computer
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } op { $host }
assistant-new-chat = Nieuwe chat
assistant-no-model = Voeg eerst een model toe in Instellingen.
assistant-open-settings = Model toevoegen
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Het gesprek past niet meer in wat { $model } in één keer kan lezen. Begin een nieuwe chat, of kies een model dat meer kan lezen.
assistant-problem-credit = { $provider } meldt dat het account geen tegoed heeft. Voor een nieuw account moet je eerst tegoed kopen op de site van { $provider } voordat de sleutel werkt; probeer het daarna opnieuw.
assistant-problem-key = { $provider } heeft de API-sleutel geweigerd. Controleer die in Instellingen.
assistant-problem-model = { $provider } heeft geen model met de naam { $model }. Controleer de naam in Instellingen.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } heeft niet geantwoord: { $message }
assistant-problem-rate = { $provider } vraagt om het rustiger aan te doen. Probeer het zo opnieuw.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } vraagt om het rustiger aan te doen: { $message }
assistant-problem-refused = { $model } weigerde te antwoorden.
assistant-problem-unavailable = { $provider } is bezet of heeft problemen. Probeer het zo opnieuw.
assistant-problem-unreachable = plaa kon { $provider } niet bereiken. Controleer je verbinding, of dat de server draait.
assistant-running = Bezig…
assistant-send = Versturen
assistant-shortened = De chat was te lang voor { $model } om in één keer te lezen, dus eerdere resultaten van tools zijn weggelaten. De assistent kijkt misschien nog eens naar wat hij heeft gezien.
assistant-stop = Stoppen
assistant-stopped = Gestopt.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Je praat nu met { $model }.
assistant-thinking = Denkt na…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Gedachten
assistant-title = Assistent
settings-assistant-add = Model toevoegen
settings-assistant-add-button = Toevoegen
# $model is the model's name.
settings-assistant-added = { $model } heeft geantwoord en is toegevoegd.
settings-assistant-added-tag = Toegevoegd
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adres, bijvoorbeeld { $example }
settings-assistant-change-key = Sleutel wijzigen
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-compatibele server
# The menu of how much a local model reads at once.
settings-assistant-context = Context
settings-assistant-context-note = Met meer kan de assistent meer van een chat in gedachten houden, maar het model gebruikt dan meer geheugen en antwoordt mogelijk trager.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Pagina openen
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } draait op { $address }.
settings-assistant-found-none = { $provider } heeft nog geen modellen. Download er daarmee een en zoek dan opnieuw.
settings-assistant-get-key = API-sleutel aanvragen
settings-assistant-get-server = { $provider } downloaden
settings-assistant-in-use = In gebruik
settings-assistant-key = API-sleutel
# $error is what the keychain said.
settings-assistant-key-failed = De sleutel kon niet in de beveiligde opslag worden bewaard: { $error }
settings-assistant-key-kept = Je sleutel voor { $provider } wordt bewaard in de beveiligde opslag van het systeem.
settings-assistant-key-needed = Dit model heeft een API-sleutel nodig.
settings-assistant-key-refused = { $provider } heeft deze sleutel geweigerd. Controleer of die volledig is gekopieerd, uit het juiste account.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Maak een sleutel aan op de site van { $provider } en plak die hier.
settings-assistant-look-again = Opnieuw zoeken
settings-assistant-looking = Modellen zoeken…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, bijvoorbeeld { $example }
settings-assistant-no-server = plaa heeft op deze computer geen draaiende { $provider } gevonden. Start het, of geef hieronder het adres op.
settings-assistant-no-tools = Kan geen tools gebruiken, en die heeft de assistent nodig
settings-assistant-none = Nog geen modellen. Voeg er hieronder een toe: een lokaal model, zoals dat van Ollama, blijft op deze computer; een cloudmodel heeft een API-sleutel van de aanbieder nodig.
settings-assistant-note = De modellen waarmee de assistent kan praten. Sleutels worden bewaard in de beveiligde opslag van het systeem.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Ander adres gebruiken
settings-assistant-recommended = Aanbevolen
settings-assistant-remove = Verwijderen
settings-assistant-sees = Ziet afbeeldingen
settings-assistant-trying = Proberen…
settings-assistant-use = Gebruiken
settings-assistant-use-key = Doorgaan
settings-assistant-uses-tools = Gebruikt tools
