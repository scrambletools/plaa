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
language-name = Português (Brasil)

## Common

app-name = plaa
common-cancel = Cancelar
common-close = Fechar
common-more = Mais

## Command line

usage-help =
    Uso: plaa [ARQUIVO OU ENDEREÇO]...

    Reproduz arquivos de vídeo e áudio, e transmissões em endereços como
    https://example.com/live.m3u8. Cada um abre em uma janela do plaa em
    execução, que é iniciado se necessário.

    Opções:
      -h, --help        Mostra esta ajuda
      -V, --version     Mostra a versão
          --interfaces  Mostra as interfaces com fio e se o plaa pode enviar
                        streams AVB por elas
          --mcp         Serve MCP por stdin e stdout, para que agentes de IA
                        vejam o que o plaa em execução reproduz e o controlem

## Start window

start-hint = Abra um arquivo de vídeo ou áudio, uma transmissão ou um vídeo da web.
start-open = Abrir
start-open-address = Abrir endereço
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Conectar

## Actions, as buttons, tooltips and menus name them

action-open = Abrir
action-open-address = Abrir endereço
open-file = Abrir arquivo…
open-address = Abrir endereço…
action-settings = Ajustes
action-info = Inspetor
action-loop = Loop
action-screenshot = Salvar quadro como imagem
action-play = Reproduzir
action-pause = Pausar
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Voltar { $seconds } segundo
   *[other] Voltar { $seconds } segundos
}
action-forward = { $seconds ->
    [one] Avançar { $seconds } segundo
   *[other] Avançar { $seconds } segundos
}
action-mute = Silenciar
action-unmute = Ativar som
action-speed = Velocidade
action-tracks = Áudio e legendas
action-full-screen = Tela cheia
action-playlist = Lista de reprodução
action-chapters = Capítulos
action-picture-in-picture = Picture in Picture
action-previous = Anterior
action-next = Próximo
action-exit-full-screen = Sair da tela cheia
action-quality = Qualidade
action-visualizer = Visualizador
action-network = Áudio em rede
action-cast = Transmitir
action-connect = Conectar
action-assistant = Assistente

## What a sound file shows under its cover

visualizer-spectrum = Espectro
visualizer-waveform = Forma de onda
visualizer-prism = Prisma
visualizer-cover = Só a capa

## The floating controls

overlay-keep-shown = Manter os controles fixos
overlay-auto-hide = Deixar os controles flutuando sobre a imagem

## Audio and subtitle tracks

tracks-audio = Áudio
tracks-subtitles = Legendas
tracks-off = Desativadas
# A track the file gives no name or language, by its number.
tracks-numbered = Faixa { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Só áudio

## Inspector

info-title = Inspetor
playlist-title = Lista de reprodução
chapters-numbered = Capítulo { $number }
info-file = Arquivo
info-name = Nome
info-address = Endereço
info-folder = Pasta
info-size = Tamanho
info-format = Formato
info-duration = Duração
info-bit-rate = Taxa de bits
info-video = Vídeo
info-audio = Áudio
info-codec = Codec
info-dimensions = Dimensões
info-frame-rate = Taxa de quadros
info-frames-per-second = { $rate } quadros por segundo
info-pixel-format = Formato de pixel
info-decoding = Decodificação
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Placa de vídeo ({ $api })
info-decoding-software = Processador
info-sample-rate = Taxa de amostragem
info-hertz = { $rate } Hz
info-channels = Canais
info-language = Idioma
info-no-sound = Sem som
info-tags = Tags
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = Outros listeners
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Classe SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Um novo formato desconecta { $count ->
        [one] o outro listener do talker.
       *[other] os outros { $count } listeners do talker.
    }

## Dialogs

dialog-open = Abrir
dialog-filter-media = Vídeo e áudio
dialog-filter-all = Todos os arquivos

address-title = Abrir endereço
address-note = Uma página da web com um vídeo, como a de um vídeo do YouTube, ou uma transmissão ou um arquivo na rede, como https://example.com/live.m3u8.
address-label = Endereço
address-open = Abrir
address-recent = Recentes
address-forget = Remover dos recentes

## Notices

notice-dialog-failed = Não foi possível abrir o diálogo de arquivos: { $error }
notice-screenshot-saved = “{ $name }” salvo.
notice-screenshot-failed = Não foi possível salvar a imagem: { $error }
notice-no-sound = Reproduzindo sem som: { $reason }
notice-loop-from = Loop a partir de { $time }. Pressione de novo onde ele deve terminar.
notice-loop-between = Loop de { $from } a { $to }.
notice-loop-off = Loop desativado.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Isto parece uma página da web. O plaa reproduz os vídeos de páginas da web com o yt-dlp, que não está instalado.
error-empty-list = Esta lista não tem nada para reproduzir.

## Settings

settings-title = Ajustes
settings-tab-appearance = Aparência
settings-tab-playback = Reprodução
settings-tab-storage = Armazenamento
settings-tab-network = Rede
settings-tab-language = Idioma
settings-tab-agents = Agentes
settings-tab-assistant = Assistente
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versão de desenvolvimento, { $build })
settings-choose = Escolher…

