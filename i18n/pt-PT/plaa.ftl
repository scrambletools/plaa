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
language-name = Português (Portugal)

## Common

app-name = plaa
common-cancel = Cancelar
common-close = Fechar
common-more = Mais

## Command line

usage-help =
    Utilização: plaa [FICHEIRO OU ENDEREÇO]...

    Reproduz ficheiros de vídeo e áudio, e transmissões em endereços como
    https://example.com/live.m3u8. Cada um abre numa janela do plaa em
    execução, que é iniciado se for preciso.

    Opções:
      -h, --help        Mostra esta ajuda
      -V, --version     Mostra a versão
          --interfaces  Mostra as interfaces com fios e se o plaa pode enviar
                        streams AVB por elas
          --mcp         Serve MCP por stdin e stdout, para que agentes de IA
                        vejam o que o plaa em execução reproduz e o controlem

## Start window

start-hint = Abra um ficheiro de vídeo ou áudio, uma transmissão ou um vídeo da web.
start-open = Abrir
start-open-address = Abrir endereço

## Actions, as buttons, tooltips and menus name them

action-open = Abrir
action-open-address = Abrir endereço
open-file = Abrir ficheiro…
open-address = Abrir endereço…
action-settings = Definições
action-info = Inspetor
action-loop = Ciclo
action-screenshot = Guardar fotograma como imagem
action-play = Reproduzir
action-pause = Pausa
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Recuar { $seconds } segundo
   *[other] Recuar { $seconds } segundos
}
action-forward = { $seconds ->
    [one] Avançar { $seconds } segundo
   *[other] Avançar { $seconds } segundos
}
action-mute = Desativar som
action-unmute = Ativar som
action-speed = Velocidade
action-tracks = Áudio e legendas
action-full-screen = Ecrã completo
action-playlist = Lista de reprodução
action-chapters = Capítulos
action-picture-in-picture = Imagem em imagem
action-previous = Anterior
action-next = Seguinte
action-exit-full-screen = Sair do ecrã completo
action-quality = Qualidade
action-visualizer = Visualizador
action-network = Áudio em rede
action-cast = Transmitir
action-assistant = Assistente

## What a sound file shows under its cover

visualizer-spectrum = Espetro
visualizer-waveform = Forma de onda
visualizer-prism = Prisma
visualizer-cover = Só a capa

## The floating controls

overlay-keep-shown = Manter os controlos fixos
overlay-auto-hide = Deixar os controlos a flutuar sobre a imagem

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
info-file = Ficheiro
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
info-frame-rate = Taxa de fotogramas
info-frames-per-second = { $rate } fotogramas por segundo
info-pixel-format = Formato de píxel
info-decoding = Descodificação
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Placa gráfica ({ $api })
info-decoding-software = Processador
info-sample-rate = Taxa de amostragem
info-hertz = { $rate } Hz
info-channels = Canais
info-language = Idioma
info-no-sound = Sem som
info-tags = Etiquetas

## Dialogs

dialog-open = Abrir
dialog-filter-media = Vídeo e áudio
dialog-filter-all = Todos os ficheiros

address-title = Abrir endereço
address-note = Uma página web com um vídeo, como a de um vídeo do YouTube, ou uma transmissão ou um ficheiro na rede, como https://example.com/live.m3u8.
address-label = Endereço
address-open = Abrir
address-recent = Recentes
address-forget = Remover dos recentes

## Notices

notice-dialog-failed = Não foi possível abrir a caixa de diálogo de ficheiros: { $error }
notice-screenshot-saved = «{ $name }» guardado.
notice-screenshot-failed = Não foi possível guardar a imagem: { $error }
notice-no-sound = A reproduzir sem som: { $reason }
notice-loop-from = Ciclo a partir de { $time }. Prima novamente onde deve terminar.
notice-loop-between = Ciclo de { $from } a { $to }.
notice-loop-off = Ciclo desativado.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Isto parece uma página web. O plaa reproduz os vídeos das páginas web com o yt-dlp, que não está instalado.
error-empty-list = Esta lista não tem nada para reproduzir.

## Settings

