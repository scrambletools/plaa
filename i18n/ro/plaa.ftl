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
language-name = Română

## Common

app-name = plaa
common-cancel = Anulează
common-close = Închide
common-more = Mai multe

## Command line

usage-help =
    Utilizare: plaa [FIȘIER SAU ADRESĂ]...

    Redă fișiere video și audio, precum și fluxuri de la adrese precum
    https://example.com/live.m3u8. Fiecare se deschide într-o fereastră a
    instanței plaa care rulează, pornită la nevoie.

    Opțiuni:
      -h, --help        Afișează acest ajutor
      -V, --version     Afișează versiunea
          --interfaces  Afișează interfețele cu fir și dacă plaa poate
                        trimite fluxuri AVB pe ele
          --mcp         Servește MCP pe stdin și stdout, ca agenții AI să vadă ce
                        redă instanța plaa care rulează și s-o controleze

## Start window

start-hint = Deschide un fișier video sau audio, un flux sau un videoclip de pe web.
start-open = Deschide
start-open-address = Deschide o adresă

## Actions, as buttons, tooltips and menus name them

action-open = Deschide
action-open-address = Deschide o adresă
open-file = Deschide un fișier…
open-address = Deschide o adresă…
action-settings = Configurări
action-info = Inspector
action-loop = Buclă
action-screenshot = Salvează cadrul ca imagine
action-play = Redă
action-pause = Pauză
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Înapoi cu { $seconds } secundă
    [few] Înapoi cu { $seconds } secunde
   *[other] Înapoi cu { $seconds } de secunde
}
action-forward = { $seconds ->
    [one] Înainte cu { $seconds } secundă
    [few] Înainte cu { $seconds } secunde
   *[other] Înainte cu { $seconds } de secunde
}
action-mute = Dezactivează sunetul
action-unmute = Activează sunetul
action-speed = Viteză
action-tracks = Audio și subtitrări
action-full-screen = Ecran complet
action-playlist = Listă de redare
action-chapters = Capitole
action-picture-in-picture = Imagine în imagine
action-previous = Anterior
action-next = Următor
action-exit-full-screen = Ieși din modul ecran complet
action-quality = Calitate
action-visualizer = Vizualizator
action-network = Audio în rețea
action-cast = Proiectează
action-assistant = Asistent

## What a sound file shows under its cover

visualizer-spectrum = Spectru
visualizer-waveform = Formă de undă
visualizer-prism = Prismă
visualizer-cover = Doar coperta

## The floating controls

overlay-keep-shown = Păstrează comenzile fixate
overlay-auto-hide = Lasă comenzile să plutească deasupra imaginii

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Subtitrări
tracks-off = Niciuna
# A track the file gives no name or language, by its number.
tracks-numbered = Pista { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Doar audio

## Inspector

info-title = Inspector
playlist-title = Listă de redare
chapters-numbered = Capitolul { $number }
info-file = Fișier
info-name = Nume
info-address = Adresă
info-folder = Dosar
info-size = Dimensiune
info-format = Format
info-duration = Durată
info-bit-rate = Rată de biți
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Dimensiuni
info-frame-rate = Rată de cadre
info-frames-per-second = { $rate } cadre/s
info-pixel-format = Format de pixeli
info-decoding = Decodare
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Placa grafică ({ $api })
info-decoding-software = Procesor
info-sample-rate = Rată de eșantionare
info-hertz = { $rate } Hz
info-channels = Canale
info-language = Limbă
info-no-sound = Fără sunet
info-tags = Etichete

## Dialogs

dialog-open = Deschide
dialog-filter-media = Video și audio
dialog-filter-all = Toate fișierele

address-title = Deschide o adresă
address-note = O pagină web cu un videoclip, precum cea a unui videoclip YouTube, sau un flux ori un fișier din rețea, precum https://example.com/live.m3u8.
address-label = Adresă
address-open = Deschide
address-recent = Recente
address-forget = Elimină din recente

## Notices

notice-dialog-failed = Dialogul de fișiere nu a putut fi deschis: { $error }
notice-screenshot-saved = S-a salvat „{ $name }”.
notice-screenshot-failed = Imaginea nu a putut fi salvată: { $error }
notice-no-sound = Se redă fără sunet: { $reason }
notice-loop-from = Bucla începe la { $time }. Apasă din nou unde trebuie să se termine.
notice-loop-between = Se redă în buclă de la { $from } la { $to }.
notice-loop-off = Bucla este dezactivată.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Aceasta pare o pagină web. plaa redă videoclipurile de pe pagini web cu yt-dlp, care nu este instalat.
error-empty-list = Această listă nu are nimic de redat.

## Settings

settings-title = Configurări
settings-tab-appearance = Aspect
settings-tab-playback = Redare
settings-tab-network = Rețea
settings-tab-language = Limbă
settings-tab-agents = Agenți
settings-tab-assistant = Asistent
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versiune de dezvoltare, { $build })
settings-choose = Alege…

