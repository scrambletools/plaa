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
language-name = Italiano

## Common

app-name = plaa
common-cancel = Annulla
common-close = Chiudi
common-more = Altro

## Command line

usage-help =
    Uso: plaa [FILE O INDIRIZZO]...

    Riproduce file video e audio, e stream a indirizzi come
    https://example.com/live.m3u8. Ognuno si apre in una finestra di plaa in
    esecuzione, che si avvia se necessario.

    Opzioni:
      -h, --help        Mostra questo aiuto
      -V, --version     Mostra la versione
          --interfaces  Mostra le interfacce cablate e se plaa può inviarvi
                        stream AVB
          --mcp         Serve MCP su stdin e stdout, perché gli agenti IA vedano
                        cosa riproduce plaa in esecuzione e lo controllino

## Start window

start-hint = Apri un file video o audio, uno stream o un video web.
start-open = Apri
start-open-address = Apri indirizzo
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Collega

## Actions, as buttons, tooltips and menus name them

action-open = Apri
action-open-address = Apri indirizzo
open-file = Apri file…
open-address = Apri indirizzo…
action-settings = Impostazioni
action-info = Inspector
action-loop = Loop
action-screenshot = Salva fotogramma come immagine
action-play = Riproduci
action-pause = Pausa
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Indietro di { $seconds } secondo
   *[other] Indietro di { $seconds } secondi
}
action-forward = { $seconds ->
    [one] Avanti di { $seconds } secondo
   *[other] Avanti di { $seconds } secondi
}
action-mute = Disattiva audio
action-unmute = Riattiva audio
action-speed = Velocità
action-tracks = Audio e sottotitoli
action-full-screen = Schermo intero
action-playlist = Playlist
action-chapters = Capitoli
action-picture-in-picture = Picture in Picture
action-previous = Precedente
action-next = Successivo
action-exit-full-screen = Esci da schermo intero
action-quality = Qualità
action-visualizer = Visualizzatore
action-network = Audio in rete
action-cast = Trasmetti
action-connect = Collega
action-assistant = Assistente

## What a sound file shows under its cover

visualizer-spectrum = Spettro
visualizer-waveform = Forma d'onda
visualizer-prism = Prisma
visualizer-cover = Solo copertina

## The floating controls

overlay-keep-shown = Mantieni i controlli ancorati
overlay-auto-hide = Fai fluttuare i controlli sopra l'immagine

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Sottotitoli
tracks-off = Disattivati
# A track the file gives no name or language, by its number.
tracks-numbered = Traccia { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Solo audio

## Inspector

info-title = Inspector
playlist-title = Playlist
chapters-numbered = Capitolo { $number }
info-file = File
info-name = Nome
info-address = Indirizzo
info-folder = Cartella
info-size = Dimensioni
info-format = Formato
info-duration = Durata
info-bit-rate = Bitrate
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Risoluzione
info-frame-rate = Frequenza fotogrammi
info-frames-per-second = { $rate } fotogrammi al secondo
info-pixel-format = Formato pixel
info-decoding = Decodifica
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Scheda grafica ({ $api })
info-decoding-software = Processore
info-sample-rate = Frequenza di campionamento
info-hertz = { $rate } Hz
info-channels = Canali
info-language = Lingua
info-no-sound = Nessun audio
info-tags = Tag
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = Altri listener
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Classe SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Un nuovo formato scollega { $count ->
        [one] l'altro listener del talker.
       *[other] gli altri { $count } listener del talker.
    }

## Dialogs

dialog-open = Apri
dialog-filter-media = Video e audio
dialog-filter-all = Tutti i file

address-title = Apri indirizzo
address-note = Una pagina web con un video, come quella di un video di YouTube, oppure uno stream o un file in rete, come https://example.com/live.m3u8.
address-label = Indirizzo
address-open = Apri
address-recent = Recenti
address-forget = Rimuovi dai recenti

## Notices

notice-dialog-failed = Impossibile aprire la finestra dei file: { $error }
notice-screenshot-saved = “{ $name }” salvato.
notice-screenshot-failed = Impossibile salvare l'immagine: { $error }
notice-no-sound = Riproduzione senza audio: { $reason }
notice-loop-from = Loop da { $time }. Premi di nuovo dove deve finire.
notice-loop-between = Loop da { $from } a { $to }.
notice-loop-off = Loop disattivato.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Sembra una pagina web. plaa riproduce i video delle pagine web con yt-dlp, che non è installato.
error-empty-list = Questo elenco non contiene niente da riprodurre.

## Settings

settings-title = Impostazioni
settings-tab-appearance = Aspetto
settings-tab-playback = Riproduzione
settings-tab-storage = Archiviazione
settings-tab-network = Rete
settings-tab-language = Lingua
settings-tab-agents = Agenti
settings-tab-assistant = Assistente
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versione di sviluppo, { $build })
settings-choose = Scegli…