settings-title = Definições
settings-tab-appearance = Aspeto
settings-tab-playback = Reprodução
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
settings-system-accent = Utilizar a cor de destaque do sistema
settings-omarchy-note = As cores são criadas a partir da cor de destaque de «{ $theme }».
settings-system-accent-note = As cores são criadas a partir da cor de destaque do sistema.
settings-system-accent-none = O sistema não tem cor de destaque, pelo que o plaa utiliza a cor escolhida abaixo.
settings-accent-chosen-note = As cores são criadas a partir da cor escolhida abaixo.
settings-controls = Controlos
settings-overlay = Controlos flutuantes
settings-overlay-note = A barra de ferramentas e os controlos de reprodução flutuam sobre a imagem ou a capa e recolhem-se enquanto o ponteiro está parado ou fora da janela.
settings-overlay-delay = Ocultar após
settings-overlay-delay-note = Quanto tempo o ponteiro fica parado sobre o que está a ser reproduzido antes de os controlos se recolherem.
settings-seconds = { $seconds } s
settings-transparency = Transparência
settings-transparency-note = Quanto da imagem se vê através dos controlos flutuantes.
settings-percent = { $percent }%
settings-corner-radius = Raio dos cantos
settings-corner-radius-note = Para caixas de diálogo e os controlos flutuantes.
settings-pixels = { $pixels } px
settings-animations = Animações
settings-animations-note = Barras e painéis deslizantes, caixas de diálogo que crescem e botões com efeito de mola.
settings-animations-reduced = Desativadas enquanto o sistema pedir movimento reduzido.

settings-hardware = Utilizar a placa gráfica
settings-hardware-note = Descodifica vídeos e codifica exportações na placa gráfica: gasta menos energia, reproduz vídeos grandes com fluidez e exporta mais depressa. Aplica-se aos ficheiros abertos a seguir.
settings-audio-device = Saída de som
settings-audio-device-system = Predefinição do sistema
settings-resume = Retomar onde ficou
settings-resume-note = Os ficheiros começam onde estavam quando foram fechados pela última vez.
settings-folder-playlists = Reproduzir os outros ficheiros da pasta
settings-folder-playlists-note = Ao abrir um ficheiro, os outros da mesma pasta entram na lista e são reproduzidos a seguir.
settings-streams = Transmissões e vídeos da web
settings-stream-quality = Qualidade
settings-stream-quality-note = Para transmissões e vídeos da web disponíveis em várias qualidades. Aplica-se ao que for aberto a seguir.
settings-quality-auto = Automática, com a altura do ecrã
settings-quality-best = Máxima
settings-web-pages = Reproduzir vídeos da web com o yt-dlp
settings-web-pages-found = O yt-dlp encontra o vídeo em páginas como as do YouTube.
settings-web-pages-missing = O yt-dlp não está instalado. Instale-o para reproduzir os vídeos de páginas como as do YouTube.
settings-folders = Pastas
settings-recordings = Gravações
settings-screenshots = Fotogramas guardados