settings-appearance-system = Urmează sistemul
settings-appearance-light = Luminos
settings-appearance-dark = Întunecat
settings-colors = Culori
settings-system-accent = Folosește culoarea de accent a sistemului
settings-omarchy-note = Culorile sunt create din culoarea de accent a temei „{ $theme }”.
settings-system-accent-note = Culorile sunt create din culoarea de accent a sistemului.
settings-system-accent-none = Sistemul nu are o culoare de accent, așa că plaa o folosește pe cea aleasă mai jos.
settings-accent-chosen-note = Culorile sunt create din culoarea aleasă mai jos.
settings-controls = Comenzi
settings-overlay = Comenzi plutitoare
settings-overlay-note = Bara de instrumente și comenzile de redare plutesc deasupra imaginii sau a copertei și se retrag cât timp indicatorul stă nemișcat sau este în afara ferestrei.
settings-overlay-delay = Ascunde după
settings-overlay-delay-note = Cât timp stă indicatorul nemișcat deasupra a ceea ce se redă înainte ca comenzile să se retragă.
settings-seconds = { $seconds } s
settings-transparency = Transparență
settings-transparency-note = Cât din imagine se vede prin comenzile plutitoare.
settings-percent = { $percent }%
settings-corner-radius = Raza colțurilor
settings-corner-radius-note = Pentru dialoguri și comenzile plutitoare.
settings-pixels = { $pixels } px
settings-animations = Animații
settings-animations-note = Bare și panouri care glisează, dialoguri care cresc și butoane elastice.
settings-animations-reduced = Dezactivate cât timp sistemul cere mișcare redusă.

settings-hardware = Folosește placa grafică
settings-hardware-note = Decodează videoclipurile și codifică exporturile pe ea: consumă mai puțină energie, redă fluid videoclipurile mari și exportă mai repede. Se aplică fișierelor deschise ulterior.
settings-audio-device = Ieșire de sunet
settings-audio-device-system = Ieșirea implicită a sistemului
settings-resume = Reia de unde ai rămas
settings-resume-note = Fișierele pornesc de unde erau când au fost închise ultima dată.
settings-folder-playlists = Redă celelalte fișiere din dosar
settings-folder-playlists-note = Deschiderea unui fișier le listează pe celelalte din dosarul său, care sunt redate după el.
settings-streams = Fluxuri și videoclipuri web
settings-stream-quality = Calitate
settings-stream-quality-note = Pentru fluxurile și videoclipurile web disponibile în mai multe calități. Se aplică pentru ce se deschide ulterior.
settings-quality-auto = Automată, la înălțimea ecranului
settings-quality-best = Maximă
settings-web-pages = Redă videoclipurile web cu yt-dlp
settings-web-pages-found = yt-dlp găsește videoclipul pe pagini precum cele de pe YouTube.
settings-web-pages-missing = yt-dlp nu este instalat. Instalează-l pentru a reda videoclipurile de pe pagini precum cele de pe YouTube.
settings-folders = Dosare
settings-recordings = Înregistrări
settings-screenshots = Cadre salvate

