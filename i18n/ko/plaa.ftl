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
language-name = 한국어

## Common

app-name = plaa
common-cancel = 취소
common-close = 닫기
common-more = 더 보기

## Command line

usage-help =
    사용법: plaa [FILE OR ADDRESS]...

    비디오 및 오디오 파일과 https://example.com/live.m3u8 같은 주소의
    스트림을 재생합니다. 각각 실행 중인 plaa의 창에서 열리며, 필요하면
    plaa가 시작됩니다.

    옵션:
      -h, --help        이 도움말 보기
      -V, --version     버전 보기
          --interfaces  유선 인터페이스와, 각 인터페이스에서 plaa가 AVB
                        스트림을 보낼 수 있는지 보기
          --mcp         stdin과 stdout으로 MCP를 제공하여 AI 에이전트가
                        실행 중인 plaa의 재생 내용을 보고 제어하게 합니다

## Start window

start-hint = 비디오 또는 오디오 파일, 스트림, 웹 동영상을 여세요.
start-open = 열기
start-open-address = 주소 열기
# Opens a list of the talkers on the AVB network to listen to.
start-connect = 연결

## Actions, as buttons, tooltips and menus name them

action-open = 열기
action-open-address = 주소 열기
open-file = 파일 열기…
open-address = 주소 열기…
action-settings = 설정
action-info = 속성
action-loop = 반복
action-screenshot = 프레임을 이미지로 저장
action-play = 재생
action-pause = 일시 정지
# $seconds is how far the button moves, such as 10.
action-back = { $seconds }초 뒤로
action-forward = { $seconds }초 앞으로
action-mute = 음소거
action-unmute = 음소거 해제
action-speed = 재생 속도
action-tracks = 오디오 및 자막
action-full-screen = 전체 화면
action-playlist = 재생 목록
action-chapters = 챕터
action-picture-in-picture = 화면 속 화면
action-previous = 이전
action-next = 다음
action-exit-full-screen = 전체 화면 종료
action-quality = 화질
action-visualizer = 비주얼라이저
action-network = 네트워크 오디오
action-cast = 전송
action-connect = 연결
action-assistant = 어시스턴트

## What a sound file shows under its cover

visualizer-spectrum = 스펙트럼
visualizer-waveform = 파형
visualizer-prism = 프리즘
visualizer-cover = 아트워크만

## The floating controls

overlay-keep-shown = 컨트롤 고정
overlay-auto-hide = 컨트롤을 영상 위에 띄우기

## Audio and subtitle tracks