settings-appearance-system = Segui il sistema
settings-appearance-light = Chiaro
settings-appearance-dark = Scuro
settings-colors = Colori
settings-system-accent = Usa il colore principale del sistema
settings-omarchy-note = I colori derivano dal colore principale di “{ $theme }”.
settings-system-accent-note = I colori derivano dal colore principale del sistema.
settings-system-accent-none = Il sistema non ha un colore principale, quindi plaa usa quello scelto qui sotto.
settings-accent-chosen-note = I colori derivano dal colore scelto qui sotto.
settings-controls = Controlli
settings-overlay = Controlli fluttuanti
settings-overlay-note = La barra degli strumenti e i controlli di riproduzione fluttuano sopra l'immagine o la copertina e scivolano via mentre il puntatore è fermo o fuori dalla finestra.
settings-overlay-delay = Nascondi dopo
settings-overlay-delay-note = Quanto deve restare fermo il puntatore su ciò che viene riprodotto prima che i controlli scivolino via.
settings-seconds = { $seconds } s
settings-transparency = Trasparenza
settings-transparency-note = Quanto dell'immagine si vede attraverso i controlli fluttuanti.
settings-percent = { $percent }%
settings-corner-radius = Raggio degli angoli
settings-corner-radius-note = Per le finestre di dialogo e i controlli fluttuanti.
settings-pixels = { $pixels } px
settings-animations = Animazioni
settings-animations-note = Barre e pannelli che scorrono, finestre che si espandono e pulsanti elastici.
settings-animations-reduced = Disattivate finché il sistema richiede di ridurre il movimento.

settings-hardware = Usa la scheda grafica
settings-hardware-note = Decodifica i video e codifica le esportazioni sulla scheda grafica: consuma meno energia, riproduce i video grandi in modo fluido ed esporta più velocemente. Vale per i file aperti in seguito.
settings-audio-device = Uscita audio
settings-audio-device-system = Predefinita di sistema
settings-resume = Riprendi da dove avevi interrotto
settings-resume-note = I file ripartono da dove erano quando sono stati chiusi l'ultima volta.
settings-folder-playlists = Riproduci gli altri file della cartella
settings-folder-playlists-note = Aprendo un file, gli altri della sua cartella vengono messi in elenco e riprodotti dopo.
settings-streams = Stream e video web
settings-stream-quality = Qualità
settings-stream-quality-note = Per stream e video web disponibili in più qualità. Vale per ciò che si apre in seguito.
settings-quality-auto = Automatica, alta quanto lo schermo
settings-quality-best = Massima
settings-web-pages = Riproduci i video web con yt-dlp
settings-web-pages-found = yt-dlp trova il video su pagine come quelle di YouTube.
settings-web-pages-missing = yt-dlp non è installato. Installalo per riprodurre i video su pagine come quelle di YouTube.
settings-default-app = App predefinita
settings-default-app-label = Apri i file con plaa
settings-default-app-note = Rendi plaa l’app che apre i file video e audio.
settings-default-app-note-windows = Windows permette di scegliere le app predefinite solo nelle sue Impostazioni. Questo apre lì la pagina di plaa.
settings-default-app-note-macos = macOS chiede di confermare ogni tipo: MP4, MOV, MKV, WebM, MP3, M4A, FLAC e WAV.
settings-default-app-status = { $set } tipi di file su { $total } si aprono con plaa.
settings-default-app-button = Imposta come predefinita
settings-default-app-button-windows = Apri Impostazioni
settings-default-app-no-entry = La voce desktop di plaa non è installata, quindi il sistema non può aprire file con essa. Installa plaa da un pacchetto.
settings-default-app-no-bundle = Apri plaa da plaa.app per impostarla come predefinita.
settings-default-app-failed = Impossibile impostare plaa come predefinita: { $error }
settings-folders = Cartelle
settings-sound-recordings = Registrazioni audio
settings-video-recordings = Registrazioni video
settings-screenshots = Fotogrammi salvati

