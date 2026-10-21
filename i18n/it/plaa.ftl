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
settings-tab-network = Rete
settings-tab-language = Lingua
settings-tab-agents = Agenti
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
settings-folders = Cartelle
settings-recordings = Registrazioni
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
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Consentire a { $agent } di controllare plaa?
agent-prompt-body = { $agent } chiede di usare il controllo esterno di plaa per vedere cosa viene riprodotto e controllarlo. Puoi disattivare il controllo esterno in Impostazioni.
agent-prompt-allow = Consenti
agent-prompt-deny = Non consentire
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Consentire a { $agent } di vedere cosa viene riprodotto qui?
agent-ask-play = Consentire a { $agent } di controllare la riproduzione qui?
agent-ask-open = Consentire a { $agent } di aprire o chiudere una finestra?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } chiede di usare “{ $tool }”. In Impostazioni scegli su cosa chiede plaa.
