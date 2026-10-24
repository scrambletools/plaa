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
language-name = Español

## Common

app-name = plaa
common-cancel = Cancelar
common-close = Cerrar
common-more = Más

## Command line

usage-help =
    Uso: plaa [ARCHIVO O DIRECCIÓN]...

    Reproduce archivos de vídeo y audio, y transmisiones en direcciones como
    https://example.com/live.m3u8. Cada uno se abre en una ventana de plaa
    en ejecución, que se inicia si hace falta.

    Opciones:
      -h, --help        Mostrar esta ayuda
      -V, --version     Mostrar la versión
          --interfaces  Mostrar las interfaces cableadas y si plaa puede enviar
                        streams AVB por ellas
          --mcp         Servir MCP por stdin y stdout, para que agentes de IA
                        vean lo que reproduce plaa en ejecución y lo controlen

## Start window

start-hint = Abre un archivo de vídeo o audio, una transmisión o un vídeo web.
start-open = Abrir
start-open-address = Abrir dirección
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Conectar

## Actions, as buttons, tooltips and menus name them

action-open = Abrir
action-open-address = Abrir dirección
open-file = Abrir archivo…
open-address = Abrir dirección…
action-settings = Ajustes
action-info = Inspector
action-loop = Bucle
action-screenshot = Guardar fotograma como imagen
action-play = Reproducir
action-pause = Pausa
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Retroceder { $seconds } segundo
   *[other] Retroceder { $seconds } segundos
}
action-forward = { $seconds ->
    [one] Avanzar { $seconds } segundo
   *[other] Avanzar { $seconds } segundos
}
action-mute = Silenciar
action-unmute = Activar sonido
action-speed = Velocidad
action-tracks = Audio y subtítulos
action-full-screen = Pantalla completa
action-playlist = Lista de reproducción
action-chapters = Capítulos
action-picture-in-picture = Imagen en imagen
action-previous = Anterior
action-next = Siguiente
action-exit-full-screen = Salir de pantalla completa
action-quality = Calidad
action-visualizer = Visualizador
action-network = Audio en red
action-cast = Enviar
action-connect = Conectar
action-assistant = Asistente

## What a sound file shows under its cover

visualizer-spectrum = Espectro
visualizer-waveform = Forma de onda
visualizer-prism = Prisma
visualizer-cover = Solo carátula

## The floating controls

overlay-keep-shown = Mantener los controles fijos
overlay-auto-hide = Hacer flotar los controles sobre la imagen

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Subtítulos
tracks-off = Desactivados
# A track the file gives no name or language, by its number.
tracks-numbered = Pista { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Solo audio

## Inspector

info-title = Inspector
playlist-title = Lista de reproducción
chapters-numbered = Capítulo { $number }
info-file = Archivo
info-name = Nombre
info-address = Dirección
info-folder = Carpeta
info-size = Tamaño
info-format = Formato
info-duration = Duración
info-bit-rate = Tasa de bits
info-video = Vídeo
info-audio = Audio
info-codec = Códec
info-dimensions = Dimensiones
info-frame-rate = Velocidad de fotogramas
info-frames-per-second = { $rate } fotogramas por segundo
info-pixel-format = Formato de píxel
info-decoding = Decodificación
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Tarjeta gráfica ({ $api })
info-decoding-software = Procesador
info-sample-rate = Frecuencia de muestreo
info-hertz = { $rate } Hz
info-channels = Canales
info-language = Idioma
info-no-sound = Sin sonido
info-tags = Etiquetas
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = Otros listeners
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Clase SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Un formato nuevo desconecta { $count ->
        [one] el otro listener del talker.
       *[other] los otros { $count } listeners del talker.
    }

## Dialogs

dialog-open = Abrir
dialog-filter-media = Vídeo y audio
dialog-filter-all = Todos los archivos

address-title = Abrir dirección
address-note = Una página web con un vídeo, como la de un vídeo de YouTube, o una transmisión o un archivo en la red, como https://example.com/live.m3u8.
address-label = Dirección
address-open = Abrir
address-recent = Recientes
address-forget = Quitar de recientes

## Notices

notice-dialog-failed = No se ha podido abrir el diálogo de archivos: { $error }
notice-screenshot-saved = Se ha guardado “{ $name }”.
notice-screenshot-failed = No se ha podido guardar la imagen: { $error }
notice-no-sound = Reproduciendo sin sonido: { $reason }
notice-loop-from = Bucle desde { $time }. Pulsa de nuevo donde deba terminar.
notice-loop-between = Bucle de { $from } a { $to }.
notice-loop-off = Bucle desactivado.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Esto parece una página web. plaa reproduce los vídeos de las páginas web con yt-dlp, que no está instalado.
error-empty-list = Esta lista no tiene nada que reproducir.

## Settings

settings-title = Ajustes
settings-tab-appearance = Apariencia
settings-tab-playback = Reproducción
settings-tab-storage = Almacenamiento
settings-tab-network = Red
settings-tab-language = Idioma
settings-tab-agents = Agentes
settings-tab-assistant = Asistente
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versión de desarrollo, { $build })
settings-choose = Elegir…

