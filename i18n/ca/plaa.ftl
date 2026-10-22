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
language-name = Català

## Common

app-name = plaa
common-cancel = Cancel·la
common-close = Tanca
common-more = Més

## Command line

usage-help =
    Ús: plaa [FITXER O ADREÇA]...

    Reprodueix fitxers de vídeo i d'àudio, i fluxos en adreces com ara
    https://example.com/live.m3u8. Cadascun s'obre en una finestra del plaa
    en execució, que s'inicia si cal.

    Opcions:
      -h, --help        Mostra aquesta ajuda
      -V, --version     Mostra la versió
          --interfaces  Mostra les interfícies amb cable, i si el plaa hi
                        pot enviar fluxos AVB
          --mcp         Serveix MCP per stdin i stdout, perquè els agents d'IA
                        vegin què reprodueix el plaa en execució i el controlin

## Start window

start-hint = Obre un fitxer de vídeo o d'àudio, un flux o un vídeo web.
start-open = Obre
start-open-address = Obre una adreça
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Connecta

## Actions, as buttons, tooltips and menus name them

action-open = Obre
action-open-address = Obre una adreça
open-file = Obre un fitxer…
open-address = Obre una adreça…
action-settings = Configuració
action-info = Inspector
action-loop = Bucle
action-screenshot = Desa el fotograma com a imatge
action-play = Reprodueix
action-pause = Posa en pausa
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Retrocedeix { $seconds } segon
   *[other] Retrocedeix { $seconds } segons
}
action-forward = { $seconds ->
    [one] Avança { $seconds } segon
   *[other] Avança { $seconds } segons
}
action-mute = Silencia
action-unmute = Activa el so
action-speed = Velocitat
action-tracks = Àudio i subtítols
action-full-screen = Pantalla completa
action-playlist = Llista de reproducció
action-chapters = Capítols
action-picture-in-picture = Imatge sobre imatge
action-previous = Anterior
action-next = Següent
action-exit-full-screen = Surt de la pantalla completa
action-quality = Qualitat
action-visualizer = Visualitzador
action-network = Àudio en xarxa
action-cast = Emet
action-connect = Connecta
action-assistant = Assistent

## What a sound file shows under its cover

visualizer-spectrum = Espectre
visualizer-waveform = Forma d'ona
visualizer-prism = Prisma
visualizer-cover = Només la portada

## The floating controls

overlay-keep-shown = Mantén els controls fixos
overlay-auto-hide = Fes flotar els controls sobre la imatge

## Audio and subtitle tracks

tracks-audio = Àudio
tracks-subtitles = Subtítols
tracks-off = Desactivat
# A track the file gives no name or language, by its number.
tracks-numbered = Pista { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Només àudio

## Inspector

info-title = Inspector
playlist-title = Llista de reproducció
chapters-numbered = Capítol { $number }
info-file = Fitxer
info-name = Nom
info-address = Adreça
info-folder = Carpeta
info-size = Mida
info-format = Format
info-duration = Durada
info-bit-rate = Taxa de bits
info-video = Vídeo
info-audio = Àudio
info-codec = Còdec
info-dimensions = Dimensions
info-frame-rate = Freqüència de fotogrames
info-frames-per-second = { $rate } fotogrames per segon
info-pixel-format = Format de píxel
info-decoding = Descodificació
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Targeta gràfica ({ $api })
info-decoding-software = Processador
info-sample-rate = Freqüència de mostreig
info-hertz = { $rate } Hz
info-channels = Canals
info-language = Llengua
info-no-sound = Sense so
info-tags = Etiquetes

## Dialogs

dialog-open = Obre
dialog-filter-media = Vídeo i àudio
dialog-filter-all = Tots els fitxers

address-title = Obre una adreça
address-note = Una pàgina web amb un vídeo, com ara la d'un vídeo de YouTube, o un flux o un fitxer a la xarxa, com ara https://example.com/live.m3u8.
address-label = Adreça
address-open = Obre
address-recent = Recents
address-forget = Elimina dels recents

## Notices

notice-dialog-failed = No s'ha pogut obrir el diàleg de fitxers: { $error }
notice-screenshot-saved = S'ha desat «{ $name }».
notice-screenshot-failed = No s'ha pogut desar la imatge: { $error }
notice-no-sound = Es reprodueix sense so: { $reason }
notice-loop-from = Bucle des de { $time }. Torna a prémer on hagi d'acabar.
notice-loop-between = Bucle de { $from } a { $to }.
notice-loop-off = Bucle desactivat.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Sembla una pàgina web. El plaa reprodueix els vídeos de les pàgines web amb el yt-dlp, que no està instal·lat.
error-empty-list = Aquesta llista no té res per reproduir.

## Settings

settings-title = Configuració
settings-tab-appearance = Aparença
settings-tab-playback = Reproducció
settings-tab-network = Xarxa
settings-tab-language = Llengua
settings-tab-agents = Agents
settings-tab-assistant = Assistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versió de desenvolupament, { $build })
settings-choose = Tria…

