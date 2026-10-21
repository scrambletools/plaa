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
language-name = Deutsch

## Common

app-name = plaa
common-cancel = Abbrechen
common-close = Schließen
common-more = Mehr

## Command line

usage-help =
    Aufruf: plaa [DATEI ODER ADRESSE]...

    Video- und Audiodateien sowie Streams unter Adressen wie
    https://example.com/live.m3u8 abspielen. Jede Datei und jeder Stream
    öffnet sich in einem Fenster des laufenden plaa; läuft plaa noch nicht,
    wird es gestartet.

    Optionen:
      -h, --help        Diese Hilfe anzeigen
      -V, --version     Die Version anzeigen
          --interfaces  Die kabelgebundenen Schnittstellen anzeigen und ob plaa
                        darauf AVB-Streams senden kann
          --mcp         MCP über stdin und stdout anbieten, damit KI-Agenten
                        sehen, was das laufende plaa abspielt, und es steuern

## Start window

start-hint = Öffnen Sie eine Video- oder Audiodatei, einen Stream oder ein Webvideo.
start-open = Öffnen
start-open-address = Adresse öffnen

## Actions, as buttons, tooltips and menus name them

action-open = Öffnen
action-open-address = Adresse öffnen
open-file = Datei öffnen …
open-address = Adresse öffnen …
action-settings = Einstellungen
action-info = Informationen
action-loop = Schleife
action-screenshot = Frame als Bild speichern
action-play = Abspielen
action-pause = Pause
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } Sekunde zurück
   *[other] { $seconds } Sekunden zurück
}
action-forward = { $seconds ->
    [one] { $seconds } Sekunde vor
   *[other] { $seconds } Sekunden vor
}
action-mute = Stummschalten
action-unmute = Ton einschalten
action-speed = Geschwindigkeit
action-tracks = Audio und Untertitel
action-full-screen = Vollbild
action-playlist = Wiedergabeliste
action-chapters = Kapitel
action-picture-in-picture = Bild-in-Bild
action-previous = Vorheriges
action-next = Nächstes
action-exit-full-screen = Vollbild beenden
action-quality = Qualität
action-visualizer = Visualisierung
action-network = Netzwerk-Audio
action-cast = Streamen
action-assistant = Assistent

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Wellenform
visualizer-prism = Prisma
visualizer-cover = Nur Cover

## The floating controls