tracks-audio = 오디오
tracks-subtitles = 자막
tracks-off = 끔
# A track the file gives no name or language, by its number.
tracks-numbered = 트랙 { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = 오디오만

## Inspector

info-title = 속성
playlist-title = 재생 목록
chapters-numbered = 챕터 { $number }
info-file = 파일
info-name = 이름
info-address = 주소
info-folder = 폴더
info-size = 크기
info-format = 포맷
info-duration = 길이
info-bit-rate = 비트레이트
info-video = 비디오
info-audio = 오디오
info-codec = 코덱
info-dimensions = 치수
info-frame-rate = 프레임 레이트
info-frames-per-second = 초당 { $rate }프레임
info-pixel-format = 픽셀 포맷
info-decoding = 디코딩
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = 그래픽 카드({ $api })
info-decoding-software = 프로세서
info-sample-rate = 샘플 레이트
info-hertz = { $rate }Hz
info-channels = 채널
info-language = 언어
info-no-sound = 사운드 없음
info-tags = 태그

## Dialogs

dialog-open = 열기
dialog-filter-media = 비디오 및 오디오
dialog-filter-all = 모든 파일

address-title = 주소 열기
address-note = YouTube 동영상 페이지처럼 동영상이 있는 웹 페이지, 또는 https://example.com/live.m3u8 같은 네트워크상의 스트림이나 파일입니다.
address-label = 주소
address-open = 열기
address-recent = 최근 항목
address-forget = 최근 항목에서 제거

## Notices

notice-dialog-failed = 파일 대화상자를 열 수 없습니다: { $error }
notice-screenshot-saved = “{ $name }”을(를) 저장했습니다.
notice-screenshot-failed = 이미지를 저장할 수 없습니다: { $error }
notice-no-sound = 사운드 없이 재생하는 중: { $reason }
notice-loop-from = { $time }부터 반복합니다. 끝낼 위치에서 다시 누르세요.
notice-loop-between = { $from }에서 { $to }까지 반복하는 중입니다.
notice-loop-off = 반복을 껐습니다.

## Errors, shown in place of what could not play

error-needs-yt-dlp = 웹 페이지인 것 같습니다. plaa는 웹 페이지의 동영상을 yt-dlp로 재생하지만, yt-dlp가 설치되어 있지 않습니다.
error-empty-list = 이 목록에는 재생할 항목이 없습니다.

## Settings

settings-title = 설정
settings-tab-appearance = 모양
settings-tab-playback = 재생
settings-tab-network = 네트워크
settings-tab-language = 언어
settings-tab-agents = 에이전트
settings-tab-assistant = 어시스턴트
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (개발 빌드, { $build })
settings-choose = 선택…

settings-appearance-system = 시스템 설정 따르기
settings-appearance-light = 라이트
settings-appearance-dark = 다크
settings-colors = 색상
settings-system-accent = 시스템 강조 색상 사용
settings-omarchy-note = “{ $theme }”의 강조 색상으로 색상을 만듭니다.
settings-system-accent-note = 시스템 강조 색상으로 색상을 만듭니다.
settings-system-accent-none = 시스템에 강조 색상이 없어 plaa는 아래에서 선택한 색상을 사용합니다.
settings-accent-chosen-note = 아래에서 선택한 색상으로 색상을 만듭니다.
settings-controls = 컨트롤
settings-overlay = 컨트롤 띄우기
settings-overlay-note = 도구 막대와 재생 컨트롤이 영상이나 아트워크 위에 떠 있으며, 포인터가 멈춰 있거나 창 밖에 있는 동안에는 밀려나듯 사라집니다.
settings-overlay-delay = 숨기기까지
settings-overlay-delay-note = 재생 중인 화면 위에서 포인터가 멈춘 뒤 컨트롤이 사라질 때까지의 시간입니다.
settings-seconds = { $seconds }초
settings-transparency = 투명도
settings-transparency-note = 떠 있는 컨트롤을 통해 영상이 비치는 정도입니다.
settings-percent = { $percent }%
settings-corner-radius = 모서리 반경
settings-corner-radius-note = 대화상자와 떠 있는 컨트롤에 적용됩니다.
settings-pixels = { $pixels }px
settings-animations = 애니메이션
settings-animations-note = 미끄러지는 막대와 패널, 커지는 대화상자, 튕기는 버튼.
settings-animations-reduced = 시스템에서 동작 줄이기를 요청하는 동안에는 꺼집니다.

settings-hardware = 그래픽 카드 사용
settings-hardware-note = 비디오 디코딩과 내보내기 인코딩을 그래픽 카드에서 처리합니다. 전력을 덜 쓰고, 큰 비디오도 부드럽게 재생하며, 더 빠르게 내보냅니다. 이후에 여는 파일부터 적용됩니다.
settings-audio-device = 사운드 출력
settings-audio-device-system = 시스템 기본값
settings-resume = 마지막 위치부터 이어서 재생
settings-resume-note = 파일을 마지막으로 닫았던 위치부터 시작합니다.
settings-folder-playlists = 폴더의 다른 파일도 재생
settings-folder-playlists-note = 파일 하나를 열면 같은 폴더의 다른 파일이 목록에 추가되어 이어서 재생됩니다.
settings-streams = 스트림 및 웹 동영상
settings-stream-quality = 화질
settings-stream-quality-note = 여러 화질로 제공되는 스트림과 웹 동영상에 적용됩니다. 이후에 여는 항목부터 적용됩니다.
settings-quality-auto = 자동, 화면 높이에 맞춤
settings-quality-best = 최고
settings-web-pages = yt-dlp로 웹 동영상 재생
settings-web-pages-found = yt-dlp가 YouTube 같은 페이지에서 동영상을 찾습니다.
settings-web-pages-missing = yt-dlp가 설치되어 있지 않습니다. YouTube 같은 페이지의 동영상을 재생하려면 yt-dlp를 설치하세요.
settings-folders = 폴더
settings-recordings = 녹음 및 녹화
settings-screenshots = 저장한 프레임

settings-interface-language = 인터페이스 언어
# The first entry of the language list.
settings-language-system = 시스템 설정 따르기
settings-track-languages = 트랙 언어
settings-track-languages-note = en이나 fra 같은 언어 코드를 선호하는 순서대로 쉼표로 구분하여 입력합니다. 파일에 이 언어의 오디오와 자막이 있으면 plaa가 선택합니다.
settings-audio-languages = 오디오
settings-subtitle-languages = 자막

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB 인터페이스
settings-avb-interface-note = plaa가 AVB 및 Milan 토커와 리스너를 찾는 유선 포트입니다. 보내려면 Intel I210처럼 PTP 하드웨어 클록이 있는 포트가 필요합니다.
settings-avb-off = 끔
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (목록만: PTP 클록 없음)
settings-avb-off-mac = { $interface } (목록만: AVB 꺼짐)
settings-avb-interface-note-mac = plaa가 AVB 및 Milan 토커와 리스너를 찾는 유선 포트입니다. 보낼 때는 이 Mac 자체의 AVB 엔티티를 거치므로, Audio MIDI 설정의 네트워크 기기 브라우저에서 이 포트의 AVB를 켜야 합니다.
settings-avb-talker-note-mac = plaa는 마지막으로 재생한 창의 사운드를 이 Mac 자체의 AVB 엔티티를 거쳐 연결된 리스너로 보냅니다.
settings-avb-talker = 재생 중인 사운드 보내기
settings-avb-talker-note = plaa가 Milan 토커가 되어, 마지막으로 재생한 창의 사운드를 연결된 리스너로 보냅니다.
settings-avb-name = 네트워크상의 이름
settings-avb-name-note = 컨트롤러에 plaa가 “{ $name }”(으)로 표시됩니다. 이름을 바꾸려면 Enter를 누르세요.
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer }의 plaa
avb-could-not-start = { $interface }을(를) 사용할 수 없습니다: { $error }
network-title = 네트워크 오디오
network-off = 네트워크에서 재생하려면 설정에서 재생 중인 사운드 보내기를 켜세요.
network-blocked = 보내지 않음: { $reason }
network-waiting = 보낼 준비가 되었습니다. 리스너가 연결되기를 기다리는 중입니다.
network-sending = 연결된 리스너로 보내는 중입니다.
network-looking = 리스너를 찾는 중…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = plaa 스트림을 보낼 때 기준이 되는 네트워크 시간을 유지하는 ptp4l을 시작하는 중…
network-ptp-elsewhere = ptp4l이 다른 인터페이스에서 실행 중이므로 여기서는 plaa 스트림이 네트워크 시간에 맞출 수 없어, 리스너가 재생하지 못할 수 있습니다. 설정에서 해당 인터페이스를 선택하거나 이 인터페이스에서 ptp4l을 실행하세요.
network-ptp-missing = ptp4l이 실행 중이 아니므로 plaa 스트림이 네트워크 시간에 맞출 수 없어, 리스너가 재생하지 못할 수 있습니다. plaa 패키지와 함께 linuxptp를 설치하거나, ptp4l을 실행하는 triib을 설치하세요.
network-ptp-failed = ptp4l이 실행 중이 아니므로 plaa 스트림이 네트워크 시간에 맞출 수 없어, 리스너가 재생하지 못할 수 있습니다. plaa가 ptp4l을 시작할 수 없습니다: { $reason }
network-no-listeners = 네트워크에 plaa의 사운드를 재생할 수 있는 리스너가 없습니다.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener }에서 재생
network-stop-on = { $listener }에서 재생 중지
network-choose-format = { $listener }에서 재생할 포맷 선택
network-failed = 연결할 수 없습니다: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = 여기서 재생하려면 triib을 업데이트하세요.
network-choose = 리스너를 선택하면 스트림을 받는 방식이 표시됩니다.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = 여기서 듣기
network-no-sources = 네트워크에 plaa가 재생하는 포맷으로 보내거나 그런 포맷으로 바꿀 수 있는 토커가 없습니다.
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker } 듣기
network-listen-stop = { $talker } 듣기 중지
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = { $talker }을(를) { $format }(으)로 바꾸고 듣기
network-listen-change-cuts-off = { $talker }을(를) { $format }(으)로 바꾸고 듣기, { $count ->
       *[other] 다른 리스너 { $count }개의 연결이 끊어짐
    }