settings-appearance-system = Seguir o sistema
settings-appearance-light = Claro
settings-appearance-dark = Escuro
settings-colors = Cores
settings-system-accent = Usar a cor de destaque do sistema
settings-omarchy-note = As cores são criadas a partir da cor de destaque de “{ $theme }”.
settings-system-accent-note = As cores são criadas a partir da cor de destaque do sistema.
settings-system-accent-none = O sistema não tem cor de destaque, então o plaa usa a cor escolhida abaixo.
settings-accent-chosen-note = As cores são criadas a partir da cor escolhida abaixo.
settings-controls = Controles
settings-overlay = Controles flutuantes
settings-overlay-note = A barra de ferramentas e os controles de reprodução flutuam sobre a imagem ou a capa e se recolhem enquanto o ponteiro está parado ou fora da janela.
settings-overlay-delay = Ocultar após
settings-overlay-delay-note = Quanto tempo o ponteiro fica parado sobre o que está tocando antes que os controles se recolham.
settings-seconds = { $seconds } s
settings-transparency = Transparência
settings-transparency-note = Quanto da imagem aparece através dos controles flutuantes.
settings-percent = { $percent }%
settings-corner-radius = Raio dos cantos
settings-corner-radius-note = Para diálogos e os controles flutuantes.
settings-pixels = { $pixels } px
settings-animations = Animações
settings-animations-note = Barras e painéis deslizantes, diálogos que crescem e botões com efeito de mola.
settings-animations-reduced = Desativadas enquanto o sistema pedir movimento reduzido.

settings-hardware = Usar a placa de vídeo
settings-hardware-note = Decodifica vídeos e codifica exportações na placa de vídeo: usa menos energia, reproduz vídeos grandes com fluidez e exporta mais rápido. Vale para arquivos abertos depois.
settings-audio-device = Saída de som
settings-audio-device-system = Padrão do sistema
settings-resume = Continuar de onde parou
settings-resume-note = Os arquivos começam onde estavam quando foram fechados pela última vez.
settings-folder-playlists = Reproduzir os outros arquivos da pasta
settings-folder-playlists-note = Ao abrir um arquivo, os outros da mesma pasta entram na lista e são reproduzidos depois dele.
settings-streams = Transmissões e vídeos da web
settings-stream-quality = Qualidade
settings-stream-quality-note = Para transmissões e vídeos da web disponíveis em várias qualidades. Vale para o que for aberto depois.
settings-quality-auto = Automática, da altura da tela
settings-quality-best = Máxima
settings-web-pages = Reproduzir vídeos da web com o yt-dlp
settings-web-pages-found = O yt-dlp encontra o vídeo em páginas como as do YouTube.
settings-web-pages-missing = O yt-dlp não está instalado. Instale-o para reproduzir os vídeos de páginas como as do YouTube.
settings-default-app = App padrão
settings-default-app-label = Abrir arquivos com o plaa
settings-default-app-note = Faça do plaa o app que abre arquivos de vídeo e de som.
settings-default-app-note-windows = O Windows só permite escolher apps padrão nas próprias Configurações. Isto abre a página do plaa lá.
settings-default-app-note-macos = O macOS pede para confirmar cada tipo: MP4, MOV, MKV, WebM, MP3, M4A, FLAC e WAV.
settings-default-app-status = { $set } de { $total } tipos de arquivo abrem com o plaa.
settings-default-app-button = Definir como padrão
settings-default-app-button-windows = Abrir Configurações
settings-default-app-no-entry = A entrada de área de trabalho do plaa não está instalada, então o sistema não pode abrir arquivos com ele. Instale o plaa de um pacote.
settings-default-app-no-bundle = Abra o plaa pelo plaa.app para defini-lo como padrão.
settings-default-app-failed = Não foi possível definir o plaa como padrão: { $error }
settings-folders = Pastas
settings-sound-recordings = Gravações de som
settings-video-recordings = Gravações de vídeo
settings-screenshots = Quadros salvos