settings-appearance-system = Seguir el sistema
settings-appearance-light = Claro
settings-appearance-dark = Oscuro
settings-colors = Colores
settings-system-accent = Usar el color de acento del sistema
settings-omarchy-note = Los colores se crean a partir del color de acento de “{ $theme }”.
settings-system-accent-note = Los colores se crean a partir del color de acento del sistema.
settings-system-accent-none = El sistema no tiene color de acento, así que plaa usa el elegido abajo.
settings-accent-chosen-note = Los colores se crean a partir del color elegido abajo.
settings-controls = Controles
settings-overlay = Controles flotantes
settings-overlay-note = La barra de herramientas y los controles de reproducción flotan sobre la imagen o la carátula y se ocultan deslizándose mientras el puntero está quieto o fuera de la ventana.
settings-overlay-delay = Ocultar tras
settings-overlay-delay-note = Cuánto tiempo debe estar quieto el puntero sobre lo que se reproduce antes de que los controles se oculten.
settings-seconds = { $seconds } s
settings-transparency = Transparencia
settings-transparency-note = Cuánto se ve la imagen a través de los controles flotantes.
settings-percent = { $percent }%
settings-corner-radius = Radio de las esquinas
settings-corner-radius-note = Para los diálogos y los controles flotantes.
settings-pixels = { $pixels } px
settings-animations = Animaciones
settings-animations-note = Barras y paneles que se deslizan, diálogos que crecen y botones con rebote.
settings-animations-reduced = Desactivadas mientras el sistema pida reducir el movimiento.

settings-hardware = Usar la tarjeta gráfica
settings-hardware-note = Decodifica los vídeos y codifica las exportaciones con ella: consume menos energía, reproduce con fluidez los vídeos grandes y exporta más rápido. Se aplica a los archivos que se abran después.
settings-audio-device = Salida de sonido
settings-audio-device-system = Predeterminada del sistema
settings-resume = Continuar donde lo dejaste
settings-resume-note = Los archivos empiezan donde estaban la última vez que se cerraron.
settings-folder-playlists = Reproducir los demás archivos de la carpeta
settings-folder-playlists-note = Al abrir un archivo, los demás de su carpeta se añaden a la lista y se reproducen después.
settings-streams = Transmisiones y vídeos web
settings-stream-quality = Calidad
settings-stream-quality-note = Para transmisiones y vídeos web disponibles en varias calidades. Se aplica a lo que se abra después.
settings-quality-auto = Automática, tan alta como la pantalla
settings-quality-best = Máxima
settings-web-pages = Reproducir vídeos web con yt-dlp
settings-web-pages-found = yt-dlp encuentra el vídeo en páginas como las de YouTube.
settings-web-pages-missing = yt-dlp no está instalado. Instálalo para reproducir los vídeos de páginas como las de YouTube.
settings-default-app = App predeterminada
settings-default-app-label = Abrir archivos con plaa
settings-default-app-note = Haz que plaa sea la app que abre los archivos de vídeo y de sonido.
settings-default-app-note-windows = Windows solo permite elegir las apps predeterminadas en su propia Configuración. Esto abre allí la página de plaa.
settings-default-app-note-macos = macOS te pide confirmar cada tipo: MP4, MOV, MKV, WebM, MP3, M4A, FLAC y WAV.
settings-default-app-status = { $set } de { $total } tipos de archivo se abren con plaa.
settings-default-app-button = Establecer como predeterminada
settings-default-app-button-windows = Abrir Configuración
settings-default-app-no-entry = La entrada de escritorio de plaa no está instalada, así que el sistema no puede abrir archivos con ella. Instala plaa desde un paquete.
settings-default-app-no-bundle = Abre plaa desde plaa.app para establecerla como predeterminada.
settings-default-app-failed = No se pudo establecer plaa como predeterminada: { $error }
settings-folders = Carpetas
settings-sound-recordings = Grabaciones de sonido
settings-video-recordings = Grabaciones de vídeo
settings-screenshots = Fotogramas guardados