network-listen-frames = 받은 프레임: { $frames }
network-listen-update-triib = 여기서 들으려면 triib을 업데이트하세요: plaa는 어차피 이 컴퓨터에 도달하는 것만 재생합니다.
network-listen-lite = 이 포트는 스트림을 예약하지 않는 AVB Lite로 실행 중입니다: plaa는 어차피 이 컴퓨터에 도달하는 것만 재생합니다.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = 토커를 찾는 중…
# plaa listens through its entity, which exists only while it sends.
connect-off = 여기서 토커를 들으려면 설정에서 재생 중인 사운드 보내기를 켜세요.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa가 여기서 들을 수 없음: { $reason }
connect-sending = plaa가 재생하는 포맷으로 보내는 중
connect-changing = plaa가 재생하는 포맷으로 변경
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, { $format } 사용
connect-change-cuts-off = { $talker }, { $format } 사용, { $count ->
       *[other] 다른 리스너 { $count }개의 연결이 끊어짐
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = 상태
network-not-connected = 연결되지 않음
network-from = { $stream }에서
network-from-receiving = { $stream }에서 수신 중
network-from-waiting = { $stream }에서, 토커를 기다리는 중
network-from-failed = { $stream }에서, 토커의 예약 실패: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = 미디어 클록
network-locked = 스트림에 고정됨
network-not-locked = 고정되지 않음
network-format = 포맷
# The formats both the listener and plaa's stream can be in.
network-formats = plaa에서 받을 수 있는 포맷
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = 스트림 ID
# The MAC address the stream is sent to.
network-destination = 대상
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = 지연 시간
network-microseconds = { $time }µs
network-frames = 받은 프레임
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = 순서가 어긋난 프레임
# How often the stream stopped coming while it was taken.
network-interrupted = 중단 횟수
network-entity-id = 엔티티 ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = 대역폭 부족
msrp-failure-2 = 브리지 리소스 부족
msrp-failure-3 = 트래픽 클래스의 대역폭 부족
msrp-failure-4 = 다른 토커가 사용 중인 스트림 ID
msrp-failure-5 = 이미 사용 중인 대상 주소
msrp-failure-6 = 더 높은 순위의 스트림에 밀려남
msrp-failure-7 = 보고된 지연 시간이 변경됨
msrp-failure-8 = 송신 포트가 AVB를 지원하지 않음
msrp-failure-9 = 다른 대상 주소를 사용해야 함
msrp-failure-10 = MSRP 리소스 부족
msrp-failure-11 = MMRP 리소스 부족
msrp-failure-12 = 대상 주소를 저장할 수 없음
msrp-failure-13 = 우선순위가 SR 클래스 우선순위가 아님
msrp-failure-14 = 프레임이 매체에 비해 너무 큼
msrp-failure-15 = 팬인 포트 한도에 도달함
msrp-failure-16 = 등록된 스트림의 첫 번째 값이 변경됨
msrp-failure-17 = 송신 포트에서 VLAN이 차단됨
msrp-failure-18 = 송신 포트에서 VLAN 태깅이 꺼져 있음
msrp-failure-19 = SR 클래스 우선순위가 일치하지 않음
msrp-failure-unknown = 알 수 없는 이유
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason } (브리지 { $bridge })