settings-interface-language = Limba interfeței
# The first entry of the language list.
settings-language-system = Limba sistemului
settings-track-languages = Limbile pistelor
settings-track-languages-note = Coduri de limbă precum en sau fra, în ordinea preferinței, separate prin virgule. plaa alege pistele audio și subtitrările în aceste limbi când un fișier le are.
settings-audio-languages = Audio
settings-subtitle-languages = Subtitrări

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interfață AVB
settings-avb-interface-note = Portul cu fir pe care plaa găsește emițătorii și receptorii AVB și Milan. Pentru trimitere este nevoie de un port cu ceas hardware PTP, precum cel al unui Intel I210.
settings-avb-off = Dezactivat
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (doar listare: fără ceas PTP)
settings-avb-off-mac = { $interface } (doar listare: AVB este dezactivat)
settings-avb-interface-note-mac = Portul cu fir pe care plaa găsește emițătorii și receptorii AVB și Milan. Trimiterea trece prin entitatea AVB proprie a acestui Mac, pentru care AVB trebuie activat pe port în Navigatorul de dispozitive de rețea din Configurare audio-MIDI.
settings-avb-talker-note-mac = plaa trimite sunetul ferestrei redate ultima dată prin entitatea AVB proprie a acestui Mac, către receptorii conectați la ea.
settings-avb-talker = Trimite ce se redă
settings-avb-talker-note = plaa devine un emițător Milan și trimite sunetul ferestrei redate ultima dată către receptorii conectați la el.
settings-avb-name = Nume în rețea
settings-avb-name-note = Controlerele afișează plaa ca „{ $name }”. Apasă Enter pentru a-l redenumi.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa pe { $computer }
avb-could-not-start = Nu s-a putut folosi { $interface }: { $error }
network-title = Audio în rețea
network-off = Activează „Trimite ce se redă” în Configurări pentru a reda în rețea.
network-blocked = Nu se trimite: { $reason }
network-waiting = Gata de trimitere; se așteaptă conectarea unui receptor.
network-sending = Se trimite către receptorii conectați.
network-looking = Se caută receptori…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Se pornește ptp4l, care menține timpul de rețea după care este trimis fluxul plaa…
network-ptp-elsewhere = ptp4l rulează pe altă interfață, așa că fluxul plaa nu are aici timp de rețea și este posibil ca receptorii să nu-l redea. Alege acea interfață în Configurări sau rulează ptp4l pe aceasta.
network-ptp-missing = ptp4l nu rulează, așa că fluxul plaa nu are timp de rețea și este posibil ca receptorii să nu-l redea. Instalează linuxptp împreună cu pachetul plaa sau triib, care îl rulează.
network-ptp-failed = ptp4l nu rulează, așa că fluxul plaa nu are timp de rețea și este posibil ca receptorii să nu-l redea. plaa nu l-a putut porni: { $reason }
network-no-listeners = Niciun receptor din rețea nu poate reda sunetul plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Redă pe { $listener }
network-stop-on = Oprește redarea pe { $listener }
network-choose-format = Alege formatul în care să redai pe { $listener }
network-failed = Nu s-a putut conecta: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Actualizează triib pentru a reda aici.
network-choose = Alege un receptor pentru a vedea cum își primește fluxul.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Ascultă aici
network-no-talkers = Niciun emițător din rețea nu trimite într-un format pe care plaa îl poate reda.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Ascultă { $talker }
network-listen-stop = Nu mai asculta { $talker }
network-listen-frames = Cadre primite: { $frames }
network-listen-update-triib = Actualizează triib pentru a asculta aici: plaa redă doar ce ajunge oricum la acest computer.
network-listen-lite = Portul folosește AVB Lite, care nu rezervă fluxuri: plaa redă doar ce ajunge oricum la acest computer.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Stare
network-not-connected = Neconectat
network-from = De la { $stream }
network-from-receiving = De la { $stream }, se recepționează
network-from-waiting = De la { $stream }, se așteaptă emițătorul
network-from-failed = De la { $stream }, rezervarea emițătorului a eșuat: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Ceas media
network-locked = Sincronizat cu fluxul
network-not-locked = Nesincronizat
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formate pe care le poate primi de la plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID flux
# The MAC address the stream is sent to.
network-destination = Destinație
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latență
network-microseconds = { $time } µs
network-frames = Cadre primite
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Cadre în afara secvenței
# How often the stream stopped coming while it was taken.
network-interrupted = Întreruperi
network-entity-id = ID entitate

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = lățime de bandă insuficientă
msrp-failure-2 = resurse insuficiente ale punții
msrp-failure-3 = lățime de bandă insuficientă pentru clasa de trafic
msrp-failure-4 = ID-ul fluxului este folosit de alt emițător
msrp-failure-5 = adresa de destinație este deja folosită
msrp-failure-6 = înlocuit de un flux de rang superior
msrp-failure-7 = latența raportată s-a schimbat
msrp-failure-8 = portul de ieșire nu acceptă AVB
msrp-failure-9 = folosește altă adresă de destinație
msrp-failure-10 = resurse MSRP epuizate
msrp-failure-11 = resurse MMRP epuizate
msrp-failure-12 = adresa de destinație nu poate fi stocată
msrp-failure-13 = prioritatea nu este o prioritate de clasă SR
msrp-failure-14 = cadre prea mari pentru mediu
msrp-failure-15 = s-a atins limita fan-in a portului
msrp-failure-16 = prima valoare s-a schimbat pentru un flux înregistrat
msrp-failure-17 = VLAN blocat pe portul de ieșire
msrp-failure-18 = etichetarea VLAN este dezactivată pe portul de ieșire
msrp-failure-19 = nepotrivire de prioritate a clasei SR
msrp-failure-unknown = motiv necunoscut
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, la puntea { $bridge }