overlay-keep-shown = Steuerelemente angedockt lassen
overlay-auto-hide = Steuerelemente über dem Bild schweben lassen

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Untertitel
tracks-off = Aus
# A track the file gives no name or language, by its number.
tracks-numbered = Spur { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Nur Audio

## Inspector

info-title = Informationen
playlist-title = Wiedergabeliste
chapters-numbered = Kapitel { $number }
info-file = Datei
info-name = Name
info-address = Adresse
info-folder = Ordner
info-size = Größe
info-format = Format
info-duration = Dauer
info-bit-rate = Bitrate
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Abmessungen
info-frame-rate = Bildrate
info-frames-per-second = { $rate } Bilder pro Sekunde
info-pixel-format = Pixelformat
info-decoding = Decodierung
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Grafikkarte ({ $api })
info-decoding-software = Prozessor
info-sample-rate = Abtastrate
info-hertz = { $rate } Hz
info-channels = Kanäle
info-language = Sprache
info-no-sound = Kein Ton
info-tags = Tags

## Dialogs

dialog-open = Öffnen
dialog-filter-media = Video und Audio
dialog-filter-all = Alle Dateien

address-title = Adresse öffnen
address-note = Eine Webseite mit einem Video, etwa einem YouTube-Video, oder ein Stream oder eine Datei im Netzwerk, etwa https://example.com/live.m3u8.
address-label = Adresse
address-open = Öffnen
address-recent = Zuletzt verwendet
address-forget = Aus „Zuletzt verwendet“ entfernen

## Notices

notice-dialog-failed = Der Dateidialog konnte nicht geöffnet werden: { $error }
notice-screenshot-saved = „{ $name }“ gespeichert.
notice-screenshot-failed = Das Bild konnte nicht gespeichert werden: { $error }
notice-no-sound = Wiedergabe ohne Ton: { $reason }
notice-loop-from = Schleife ab { $time }. Drücken Sie erneut an der Stelle, an der sie enden soll.
notice-loop-between = Schleife von { $from } bis { $to }.
notice-loop-off = Schleife aus.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Dies scheint eine Webseite zu sein. plaa spielt Videos auf Webseiten mit yt-dlp ab, das nicht installiert ist.
error-empty-list = Diese Liste enthält nichts zum Abspielen.

## Settings

settings-title = Einstellungen
settings-tab-appearance = Erscheinungsbild
settings-tab-playback = Wiedergabe
settings-tab-network = Netzwerk
settings-tab-language = Sprache
settings-tab-agents = Agenten
settings-tab-assistant = Assistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (Entwicklungsversion, { $build })
settings-choose = Auswählen …

settings-appearance-system = Wie im System
settings-appearance-light = Hell
settings-appearance-dark = Dunkel
settings-colors = Farben
settings-system-accent = Akzentfarbe des Systems verwenden
settings-omarchy-note = Die Farben werden aus der Akzentfarbe von „{ $theme }“ abgeleitet.
settings-system-accent-note = Die Farben werden aus der Akzentfarbe des Systems abgeleitet.
settings-system-accent-none = Das System hat keine Akzentfarbe, daher verwendet plaa die unten gewählte.
settings-accent-chosen-note = Die Farben werden aus der unten gewählten Farbe abgeleitet.
settings-controls = Steuerelemente
settings-overlay = Schwebende Steuerelemente
settings-overlay-note = Symbolleiste und Wiedergabesteuerung schweben über dem Bild oder dem Cover und gleiten weg, solange der Zeiger ruht oder außerhalb des Fensters ist.
settings-overlay-delay = Ausblenden nach
settings-overlay-delay-note = Wie lange der Zeiger über der Wiedergabe ruht, bevor die Steuerelemente weggleiten.
settings-seconds = { $seconds } s
settings-transparency = Transparenz
settings-transparency-note = Wie viel vom Bild durch die schwebenden Steuerelemente scheint.
settings-percent = { $percent } %
settings-corner-radius = Eckenradius
settings-corner-radius-note = Für Dialoge und die schwebenden Steuerelemente.
settings-pixels = { $pixels } px
settings-animations = Animationen
settings-animations-note = Gleitende Leisten und Bereiche, wachsende Dialoge und federnde Schaltflächen.
settings-animations-reduced = Aus, solange im System „Bewegung reduzieren“ aktiv ist.

settings-hardware = Grafikkarte verwenden
settings-hardware-note = Decodiert Videos und codiert Exporte auf der Grafikkarte: Das braucht weniger Strom, spielt große Videos flüssig ab und exportiert schneller. Gilt für danach geöffnete Dateien.
settings-audio-device = Tonausgabe
settings-audio-device-system = Systemstandard
settings-resume = Dort fortsetzen, wo Sie aufgehört haben
settings-resume-note = Dateien beginnen dort, wo sie zuletzt geschlossen wurden.
settings-folder-playlists = Weitere Dateien im Ordner abspielen
settings-folder-playlists-note = Beim Öffnen einer Datei werden die anderen in ihrem Ordner aufgelistet und danach abgespielt.
settings-streams = Streams und Webvideos
settings-stream-quality = Qualität
settings-stream-quality-note = Für Streams und Webvideos, die es in mehreren Qualitäten gibt. Gilt für alles, was danach geöffnet wird.
settings-quality-auto = Automatisch, so hoch wie der Bildschirm
settings-quality-best = Beste
settings-web-pages = Webvideos mit yt-dlp abspielen
settings-web-pages-found = yt-dlp findet das Video auf Seiten wie denen von YouTube.
settings-web-pages-missing = yt-dlp ist nicht installiert. Installieren Sie es, um Videos auf Seiten wie denen von YouTube abzuspielen.
settings-folders = Ordner
settings-recordings = Aufnahmen
settings-screenshots = Gespeicherte Frames

settings-interface-language = Sprache der Oberfläche
# The first entry of the language list.
settings-language-system = Wie im System
settings-track-languages = Sprachen der Spuren
settings-track-languages-note = Sprachcodes wie en oder fra, die bevorzugte zuerst, durch Kommas getrennt. plaa wählt Audio und Untertitel in diesen Sprachen, wenn eine Datei sie enthält.
settings-audio-languages = Audio
settings-subtitle-languages = Untertitel

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-Schnittstelle
settings-avb-interface-note = Der kabelgebundene Port, an dem plaa AVB- und Milan-Talker und -Listener findet. Zum Senden ist ein Port mit PTP-Hardwareuhr nötig, etwa der eines Intel I210.
settings-avb-off = Aus
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (nur Auflisten: keine PTP-Uhr)
settings-avb-off-mac = { $interface } (nur Auflisten: AVB ist aus)
settings-avb-interface-note-mac = Der kabelgebundene Port, an dem plaa AVB- und Milan-Talker und -Listener findet. Gesendet wird über die eigene AVB-Entität dieses Mac; dafür muss AVB für den Port im Netzwerkgeräte-Browser von Audio-MIDI-Setup eingeschaltet sein.
settings-avb-talker-note-mac = plaa sendet den Ton des zuletzt abgespielten Fensters über die eigene AVB-Entität dieses Mac an die damit verbundenen Listener.
settings-avb-talker = Wiedergabe senden
settings-avb-talker-note = plaa wird zu einem Milan-Talker und sendet den Ton des zuletzt abgespielten Fensters an die damit verbundenen Listener.
settings-avb-name = Name im Netzwerk
settings-avb-name-note = Controller zeigen plaa als „{ $name }“ an. Drücken Sie Enter, um den Namen zu ändern.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa auf { $computer }
avb-could-not-start = { $interface } konnte nicht verwendet werden: { $error }
network-title = Netzwerk-Audio
network-off = Schalten Sie in den Einstellungen „Wiedergabe senden“ ein, um im Netzwerk abzuspielen.
network-blocked = Es wird nicht gesendet: { $reason }
network-waiting = Bereit zum Senden, es wird auf die Verbindung eines Listeners gewartet.
network-sending = Wird an die verbundenen Listener gesendet.
network-looking = Listener werden gesucht …
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l wird gestartet, das die Netzwerkzeit bereitstellt, nach der plaa seinen Stream sendet …
network-ptp-elsewhere = ptp4l läuft auf einer anderen Schnittstelle, daher hat der Stream von plaa hier keine Netzwerkzeit, und Listener können ihn möglicherweise nicht abspielen. Wählen Sie diese Schnittstelle in den Einstellungen oder starten Sie ptp4l auf dieser.
network-ptp-missing = ptp4l läuft nicht, daher hat der Stream von plaa keine Netzwerkzeit, und Listener können ihn möglicherweise nicht abspielen. Installieren Sie linuxptp mit dem Paket von plaa oder installieren Sie triib, das ptp4l ausführt.
network-ptp-failed = ptp4l läuft nicht, daher hat der Stream von plaa keine Netzwerkzeit, und Listener können ihn möglicherweise nicht abspielen. plaa konnte es nicht starten: { $reason }
network-no-listeners = Kein Listener im Netzwerk kann den Ton von plaa abspielen.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Auf { $listener } abspielen
network-stop-on = Wiedergabe auf { $listener } beenden
network-choose-format = Format für die Wiedergabe auf { $listener } wählen
network-failed = Verbindung fehlgeschlagen: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Aktualisieren Sie triib, um hier abzuspielen.
network-choose = Wählen Sie einen Listener, um zu sehen, wie er seinen Stream empfängt.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Hier hören
network-no-talkers = Kein Talker im Netzwerk sendet in einem Format, das plaa abspielen kann.
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker } hören
network-listen-stop = { $talker } nicht mehr hören
network-listen-frames = Empfangene Frames: { $frames }
network-listen-update-triib = Aktualisieren Sie triib, um hier zu hören: plaa spielt nur ab, was ohnehin bei diesem Computer ankommt.
network-listen-lite = Der Port nutzt AVB Lite, das keine Streams reserviert: plaa spielt nur ab, was ohnehin bei diesem Computer ankommt.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Nicht verbunden
network-from = Von { $stream }
network-from-receiving = Von { $stream }, Empfang läuft
network-from-waiting = Von { $stream }, wartet auf den Talker
network-from-failed = Von { $stream }, die Reservierung des Talkers ist fehlgeschlagen: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Medientakt
network-locked = Auf den Stream synchronisiert
network-not-locked = Nicht synchronisiert
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formate, die er von plaa empfangen kann
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream-ID
# The MAC address the stream is sent to.
network-destination = Ziel
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latenz
network-microseconds = { $time } µs
network-frames = Empfangene Frames
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Frames außer der Reihe
# How often the stream stopped coming while it was taken.
network-interrupted = Unterbrechungen
network-entity-id = Entitäts-ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = unzureichende Bandbreite
msrp-failure-2 = unzureichende Bridge-Ressourcen
msrp-failure-3 = unzureichende Bandbreite für die Verkehrsklasse
msrp-failure-4 = Stream-ID wird von einem anderen Talker verwendet
msrp-failure-5 = Zieladresse wird bereits verwendet
msrp-failure-6 = von einem Stream mit höherem Rang verdrängt
msrp-failure-7 = gemeldete Latenz hat sich geändert
msrp-failure-8 = Ausgangsport ist nicht AVB-fähig
msrp-failure-9 = eine andere Zieladresse verwenden
msrp-failure-10 = keine MSRP-Ressourcen mehr
msrp-failure-11 = keine MMRP-Ressourcen mehr
msrp-failure-12 = Zieladresse kann nicht gespeichert werden
msrp-failure-13 = Priorität ist keine SR-Klassen-Priorität
msrp-failure-14 = Frames zu groß für das Medium
msrp-failure-15 = Fan-in-Port-Limit erreicht
msrp-failure-16 = erster Wert für einen registrierten Stream geändert
msrp-failure-17 = VLAN am Ausgangsport blockiert
msrp-failure-18 = VLAN-Tagging am Ausgangsport deaktiviert
msrp-failure-19 = SR-Klassen-Priorität stimmt nicht überein
msrp-failure-unknown = unbekannter Grund
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, an der Bridge { $bridge }