## The macOS menu bar

menu-about = plaa에 관하여
menu-settings = 설정…
menu-services = 서비스
menu-hide = plaa 가리기
menu-hide-others = 기타 가리기
menu-show-all = 모두 보기
menu-quit = plaa 종료
menu-file = 파일
menu-open = 열기…
menu-open-address = 주소 열기…
menu-close = 윈도우 닫기
menu-edit = 편집
menu-cut = 오려두기
menu-copy = 복사하기
menu-paste = 붙여넣기
menu-select-all = 모두 선택
menu-undo = 실행 취소
menu-redo = 실행 복귀
menu-trim = 다듬기…
menu-split = 클립 분할
menu-rotate-left = 왼쪽으로 회전
menu-rotate-right = 오른쪽으로 회전
menu-flip-horizontal = 수평으로 뒤집기
menu-flip-vertical = 수직으로 뒤집기
menu-revert = 원본으로 되돌리기
menu-export = 내보내기…
menu-view = 보기
menu-playback = 재생
menu-play-pause = 재생/일시 정지
menu-to-start = 처음으로 이동
menu-previous-chapter = 이전 챕터
menu-next-chapter = 다음 챕터
menu-faster = 빠르게
menu-slower = 느리게
menu-normal-speed = 보통 속도
menu-ab-loop = 두 지점 사이 반복
menu-volume-up = 음량 높이기
menu-volume-down = 음량 낮추기
menu-mute = 음소거/음소거 해제
menu-next-subtitles = 다음 자막
menu-next-audio = 다음 오디오 트랙
menu-window = 윈도우
menu-minimize = 최소화
menu-zoom = 확대/축소
menu-bring-all-to-front = 모두 앞으로 가져오기