settings-interface-language = Idioma da interface
# The first entry of the language list.
settings-language-system = Seguir o sistema
settings-track-languages = Idiomas das faixas
settings-track-languages-note = Códigos de idioma como en ou fra, em ordem de preferência, separados por vírgulas. O plaa escolhe áudio e legendas nesses idiomas quando um arquivo os tem.
settings-audio-languages = Áudio
settings-subtitle-languages = Legendas

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interface AVB
settings-avb-interface-note = A porta com fio em que o plaa encontra talkers e listeners AVB e Milan. Para enviar, é preciso uma porta com relógio de hardware PTP, como a de uma Intel I210.
settings-avb-off = Desativada
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (só lista: sem relógio PTP)
settings-avb-off-mac = { $interface } (só lista: AVB desativado)
settings-avb-interface-note-mac = A porta com fio em que o plaa encontra talkers e listeners AVB e Milan. O envio passa pela entidade AVB própria deste Mac, que precisa do AVB ativado para a porta no Navegador de Dispositivos de Rede da Configuração de Áudio e MIDI.
settings-avb-talker-note-mac = O plaa envia o som da última janela reproduzida pela entidade AVB própria deste Mac aos listeners conectados a ela.
settings-avb-talker = Enviar o que está tocando
settings-avb-talker-note = O plaa se torna um talker Milan e envia o som da última janela reproduzida aos listeners conectados a ele.
settings-avb-name = Nome na rede
settings-avb-name-note = Os controladores mostram o plaa como “{ $name }”. Pressione Enter para renomeá-lo.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa em { $computer }
avb-could-not-start = Não foi possível usar { $interface }: { $error }
network-title = Áudio em rede
network-off = Ative “Enviar o que está tocando” nos Ajustes para reproduzir na rede.
network-blocked = Sem envio: { $reason }
network-waiting = Pronto para enviar, aguardando um listener se conectar.
network-sending = Enviando aos listeners conectados.
network-looking = Procurando listeners…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Iniciando o ptp4l, que mantém o horário de rede pelo qual o stream do plaa é enviado…
network-ptp-elsewhere = O ptp4l está em execução em outra interface, então o stream do plaa não tem horário de rede aqui e os listeners podem não reproduzi-lo. Escolha essa interface nos Ajustes ou execute o ptp4l nesta.
network-ptp-missing = O ptp4l não está em execução, então o stream do plaa não tem horário de rede e os listeners podem não reproduzi-lo. Instale o linuxptp com o pacote do plaa, ou instale o triib, que o executa.
network-ptp-failed = O ptp4l não está em execução, então o stream do plaa não tem horário de rede e os listeners podem não reproduzi-lo. O plaa não conseguiu iniciá-lo: { $reason }
network-no-listeners = Nenhum listener na rede pode reproduzir o som do plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Reproduzir em { $listener }
network-stop-on = Parar de reproduzir em { $listener }
network-choose-format = Escolha um formato para reproduzir em { $listener }
network-failed = Não foi possível conectar: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Atualize o triib para reproduzir aqui.
network-choose = Escolha um listener para ver como ele recebe o stream.
network-no-sources = Nenhum talker na rede envia em um formato que o plaa reproduz, nem pode mudar para um.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Parar de ouvir { $talker }
network-listen-update-triib = Atualize o triib para ouvir aqui: o plaa só reproduz o que chega a este computador de qualquer forma.
network-listen-lite = A porta usa AVB Lite, que não reserva streams: o plaa só reproduz o que chega a este computador de qualquer forma.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Procurando talkers…
# plaa listens through its entity, which exists only while it sends.
connect-off = Ative “Enviar o que está tocando” nos Ajustes para ouvir talkers aqui.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = O plaa não pode ouvir aqui: { $reason }
connect-sending = Enviando em um formato que o plaa reproduz
connect-changing = Mudando para um formato que o plaa reproduz
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, em { $format }
connect-change-cuts-off = { $talker }, em { $format }, desconectando { $count ->
        [one] seu outro listener
       *[other] seus outros { $count } listeners
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Não conectado
network-from = De { $stream }
network-from-receiving = De { $stream }, recebendo
network-from-waiting = De { $stream }, aguardando o talker
network-from-failed = De { $stream }, a reserva do talker falhou: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Relógio de mídia
network-locked = Sincronizado com o stream
network-not-locked = Não sincronizado
network-format = Formato
# The formats both the listener and plaa's stream can be in.
network-formats = Formatos que ele aceita do plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID do stream
# The MAC address the stream is sent to.
network-destination = Destino
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latência
network-microseconds = { $time } µs
network-frames = Quadros recebidos
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Quadros fora de sequência
# How often the stream stopped coming while it was taken.
network-interrupted = Interrupções
# Frames that came after, or long before, their time to be played.
network-late = Quadros atrasados
network-early = Quadros adiantados
network-entity-id = ID da entidade

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = largura de banda insuficiente
msrp-failure-2 = recursos insuficientes na ponte
msrp-failure-3 = largura de banda insuficiente para a classe de tráfego
msrp-failure-4 = ID de stream em uso por outro talker
msrp-failure-5 = endereço de destino já em uso
msrp-failure-6 = substituído por um stream de nível mais alto
msrp-failure-7 = a latência informada mudou
msrp-failure-8 = a porta de saída não suporta AVB
msrp-failure-9 = use outro endereço de destino
msrp-failure-10 = sem recursos MSRP
msrp-failure-11 = sem recursos MMRP
msrp-failure-12 = não é possível armazenar o endereço de destino
msrp-failure-13 = a prioridade não é uma prioridade de classe SR
msrp-failure-14 = quadros grandes demais para o meio
msrp-failure-15 = limite de portas fan-in atingido
msrp-failure-16 = primeiro valor alterado para um stream registrado
msrp-failure-17 = VLAN bloqueada na porta de saída
msrp-failure-18 = marcação de VLAN desativada na porta de saída
msrp-failure-19 = prioridade de classe SR incompatível
msrp-failure-unknown = motivo desconhecido
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, na ponte { $bridge }

## The macOS menu bar

menu-about = Sobre o plaa
menu-settings = Ajustes…
menu-services = Serviços
menu-hide = Ocultar plaa
menu-hide-others = Ocultar Outros
menu-show-all = Mostrar Tudo
menu-quit = Encerrar plaa
menu-file = Arquivo
menu-open = Abrir…
menu-open-address = Abrir Endereço…
menu-close = Fechar Janela
menu-edit = Editar
menu-cut = Recortar
menu-copy = Copiar
menu-paste = Colar
menu-select-all = Selecionar Tudo
menu-undo = Desfazer
menu-redo = Refazer
menu-trim = Aparar…
menu-split = Dividir Clipe
menu-rotate-left = Girar para a Esquerda
menu-rotate-right = Girar para a Direita
menu-flip-horizontal = Espelhar na Horizontal
menu-flip-vertical = Espelhar na Vertical
menu-revert = Reverter ao Original
menu-export = Exportar…
menu-view = Visualizar
menu-playback = Reprodução
menu-play-pause = Reproduzir ou Pausar
menu-to-start = Ir para o Início
menu-previous-chapter = Capítulo Anterior
menu-next-chapter = Próximo Capítulo
menu-faster = Mais Rápido
menu-slower = Mais Lento
menu-normal-speed = Velocidade Normal
menu-ab-loop = Loop Entre Dois Pontos
menu-volume-up = Aumentar Volume
menu-volume-down = Diminuir Volume
menu-mute = Silenciar ou Ativar Som
menu-next-subtitles = Próximas Legendas
menu-next-audio = Próxima Faixa de Áudio
menu-window = Janela
menu-minimize = Minimizar
menu-zoom = Zoom
menu-bring-all-to-front = Trazer Todas para a Frente

## Editing

action-edit = Editar
title-edited = { $name } (editado)
edit-trim = Aparar
edit-trim-apply = Aparar
edit-split = Dividir clipe
edit-remove-clip = Remover clipe
edit-done = Concluído
edit-rotate-left = Girar para a esquerda
edit-rotate-right = Girar para a direita
edit-flip-horizontal = Espelhar na horizontal
edit-flip-vertical = Espelhar na vertical
edit-undo = Desfazer
edit-redo = Refazer
edit-revert = Reverter ao original
edit-export = Exportar…

## Exporting

export-title = Exportar
export-format = Formato
export-size = Tamanho
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Só som
export-size-original = { $lines }p, como está
export-on-graphics-card = Codificado na placa de vídeo ({ $api })
export-on-processor = Codificado no processador, o que demora mais
export-no-encoder = Nenhum codificador para este formato abre aqui
export-checking = Procurando um codificador…
export-sound-format = Formato de som
export-quality = Qualidade
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Comprimido; toca em quase todo lugar, inclusive nos dispositivos da Apple
export-mp3-note = Comprimido; toca em todo lugar
export-opus-note = Comprimido; o melhor som para o tamanho, embora nem todo player o aceite
export-flac-note = Sem perdas: o som exatamente como é, em um formato aberto
export-alac-note = Sem perdas: o som exatamente como é, do jeito que os dispositivos da Apple o reproduzem
export-wav-note = Sem compressão: o som exatamente como é, nos maiores arquivos
export-kilobits = { $rate } kb/s
export-bits = { $bits } bits
export-rate-lower = Salvo a { $rate } kHz, abaixo dos { $file } kHz do arquivo
export-choose = Exportar…
export-name = { $name } exportado
export-name-edited = { $name } editado
export-progress = Exportando “{ $name }”
export-stop = Parar a exportação
notice-exported = “{ $name }” exportado
notice-export-stopped = Exportação interrompida
notice-export-failed = Não foi possível exportar: { $error }
close-title = Fechar sem exportar?
close-unexported = As alterações em “{ $name }” serão perdidas se você não as exportar.
close-exporting = A exportação de “{ $name }” será interrompida, e o que já foi gravado será perdido.
close-anyway = Fechar
close-quit-anyway = Encerrar

## Recording

record-title = Gravação de som
record-file-name = Gravação de som
record-start = Gravar
record-stop = Parar a gravação
record-no-input = Nenhum microfone
record-microphone = Microfone
record-default-microphone = O padrão do sistema
record-playing = O que o computador reproduz
record-mono = mono
record-stereo = estéreo
record-channels = { $count ->
    [one] { $count } canal
   *[other] { $count } canais
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, salvo a { $saved } kHz
action-record-sound = Gravar som
open-record-sound = Nova gravação de som
start-record-sound = Gravar som
menu-record-sound = Nova Gravação de Som
record-screen-title = Gravação de tela
record-screen-file-name = Gravação de tela
record-screen-whole = Tela
record-screen-number = Tela { $number }
record-screen-main = Tela { $number } (principal)
record-screens = Telas
record-windows = Janelas
record-screen-what = O que gravar
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Todos os quadros
record-rate-tip = Quadros por segundo
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] { $count } descartado
   *[other] { $count } descartados
}
record-screen-window = Janela
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Não está mais compartilhado
record-screen-again = Escolher de novo
record-screen-failed = Não foi possível gravar a tela
record-sound = Som
record-no-sound = Sem som
# How much louder the input is taken, for a quiet microphone.
record-gain = Mais alto
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajustar o nível automaticamente
record-hide-title = O plaa fica oculto enquanto grava
record-hide-tray = Para parar a gravação, clique no ponto vermelho que o plaa coloca na bandeja do sistema ou abra o plaa de novo.
record-hide-reopen = Para parar a gravação, abra o plaa de novo.
record-hide-start = Iniciar gravação
record-tray-title = O plaa está gravando a tela
record-tray-tip = Clique para parar a gravação
action-record-screen = Gravar tela
open-record-screen = Nova gravação de tela
start-record-screen = Gravar tela
menu-record-screen = Nova Gravação de Tela
record-camera-title = Gravação da câmera
record-camera-file-name = Gravação da câmera
record-camera = Câmera
record-camera-failed = Não foi possível abrir a câmera
record-name-title = Dê um nome à gravação
record-name-note = Ela foi salva como “{ $name }” em { $folder }.
record-name-label = Nome
record-name-keep = Manter
record-name-rename = Renomear
record-name-taken = Já existe um arquivo chamado “{ $name }” lá.
record-name-invalid = Um nome não pode ter nenhum destes caracteres: / \ : * ? " < > |
record-name-failed = Não foi possível renomear: { $error }
action-record-camera = Gravar câmera
open-record-camera = Nova gravação da câmera
start-record-camera = Gravar câmera
menu-record-camera = Nova Gravação da Câmera
notice-no-camera = Não há câmera para gravar
notice-recorded = “{ $name }” salvo em { $folder }
notice-record-failed = Não foi possível gravar: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Permitir controle externo
settings-allow-outside-control-note = Agentes de IA como o Claude Code podem ver o que toca no plaa e controlá-lo, por meio do plaa --mcp. O plaa pergunta antes de cada novo agente.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Esquecer
settings-ask-before-note = Perguntar antes que um agente:
settings-ask-reading = Veja o que está tocando, ou a imagem
settings-ask-playing = Controle a reprodução, o som ou a tela cheia
settings-ask-opening = Abra um arquivo ou feche uma janela
settings-ask-editing = Edite um arquivo ou o exporte
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Permitir que { $agent } controle o plaa?
agent-prompt-body = { $agent } pede para usar o controle externo do plaa, para ver o que está tocando e controlá-lo. Você pode desativar o controle externo em Ajustes.
agent-prompt-allow = Permitir
agent-prompt-deny = Não permitir
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Permitir que { $agent } veja o que está tocando aqui?
agent-ask-play = Permitir que { $agent } controle a reprodução aqui?
agent-ask-open = Permitir que { $agent } abra ou feche uma janela?
agent-ask-edit = Permitir que { $agent } edite ou exporte aqui?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } pede para usar “{ $tool }”. Em Ajustes, você escolhe sobre o que o plaa pergunta.