settings-appearance-system = Segueix el sistema
settings-appearance-light = Clar
settings-appearance-dark = Fosc
settings-colors = Colors
settings-system-accent = Utilitza el color d'accent del sistema
settings-omarchy-note = Els colors es creen a partir de l'accent de «{ $theme }».
settings-system-accent-note = Els colors es creen a partir del color d'accent del sistema.
settings-system-accent-none = El sistema no té cap color d'accent, de manera que el plaa utilitza el que s'ha triat a continuació.
settings-accent-chosen-note = Els colors es creen a partir del color triat a continuació.
settings-controls = Controls
settings-overlay = Controls flotants
settings-overlay-note = La barra d'eines i els controls de reproducció suren sobre la imatge o la portada i s'amaguen mentre el punter està quiet o és fora de la finestra.
settings-overlay-delay = Amaga després de
settings-overlay-delay-note = Quant de temps ha d'estar quiet el punter sobre el que es reprodueix perquè els controls s'amaguin.
settings-seconds = { $seconds } s
settings-transparency = Transparència
settings-transparency-note = Quanta part de la imatge es veu a través dels controls flotants.
settings-percent = { $percent } %
settings-corner-radius = Radi de les cantonades
settings-corner-radius-note = Per als diàlegs i els controls flotants.
settings-pixels = { $pixels } px
settings-animations = Animacions
settings-animations-note = Barres i panells que llisquen, diàlegs que creixen i botons elàstics.
settings-animations-reduced = Desactivades mentre el sistema demani moviment reduït.

settings-hardware = Utilitza la targeta gràfica
settings-hardware-note = Descodifica el vídeo i codifica les exportacions amb la targeta gràfica: consumeix menys energia, reprodueix vídeos grans amb fluïdesa i exporta més de pressa. S'aplica als fitxers que s'obrin després.
settings-audio-device = Sortida de so
settings-audio-device-system = Predeterminada del sistema
settings-resume = Continua on ho vas deixar
settings-resume-note = Els fitxers comencen on eren quan es van tancar per última vegada.
settings-folder-playlists = Reprodueix els altres fitxers de la carpeta
settings-folder-playlists-note = En obrir un fitxer, els altres de la mateixa carpeta s'afegeixen a la llista i es reprodueixen després.
settings-streams = Fluxos i vídeos web
settings-stream-quality = Qualitat
settings-stream-quality-note = Per als fluxos i vídeos web que es poden obtenir en diverses qualitats. S'aplica al que s'obri després.
settings-quality-auto = Automàtica, tan alta com la pantalla
settings-quality-best = Màxima
settings-web-pages = Reprodueix els vídeos web amb el yt-dlp
settings-web-pages-found = El yt-dlp troba el vídeo en pàgines com les de YouTube.
settings-web-pages-missing = El yt-dlp no està instal·lat. Instal·la'l per reproduir els vídeos de pàgines com les de YouTube.
settings-folders = Carpetes
settings-recordings = Enregistraments
settings-screenshots = Fotogrames desats