## The macOS menu bar

menu-about = Über plaa
menu-settings = Einstellungen …
menu-services = Dienste
menu-hide = plaa ausblenden
menu-hide-others = Andere ausblenden
menu-show-all = Alle einblenden
menu-quit = plaa beenden
menu-file = Ablage
menu-open = Öffnen …
menu-open-address = Adresse öffnen …
menu-close = Fenster schließen
menu-edit = Bearbeiten
menu-cut = Ausschneiden
menu-copy = Kopieren
menu-paste = Einsetzen
menu-select-all = Alles auswählen
menu-undo = Widerrufen
menu-redo = Wiederholen
menu-trim = Trimmen …
menu-split = Clip teilen
menu-rotate-left = Nach links drehen
menu-rotate-right = Nach rechts drehen
menu-flip-horizontal = Horizontal spiegeln
menu-flip-vertical = Vertikal spiegeln
menu-revert = Auf Original zurücksetzen
menu-export = Exportieren …
menu-view = Darstellung
menu-playback = Wiedergabe
menu-play-pause = Abspielen oder pausieren
menu-to-start = Zum Anfang
menu-previous-chapter = Vorheriges Kapitel
menu-next-chapter = Nächstes Kapitel
menu-faster = Schneller
menu-slower = Langsamer
menu-normal-speed = Normale Geschwindigkeit
menu-ab-loop = Schleife zwischen zwei Punkten
menu-volume-up = Lauter
menu-volume-down = Leiser
menu-mute = Stummschalten oder Ton einschalten
menu-next-subtitles = Nächste Untertitel
menu-next-audio = Nächste Audiospur
menu-window = Fenster
menu-minimize = Im Dock ablegen
menu-zoom = Zoomen
menu-bring-all-to-front = Alle nach vorne bringen