settings-interface-language = Idioma da interface
# The first entry of the language list.
settings-language-system = Seguir o sistema
settings-track-languages = Idiomas das faixas
settings-track-languages-note = Códigos de idioma como en ou fra, por ordem de preferência, separados por vírgulas. O plaa escolhe o áudio e as legendas nestes idiomas quando um ficheiro os tem.
settings-audio-languages = Áudio
settings-subtitle-languages = Legendas

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interface AVB
settings-avb-interface-note = A porta com fios em que o plaa encontra talkers e listeners AVB e Milan. O envio requer uma porta com relógio de hardware PTP, como a de uma Intel I210.
settings-avb-off = Desativada
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (só lista: sem relógio PTP)
settings-avb-off-mac = { $interface } (só lista: AVB desativado)
settings-avb-interface-note-mac = A porta com fios em que o plaa encontra talkers e listeners AVB e Milan. O envio passa pela entidade AVB própria deste Mac, que precisa de ter o AVB ativado para a porta no navegador de dispositivos de rede da Configuração de áudio e MIDI.
settings-avb-talker-note-mac = O plaa envia o som da última janela reproduzida através da entidade AVB própria deste Mac para os listeners ligados a ela.
settings-avb-talker = Enviar o que está a tocar
settings-avb-talker-note = O plaa torna-se um talker Milan e envia o som da última janela reproduzida para os listeners ligados a ele.
settings-avb-name = Nome na rede
settings-avb-name-note = Os controladores mostram o plaa como «{ $name }». Prima Enter para lhe mudar o nome.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa em { $computer }
avb-could-not-start = Não foi possível utilizar { $interface }: { $error }
network-title = Áudio em rede
network-off = Ative «Enviar o que está a tocar» nas Definições para reproduzir na rede.
network-blocked = Sem envio: { $reason }
network-waiting = Pronto para enviar, à espera que um listener se ligue.
network-sending = A enviar para os listeners ligados.
network-looking = A procurar listeners…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = A iniciar o ptp4l, que mantém a hora de rede pela qual o stream do plaa é enviado…
network-ptp-elsewhere = O ptp4l está em execução noutra interface, pelo que o stream do plaa não tem hora de rede aqui e os listeners podem não o reproduzir. Escolha essa interface nas Definições ou execute o ptp4l nesta.
network-ptp-missing = O ptp4l não está em execução, pelo que o stream do plaa não tem hora de rede e os listeners podem não o reproduzir. Instale o linuxptp com o pacote do plaa, ou instale o triib, que o executa.
network-ptp-failed = O ptp4l não está em execução, pelo que o stream do plaa não tem hora de rede e os listeners podem não o reproduzir. O plaa não conseguiu iniciá-lo: { $reason }
network-no-listeners = Nenhum listener na rede consegue reproduzir o som do plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Reproduzir em { $listener }
network-stop-on = Parar de reproduzir em { $listener }
network-choose-format = Escolha um formato para reproduzir em { $listener }
network-failed = Não foi possível ligar: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Atualize o triib para reproduzir aqui.
network-choose = Escolha um listener para ver como recebe o stream.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Ouvir aqui
network-no-talkers = Nenhum talker na rede envia num formato que o plaa consiga reproduzir.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Ouvir { $talker }
network-listen-stop = Parar de ouvir { $talker }
network-listen-frames = Tramas recebidas: { $frames }
network-listen-update-triib = Atualize o triib para ouvir aqui: o plaa só reproduz o que chega a este computador de qualquer forma.
network-listen-lite = A porta utiliza AVB Lite, que não reserva streams: o plaa só reproduz o que chega a este computador de qualquer forma.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Estado
network-not-connected = Não ligado
network-from = De { $stream }
network-from-receiving = De { $stream }, a receber
network-from-waiting = De { $stream }, à espera do talker
network-from-failed = De { $stream }, a reserva do talker falhou: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Relógio de multimédia
network-locked = Sincronizado com o stream
network-not-locked = Não sincronizado
network-format = Formato
# The formats both the listener and plaa's stream can be in.
network-formats = Formatos que aceita do plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID do stream
# The MAC address the stream is sent to.
network-destination = Destino
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latência
network-microseconds = { $time } µs
network-frames = Tramas recebidas
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Tramas fora de sequência
# How often the stream stopped coming while it was taken.
network-interrupted = Interrupções
network-entity-id = ID da entidade

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = largura de banda insuficiente
msrp-failure-2 = recursos insuficientes na ponte
msrp-failure-3 = largura de banda insuficiente para a classe de tráfego
msrp-failure-4 = ID de stream em uso por outro talker
msrp-failure-5 = endereço de destino já em uso
msrp-failure-6 = substituído por um stream de nível superior
msrp-failure-7 = a latência comunicada mudou
msrp-failure-8 = a porta de saída não suporta AVB
msrp-failure-9 = utilize outro endereço de destino
msrp-failure-10 = sem recursos MSRP
msrp-failure-11 = sem recursos MMRP
msrp-failure-12 = não é possível guardar o endereço de destino
msrp-failure-13 = a prioridade não é uma prioridade de classe SR
msrp-failure-14 = tramas demasiado grandes para o meio
msrp-failure-15 = limite de portas fan-in atingido
msrp-failure-16 = primeiro valor alterado num stream registado
msrp-failure-17 = VLAN bloqueada na porta de saída
msrp-failure-18 = etiquetagem de VLAN desativada na porta de saída
msrp-failure-19 = a prioridade de classe SR não corresponde
msrp-failure-unknown = motivo desconhecido
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, na ponte { $bridge }