settings-interface-language = Llengua de la interfície
# The first entry of the language list.
settings-language-system = Segueix el sistema
settings-track-languages = Llengües de les pistes
settings-track-languages-note = Codis de llengua com ara en o fra, primer els més preferits, separats per comes. El plaa tria l'àudio i els subtítols en aquestes llengües quan un fitxer en té.
settings-audio-languages = Àudio
settings-subtitle-languages = Subtítols

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interfície AVB
settings-avb-interface-note = El port amb cable on el plaa troba emissors i receptors AVB i Milan. Per enviar cal un port amb rellotge PTP de maquinari, com el d'una Intel I210.
settings-avb-off = Desactivada
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (només visualització: sense rellotge PTP)
settings-avb-off-mac = { $interface } (només visualització: l'AVB està desactivat)
settings-avb-interface-note-mac = El port amb cable on el plaa troba emissors i receptors AVB i Milan. L'enviament passa per l'entitat AVB pròpia d'aquest Mac, que necessita l'AVB activat per al port al navegador de dispositius de xarxa de la Configuració d'Àudio MIDI.
settings-avb-talker-note-mac = El plaa envia el so de l'última finestra que ha reproduït a través de l'entitat AVB pròpia d'aquest Mac, als receptors que hi estan connectats.
settings-avb-talker = Envia el que es reprodueix
settings-avb-talker-note = El plaa es converteix en un emissor Milan i envia el so de l'última finestra que ha reproduït als receptors que hi estan connectats.
settings-avb-name = Nom a la xarxa
settings-avb-name-note = Els controladors mostren el plaa com a «{ $name }». Prem Enter per canviar-li el nom.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa a { $computer }
avb-could-not-start = No s'ha pogut utilitzar { $interface }: { $error }
network-title = Àudio en xarxa
network-off = Activa «Envia el que es reprodueix» a Configuració per reproduir a la xarxa.
network-blocked = No s'envia: { $reason }
network-waiting = A punt per enviar, s'espera que es connecti un receptor.
network-sending = S'envia als receptors connectats.
network-looking = S'estan cercant receptors…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = S'està iniciant el ptp4l, que manté l'hora de xarxa amb què s'envia el flux del plaa…
network-ptp-elsewhere = El ptp4l s'executa en una altra interfície, de manera que el flux del plaa no té hora de xarxa aquí i és possible que els receptors no el puguin reproduir. Tria aquella interfície a Configuració, o executa el ptp4l en aquesta.
network-ptp-missing = El ptp4l no s'està executant, de manera que el flux del plaa no té hora de xarxa i és possible que els receptors no el puguin reproduir. Instal·la el linuxptp amb el paquet del plaa, o el triib, que l'executa.
network-ptp-failed = El ptp4l no s'està executant, de manera que el flux del plaa no té hora de xarxa i és possible que els receptors no el puguin reproduir. El plaa no l'ha pogut iniciar: { $reason }
network-no-listeners = Cap receptor de la xarxa no pot reproduir el so del plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Reprodueix a { $listener }
network-stop-on = Deixa de reproduir a { $listener }
network-choose-format = Tria un format per reproduir a { $listener }
network-failed = No s'ha pogut connectar: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Actualitza el triib per reproduir aquí.
network-choose = Tria un receptor per veure com rep el flux.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Escolta aquí
network-no-sources = Cap emissor de la xarxa no envia en un format que el plaa reprodueix, ni pot passar a un d'aquests formats.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Escolta { $talker }
network-listen-stop = Deixa d'escoltar { $talker }
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = Canvia { $talker } a { $format } i escolta
network-listen-change-cuts-off = Canvia { $talker } a { $format } i escolta, desconnectant { $count ->
        [one] el seu altre receptor
       *[other] els seus altres { $count } receptors
    }
network-listen-frames = Trames rebudes: { $frames }
network-listen-update-triib = Actualitza el triib per escoltar aquí: el plaa només reprodueix el que arriba de totes maneres a aquest ordinador.
network-listen-lite = El port fa servir AVB Lite, que no reserva fluxos: el plaa només reprodueix el que arriba de totes maneres a aquest ordinador.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = S'estan cercant emissors…
# plaa listens through its entity, which exists only while it sends.
connect-off = Activa «Envia el que es reprodueix» a Configuració per escoltar emissors aquí.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = El plaa no pot escoltar aquí: { $reason }
connect-sending = Envien en un format que el plaa reprodueix
connect-changing = Canvien a un format que el plaa reprodueix
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, en { $format }
connect-change-cuts-off = { $talker }, en { $format }, desconnectant { $count ->
        [one] el seu altre receptor
       *[other] els seus altres { $count } receptors
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Estat
network-not-connected = No connectat
network-from = De { $stream }
network-from-receiving = De { $stream }, rebent
network-from-waiting = De { $stream }, esperant l'emissor
network-from-failed = De { $stream }, ha fallat la reserva de l'emissor: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Rellotge multimèdia
network-locked = Sincronitzat amb el flux
network-not-locked = No sincronitzat
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formats que pot rebre del plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID del flux
# The MAC address the stream is sent to.
network-destination = Destinació
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latència
network-microseconds = { $time } µs
network-frames = Trames rebudes
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Trames fora de seqüència
# How often the stream stopped coming while it was taken.
network-interrupted = Interrupcions
network-entity-id = ID de l'entitat

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = amplada de banda insuficient
msrp-failure-2 = recursos insuficients al commutador
msrp-failure-3 = amplada de banda insuficient per a la classe de trànsit
msrp-failure-4 = un altre emissor utilitza l'ID del flux
msrp-failure-5 = l'adreça de destinació ja està en ús
msrp-failure-6 = desplaçat per un flux de rang superior
msrp-failure-7 = la latència informada ha canviat
msrp-failure-8 = el port de sortida no admet AVB
msrp-failure-9 = utilitza una adreça de destinació diferent
msrp-failure-10 = no queden recursos MSRP
msrp-failure-11 = no queden recursos MMRP
msrp-failure-12 = no es pot emmagatzemar l'adreça de destinació
msrp-failure-13 = la prioritat no és una prioritat de classe SR
msrp-failure-14 = trames massa grans per al medi
msrp-failure-15 = s'ha arribat al límit de ports fan-in
msrp-failure-16 = ha canviat el primer valor d'un flux registrat
msrp-failure-17 = VLAN bloquejada al port de sortida
msrp-failure-18 = etiquetatge VLAN desactivat al port de sortida
msrp-failure-19 = les prioritats de classe SR no coincideixen
msrp-failure-unknown = motiu desconegut
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, al commutador { $bridge }

## The macOS menu bar

menu-about = Quant al plaa
menu-settings = Configuració…
menu-services = Serveis
menu-hide = Amaga el plaa
menu-hide-others = Amaga els altres
menu-show-all = Mostra-ho tot
menu-quit = Surt del plaa
menu-file = Fitxer
menu-open = Obre…
menu-open-address = Obre una adreça…
menu-close = Tanca la finestra
menu-edit = Edició
menu-cut = Retalla
menu-copy = Copia
menu-paste = Enganxa
menu-select-all = Selecciona-ho tot
menu-undo = Desfés
menu-redo = Refés
menu-trim = Escurça…
menu-split = Divideix el clip
menu-rotate-left = Gira a l'esquerra
menu-rotate-right = Gira a la dreta
menu-flip-horizontal = Inverteix horitzontalment
menu-flip-vertical = Inverteix verticalment
menu-revert = Torna a l'original
menu-export = Exporta…
menu-view = Visualització
menu-playback = Reproducció
menu-play-pause = Reprodueix o posa en pausa
menu-to-start = Vés a l'inici
menu-previous-chapter = Capítol anterior
menu-next-chapter = Capítol següent
menu-faster = Més ràpid
menu-slower = Més lent
menu-normal-speed = Velocitat normal
menu-ab-loop = Bucle entre dos punts
menu-volume-up = Més fort
menu-volume-down = Més fluix
menu-mute = Silencia o activa el so
menu-next-subtitles = Subtítols següents
menu-next-audio = Pista d'àudio següent
menu-window = Finestra
menu-minimize = Minimitza
menu-zoom = Redimensiona
menu-bring-all-to-front = Porta-ho tot al davant

## Editing

action-edit = Edita
title-edited = { $name } (editat)
edit-trim = Escurça
edit-trim-apply = Escurça
edit-split = Divideix el clip
edit-remove-clip = Elimina el clip
edit-done = Fet
edit-rotate-left = Gira a l'esquerra
edit-rotate-right = Gira a la dreta
edit-flip-horizontal = Inverteix horitzontalment
edit-flip-vertical = Inverteix verticalment
edit-undo = Desfés
edit-redo = Refés
edit-revert = Torna a l'original
edit-export = Exporta…

## Exporting

export-title = Exporta
export-format = Format
export-size = Mida
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Només so
export-size-original = { $lines }p, tal com és
export-on-graphics-card = Codificat amb la targeta gràfica ({ $api })
export-on-processor = Codificat amb el processador, que triga més
export-no-encoder = Aquí no s'obre cap codificador per a aquest format
export-checking = S'està cercant un codificador…
export-sound-format = Format de so
export-quality = Qualitat
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Comprimit; es reprodueix gairebé a tot arreu, també als dispositius d'Apple
export-mp3-note = Comprimit; es reprodueix a tot arreu
export-opus-note = Comprimit; el millor so per a la seva mida, tot i que no tots els reproductors l'admeten
export-flac-note = Sense pèrdua: el so exactament tal com és, en un format obert
export-alac-note = Sense pèrdua: el so exactament tal com és, tal com el reprodueixen els dispositius d'Apple
export-wav-note = Sense comprimir: el so exactament tal com és, en els fitxers més grans
export-kilobits = { $rate } kb/s
export-bits = { $bits } bits
export-rate-lower = Desat a { $rate } kHz, per sota dels { $file } kHz del fitxer
export-choose = Exporta…
export-name = { $name } exportat
export-name-edited = { $name } editat
export-progress = S'està exportant «{ $name }»
export-stop = Atura l'exportació
notice-exported = S'ha exportat «{ $name }»
notice-export-stopped = S'ha aturat l'exportació
notice-export-failed = No s'ha pogut exportar: { $error }
close-title = Vols tancar sense exportar?
close-unexported = Els canvis a «{ $name }» es perden si no els exportes.
close-exporting = L'exportació de «{ $name }» s'atura, i el que s'ha escrit fins ara es perd.
close-anyway = Tanca
close-quit-anyway = Surt

## Recording

record-title = Enregistrament de so
record-file-name = Enregistrament de so
record-start = Enregistra
record-stop = Atura l'enregistrament
record-no-input = Cap micròfon
record-microphone = Micròfon
record-default-microphone = El predeterminat del sistema
record-playing = El que reprodueix l'ordinador
record-mono = mono
record-stereo = estèreo
record-channels = { $count ->
    [one] { $count } canal
   *[other] { $count } canals
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, desat a { $saved } kHz
action-record-sound = Enregistra so
open-record-sound = Enregistrament de so nou
start-record-sound = Enregistra so
menu-record-sound = Enregistrament de so nou
record-screen-title = Enregistrament de pantalla
record-screen-file-name = Enregistrament de pantalla
record-screen-whole = Pantalla
record-screen-number = Pantalla { $number }
record-screen-main = Pantalla { $number } (principal)
record-screens = Pantalles
record-windows = Finestres
record-screen-what = Què s'ha d'enregistrar
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Tots els fotogrames
record-rate-tip = Fotogrames per segon
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } descartat
   *[other] { $size } · { $count } descartats
}
record-screen-window = Finestra
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Ja no es comparteix
record-screen-again = Torna a triar
record-screen-failed = No s'ha pogut enregistrar la pantalla
record-sound = So
record-no-sound = Sense so
# How much louder the input is taken, for a quiet microphone.
record-gain = Guany
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajusta el nivell automàticament
record-hide-title = El plaa s'amaga mentre enregistra
record-hide-tray = Per aturar l'enregistrament, fes clic al punt vermell que el plaa posa a la safata del sistema, o torna a obrir el plaa.
record-hide-reopen = Per aturar l'enregistrament, torna a obrir el plaa.
record-hide-start = Comença a enregistrar
record-tray-title = El plaa està enregistrant la pantalla
record-tray-tip = Fes clic per aturar l'enregistrament
action-record-screen = Enregistra la pantalla
open-record-screen = Enregistrament de pantalla nou
start-record-screen = Enregistra la pantalla
menu-record-screen = Enregistrament de pantalla nou
record-camera-title = Enregistrament de càmera
record-camera-file-name = Enregistrament de càmera
record-camera = Càmera
record-camera-failed = No s'ha pogut obrir la càmera
action-record-camera = Enregistra amb la càmera
open-record-camera = Enregistrament de càmera nou
start-record-camera = Enregistra amb la càmera
menu-record-camera = Enregistrament de càmera nou
notice-no-camera = No hi ha cap càmera per enregistrar
notice-recorded = S'ha desat «{ $name }» a { $folder }
notice-record-failed = No s'ha pogut enregistrar: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Permet el control extern
settings-allow-outside-control-note = Els agents d'IA com ara Claude Code poden veure què es reprodueix al plaa i controlar-lo mitjançant plaa --mcp. El plaa pregunta abans de cada agent nou.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permesos: { $agents }
settings-forget-agents = Oblida
settings-ask-before-note = Pregunta abans que un agent:
settings-ask-reading = Vegi què es reprodueix, o la imatge
settings-ask-playing = Controli la reproducció, el so o la pantalla completa
settings-ask-opening = Obri un fitxer o tanqui una finestra
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vols permetre que { $agent } controli el plaa?
agent-prompt-body = { $agent } demana fer servir el control extern del plaa per veure què es reprodueix i controlar-lo. Pots desactivar el control extern a Configuració.
agent-prompt-allow = Permet
agent-prompt-deny = No permetis
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Vols permetre que { $agent } vegi què es reprodueix aquí?
agent-ask-play = Vols permetre que { $agent } controli la reproducció aquí?
agent-ask-open = Vols permetre que { $agent } obri o tanqui una finestra?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } demana fer servir «{ $tool }». A Configuració pots triar què pregunta el plaa.