## Editing

action-edit = Bearbeiten
title-edited = { $name } (bearbeitet)
edit-trim = Trimmen
edit-trim-apply = Trimmen
edit-split = Clip teilen
edit-remove-clip = Clip entfernen
edit-done = Fertig
edit-rotate-left = Nach links drehen
edit-rotate-right = Nach rechts drehen
edit-flip-horizontal = Horizontal spiegeln
edit-flip-vertical = Vertikal spiegeln
edit-undo = Rückgängig
edit-redo = Wiederholen
edit-revert = Auf Original zurücksetzen
edit-export = Exportieren …

## Exporting

export-title = Exportieren
export-format = Format
export-size = Größe
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Nur Ton
export-size-original = { $lines }p, unverändert
export-on-graphics-card = Auf der Grafikkarte codiert ({ $api })
export-on-processor = Auf dem Prozessor codiert, was länger dauert
export-no-encoder = Für dieses Format lässt sich hier kein Encoder öffnen
export-checking = Encoder wird gesucht …
export-sound-format = Tonformat
export-quality = Qualität
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Komprimiert; läuft fast überall, auch auf Geräten von Apple
export-mp3-note = Komprimiert; läuft überall
export-opus-note = Komprimiert; der beste Klang für seine Größe, aber nicht jeder Player spielt es ab
export-flac-note = Verlustfrei: der Ton genau so, wie er ist, in einem offenen Format
export-alac-note = Verlustfrei: der Ton genau so, wie er ist, so wie Geräte von Apple ihn abspielen
export-wav-note = Unkomprimiert: der Ton genau so, wie er ist, in den größten Dateien
export-kilobits = { $rate } kb/s
export-bits = { $bits } Bit
export-rate-lower = Mit { $rate } kHz gespeichert, unter den { $file } kHz der Datei
export-choose = Exportieren …
export-name = { $name } Export
export-name-edited = { $name } bearbeitet
export-progress = „{ $name }“ wird exportiert
export-stop = Export abbrechen
notice-exported = „{ $name }“ exportiert
notice-export-stopped = Export abgebrochen
notice-export-failed = Export fehlgeschlagen: { $error }
close-title = Schließen, ohne zu exportieren?
close-unexported = Die Änderungen an „{ $name }“ gehen verloren, wenn Sie sie nicht exportieren.
close-exporting = Der Export von „{ $name }“ wird abgebrochen, und was bisher geschrieben wurde, geht verloren.
close-anyway = Schließen
close-quit-anyway = Beenden