## The macOS menu bar

menu-about = Acerca do plaa
menu-settings = Definições…
menu-services = Serviços
menu-hide = Ocultar plaa
menu-hide-others = Ocultar outros
menu-show-all = Mostrar tudo
menu-quit = Sair do plaa
menu-file = Ficheiro
menu-open = Abrir…
menu-open-address = Abrir endereço…
menu-close = Fechar janela
menu-edit = Edição
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Colar
menu-select-all = Selecionar tudo
menu-undo = Anular
menu-redo = Refazer
menu-trim = Aparar…
menu-split = Dividir clip
menu-rotate-left = Rodar para a esquerda
menu-rotate-right = Rodar para a direita
menu-flip-horizontal = Inverter na horizontal
menu-flip-vertical = Inverter na vertical
menu-revert = Reverter para o original
menu-export = Exportar…
menu-view = Visualização
menu-playback = Reprodução
menu-play-pause = Reproduzir ou pausar
menu-to-start = Ir para o início
menu-previous-chapter = Capítulo anterior
menu-next-chapter = Capítulo seguinte
menu-faster = Mais rápido
menu-slower = Mais lento
menu-normal-speed = Velocidade normal
menu-ab-loop = Ciclo entre dois pontos
menu-volume-up = Aumentar o volume
menu-volume-down = Diminuir o volume
menu-mute = Desativar ou ativar o som
menu-next-subtitles = Legendas seguintes
menu-next-audio = Faixa de áudio seguinte
menu-window = Janela
menu-minimize = Minimizar
menu-zoom = Redimensionar
menu-bring-all-to-front = Passar tudo para a frente

## Editing

action-edit = Editar
title-edited = { $name } (editado)
edit-trim = Aparar
edit-trim-apply = Aparar
edit-split = Dividir clip
edit-remove-clip = Remover clip
edit-done = Concluído
edit-rotate-left = Rodar para a esquerda
edit-rotate-right = Rodar para a direita
edit-flip-horizontal = Inverter na horizontal
edit-flip-vertical = Inverter na vertical
edit-undo = Anular
edit-redo = Refazer
edit-revert = Reverter para o original
edit-export = Exportar…

## Exporting

export-title = Exportar
export-format = Formato
export-size = Tamanho
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Só som
export-size-original = { $lines }p, tal como está
export-on-graphics-card = Codificado na placa gráfica ({ $api })
export-on-processor = Codificado no processador, o que demora mais
export-no-encoder = Nenhum codificador para este formato abre aqui
export-checking = A procurar um codificador…
export-sound-format = Formato de som
export-quality = Qualidade
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Comprimido; reproduz-se em quase todo o lado, incluindo nos dispositivos da Apple
export-mp3-note = Comprimido; reproduz-se em todo o lado
export-opus-note = Comprimido; o melhor som para o tamanho, embora nem todos os leitores o suportem
export-flac-note = Sem perdas: o som exatamente como é, num formato aberto
export-alac-note = Sem perdas: o som exatamente como é, tal como os dispositivos da Apple o reproduzem
export-wav-note = Sem compressão: o som exatamente como é, nos ficheiros maiores
export-kilobits = { $rate } kb/s
export-bits = { $bits } bits
export-rate-lower = Guardado a { $rate } kHz, abaixo dos { $file } kHz do ficheiro
export-choose = Exportar…
export-name = { $name } exportado
export-name-edited = { $name } editado
export-progress = A exportar «{ $name }»
export-stop = Parar a exportação
notice-exported = «{ $name }» exportado
notice-export-stopped = Exportação interrompida
notice-export-failed = Não foi possível exportar: { $error }
close-title = Fechar sem exportar?
close-unexported = As alterações a «{ $name }» perdem-se se não as exportar.
close-exporting = A exportação de «{ $name }» é interrompida, e o que já foi escrito perde-se.
close-anyway = Fechar
close-quit-anyway = Sair

## Recording