settings-interface-language = Lingua dell'interfaccia
# The first entry of the language list.
settings-language-system = Segui il sistema
settings-track-languages = Lingue delle tracce
settings-track-languages-note = Codici di lingua come en o fra, in ordine di preferenza, separati da virgole. plaa sceglie audio e sottotitoli in queste lingue quando un file li ha.
settings-audio-languages = Audio
settings-subtitle-languages = Sottotitoli

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interfaccia AVB
settings-avb-interface-note = La porta cablata su cui plaa trova talker e listener AVB e Milan. Per inviare serve una porta con un clock hardware PTP, come quella di una Intel I210.
settings-avb-off = Disattivata
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (solo elenco: nessun clock PTP)
settings-avb-off-mac = { $interface } (solo elenco: AVB disattivato)
settings-avb-interface-note-mac = La porta cablata su cui plaa trova talker e listener AVB e Milan. L'invio passa per l'entità AVB propria di questo Mac, che richiede AVB attivato per la porta nel browser dei dispositivi di rete di Configurazione MIDI Audio.
settings-avb-talker-note-mac = plaa invia il suono dell'ultima finestra riprodotta tramite l'entità AVB propria di questo Mac ai listener collegati a essa.
settings-avb-talker = Invia ciò che viene riprodotto
settings-avb-talker-note = plaa diventa un talker Milan e invia il suono dell'ultima finestra riprodotta ai listener collegati.
settings-avb-name = Nome in rete
settings-avb-name-note = I controller mostrano plaa come “{ $name }”. Premi Enter per rinominarlo.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa su { $computer }
avb-could-not-start = Impossibile usare { $interface }: { $error }
network-title = Audio in rete
network-off = Attiva “Invia ciò che viene riprodotto” in Impostazioni per riprodurre in rete.
network-blocked = Invio non attivo: { $reason }
network-waiting = Pronto per l'invio, in attesa che si colleghi un listener.
network-sending = Invio ai listener collegati.
network-looking = Ricerca dei listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Avvio di ptp4l, che mantiene l'ora di rete con cui viene inviato lo stream di plaa…
network-ptp-elsewhere = ptp4l è in esecuzione su un'altra interfaccia, quindi qui lo stream di plaa non ha un'ora di rete e i listener potrebbero non riprodurlo. Scegli quell'interfaccia in Impostazioni, oppure esegui ptp4l su questa.
network-ptp-missing = ptp4l non è in esecuzione, quindi lo stream di plaa non ha un'ora di rete e i listener potrebbero non riprodurlo. Installa linuxptp con il pacchetto di plaa, oppure installa triib, che lo esegue.
network-ptp-failed = ptp4l non è in esecuzione, quindi lo stream di plaa non ha un'ora di rete e i listener potrebbero non riprodurlo. plaa non è riuscito ad avviarlo: { $reason }
network-no-listeners = Nessun listener in rete può riprodurre il suono di plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Riproduci su { $listener }
network-stop-on = Interrompi la riproduzione su { $listener }
network-choose-format = Scegli un formato per riprodurre su { $listener }
network-failed = Impossibile collegarsi: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Aggiorna triib per riprodurre qui.
network-choose = Scegli un listener per vedere come riceve il suo stream.
network-no-sources = Nessun talker in rete invia in un formato che plaa riproduce, né può passare a uno di questi.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Smetti di ascoltare { $talker }
network-listen-update-triib = Aggiorna triib per ascoltare qui: plaa riproduce solo ciò che arriva comunque a questo computer.
network-listen-lite = La porta usa AVB Lite, che non prenota stream: plaa riproduce solo ciò che arriva comunque a questo computer.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Ricerca dei talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = Attiva “Invia ciò che viene riprodotto” in Impostazioni per ascoltare i talker qui.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa non può ascoltare qui: { $reason }
connect-sending = Inviano in un formato che plaa riproduce
connect-changing = Passano a un formato che plaa riproduce
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, in { $format }
connect-change-cuts-off = { $talker }, in { $format }, scollegando { $count ->
        [one] il suo altro listener
       *[other] i suoi altri { $count } listener
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Stato
network-not-connected = Non collegato
network-from = Da { $stream }
network-from-receiving = Da { $stream }, in ricezione
network-from-waiting = Da { $stream }, in attesa del talker
network-from-failed = Da { $stream }, la prenotazione del talker non è riuscita: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Clock media
network-locked = Agganciato allo stream
network-not-locked = Non agganciato
network-format = Formato
# The formats both the listener and plaa's stream can be in.
network-formats = Formati che può ricevere da plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID stream
# The MAC address the stream is sent to.
network-destination = Destinazione
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latenza
network-microseconds = { $time } µs
network-frames = Frame ricevuti
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Frame fuori sequenza
# How often the stream stopped coming while it was taken.
network-interrupted = Interruzioni
# Frames that came after, or long before, their time to be played.
network-late = Frame in ritardo
network-early = Frame in anticipo
network-entity-id = ID entità

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = larghezza di banda insufficiente
msrp-failure-2 = risorse del bridge insufficienti
msrp-failure-3 = larghezza di banda insufficiente per la classe di traffico
msrp-failure-4 = ID stream in uso da parte di un altro talker
msrp-failure-5 = indirizzo di destinazione già in uso
msrp-failure-6 = scavalcato da uno stream di rango superiore
msrp-failure-7 = la latenza segnalata è cambiata
msrp-failure-8 = la porta di uscita non supporta AVB
msrp-failure-9 = usa un indirizzo di destinazione diverso
msrp-failure-10 = risorse MSRP esaurite
msrp-failure-11 = risorse MMRP esaurite
msrp-failure-12 = impossibile memorizzare l'indirizzo di destinazione
msrp-failure-13 = la priorità non è una priorità di classe SR
msrp-failure-14 = frame troppo grandi per il mezzo
msrp-failure-15 = limite di porte fan-in raggiunto
msrp-failure-16 = primo valore cambiato per uno stream registrato
msrp-failure-17 = VLAN bloccata sulla porta di uscita
msrp-failure-18 = tagging VLAN disattivato sulla porta di uscita
msrp-failure-19 = priorità di classe SR non corrispondente
msrp-failure-unknown = motivo sconosciuto
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, sul bridge { $bridge }

## The macOS menu bar

menu-about = Informazioni su plaa
menu-settings = Impostazioni…
menu-services = Servizi
menu-hide = Nascondi plaa
menu-hide-others = Nascondi altre
menu-show-all = Mostra tutte
menu-quit = Esci da plaa
menu-file = File
menu-open = Apri…
menu-open-address = Apri indirizzo…
menu-close = Chiudi finestra
menu-edit = Modifica
menu-cut = Taglia
menu-copy = Copia
menu-paste = Incolla
menu-select-all = Seleziona tutto
menu-undo = Annulla
menu-redo = Ripeti
menu-trim = Rifinisci…
menu-split = Dividi clip
menu-rotate-left = Ruota a sinistra
menu-rotate-right = Ruota a destra
menu-flip-horizontal = Rifletti orizzontalmente
menu-flip-vertical = Rifletti verticalmente
menu-revert = Ripristina originale
menu-export = Esporta…
menu-view = Vista
menu-playback = Riproduzione
menu-play-pause = Riproduci o metti in pausa
menu-to-start = Vai all'inizio
menu-previous-chapter = Capitolo precedente
menu-next-chapter = Capitolo successivo
menu-faster = Più veloce
menu-slower = Più lento
menu-normal-speed = Velocità normale
menu-ab-loop = Loop tra due punti
menu-volume-up = Alza volume
menu-volume-down = Abbassa volume
menu-mute = Disattiva o riattiva audio
menu-next-subtitles = Sottotitoli successivi
menu-next-audio = Traccia audio successiva
menu-window = Finestra
menu-minimize = Contrai
menu-zoom = Zoom
menu-bring-all-to-front = Porta tutto in primo piano

## Editing

action-edit = Modifica
title-edited = { $name } (modificato)
edit-trim = Rifinisci
edit-trim-apply = Rifinisci
edit-split = Dividi clip
edit-remove-clip = Rimuovi clip
edit-done = Fine
edit-rotate-left = Ruota a sinistra
edit-rotate-right = Ruota a destra
edit-flip-horizontal = Rifletti orizzontalmente
edit-flip-vertical = Rifletti verticalmente
edit-undo = Annulla
edit-redo = Ripeti
edit-revert = Ripristina originale
edit-export = Esporta…

## Exporting

export-title = Esporta
export-format = Formato
export-size = Dimensioni
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Solo audio
export-size-original = { $lines }p, così com'è
export-on-graphics-card = Codificato sulla scheda grafica ({ $api })
export-on-processor = Codificato sul processore, che richiede più tempo
export-no-encoder = Qui non si apre nessun codificatore per questo formato
export-checking = Ricerca di un codificatore…
export-sound-format = Formato audio
export-quality = Qualità
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Compresso; si riproduce quasi ovunque, anche sui dispositivi Apple
export-mp3-note = Compresso; si riproduce ovunque
export-opus-note = Compresso; il suono migliore per le sue dimensioni, anche se non tutti i lettori lo supportano
export-flac-note = Senza perdita: il suono esattamente com'è, in un formato aperto
export-alac-note = Senza perdita: il suono esattamente com'è, come lo riproducono i dispositivi Apple
export-wav-note = Non compresso: il suono esattamente com'è, nei file più grandi
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Salvato a { $rate } kHz, sotto i { $file } kHz del file
export-choose = Esporta…
export-name = { $name } esportato
export-name-edited = { $name } modificato
export-progress = Esportazione di “{ $name }”
export-stop = Interrompi l'esportazione
notice-exported = “{ $name }” esportato
notice-export-stopped = Esportazione interrotta
notice-export-failed = Impossibile esportare: { $error }
close-title = Chiudere senza esportare?
close-unexported = Le modifiche a “{ $name }” andranno perse se non le esporti.
close-exporting = L'esportazione di “{ $name }” si interrompe e quanto scritto finora va perso.
close-anyway = Chiudi
close-quit-anyway = Esci

## Recording

record-title = Registrazione audio
record-file-name = Registrazione audio
record-start = Registra
record-stop = Interrompi la registrazione
record-no-input = Nessun microfono
record-microphone = Microfono
record-default-microphone = Quello predefinito di sistema
record-playing = Ciò che riproduce il computer
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } canale
   *[other] { $count } canali
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, salvato a { $saved } kHz
action-record-sound = Registra audio
open-record-sound = Nuova registrazione audio
start-record-sound = Registra audio
menu-record-sound = Nuova registrazione audio
record-screen-title = Registrazione schermo
record-screen-file-name = Registrazione schermo
record-screen-whole = Schermo
record-screen-number = Schermo { $number }
record-screen-main = Schermo { $number } (principale)
record-screens = Schermi
record-windows = Finestre
record-screen-what = Cosa registrare
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Ogni fotogramma
record-rate-tip = Fotogrammi al secondo
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] { $count } perso
   *[other] { $count } persi
}
record-screen-window = Finestra
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Non più condiviso
record-screen-again = Scegli di nuovo
record-screen-failed = Impossibile registrare lo schermo
record-sound = Audio
record-no-sound = Nessun audio
# How much louder the input is taken, for a quiet microphone.
record-gain = Più forte
record-gain-value = +{ $decibels } dB
record-level-automatically = Regola il livello automaticamente
record-hide-title = plaa si nasconde durante la registrazione
record-hide-tray = Per interrompere la registrazione, fai clic sul punto rosso che plaa mette nell'area di notifica, oppure riapri plaa.
record-hide-reopen = Per interrompere la registrazione, riapri plaa.
record-hide-start = Avvia registrazione
record-tray-title = plaa sta registrando lo schermo
record-tray-tip = Fai clic per interrompere la registrazione
action-record-screen = Registra schermo
open-record-screen = Nuova registrazione schermo
start-record-screen = Registra schermo
menu-record-screen = Nuova registrazione schermo
record-camera-title = Registrazione videocamera
record-camera-file-name = Registrazione videocamera
record-camera = Videocamera
record-camera-failed = Impossibile aprire la videocamera
record-name-title = Dai un nome alla registrazione
record-name-note = È salvata come “{ $name }” in { $folder }.
record-name-label = Nome
record-name-keep = Mantieni
record-name-rename = Rinomina
record-name-taken = Esiste già un file chiamato “{ $name }”.
record-name-invalid = Un nome non può contenere nessuno di questi caratteri: / \ : * ? " < > |
record-name-failed = Impossibile rinominare: { $error }
action-record-camera = Registra videocamera
open-record-camera = Nuova registrazione videocamera
start-record-camera = Registra videocamera
menu-record-camera = Nuova registrazione videocamera
notice-no-camera = Nessuna videocamera da registrare
notice-recorded = “{ $name }” salvato in { $folder }
notice-record-failed = Impossibile registrare: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Consenti il controllo esterno
settings-allow-outside-control-note = Gli agenti IA come Claude Code possono vedere cosa riproduce plaa e controllarlo tramite plaa --mcp. plaa chiede prima di ogni nuovo agente.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Consentiti: { $agents }
settings-forget-agents = Dimentica
settings-ask-before-note = Chiedi prima che un agente:
settings-ask-reading = Veda cosa viene riprodotto, o l'immagine
settings-ask-playing = Controlli la riproduzione, l'audio o lo schermo intero
settings-ask-opening = Apra un file o chiuda una finestra
settings-ask-editing = Modifichi un file o lo esporti
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Consentire a { $agent } di controllare plaa?
agent-prompt-body = { $agent } chiede di usare il controllo esterno di plaa per vedere cosa viene riprodotto e controllarlo. Puoi disattivare il controllo esterno in Impostazioni.
agent-prompt-allow = Consenti
agent-prompt-deny = Non consentire
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Consentire a { $agent } di vedere cosa viene riprodotto qui?
agent-ask-play = Consentire a { $agent } di controllare la riproduzione qui?
agent-ask-open = Consentire a { $agent } di aprire o chiudere una finestra?
agent-ask-edit = Consentire a { $agent } di modificare o esportare qui?
agent-working = { $agent } sta lavorando…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } chiede di usare “{ $tool }”. In Impostazioni scegli su cosa chiede plaa.