## Casting to a TV or speaker on the network

cast-looking = S'estan cercant televisors i altaveus…
cast-none = No s'ha trobat cap televisor, altaveu ni Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Deixa de reproduir a { $renderer }
notice-casting = Es reprodueix a { $renderer }.
notice-cast-failed = No s'ha pogut reproduir a { $renderer }: { $reason }
cast-web-page = Encara no es poden emetre pàgines web: obre en lloc d'això l'adreça del mateix vídeo.

## The assistant panel, and its models in Settings

assistant-add-another = Afegeix un model…
assistant-add-model = L'assistent necessita un model: un model al núvol amb la seva clau d'API, o un model local.
assistant-ask = Pregunta sobre el que es reprodueix
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = El xat començava a ser massa llarg perquè { $model } el llegís d'una vegada, així que se n'ha resumit la part anterior. L'assistent pot tornar a mirar el que ha vist.
assistant-compacting = S'està resumint la part anterior del xat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } en aquest ordinador
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } a { $host }
assistant-new-chat = Xat nou
assistant-no-model = Primer afegeix un model a Configuració.
assistant-open-settings = Afegeix un model
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = La conversa ja no cap en el que { $model } pot llegir alhora. Comença un xat nou o tria un model que pugui llegir més.
assistant-problem-credit = { $provider } diu que el compte no té crèdit. Un compte nou n'ha de comprar a { $provider } perquè la clau funcioni; després torna-ho a provar.
assistant-problem-key = { $provider } ha rebutjat la clau d'API. Revisa-la a Configuració.
assistant-problem-model = { $provider } no té cap model anomenat { $model }. Comprova'n el nom a Configuració.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } no ha respost: { $message }
assistant-problem-rate = { $provider } demana anar més a poc a poc. Torna-ho a provar d'aquí a un moment.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } demana anar més a poc a poc: { $message }
assistant-problem-refused = { $model } ha declinat respondre.
assistant-problem-unavailable = { $provider } està ocupat o té problemes. Torna-ho a provar d'aquí a un moment.
assistant-problem-unreachable = El plaa no ha pogut connectar amb { $provider }. Comprova la connexió o que el servidor s'estigui executant.
assistant-running = S'està executant…
assistant-send = Envia
assistant-shortened = El xat era massa llarg perquè { $model } el llegís d'una vegada, així que s'han deixat fora els resultats anteriors de les eines. L'assistent pot tornar a mirar el que ha vist.
assistant-stop = Atura
assistant-stopped = Aturat.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Ara parles amb { $model }.
assistant-thinking = S'està pensant…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Raonament
assistant-title = Assistent
settings-assistant-add = Afegeix un model
settings-assistant-add-button = Afegeix
# $model is the model's name.
settings-assistant-added = { $model } ha respost i s'ha afegit.
settings-assistant-added-tag = Afegit
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adreça, com ara { $example }
settings-assistant-change-key = Canvia la clau
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Servidor compatible amb OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Context
settings-assistant-context-note = Més context permet que l'assistent tingui present una part més gran d'un xat, però el model ocupa més memòria i pot respondre més a poc a poc.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Obre la pàgina
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } s'està executant a { $address }.
settings-assistant-found-none = { $provider } encara no té cap model. Baixa'n un amb { $provider } i torna a cercar.
settings-assistant-get-key = Obtén una clau d'API
settings-assistant-get-server = Obtén { $provider }
settings-assistant-in-use = En ús
settings-assistant-key = Clau d'API
# $error is what the keychain said.
settings-assistant-key-failed = No s'ha pogut desar la clau al clauer: { $error }
settings-assistant-key-kept = La teva clau per a { $provider } es desa al clauer del sistema.
settings-assistant-key-needed = Aquest model necessita una clau d'API.
settings-assistant-key-refused = { $provider } ha rebutjat aquesta clau. Comprova que s'ha copiat sencera i del compte correcte.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crea una clau a { $provider } i enganxa-la aquí.
settings-assistant-look-again = Torna a cercar
settings-assistant-looking = S'estan cercant models…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, com ara { $example }
settings-assistant-no-server = El plaa no ha trobat { $provider } en execució en aquest ordinador. Inicia'l o indica'n l'adreça a sota.
settings-assistant-no-tools = No pot fer servir eines, i l'assistent les necessita
settings-assistant-none = Encara no hi ha cap model. Afegeix-ne un a sota: un model local, com els d'Ollama, es queda en aquest ordinador; un model al núvol necessita una clau d'API del seu proveïdor.
settings-assistant-note = Els models amb què pot parlar el plafó de l'assistent. Les claus es desen al clauer del sistema.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Fes servir una altra adreça
settings-assistant-recommended = Recomanat
settings-assistant-remove = Elimina
settings-assistant-sees = Veu imatges
settings-assistant-trying = S'està provant…
settings-assistant-use = Fes servir
settings-assistant-use-key = Continua
settings-assistant-uses-tools = Fa servir eines