## Recording

record-title = Tonaufnahme
record-file-name = Tonaufnahme
record-start = Aufnehmen
record-stop = Aufnahme beenden
record-no-input = Kein Mikrofon
record-microphone = Mikrofon
record-default-microphone = Systemstandard
record-playing = Was der Computer abspielt
record-mono = Mono
record-stereo = Stereo
record-channels = { $count ->
    [one] { $count } Kanal
   *[other] { $count } Kanäle
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, mit { $saved } kHz gespeichert
action-record-sound = Ton aufnehmen
open-record-sound = Neue Tonaufnahme
start-record-sound = Ton aufnehmen
menu-record-sound = Neue Tonaufnahme
record-screen-title = Bildschirmaufnahme
record-screen-file-name = Bildschirmaufnahme
record-screen-whole = Bildschirm
record-screen-number = Bildschirm { $number }
record-screen-main = Bildschirm { $number } (Hauptbildschirm)
record-screens = Bildschirme
record-windows = Fenster
record-screen-what = Was aufgenommen wird
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Jedes Bild
record-rate-tip = Bilder pro Sekunde
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } verworfen
record-screen-window = Fenster
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Nicht mehr freigegeben
record-screen-again = Neu wählen
record-screen-failed = Der Bildschirm konnte nicht aufgenommen werden
record-sound = Ton
record-no-sound = Kein Ton
# How much louder the input is taken, for a quiet microphone.
record-gain = Lauter
record-gain-value = +{ $decibels } dB
record-level-automatically = Pegel automatisch anpassen
record-hide-title = plaa wird während der Aufnahme ausgeblendet
record-hide-tray = Um die Aufnahme zu beenden, klicken Sie auf den roten Punkt, den plaa im Infobereich anzeigt, oder öffnen Sie plaa erneut.
record-hide-reopen = Um die Aufnahme zu beenden, öffnen Sie plaa erneut.
record-hide-start = Aufnahme starten
record-tray-title = plaa nimmt den Bildschirm auf
record-tray-tip = Klicken, um die Aufnahme zu beenden
action-record-screen = Bildschirm aufnehmen
open-record-screen = Neue Bildschirmaufnahme
start-record-screen = Bildschirm aufnehmen
menu-record-screen = Neue Bildschirmaufnahme
record-camera-title = Kameraaufnahme
record-camera-file-name = Kameraaufnahme
record-camera = Kamera
record-camera-failed = Die Kamera konnte nicht geöffnet werden
action-record-camera = Kamera aufnehmen
open-record-camera = Neue Kameraaufnahme
start-record-camera = Kamera aufnehmen
menu-record-camera = Neue Kameraaufnahme
notice-no-camera = Es gibt keine Kamera zum Aufnehmen
notice-recorded = „{ $name }“ in { $folder } gespeichert
notice-record-failed = Aufnahme fehlgeschlagen: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Steuerung von außen erlauben
settings-allow-outside-control-note = KI-Agenten wie Claude Code können über plaa --mcp sehen, was in plaa läuft, und es steuern. plaa fragt vor jedem neuen Agenten nach.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Erlaubt: { $agents }
settings-forget-agents = Vergessen
settings-ask-before-note = Vor diesen Aktionen eines Agenten nachfragen:
settings-ask-reading = Wiedergabe oder Bild ansehen
settings-ask-playing = Wiedergabe, Ton oder Vollbild steuern
settings-ask-opening = Datei öffnen oder Fenster schließen
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent } erlauben, plaa zu steuern?
agent-prompt-body = { $agent } möchte die Steuerung von außen in plaa nutzen, um zu sehen, was läuft, und es zu steuern. Sie können die Steuerung von außen in den Einstellungen ausschalten.
agent-prompt-allow = Erlauben
agent-prompt-deny = Nicht erlauben
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent } erlauben, zu sehen, was hier läuft?
agent-ask-play = { $agent } erlauben, die Wiedergabe hier zu steuern?
agent-ask-open = { $agent } erlauben, ein Fenster zu öffnen oder zu schließen?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } möchte „{ $tool }“ verwenden. In den Einstellungen legen Sie fest, wonach plaa fragt.