settings-interface-language = Idioma de la interfaz
# The first entry of the language list.
settings-language-system = Seguir el sistema
settings-track-languages = Idiomas de las pistas
settings-track-languages-note = Códigos de idioma como en o fra, por orden de preferencia y separados por comas. plaa elige el audio y los subtítulos en estos idiomas cuando un archivo los tiene.
settings-audio-languages = Audio
settings-subtitle-languages = Subtítulos

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interfaz AVB
settings-avb-interface-note = El puerto cableado en el que plaa encuentra talkers y listeners AVB y Milan. Para enviar hace falta un puerto con reloj de hardware PTP, como el de una Intel I210.
settings-avb-off = Desactivada
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (solo lista: sin reloj PTP)
settings-avb-off-mac = { $interface } (solo lista: AVB desactivado)
settings-avb-interface-note-mac = El puerto cableado en el que plaa encuentra talkers y listeners AVB y Milan. El envío pasa por la entidad AVB propia de este Mac, que necesita AVB activado para el puerto en el explorador de dispositivos de red de Configuración de Audio MIDI.
settings-avb-talker-note-mac = plaa envía el sonido de la última ventana reproducida a través de la entidad AVB propia de este Mac a los listeners conectados a ella.
settings-avb-talker = Enviar lo que se reproduce
settings-avb-talker-note = plaa se convierte en un talker Milan y envía el sonido de la última ventana reproducida a los listeners conectados a él.
settings-avb-name = Nombre en la red
settings-avb-name-note = Los controladores muestran plaa como “{ $name }”. Pulsa Enter para cambiarle el nombre.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa en { $computer }
avb-could-not-start = No se ha podido usar { $interface }: { $error }
network-title = Audio en red
network-off = Activa “Enviar lo que se reproduce” en Ajustes para reproducir en la red.
network-blocked = No se envía: { $reason }
network-waiting = Listo para enviar, esperando a que se conecte un listener.
network-sending = Enviando a los listeners conectados.
network-looking = Buscando listeners…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Iniciando ptp4l, que mantiene la hora de red con la que se envía el stream de plaa…
network-ptp-elsewhere = ptp4l se ejecuta en otra interfaz, así que el stream de plaa no tiene hora de red aquí y puede que los listeners no lo reproduzcan. Elige esa interfaz en Ajustes o ejecuta ptp4l en esta.
network-ptp-missing = ptp4l no se está ejecutando, así que el stream de plaa no tiene hora de red y puede que los listeners no lo reproduzcan. Instala linuxptp con el paquete de plaa, o instala triib, que lo ejecuta.
network-ptp-failed = ptp4l no se está ejecutando, así que el stream de plaa no tiene hora de red y puede que los listeners no lo reproduzcan. plaa no ha podido iniciarlo: { $reason }
network-no-listeners = Ningún listener de la red puede reproducir el sonido de plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Reproducir en { $listener }
network-stop-on = Dejar de reproducir en { $listener }
network-choose-format = Elige un formato para reproducir en { $listener }
network-failed = No se ha podido conectar: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Actualiza triib para reproducir aquí.
network-choose = Elige un listener para ver cómo recibe su stream.
network-no-sources = Ningún talker de la red envía en un formato que plaa reproduce, ni puede cambiar a uno.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Dejar de escuchar { $talker }
network-listen-update-triib = Actualiza triib para escuchar aquí: plaa solo reproduce lo que llega de todos modos a este ordenador.
network-listen-lite = El puerto usa AVB Lite, que no reserva streams: plaa solo reproduce lo que llega de todos modos a este ordenador.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Buscando talkers…
# plaa listens through its entity, which exists only while it sends.
connect-off = Activa “Enviar lo que se reproduce” en Ajustes para escuchar talkers aquí.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa no puede escuchar aquí: { $reason }
connect-sending = Enviando en un formato que plaa reproduce
connect-changing = Cambiando a un formato que plaa reproduce
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, en { $format }
connect-change-cuts-off = { $talker }, en { $format }, desconectando { $count ->
        [one] su otro listener
       *[other] sus otros { $count } listeners
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Estado
network-not-connected = No conectado
network-from = Desde { $stream }
network-from-receiving = Desde { $stream }, recibiendo
network-from-waiting = Desde { $stream }, esperando al talker
network-from-failed = Desde { $stream }, la reserva del talker ha fallado: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Reloj de medios
network-locked = Sincronizado con el stream
network-not-locked = No sincronizado
network-format = Formato
# The formats both the listener and plaa's stream can be in.
network-formats = Formatos que puede recibir de plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID del stream
# The MAC address the stream is sent to.
network-destination = Destino
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latencia
network-microseconds = { $time } µs
network-frames = Tramas recibidas
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Tramas fuera de secuencia
# How often the stream stopped coming while it was taken.
network-interrupted = Interrupciones
# Frames that came after, or long before, their time to be played.
network-late = Tramas retrasadas
network-early = Tramas adelantadas
network-entity-id = ID de entidad

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = ancho de banda insuficiente
msrp-failure-2 = recursos del puente insuficientes
msrp-failure-3 = ancho de banda insuficiente para la clase de tráfico
msrp-failure-4 = ID de stream en uso por otro talker
msrp-failure-5 = dirección de destino ya en uso
msrp-failure-6 = desplazado por un stream de mayor rango
msrp-failure-7 = la latencia notificada ha cambiado
msrp-failure-8 = el puerto de salida no admite AVB
msrp-failure-9 = usa otra dirección de destino
msrp-failure-10 = sin recursos MSRP
msrp-failure-11 = sin recursos MMRP
msrp-failure-12 = no se puede guardar la dirección de destino
msrp-failure-13 = la prioridad no es una prioridad de clase SR
msrp-failure-14 = tramas demasiado grandes para el medio
msrp-failure-15 = límite de puertos fan-in alcanzado
msrp-failure-16 = primer valor cambiado en un stream registrado
msrp-failure-17 = VLAN bloqueada en el puerto de salida
msrp-failure-18 = etiquetado VLAN desactivado en el puerto de salida
msrp-failure-19 = la prioridad de clase SR no coincide
msrp-failure-unknown = motivo desconocido
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, en el puente { $bridge }

## The macOS menu bar

menu-about = Acerca de plaa
menu-settings = Ajustes…
menu-services = Servicios
menu-hide = Ocultar plaa
menu-hide-others = Ocultar otros
menu-show-all = Mostrar todo
menu-quit = Salir de plaa
menu-file = Archivo
menu-open = Abrir…
menu-open-address = Abrir dirección…
menu-close = Cerrar ventana
menu-edit = Edición
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Pegar
menu-select-all = Seleccionar todo
menu-undo = Deshacer
menu-redo = Rehacer
menu-trim = Acortar…
menu-split = Dividir clip
menu-rotate-left = Girar a la izquierda
menu-rotate-right = Girar a la derecha
menu-flip-horizontal = Voltear horizontalmente
menu-flip-vertical = Voltear verticalmente
menu-revert = Volver al original
menu-export = Exportar…
menu-view = Visualización
menu-playback = Reproducción
menu-play-pause = Reproducir o pausar
menu-to-start = Ir al principio
menu-previous-chapter = Capítulo anterior
menu-next-chapter = Capítulo siguiente
menu-faster = Más rápido
menu-slower = Más lento
menu-normal-speed = Velocidad normal
menu-ab-loop = Bucle entre dos puntos
menu-volume-up = Subir volumen
menu-volume-down = Bajar volumen
menu-mute = Silenciar o activar sonido
menu-next-subtitles = Subtítulos siguientes
menu-next-audio = Pista de audio siguiente
menu-window = Ventana
menu-minimize = Minimizar
menu-zoom = Zoom
menu-bring-all-to-front = Traer todo al frente

## Editing

action-edit = Editar
title-edited = { $name } (editado)
edit-trim = Acortar
edit-trim-apply = Acortar
edit-split = Dividir clip
edit-remove-clip = Eliminar clip
edit-done = Listo
edit-rotate-left = Girar a la izquierda
edit-rotate-right = Girar a la derecha
edit-flip-horizontal = Voltear horizontalmente
edit-flip-vertical = Voltear verticalmente
edit-undo = Deshacer
edit-redo = Rehacer
edit-revert = Volver al original
edit-export = Exportar…

## Exporting

export-title = Exportar
export-format = Formato
export-size = Tamaño
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Solo sonido
export-size-original = { $lines }p, tal cual
export-on-graphics-card = Codificado en la tarjeta gráfica ({ $api })
export-on-processor = Codificado en el procesador, que tarda más
export-no-encoder = Aquí no se abre ningún codificador para este formato
export-checking = Buscando un codificador…
export-sound-format = Formato de sonido
export-quality = Calidad
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Comprimido; se reproduce casi en todas partes, incluidos los dispositivos de Apple
export-mp3-note = Comprimido; se reproduce en todas partes
export-opus-note = Comprimido; el mejor sonido para su tamaño, aunque no todos los reproductores lo admiten
export-flac-note = Sin pérdida: el sonido tal cual, en un formato abierto
export-alac-note = Sin pérdida: el sonido tal cual, como lo reproducen los dispositivos de Apple
export-wav-note = Sin comprimir: el sonido tal cual, en los archivos más grandes
export-kilobits = { $rate } kb/s
export-bits = { $bits } bits
export-rate-lower = Guardado a { $rate } kHz, por debajo de los { $file } kHz del archivo
export-choose = Exportar…
export-name = { $name } exportado
export-name-edited = { $name } editado
export-progress = Exportando “{ $name }”
export-stop = Detener la exportación
notice-exported = Se ha exportado “{ $name }”
notice-export-stopped = Exportación detenida
notice-export-failed = No se ha podido exportar: { $error }
close-title = ¿Cerrar sin exportar?
close-unexported = Los cambios en “{ $name }” se perderán si no los exportas.
close-exporting = La exportación de “{ $name }” se detiene y se pierde lo que haya escrito hasta ahora.
close-anyway = Cerrar
close-quit-anyway = Salir

## Recording

record-title = Grabación de sonido
record-file-name = Grabación de sonido
record-start = Grabar
record-stop = Detener la grabación
record-no-input = Sin micrófono
record-microphone = Micrófono
record-default-microphone = El predeterminado del sistema
record-playing = Lo que reproduce el ordenador
record-mono = mono
record-stereo = estéreo
record-channels = { $count ->
    [one] { $count } canal
   *[other] { $count } canales
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, guardado a { $saved } kHz
action-record-sound = Grabar sonido
open-record-sound = Nueva grabación de sonido
start-record-sound = Grabar sonido
menu-record-sound = Nueva grabación de sonido
record-screen-title = Grabación de pantalla
record-screen-file-name = Grabación de pantalla
record-screen-whole = Pantalla
record-screen-number = Pantalla { $number }
record-screen-main = Pantalla { $number } (principal)
record-screens = Pantallas
record-windows = Ventanas
record-screen-what = Qué grabar
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Todos los fotogramas
record-rate-tip = Fotogramas por segundo
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] { $count } descartado
   *[other] { $count } descartados
}
record-screen-window = Ventana
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Ya no se comparte
record-screen-again = Elegir de nuevo
record-screen-failed = No se ha podido grabar la pantalla
record-sound = Sonido
record-no-sound = Sin sonido
# How much louder the input is taken, for a quiet microphone.
record-gain = Más fuerte
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajustar el nivel automáticamente
record-hide-title = plaa se oculta mientras graba
record-hide-tray = Para detener la grabación, haz clic en el punto rojo que plaa pone en la bandeja del sistema, o vuelve a abrir plaa.
record-hide-reopen = Para detener la grabación, vuelve a abrir plaa.
record-hide-start = Empezar a grabar
record-tray-title = plaa está grabando la pantalla
record-tray-tip = Haz clic para detener la grabación
action-record-screen = Grabar pantalla
open-record-screen = Nueva grabación de pantalla
start-record-screen = Grabar pantalla
menu-record-screen = Nueva grabación de pantalla
record-camera-title = Grabación de cámara
record-camera-file-name = Grabación de cámara
record-camera = Cámara
record-camera-failed = No se ha podido abrir la cámara
record-name-title = Ponle nombre a la grabación
record-name-note = Se ha guardado como “{ $name }” en { $folder }.
record-name-label = Nombre
record-name-keep = Mantener
record-name-rename = Cambiar nombre
record-name-taken = Ya hay un archivo llamado “{ $name }” ahí.
record-name-invalid = Un nombre no puede contener ninguno de estos caracteres: / \ : * ? " < > |
record-name-failed = No se ha podido cambiar el nombre: { $error }
action-record-camera = Grabar cámara
open-record-camera = Nueva grabación de cámara
start-record-camera = Grabar cámara
menu-record-camera = Nueva grabación de cámara
notice-no-camera = No hay ninguna cámara para grabar
notice-recorded = Se ha guardado “{ $name }” en { $folder }
notice-record-failed = No se ha podido grabar: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Permitir el control externo
settings-allow-outside-control-note = Los agentes de IA como Claude Code pueden ver lo que se reproduce en plaa y controlarlo mediante plaa --mcp. plaa pregunta antes de cada agente nuevo.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Olvidar
settings-ask-before-note = Preguntar antes de que un agente:
settings-ask-reading = Vea lo que se reproduce o la imagen
settings-ask-playing = Controle la reproducción, el sonido o la pantalla completa
settings-ask-opening = Abra un archivo o cierre una ventana
settings-ask-editing = Edite un archivo o lo exporte
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = ¿Permitir que { $agent } controle plaa?
agent-prompt-body = { $agent } pide usar el control externo de plaa para ver lo que se reproduce y controlarlo. Puedes desactivar el control externo en Ajustes.
agent-prompt-allow = Permitir
agent-prompt-deny = No permitir
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = ¿Permitir que { $agent } vea lo que se reproduce aquí?
agent-ask-play = ¿Permitir que { $agent } controle la reproducción aquí?
agent-ask-open = ¿Permitir que { $agent } abra o cierre una ventana?
agent-ask-edit = ¿Permitir que { $agent } edite o exporte aquí?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } pide usar “{ $tool }”. En Ajustes eliges sobre qué pregunta plaa.