## Editing

action-edit = 편집
title-edited = { $name } (편집됨)
edit-trim = 다듬기
edit-trim-apply = 다듬기
edit-split = 클립 분할
edit-remove-clip = 클립 제거
edit-done = 완료
edit-rotate-left = 왼쪽으로 회전
edit-rotate-right = 오른쪽으로 회전
edit-flip-horizontal = 수평으로 뒤집기
edit-flip-vertical = 수직으로 뒤집기
edit-undo = 실행 취소
edit-redo = 다시 실행
edit-revert = 원본으로 되돌리기
edit-export = 내보내기…

## Exporting

export-title = 내보내기
export-format = 포맷
export-size = 크기
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = 사운드만
export-size-original = { $lines }p, 원본 그대로
export-on-graphics-card = 그래픽 카드에서 인코딩({ $api })
export-on-processor = 프로세서에서 인코딩하므로 시간이 더 걸림
export-no-encoder = 여기서는 이 포맷의 인코더를 열 수 없음
export-checking = 인코더를 찾는 중…
export-sound-format = 사운드 포맷
export-quality = 음질
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = 압축. Apple 기기를 포함해 거의 모든 곳에서 재생됩니다
export-mp3-note = 압축. 어디서나 재생됩니다
export-opus-note = 압축. 크기 대비 음질이 가장 좋지만, 일부 플레이어에서는 재생되지 않습니다
export-flac-note = 무손실. 사운드를 그대로, 개방형 포맷으로 담습니다
export-alac-note = 무손실. 사운드를 그대로, Apple 기기에서 재생되는 방식으로 담습니다
export-wav-note = 비압축. 사운드를 그대로 담지만 파일이 가장 큽니다
export-kilobits = { $rate }kb/s
export-bits = { $bits }비트
export-rate-lower = 파일의 { $file }kHz보다 낮은 { $rate }kHz로 저장됩니다
export-choose = 내보내기…
export-name = { $name }(내보냄)
export-name-edited = { $name }(편집됨)
export-progress = “{ $name }” 내보내는 중
export-stop = 내보내기 중지
notice-exported = “{ $name }”을(를) 내보냈습니다
notice-export-stopped = 내보내기를 중지했습니다
notice-export-failed = 내보낼 수 없습니다: { $error }
close-title = 내보내지 않고 닫겠습니까?
close-unexported = 내보내지 않으면 “{ $name }”의 변경 사항이 사라집니다.
close-exporting = “{ $name }” 내보내기가 중지되고, 지금까지 기록한 내용은 사라집니다.
close-anyway = 닫기
close-quit-anyway = 종료

## Recording