## Casting to a TV or speaker on the network

cast-looking = Procurando TVs e alto-falantes…
cast-none = Nenhuma TV, alto-falante ou Chromecast foi encontrado.
# $renderer is the TV's or speaker's name.
cast-stop = Parar de reproduzir em { $renderer }
notice-casting = Reproduzindo em { $renderer }.
notice-cast-failed = Não foi possível reproduzir em { $renderer }: { $reason }
cast-web-page = Ainda não é possível transmitir páginas da web: abra o endereço do próprio vídeo.

## The assistant panel, and its models in Settings

assistant-add-another = Adicionar um modelo…
assistant-add-model = O assistente precisa de um modelo: um modelo na nuvem com sua chave de API, ou um modelo local.
assistant-ask = Pergunte sobre o que está tocando
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = O chat estava ficando longo demais para { $model } ler de uma vez, então a parte anterior foi resumida. O assistente pode olhar de novo o que viu.
assistant-compacting = Resumindo a parte anterior do chat
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } neste computador
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } em { $host }
assistant-new-chat = Nova conversa
assistant-no-model = Primeiro, adicione um modelo em Ajustes.
assistant-open-settings = Adicionar um modelo
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = A conversa não cabe mais no que { $model } consegue ler de uma vez. Comece uma nova conversa ou escolha um modelo que consiga ler mais.
assistant-problem-credit = { $provider } diz que a conta não tem crédito. Uma conta nova precisa comprar crédito no site de { $provider } antes que a chave funcione; depois, tente novamente.
assistant-problem-key = { $provider } recusou a chave de API. Verifique-a em Ajustes.
assistant-problem-model = { $provider } não tem nenhum modelo chamado { $model }. Verifique o nome em Ajustes.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } não respondeu: { $message }
assistant-problem-rate = { $provider } pede para ir mais devagar. Tente novamente daqui a pouco.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } pede para ir mais devagar: { $message }
assistant-problem-refused = { $model } se recusou a responder.
assistant-problem-unavailable = { $provider } está ocupado ou com problemas. Tente novamente daqui a pouco.
assistant-problem-unreachable = O plaa não conseguiu acessar { $provider }. Verifique sua conexão ou se o servidor está em execução.
assistant-running = Executando…
assistant-send = Enviar
assistant-shortened = O chat estava longo demais para { $model } ler de uma vez, então resultados anteriores das ferramentas foram deixados de fora. O assistente pode olhar de novo o que viu.
assistant-stop = Parar
assistant-stopped = Parado.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Agora conversando com { $model }.
assistant-thinking = Pensando…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Raciocínio
assistant-title = Assistente
settings-assistant-add = Adicionar um modelo
settings-assistant-add-button = Adicionar
# $model is the model's name.
settings-assistant-added = { $model } respondeu e foi adicionado.
settings-assistant-added-tag = Adicionado
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Endereço, como { $example }
settings-assistant-change-key = Trocar chave
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Servidor compatível com OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Contexto
settings-assistant-context-note = Mais contexto permite que o assistente tenha em mente uma parte maior de uma conversa, mas o modelo usa mais memória e pode responder mais devagar.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Abrir a página
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } está em execução em { $address }.
settings-assistant-found-none = { $provider } ainda não tem modelos. Baixe um com ele e procure novamente.
settings-assistant-get-key = Obter uma chave de API
settings-assistant-get-server = Obter { $provider }
settings-assistant-in-use = Em uso
settings-assistant-key = Chave de API
# $error is what the keychain said.
settings-assistant-key-failed = Não foi possível guardar a chave no chaveiro: { $error }
settings-assistant-key-kept = Sua chave de { $provider } fica guardada no chaveiro do sistema.
settings-assistant-key-needed = Este modelo precisa de uma chave de API.
settings-assistant-key-refused = { $provider } recusou esta chave. Verifique se ela foi copiada inteira, da conta certa.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crie uma chave no site de { $provider } e cole aqui.
settings-assistant-look-again = Procurar novamente
settings-assistant-looking = Procurando modelos…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modelo, como { $example }
settings-assistant-no-server = O plaa não encontrou { $provider } em execução neste computador. Inicie-o ou informe o endereço dele abaixo.
settings-assistant-no-tools = Não pode usar ferramentas, de que o assistente precisa
settings-assistant-none = Ainda não há modelos. Adicione um abaixo: um modelo local, como os do Ollama, fica neste computador; um modelo na nuvem precisa de uma chave de API do seu provedor.
settings-assistant-note = Os modelos com que o painel do assistente pode conversar. As chaves ficam guardadas no chaveiro do sistema.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Usar outro endereço
settings-assistant-recommended = Recomendado
settings-assistant-remove = Remover
settings-assistant-sees = Vê imagens
settings-assistant-trying = Testando…
settings-assistant-use = Usar
settings-assistant-use-key = Continuar
settings-assistant-uses-tools = Usa ferramentas