## Casting to a TV or speaker on the network

cast-looking = Fernseher und Lautsprecher werden gesucht …
cast-none = Es wurde kein Fernseher, Lautsprecher oder Chromecast gefunden.
# $renderer is the TV's or speaker's name.
cast-stop = Wiedergabe auf { $renderer } beenden
notice-casting = Wiedergabe auf { $renderer }.
notice-cast-failed = Wiedergabe auf { $renderer } fehlgeschlagen: { $reason }
cast-web-page = Webseiten lassen sich noch nicht streamen: Öffnen Sie stattdessen die Adresse des Videos selbst.

## The assistant panel, and its models in Settings

assistant-add-another = Modell hinzufügen …
assistant-add-model = Der Assistent braucht ein Modell: ein Cloud-Modell mit seinem API-Schlüssel oder ein lokales Modell.
assistant-ask = Fragen zur Wiedergabe
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Der Chat wurde zu lang, als dass { $model } ihn auf einmal lesen könnte, daher wurde sein früherer Teil zusammengefasst. Der Assistent sieht sich vielleicht noch einmal an, was er gesehen hat.
assistant-compacting = Der bisherige Chat wird zusammengefasst
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } auf diesem Computer
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } auf { $host }
assistant-new-chat = Neuer Chat
assistant-no-model = Fügen Sie zuerst in den Einstellungen ein Modell hinzu.
assistant-open-settings = Modell hinzufügen
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Das Gespräch passt nicht mehr in das, was { $model } auf einmal lesen kann. Beginnen Sie einen neuen Chat oder wählen Sie ein Modell, das mehr lesen kann.
assistant-problem-credit = { $provider } meldet, dass das Konto kein Guthaben hat. Für ein neues Konto muss erst Guthaben auf der Website von { $provider } gekauft werden, bevor der Schlüssel funktioniert; versuchen Sie es dann erneut.
assistant-problem-key = { $provider } hat den API-Schlüssel abgelehnt. Prüfen Sie ihn in den Einstellungen.
assistant-problem-model = { $provider } hat kein Modell namens { $model }. Prüfen Sie den Namen in den Einstellungen.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } hat nicht geantwortet: { $message }
assistant-problem-rate = { $provider } bittet darum, langsamer zu machen. Versuchen Sie es gleich noch einmal.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } bittet darum, langsamer zu machen: { $message }
assistant-problem-refused = { $model } hat eine Antwort abgelehnt.
assistant-problem-unavailable = { $provider } ist ausgelastet oder hat Probleme. Versuchen Sie es gleich noch einmal.
assistant-problem-unreachable = plaa konnte { $provider } nicht erreichen. Prüfen Sie Ihre Verbindung oder ob der Server läuft.
assistant-running = Läuft …
assistant-send = Senden
assistant-shortened = Der Chat war zu lang, als dass { $model } ihn auf einmal lesen könnte, daher wurden frühere Tool-Ergebnisse weggelassen. Der Assistent sieht sich vielleicht noch einmal an, was er gesehen hat.
assistant-stop = Stoppen
assistant-stopped = Gestoppt.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Jetzt im Gespräch mit { $model }.
assistant-thinking = Denkt nach …
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Gedanken
assistant-title = Assistent
settings-assistant-add = Modell hinzufügen
settings-assistant-add-button = Hinzufügen
# $model is the model's name.
settings-assistant-added = { $model } hat geantwortet und wurde hinzugefügt.
settings-assistant-added-tag = Hinzugefügt
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adresse, etwa { $example }
settings-assistant-change-key = Schlüssel ändern
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-kompatibler Server
# The menu of how much a local model reads at once.
settings-assistant-context = Kontext
settings-assistant-context-note = Mit mehr kann der Assistent mehr von einem Chat im Blick behalten, aber das Modell braucht mehr Arbeitsspeicher und antwortet womöglich langsamer.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K Tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Seite öffnen
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } läuft unter { $address }.
settings-assistant-found-none = { $provider } hat noch keine Modelle. Laden Sie damit eines herunter und suchen Sie dann erneut.
settings-assistant-get-key = API-Schlüssel holen
settings-assistant-get-server = { $provider } herunterladen
settings-assistant-in-use = Wird verwendet
settings-assistant-key = API-Schlüssel
# $error is what the keychain said.
settings-assistant-key-failed = Der Schlüssel konnte nicht im Schlüsselbund gespeichert werden: { $error }
settings-assistant-key-kept = Ihr Schlüssel für { $provider } wird im Schlüsselbund des Systems aufbewahrt.
settings-assistant-key-needed = Dieses Modell braucht einen API-Schlüssel.
settings-assistant-key-refused = { $provider } hat diesen Schlüssel abgelehnt. Prüfen Sie, ob er vollständig und aus dem richtigen Konto kopiert wurde.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Erstellen Sie auf der Website von { $provider } einen Schlüssel und fügen Sie ihn hier ein.
settings-assistant-look-again = Erneut suchen
settings-assistant-looking = Modelle werden gesucht …
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modell, etwa { $example }
settings-assistant-no-server = plaa hat auf diesem Computer kein laufendes { $provider } gefunden. Starten Sie es oder geben Sie unten seine Adresse an.
settings-assistant-no-tools = Kann keine Tools nutzen, die der Assistent braucht
settings-assistant-none = Noch keine Modelle. Fügen Sie unten eines hinzu: Ein lokales Modell, etwa das von Ollama, bleibt auf diesem Computer; ein Cloud-Modell braucht einen API-Schlüssel seines Anbieters.
settings-assistant-note = Die Modelle, mit denen der Assistent sprechen kann. Schlüssel werden im Schlüsselbund des Systems aufbewahrt.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Andere Adresse verwenden
settings-assistant-recommended = Empfohlen
settings-assistant-remove = Entfernen
settings-assistant-sees = Erkennt Bilder
settings-assistant-trying = Wird getestet …
settings-assistant-use = Verwenden
settings-assistant-use-key = Weiter
settings-assistant-uses-tools = Nutzt Tools