## Casting to a TV or speaker on the network

cast-looking = Ricerca di TV e altoparlanti…
cast-none = Non sono stati trovati TV, altoparlanti o Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Interrompi la riproduzione su { $renderer }
notice-casting = Riproduzione su { $renderer }.
notice-cast-failed = Impossibile riprodurre su { $renderer }: { $reason }
cast-web-page = Non è ancora possibile trasmettere le pagine web: apri invece l'indirizzo del video stesso.

## The assistant panel, and its models in Settings

assistant-add-another = Aggiungi un modello…
assistant-add-model = L'assistente ha bisogno di un modello: un modello cloud con la sua chiave API, o un modello locale.
assistant-ask = Chiedi qualcosa su ciò che viene riprodotto
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = La chat stava diventando troppo lunga perché { $model } la legga tutta insieme, quindi la parte precedente è stata riassunta. L'assistente potrebbe riguardare ciò che ha visto.
assistant-compacting = Riassunto della parte precedente della chat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } su questo computer
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } su { $host }
assistant-new-chat = Nuova chat
assistant-no-model = Prima aggiungi un modello in Impostazioni.
assistant-open-settings = Aggiungi un modello
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = La conversazione non rientra più in ciò che { $model } può leggere in una volta. Inizia una nuova chat, o scegli un modello che possa leggere di più.
assistant-problem-credit = { $provider } dice che l'account non ha credito. Un nuovo account deve acquistarne sul sito di { $provider } prima che la sua chiave funzioni; poi riprova.
assistant-problem-key = { $provider } ha rifiutato la chiave API. Controllala in Impostazioni.
assistant-problem-model = { $provider } non ha nessun modello chiamato { $model }. Controlla il nome in Impostazioni.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } non ha risposto: { $message }
assistant-problem-rate = { $provider } chiede di rallentare. Riprova tra un momento.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } chiede di rallentare: { $message }
assistant-problem-refused = { $model } si è rifiutato di rispondere.
assistant-problem-unavailable = { $provider } è occupato o ha problemi. Riprova tra un momento.
assistant-problem-unreachable = plaa non è riuscito a raggiungere { $provider }. Controlla la connessione, o che il server sia in esecuzione.
assistant-running = In esecuzione…
assistant-send = Invia
assistant-shortened = La chat era troppo lunga perché { $model } la legga tutta insieme, quindi i risultati precedenti degli strumenti sono stati omessi. L'assistente potrebbe riguardare ciò che ha visto.
assistant-stop = Interrompi
assistant-stopped = Interrotto.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Ora stai parlando con { $model }.
assistant-thinking = Sta pensando…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Ragionamento
assistant-title = Assistente
settings-assistant-add = Aggiungi un modello
settings-assistant-add-button = Aggiungi
# $model is the model's name.
settings-assistant-added = { $model } ha risposto ed è stato aggiunto.
settings-assistant-added-tag = Aggiunto
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Indirizzo, ad esempio { $example }
settings-assistant-change-key = Cambia chiave
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Server compatibile con OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Contesto
settings-assistant-context-note = Un contesto più ampio permette all'assistente di tenere a mente una parte maggiore di una chat, ma il modello occupa più memoria e può rispondere più lentamente.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Apri la pagina
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } è in esecuzione su { $address }.
settings-assistant-found-none = { $provider } non ha ancora modelli. Scaricane uno con { $provider }, poi cerca di nuovo.
settings-assistant-get-key = Ottieni una chiave API
settings-assistant-get-server = Ottieni { $provider }
settings-assistant-in-use = In uso
settings-assistant-key = Chiave API
# $error is what the keychain said.
settings-assistant-key-failed = Impossibile conservare la chiave nel portachiavi: { $error }
settings-assistant-key-kept = La tua chiave di { $provider } è conservata nel portachiavi di sistema.
settings-assistant-key-needed = Questo modello richiede una chiave API.
settings-assistant-key-refused = { $provider } ha rifiutato questa chiave. Controlla che sia stata copiata per intero, dall'account giusto.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crea una chiave sul sito di { $provider } e incollala qui.
settings-assistant-look-again = Cerca di nuovo
settings-assistant-looking = Ricerca dei modelli…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modello, ad esempio { $example }
settings-assistant-no-server = plaa non ha trovato { $provider } in esecuzione su questo computer. Avvialo, o indica il suo indirizzo qui sotto.
settings-assistant-no-tools = Non può usare strumenti, che servono all'assistente
settings-assistant-none = Ancora nessun modello. Aggiungine uno qui sotto: un modello locale, come quelli di Ollama, resta su questo computer; un modello cloud richiede una chiave API del suo fornitore.
settings-assistant-note = I modelli con cui il pannello dell'assistente può parlare. Le chiavi sono conservate nel portachiavi di sistema.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Usa un altro indirizzo
settings-assistant-recommended = Consigliato
settings-assistant-remove = Rimuovi
settings-assistant-sees = Vede le immagini
settings-assistant-trying = Prova in corso…
settings-assistant-use = Usa
settings-assistant-use-key = Continua
settings-assistant-uses-tools = Usa strumenti