## The macOS menu bar

menu-about = Despre plaa
menu-settings = Configurări…
menu-services = Servicii
menu-hide = Ascunde plaa
menu-hide-others = Ascunde-le pe celelalte
menu-show-all = Afișează-le pe toate
menu-quit = Ieși din plaa
menu-file = Fișier
menu-open = Deschide…
menu-open-address = Deschide o adresă…
menu-close = Închide fereastra
menu-edit = Editare
menu-cut = Taie
menu-copy = Copiază
menu-paste = Lipește
menu-select-all = Selectează tot
menu-undo = Anulează
menu-redo = Refă
menu-trim = Scurtează…
menu-split = Împarte clipul
menu-rotate-left = Rotește la stânga
menu-rotate-right = Rotește la dreapta
menu-flip-horizontal = Oglindește orizontal
menu-flip-vertical = Oglindește vertical
menu-revert = Revino la original
menu-export = Exportă…
menu-view = Vizualizare
menu-playback = Redare
menu-play-pause = Redă sau pune pe pauză
menu-to-start = Mergi la început
menu-previous-chapter = Capitolul anterior
menu-next-chapter = Capitolul următor
menu-faster = Mai repede
menu-slower = Mai lent
menu-normal-speed = Viteză normală
menu-ab-loop = Buclă între două puncte
menu-volume-up = Mai tare
menu-volume-down = Mai încet
menu-mute = Dezactivează sau activează sunetul
menu-next-subtitles = Subtitrările următoare
menu-next-audio = Pista audio următoare
menu-window = Fereastră
menu-minimize = Minimizează
menu-zoom = Redimensionează
menu-bring-all-to-front = Adu-le pe toate în față

## Editing

action-edit = Editează
title-edited = { $name } (editat)
edit-trim = Scurtează
edit-trim-apply = Scurtează
edit-split = Împarte clipul
edit-remove-clip = Elimină clipul
edit-done = Gata
edit-rotate-left = Rotește la stânga
edit-rotate-right = Rotește la dreapta
edit-flip-horizontal = Oglindește orizontal
edit-flip-vertical = Oglindește vertical
edit-undo = Anulează
edit-redo = Refă
edit-revert = Revino la original
edit-export = Exportă…

## Exporting

export-title = Exportă
export-format = Format
export-size = Dimensiune
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Doar sunet
export-size-original = { $lines }p, așa cum este
export-on-graphics-card = Se codifică pe placa grafică ({ $api })
export-on-processor = Se codifică pe procesor, ceea ce durează mai mult
export-no-encoder = Niciun codificator pentru acest format nu poate fi deschis aici
export-checking = Se caută un codificator…
export-sound-format = Format de sunet
export-quality = Calitate
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Comprimat; se redă aproape oriunde, inclusiv pe dispozitivele Apple
export-mp3-note = Comprimat; se redă oriunde
export-opus-note = Comprimat; cel mai bun sunet pentru dimensiunea sa, deși nu toate playerele îl acceptă
export-flac-note = Fără pierderi: sunetul exact așa cum este, într-un format deschis
export-alac-note = Fără pierderi: sunetul exact așa cum este, așa cum îl redau dispozitivele Apple
export-wav-note = Necomprimat: sunetul exact așa cum este, în cele mai mari fișiere
export-kilobits = { $rate } kb/s
export-bits = { $bits ->
    [one] { $bits } bit
    [few] { $bits } biți
   *[other] { $bits } de biți
}
export-rate-lower = Se salvează la { $rate } kHz, sub cei { $file } kHz ai fișierului
export-choose = Exportă…
export-name = { $name } exportat
export-name-edited = { $name } editat
export-progress = Se exportă „{ $name }”
export-stop = Oprește exportul
notice-exported = S-a exportat „{ $name }”
notice-export-stopped = Exportul s-a oprit
notice-export-failed = Nu s-a putut exporta: { $error }
close-title = Închizi fără să exporți?
close-unexported = Modificările aduse fișierului „{ $name }” se pierd dacă nu le exporți.
close-exporting = Exportul fișierului „{ $name }” se oprește, iar ce s-a scris până acum se pierde.
close-anyway = Închide
close-quit-anyway = Ieși