record-title = 녹음
record-file-name = 녹음
record-start = 기록
record-stop = 기록 중지
record-no-input = 마이크 없음
record-microphone = 마이크
record-default-microphone = 시스템 기본값
record-playing = 컴퓨터에서 재생 중인 사운드
record-mono = 모노
record-stereo = 스테레오
record-channels = { $count }채널
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate }kHz · { $channels }
record-input-format-lowered = { $rate }kHz · { $channels }, { $saved }kHz로 저장
action-record-sound = 녹음
open-record-sound = 새로운 녹음
start-record-sound = 녹음
menu-record-sound = 새로운 녹음
record-screen-title = 화면 기록
record-screen-file-name = 화면 기록
record-screen-whole = 화면
record-screen-number = 화면 { $number }
record-screen-main = 화면 { $number }(주 화면)
record-screens = 화면
record-windows = 창
record-screen-what = 기록할 대상
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate }fps
record-rate-every = 모든 프레임
record-rate-tip = 초당 프레임 수
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count }개 누락
record-screen-window = 창
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = 더 이상 공유되지 않음
record-screen-again = 다시 선택
record-screen-failed = 화면을 기록할 수 없습니다
record-sound = 사운드
record-no-sound = 사운드 없음
# How much louder the input is taken, for a quiet microphone.
record-gain = 증폭
record-gain-value = +{ $decibels }dB
record-level-automatically = 자동으로 레벨 조절
record-hide-title = 기록하는 동안 plaa가 숨겨집니다
record-hide-tray = 기록을 중지하려면 plaa가 트레이에 표시하는 빨간 점을 클릭하거나 plaa를 다시 여세요.
record-hide-reopen = 기록을 중지하려면 plaa를 다시 여세요.
record-hide-start = 기록 시작
record-tray-title = plaa가 화면을 기록하는 중
record-tray-tip = 클릭하여 기록 중지
action-record-screen = 화면 기록
open-record-screen = 새로운 화면 기록
start-record-screen = 화면 기록
menu-record-screen = 새로운 화면 기록
record-camera-title = 카메라 녹화
record-camera-file-name = 카메라 녹화
record-camera = 카메라
record-camera-failed = 카메라를 열 수 없습니다
action-record-camera = 카메라 녹화
open-record-camera = 새로운 카메라 녹화
start-record-camera = 카메라 녹화
menu-record-camera = 새로운 카메라 녹화
notice-no-camera = 녹화할 카메라가 없습니다
notice-recorded = “{ $name }”을(를) { $folder }에 저장했습니다
notice-record-failed = 기록할 수 없습니다: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = 외부 제어 허용
settings-allow-outside-control-note = Claude Code 같은 AI 에이전트가 plaa --mcp를 통해 plaa에서 재생 중인 내용을 보고 제어할 수 있습니다. 새 에이전트마다 plaa가 먼저 묻습니다.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = 허용됨: { $agents }
settings-forget-agents = 지우기
settings-ask-before-note = 에이전트가 다음 작업을 하기 전에 묻기:
settings-ask-reading = 재생 중인 내용이나 화면 보기
settings-ask-playing = 재생, 사운드 또는 전체 화면 제어
settings-ask-opening = 파일 열기 또는 창 닫기
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent }이(가) plaa를 제어하도록 허용하시겠습니까?
agent-prompt-body = { $agent }이(가) 재생 중인 내용을 보고 제어하기 위해 plaa의 외부 제어를 사용하려고 합니다. 외부 제어는 설정에서 끌 수 있습니다.
agent-prompt-allow = 허용
agent-prompt-deny = 허용 안 함
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent }이(가) 여기서 재생 중인 내용을 보도록 허용하시겠습니까?
agent-ask-play = { $agent }이(가) 여기서 재생을 제어하도록 허용하시겠습니까?
agent-ask-open = { $agent }이(가) 창을 열거나 닫도록 허용하시겠습니까?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent }이(가) “{ $tool }”을(를) 사용하려고 합니다. plaa가 무엇을 물을지는 설정에서 선택합니다.

## Casting to a TV or speaker on the network

cast-looking = TV와 스피커를 찾는 중…
cast-none = TV, 스피커 또는 Chromecast를 찾지 못했습니다.
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer }에서 재생 중지
notice-casting = { $renderer }에서 재생하는 중입니다.
notice-cast-failed = { $renderer }에서 재생할 수 없습니다: { $reason }
cast-web-page = 웹 페이지는 아직 전송할 수 없습니다: 대신 동영상 자체의 주소를 여세요.

## The assistant panel, and its models in Settings