## Casting to a TV or speaker on the network

cast-looking = Buscando televisores y altavoces…
cast-none = No se ha encontrado ningún televisor, altavoz ni Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Dejar de reproducir en { $renderer }
notice-casting = Reproduciendo en { $renderer }.
notice-cast-failed = No se ha podido reproducir en { $renderer }: { $reason }
cast-web-page = Todavía no se pueden enviar páginas web: abre en su lugar la dirección del propio vídeo.

## The assistant panel, and its models in Settings

assistant-add-another = Añadir un modelo…
assistant-add-model = El asistente necesita un modelo: un modelo en la nube con su clave de API, o un modelo local.
assistant-ask = Pregunta sobre lo que se reproduce
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = El chat se estaba haciendo demasiado largo para que { $model } lo lea de una vez, así que se resumió su parte anterior. Puede que el asistente vuelva a mirar lo que vio.
assistant-compacting = Resumiendo la parte anterior del chat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } en este equipo
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } en { $host }
assistant-new-chat = Nuevo chat
assistant-no-model = Primero añade un modelo en Ajustes.
assistant-open-settings = Añadir un modelo
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = La conversación ya no cabe en lo que { $model } puede leer de una vez. Empieza un nuevo chat o elige un modelo que pueda leer más.
assistant-problem-credit = { $provider } indica que la cuenta no tiene crédito. Una cuenta nueva necesita comprar crédito en el sitio web de { $provider } antes de que su clave funcione; después vuelve a intentarlo.
assistant-problem-key = { $provider } ha rechazado la clave de API. Revísala en Ajustes.
assistant-problem-model = { $provider } no tiene ningún modelo llamado { $model }. Comprueba su nombre en Ajustes.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } no ha respondido: { $message }
assistant-problem-rate = { $provider } pide ir más despacio. Vuelve a intentarlo en un momento.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } pide ir más despacio: { $message }
assistant-problem-refused = { $model } ha declinado responder.
assistant-problem-unavailable = { $provider } está ocupado o tiene problemas. Vuelve a intentarlo en un momento.
assistant-problem-unreachable = plaa no ha podido conectar con { $provider }. Comprueba tu conexión o que el servidor esté en ejecución.
assistant-running = Ejecutando…
assistant-send = Enviar
assistant-shortened = El chat era demasiado largo para que { $model } lo lea de una vez, así que se omitieron resultados anteriores de las herramientas. Puede que el asistente vuelva a mirar lo que vio.
assistant-stop = Detener
assistant-stopped = Detenido.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Ahora hablas con { $model }.
assistant-thinking = Pensando…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Razonamiento
assistant-title = Asistente
settings-assistant-add = Añadir un modelo
settings-assistant-add-button = Añadir
# $model is the model's name.
settings-assistant-added = { $model } ha respondido y se ha añadido.
settings-assistant-added-tag = Añadido
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Dirección, como { $example }
settings-assistant-change-key = Cambiar clave
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Servidor compatible con OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Contexto
settings-assistant-context-note = Más contexto permite al asistente tener presente una parte mayor de un chat, pero el modelo ocupa más memoria y puede responder más despacio.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Abrir la página
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } se está ejecutando en { $address }.
settings-assistant-found-none = { $provider } aún no tiene modelos. Descarga uno con él y vuelve a buscar.
settings-assistant-get-key = Obtener una clave de API
settings-assistant-get-server = Obtener { $provider }
settings-assistant-in-use = En uso
settings-assistant-key = Clave de API
# $error is what the keychain said.
settings-assistant-key-failed = No se pudo guardar la clave en el llavero: { $error }
settings-assistant-key-kept = Tu clave de { $provider } se guarda en el llavero del sistema.
settings-assistant-key-needed = Este modelo necesita una clave de API.
settings-assistant-key-refused = { $provider } ha rechazado esta clave. Comprueba que se copió entera y desde la cuenta correcta.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crea una clave en el sitio web de { $provider } y pégala aquí.
settings-assistant-look-again = Buscar de nuevo
settings-assistant-looking = Buscando modelos…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modelo, como { $example }
settings-assistant-no-server = plaa no ha encontrado { $provider } en ejecución en este equipo. Inícialo o indica su dirección abajo.
settings-assistant-no-tools = No puede usar herramientas, que el asistente necesita
settings-assistant-none = Aún no hay modelos. Añade uno abajo: un modelo local, como los de Ollama, se queda en este equipo; un modelo en la nube necesita una clave de API de su proveedor.
settings-assistant-note = Los modelos con los que puede hablar el panel del asistente. Las claves se guardan en el llavero del sistema.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Usar otra dirección
settings-assistant-recommended = Recomendado
settings-assistant-remove = Eliminar
settings-assistant-sees = Ve imágenes
settings-assistant-trying = Probando…
settings-assistant-use = Usar
settings-assistant-use-key = Continuar
settings-assistant-uses-tools = Usa herramientas