## Recording

record-title = Înregistrare de sunet
record-file-name = Înregistrare de sunet
record-start = Înregistrează
record-stop = Oprește înregistrarea
record-no-input = Niciun microfon
record-microphone = Microfon
record-default-microphone = Microfonul implicit al sistemului
record-playing = Ce redă computerul
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } canal
    [few] { $count } canale
   *[other] { $count } de canale
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, se salvează la { $saved } kHz
action-record-sound = Înregistrează sunet
open-record-sound = Înregistrare nouă de sunet
start-record-sound = Înregistrează sunet
menu-record-sound = Înregistrare nouă de sunet
record-screen-title = Înregistrare de ecran
record-screen-file-name = Înregistrare de ecran
record-screen-whole = Ecran
record-screen-number = Ecranul { $number }
record-screen-main = Ecranul { $number } (principal)
record-screens = Ecrane
record-windows = Ferestre
record-screen-what = Ce să înregistrezi
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Toate cadrele
record-rate-tip = Cadre pe secundă
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } cadru pierdut
    [few] { $size } · { $count } cadre pierdute
   *[other] { $size } · { $count } de cadre pierdute
}
record-screen-window = Fereastră
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Partajarea s-a încheiat
record-screen-again = Alege din nou
record-screen-failed = Ecranul nu a putut fi înregistrat
record-sound = Sunet
record-no-sound = Fără sunet
# How much louder the input is taken, for a quiet microphone.
record-gain = Amplificare
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajustează nivelul automat
record-hide-title = plaa se ascunde cât timp înregistrează
record-hide-tray = Pentru a opri înregistrarea, dă clic pe punctul roșu pe care plaa îl pune în zona de notificare sau deschide din nou plaa.
record-hide-reopen = Pentru a opri înregistrarea, deschide din nou plaa.
record-hide-start = Începe înregistrarea
record-tray-title = plaa înregistrează ecranul
record-tray-tip = Dă clic pentru a opri înregistrarea
action-record-screen = Înregistrează ecranul
open-record-screen = Înregistrare nouă de ecran
start-record-screen = Înregistrează ecranul
menu-record-screen = Înregistrare nouă de ecran
record-camera-title = Înregistrare cu camera
record-camera-file-name = Înregistrare cu camera
record-camera = Cameră
record-camera-failed = Camera nu a putut fi deschisă
action-record-camera = Înregistrează cu camera
open-record-camera = Înregistrare nouă cu camera
start-record-camera = Înregistrează cu camera
menu-record-camera = Înregistrare nouă cu camera
notice-no-camera = Nu există nicio cameră cu care să înregistrezi
notice-recorded = S-a salvat „{ $name }” în { $folder }
notice-record-failed = Nu s-a putut înregistra: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Permite controlul extern
settings-allow-outside-control-note = Agenții AI precum Claude Code pot vedea ce se redă în plaa și îl pot controla, prin plaa --mcp. plaa întreabă înaintea fiecărui agent nou.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permiși: { $agents }
settings-forget-agents = Uită
settings-ask-before-note = Întreabă înainte ca un agent să:
settings-ask-reading = Vadă ce se redă sau imaginea
settings-ask-playing = Controleze redarea, sunetul sau ecranul complet
settings-ask-opening = Deschidă un fișier sau închidă o fereastră
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Permiți ca { $agent } să controleze plaa?
agent-prompt-body = { $agent } cere să folosească controlul extern al plaa, pentru a vedea ce se redă și a-l controla. Poți dezactiva controlul extern în Configurări.
agent-prompt-allow = Permite
agent-prompt-deny = Nu permite
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Permiți ca { $agent } să vadă ce se redă aici?
agent-ask-play = Permiți ca { $agent } să controleze redarea aici?
agent-ask-open = Permiți ca { $agent } să deschidă sau să închidă o fereastră?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } cere să folosească „{ $tool }”. În Configurări alegi despre ce întreabă plaa.

## Casting to a TV or speaker on the network

cast-looking = Se caută televizoare și difuzoare…
cast-none = Nu s-a găsit niciun televizor, difuzor sau Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Oprește redarea pe { $renderer }
notice-casting = Se redă pe { $renderer }.
notice-cast-failed = Nu s-a putut reda pe { $renderer }: { $reason }
cast-web-page = Paginile web nu pot fi încă proiectate: deschide în schimb chiar adresa videoclipului.

## The assistant panel, and its models in Settings