record-title = Gravação de som
record-file-name = Gravação de som
record-start = Gravar
record-stop = Parar a gravação
record-no-input = Sem microfone
record-microphone = Microfone
record-default-microphone = O predefinido do sistema
record-playing = O que o computador reproduz
record-mono = mono
record-stereo = estéreo
record-channels = { $count ->
    [one] { $count } canal
   *[other] { $count } canais
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, guardado a { $saved } kHz
action-record-sound = Gravar som
open-record-sound = Nova gravação de som
start-record-sound = Gravar som
menu-record-sound = Nova gravação de som
record-screen-title = Gravação do ecrã
record-screen-file-name = Gravação do ecrã
record-screen-whole = Ecrã
record-screen-number = Ecrã { $number }
record-screen-main = Ecrã { $number } (principal)
record-screens = Ecrãs
record-windows = Janelas
record-screen-what = O que gravar
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Todos os fotogramas
record-rate-tip = Fotogramas por segundo
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] { $count } descartado
   *[other] { $count } descartados
}
record-screen-window = Janela
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Já não está partilhado
record-screen-again = Escolher novamente
record-screen-failed = Não foi possível gravar o ecrã
record-sound = Som
record-no-sound = Sem som
# How much louder the input is taken, for a quiet microphone.
record-gain = Mais alto
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajustar o nível automaticamente
record-hide-title = O plaa fica oculto enquanto grava
record-hide-tray = Para parar a gravação, clique no ponto vermelho que o plaa coloca na área de notificação, ou abra o plaa novamente.
record-hide-reopen = Para parar a gravação, abra o plaa novamente.
record-hide-start = Iniciar a gravação
record-tray-title = O plaa está a gravar o ecrã
record-tray-tip = Clique para parar a gravação
action-record-screen = Gravar ecrã
open-record-screen = Nova gravação do ecrã
start-record-screen = Gravar ecrã
menu-record-screen = Nova gravação do ecrã
record-camera-title = Gravação da câmara
record-camera-file-name = Gravação da câmara
record-camera = Câmara
record-camera-failed = Não foi possível abrir a câmara
action-record-camera = Gravar câmara
open-record-camera = Nova gravação da câmara
start-record-camera = Gravar câmara
menu-record-camera = Nova gravação da câmara
notice-no-camera = Não há nenhuma câmara para gravar
notice-recorded = «{ $name }» guardado em { $folder }
notice-record-failed = Não foi possível gravar: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Permitir controlo externo
settings-allow-outside-control-note = Agentes de IA como o Claude Code podem ver o que está a ser reproduzido no plaa e controlá-lo, através do plaa --mcp. O plaa pergunta antes de cada novo agente.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Esquecer
settings-ask-before-note = Perguntar antes que um agente:
settings-ask-reading = Veja o que está a ser reproduzido, ou a imagem
settings-ask-playing = Controle a reprodução, o som ou o ecrã completo
settings-ask-opening = Abra um ficheiro ou feche uma janela
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Permitir que { $agent } controle o plaa?
agent-prompt-body = { $agent } pede para usar o controlo externo do plaa, para ver o que está a ser reproduzido e controlá-lo. Pode desativar o controlo externo nas Definições.
agent-prompt-allow = Permitir
agent-prompt-deny = Não permitir
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Permitir que { $agent } veja o que está a ser reproduzido aqui?
agent-ask-play = Permitir que { $agent } controle a reprodução aqui?
agent-ask-open = Permitir que { $agent } abra ou feche uma janela?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } pede para usar «{ $tool }». Nas Definições, escolhe sobre o que o plaa pergunta.

## Casting to a TV or speaker on the network

cast-looking = A procurar televisores e altifalantes…
cast-none = Não foi encontrado nenhum televisor, altifalante ou Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Parar de reproduzir em { $renderer }
notice-casting = A reproduzir em { $renderer }.
notice-cast-failed = Não foi possível reproduzir em { $renderer }: { $reason }
cast-web-page = Ainda não é possível transmitir páginas web: abra antes o endereço do próprio vídeo.

## The assistant panel, and its models in Settings