assistant-add-another = 모델 추가…
assistant-add-model = 어시스턴트에는 모델이 필요합니다. API 키가 있는 클라우드 모델이나 로컬 모델을 추가하세요.
assistant-ask = 재생 중인 내용에 대해 질문하기
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = 대화가 { $model }이(가) 한 번에 읽기에는 너무 길어지고 있어 앞부분을 요약했습니다. 어시스턴트가 이미 본 내용을 다시 살펴볼 수 있습니다.
assistant-compacting = 이전 대화를 요약하는 중
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = 이 컴퓨터의 { $provider }
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $host }의 { $provider }
assistant-new-chat = 새 채팅
assistant-no-model = 먼저 설정에서 모델을 추가하세요.
assistant-open-settings = 모델 추가
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = 대화가 { $model }이(가) 한 번에 읽을 수 있는 양을 넘었습니다. 새 채팅을 시작하거나 더 많이 읽을 수 있는 모델을 선택하세요.
assistant-problem-credit = { $provider }에 따르면 계정에 크레딧이 없습니다. 새 계정은 { $provider } 사이트에서 크레딧을 구입해야 키를 사용할 수 있습니다. 구입한 다음 다시 시도하세요.
assistant-problem-key = { $provider }에서 API 키를 거부했습니다. 설정에서 키를 확인하세요.
assistant-problem-model = { $provider }에 { $model }(이)라는 모델이 없습니다. 설정에서 이름을 확인하세요.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model }이(가) 응답하지 않았습니다: { $message }
assistant-problem-rate = { $provider }에서 요청 속도를 낮춰 달라고 합니다. 잠시 후 다시 시도하세요.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider }에서 요청 속도를 낮춰 달라고 합니다: { $message }
assistant-problem-refused = { $model }이(가) 답변을 거절했습니다.
assistant-problem-unavailable = { $provider }이(가) 사용량이 많거나 문제가 있습니다. 잠시 후 다시 시도하세요.
assistant-problem-unreachable = prev가 { $provider }에 연결할 수 없습니다. 인터넷 연결이나 서버가 실행 중인지 확인하세요.
assistant-running = 실행 중…
assistant-send = 보내기
assistant-shortened = 대화가 { $model }이(가) 한 번에 읽기에는 너무 길어 이전 도구 결과를 뺐습니다. 어시스턴트가 이미 본 내용을 다시 살펴볼 수 있습니다.
assistant-stop = 중단
assistant-stopped = 중단되었습니다.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = 이제 { $model }과(와) 대화합니다.
assistant-thinking = 생각하는 중…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = 생각
assistant-title = 어시스턴트
settings-assistant-add = 모델 추가
settings-assistant-add-button = 추가
# $model is the model's name.
settings-assistant-added = { $model }이(가) 응답하여 추가되었습니다.
settings-assistant-added-tag = 추가됨
# $example is an address, such as http://localhost:11434.
settings-assistant-address = 주소(예: { $example })
settings-assistant-change-key = 키 변경
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI 호환 서버
# The menu of how much a local model reads at once.
settings-assistant-context = 컨텍스트
settings-assistant-context-note = 크게 설정하면 어시스턴트가 채팅의 더 많은 부분을 기억할 수 있지만, 모델이 메모리를 더 많이 사용하고 응답이 느려질 수 있습니다.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K 토큰
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = 페이지 열기
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider }이(가) { $address }에서 실행 중입니다.
settings-assistant-found-none = { $provider }에 아직 모델이 없습니다. { $provider }(으)로 모델을 다운로드한 다음 다시 찾으세요.
settings-assistant-get-key = API 키 받기
settings-assistant-get-server = { $provider } 받기
settings-assistant-in-use = 사용 중
settings-assistant-key = API 키
# $error is what the keychain said.
settings-assistant-key-failed = 키를 키체인에 보관할 수 없습니다: { $error }
settings-assistant-key-kept = { $provider } 키는 시스템 키체인에 보관되어 있습니다.
settings-assistant-key-needed = 이 모델에는 API 키가 필요합니다.
settings-assistant-key-refused = { $provider }에서 이 키를 거부했습니다. 올바른 계정의 키를 빠짐없이 복사했는지 확인하세요.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = { $provider } 사이트에서 키를 만들어 여기에 붙여 넣으세요.
settings-assistant-look-again = 다시 찾기
settings-assistant-looking = 모델을 찾는 중…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = 모델(예: { $example })
settings-assistant-no-server = prev가 이 컴퓨터에서 실행 중인 { $provider }을(를) 찾지 못했습니다. { $provider }을(를) 시작하거나 아래에 주소를 입력하세요.
settings-assistant-no-tools = 도구를 사용할 수 없음(어시스턴트에 필요함)
settings-assistant-none = 아직 모델이 없습니다. 아래에서 추가하세요. Ollama 같은 로컬 모델은 이 컴퓨터 안에서만 실행되고, 클라우드 모델은 제공업체의 API 키가 필요합니다.
settings-assistant-note = 어시스턴트 패널에서 사용할 수 있는 모델입니다. 키는 시스템 키체인에 보관됩니다.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = 다른 주소 사용
settings-assistant-recommended = 추천
settings-assistant-remove = 제거
settings-assistant-sees = 이미지 인식
settings-assistant-trying = 확인 중…
settings-assistant-use = 사용
settings-assistant-use-key = 계속
settings-assistant-uses-tools = 도구 사용