assistant-add-another = Adaugă un model…
assistant-add-model = Asistentul are nevoie de un model: un model în cloud, cu cheia sa API, sau un model local.
assistant-ask = Întreabă despre ce se redă
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Conversația devenea prea lungă pentru ca { $model } s-o citească dintr-odată, așa că partea ei anterioară a fost rezumată. Asistentul se poate uita din nou la ce a văzut.
assistant-compacting = Se rezumă partea anterioară a conversației
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } pe acest computer
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } la { $host }
assistant-new-chat = Conversație nouă
assistant-no-model = Adaugă mai întâi un model în Configurări.
assistant-open-settings = Adaugă un model
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Conversația nu mai încape în cât poate citi { $model } deodată. Începe o conversație nouă sau alege un model care poate citi mai mult.
assistant-problem-credit = { $provider } spune că acest cont nu are credit. Un cont nou are nevoie de credit cumpărat pe site-ul { $provider } înainte ca cheia sa să funcționeze; apoi încearcă din nou.
assistant-problem-key = { $provider } a respins cheia API. Verific-o în Configurări.
assistant-problem-model = { $provider } nu are niciun model numit { $model }. Verifică-i numele în Configurări.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } nu a răspuns: { $message }
assistant-problem-rate = { $provider } cere să încetinești. Încearcă din nou peste câteva momente.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } cere să încetinești: { $message }
assistant-problem-refused = { $model } a refuzat să răspundă.
assistant-problem-unavailable = { $provider } este ocupat sau are probleme. Încearcă din nou peste câteva momente.
assistant-problem-unreachable = plaa nu a putut contacta { $provider }. Verifică-ți conexiunea sau dacă serverul rulează.
assistant-running = Rulează…
assistant-send = Trimite
assistant-shortened = Conversația era prea lungă pentru ca { $model } s-o citească dintr-odată, așa că rezultatele anterioare ale instrumentelor au fost omise. Asistentul se poate uita din nou la ce a văzut.
assistant-stop = Oprește
assistant-stopped = Oprit.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Acum vorbești cu { $model }.
assistant-thinking = Se gândește…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Gânduri
assistant-title = Asistent
settings-assistant-add = Adaugă un model
settings-assistant-add-button = Adaugă
# $model is the model's name.
settings-assistant-added = { $model } a răspuns și a fost adăugat.
settings-assistant-added-tag = Adăugat
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adresă, de exemplu { $example }
settings-assistant-change-key = Schimbă cheia
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Server compatibil cu OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Context
settings-assistant-context-note = Un context mai mare îi permite asistentului să țină minte mai mult dintr-o conversație, dar modelul ocupă mai multă memorie și poate răspunde mai încet.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokeni
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Deschide pagina
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } rulează la { $address }.
settings-assistant-found-none = { $provider } nu are încă modele. Descarcă unul cu el, apoi caută din nou.
settings-assistant-get-key = Obține o cheie API
settings-assistant-get-server = Obține { $provider }
settings-assistant-in-use = În uz
settings-assistant-key = Cheie API
# $error is what the keychain said.
settings-assistant-key-failed = Cheia nu a putut fi păstrată în depozitul de chei: { $error }
settings-assistant-key-kept = Cheia ta { $provider } este păstrată în depozitul de chei al sistemului.
settings-assistant-key-needed = Acest model are nevoie de o cheie API.
settings-assistant-key-refused = { $provider } a respins această cheie. Verifică dacă a fost copiată întreagă, din contul potrivit.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Creează o cheie pe site-ul { $provider } și lipește-o aici.
settings-assistant-look-again = Caută din nou
settings-assistant-looking = Se caută modele…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, de exemplu { $example }
settings-assistant-no-server = plaa nu a găsit { $provider } pornit pe acest computer. Pornește-l sau introdu mai jos adresa lui.
settings-assistant-no-tools = Nu poate folosi instrumente, de care asistentul are nevoie
settings-assistant-none = Încă nu există modele. Adaugă unul mai jos: un model local, precum cel din Ollama, rămâne pe acest computer; un model în cloud are nevoie de o cheie API de la furnizorul său.
settings-assistant-note = Modelele cu care poate vorbi panoul asistentului. Cheile sunt păstrate în depozitul de chei al sistemului.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Folosește altă adresă
settings-assistant-recommended = Recomandat
settings-assistant-remove = Elimină
settings-assistant-sees = Vede imagini
settings-assistant-trying = Se încearcă…
settings-assistant-use = Folosește
settings-assistant-use-key = Continuă
settings-assistant-uses-tools = Folosește instrumente