assistant-add-another = Adicionar um modelo…
assistant-add-model = O assistente precisa de um modelo: um modelo na nuvem com a respetiva chave de API, ou um modelo local.
assistant-ask = Pergunte sobre o que está a ser reproduzido
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = A conversa estava a ficar demasiado longa para { $model } a ler de uma vez, por isso a parte anterior foi resumida. O assistente pode voltar a olhar para o que viu.
assistant-compacting = A resumir a parte anterior da conversa
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } neste computador
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } em { $host }
assistant-new-chat = Nova conversa
assistant-no-model = Primeiro, adicione um modelo nas Definições.
assistant-open-settings = Adicionar um modelo
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = A conversa já não cabe no que { $model } consegue ler de uma vez. Comece uma nova conversa ou escolha um modelo que consiga ler mais.
assistant-problem-credit = { $provider } indica que a conta não tem crédito. Uma conta nova tem de comprar crédito no site de { $provider } antes de a chave funcionar; depois, tente novamente.
assistant-problem-key = { $provider } recusou a chave de API. Verifique-a nas Definições.
assistant-problem-model = { $provider } não tem nenhum modelo chamado { $model }. Verifique o nome nas Definições.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } não respondeu: { $message }
assistant-problem-rate = { $provider } pede para abrandar. Tente novamente daqui a pouco.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } pede para abrandar: { $message }
assistant-problem-refused = { $model } recusou-se a responder.
assistant-problem-unavailable = { $provider } está ocupado ou com problemas. Tente novamente daqui a pouco.
assistant-problem-unreachable = O plaa não conseguiu contactar { $provider }. Verifique a sua ligação ou se o servidor está em execução.
assistant-running = Em execução…
assistant-send = Enviar
assistant-shortened = A conversa era demasiado longa para { $model } a ler de uma vez, por isso os resultados anteriores das ferramentas foram omitidos. O assistente pode voltar a olhar para o que viu.
assistant-stop = Parar
assistant-stopped = Parado.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Agora a conversar com { $model }.
assistant-thinking = A pensar…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Raciocínio
assistant-title = Assistente
settings-assistant-add = Adicionar um modelo
settings-assistant-add-button = Adicionar
# $model is the model's name.
settings-assistant-added = { $model } respondeu e foi adicionado.
settings-assistant-added-tag = Adicionado
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Endereço, por exemplo { $example }
settings-assistant-change-key = Alterar chave
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Servidor compatível com OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Contexto
settings-assistant-context-note = Mais contexto permite ao assistente ter presente uma parte maior de uma conversa, mas o modelo ocupa mais memória e pode responder mais devagar.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Abrir a página
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } está em execução em { $address }.
settings-assistant-found-none = { $provider } ainda não tem modelos. Transfira um com { $provider } e procure novamente.
settings-assistant-get-key = Obter uma chave de API
settings-assistant-get-server = Obter { $provider }
settings-assistant-in-use = Em utilização
settings-assistant-key = Chave de API
# $error is what the keychain said.
settings-assistant-key-failed = Não foi possível guardar a chave no porta-chaves: { $error }
settings-assistant-key-kept = A sua chave de { $provider } fica guardada no porta-chaves do sistema.
settings-assistant-key-needed = Este modelo precisa de uma chave de API.
settings-assistant-key-refused = { $provider } recusou esta chave. Verifique se foi copiada por inteiro, a partir da conta certa.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crie uma chave no site de { $provider } e cole-a aqui.
settings-assistant-look-again = Procurar novamente
settings-assistant-looking = A procurar modelos…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modelo, por exemplo { $example }
settings-assistant-no-server = O plaa não encontrou { $provider } em execução neste computador. Inicie-o ou indique o respetivo endereço abaixo.
settings-assistant-no-tools = Não pode utilizar ferramentas, de que o assistente precisa
settings-assistant-none = Ainda não há modelos. Adicione um abaixo: um modelo local, como os do Ollama, fica neste computador; um modelo na nuvem precisa de uma chave de API do respetivo fornecedor.
settings-assistant-note = Os modelos com que o painel do assistente pode conversar. As chaves ficam guardadas no porta-chaves do sistema.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Utilizar outro endereço
settings-assistant-recommended = Recomendado
settings-assistant-remove = Remover
settings-assistant-sees = Vê imagens
settings-assistant-trying = A testar…
settings-assistant-use = Utilizar
settings-assistant-use-key = Continuar
settings-assistant-uses-tools = Utiliza ferramentas
