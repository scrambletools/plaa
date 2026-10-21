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
language-name = 繁體中文

## Common

app-name = plaa
common-cancel = 取消
common-close = 關閉
common-more = 更多

## Command line

usage-help =
    用法：plaa [FILE OR ADDRESS]...

    播放影片和音訊檔案，以及 https://example.com/live.m3u8 等位址上的串流。
    每一項都會在執行中的 plaa 視窗內開啟，必要時會先啟動 plaa。

    選項：
      -h, --help        顯示此說明
      -V, --version     顯示版本
          --interfaces  顯示有線介面，以及 plaa 能否在這些介面上傳送
                        AVB 串流
          --mcp         透過 stdin 和 stdout 提供 MCP，讓 AI 代理程式查看
                        執行中的 plaa 播放的內容並加以控制

## Start window

start-hint = 開啟影片或音訊檔案、串流或網路影片。
start-open = 開啟
start-open-address = 開啟位址

## Actions, as buttons, tooltips and menus name them

action-open = 開啟
action-open-address = 開啟位址
open-file = 開啟檔案…
open-address = 開啟位址…
action-settings = 設定
action-info = 檢閱器
action-loop = 循環播放
action-screenshot = 將影格儲存為圖片
action-play = 播放
action-pause = 暫停
# $seconds is how far the button moves, such as 10.
action-back = 倒退 { $seconds } 秒
action-forward = 前進 { $seconds } 秒
action-mute = 靜音
action-unmute = 取消靜音
action-speed = 播放速度
action-tracks = 音訊與字幕
action-full-screen = 全螢幕
action-playlist = 播放列表
action-chapters = 章節
action-picture-in-picture = 子母畫面
action-previous = 上一個
action-next = 下一個
action-exit-full-screen = 結束全螢幕
action-quality = 畫質
action-visualizer = 視覺化效果
action-network = 網路音訊

## What a sound file shows under its cover

visualizer-spectrum = 頻譜
visualizer-waveform = 波形
visualizer-prism = 稜鏡
visualizer-cover = 僅封面

## The floating controls

overlay-keep-shown = 固定控制項
overlay-auto-hide = 讓控制項浮在畫面上

## Audio and subtitle tracks

tracks-audio = 音訊
tracks-subtitles = 字幕
tracks-off = 關閉
# A track the file gives no name or language, by its number.
tracks-numbered = 軌道 { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = 僅音訊

## Inspector

info-title = 檢閱器
playlist-title = 播放列表
chapters-numbered = 第 { $number } 章
info-file = 檔案
info-name = 名稱
info-address = 位址
info-folder = 資料夾
info-size = 大小
info-format = 格式
info-duration = 長度
info-bit-rate = 位元速率
info-video = 影片
info-audio = 音訊
info-codec = 編解碼器
info-dimensions = 尺寸
info-frame-rate = 影格速率
info-frames-per-second = 每秒 { $rate } 格
info-pixel-format = 像素格式
info-decoding = 解碼
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = 顯示卡（{ $api }）
info-decoding-software = 處理器
info-sample-rate = 取樣率
info-hertz = { $rate } Hz
info-channels = 聲道
info-language = 語言
info-no-sound = 無聲音
info-tags = 標籤

## Dialogs

dialog-open = 開啟
dialog-filter-media = 影片和音訊
dialog-filter-all = 所有檔案

address-title = 開啟位址
address-note = 含有影片的網頁（例如 YouTube 影片的頁面），或網路上的串流或檔案（例如 https://example.com/live.m3u8）。
address-label = 位址
address-open = 開啟
address-recent = 最近使用
address-forget = 從最近使用中移除

## Notices

notice-dialog-failed = 無法開啟檔案對話框：{ $error }
notice-screenshot-saved = 已儲存「{ $name }」。
notice-screenshot-failed = 無法儲存圖片：{ $error }
notice-no-sound = 正在無聲播放：{ $reason }
notice-loop-from = 從 { $time } 開始循環播放。請在結束處再按一次。
notice-loop-between = 正在循環播放 { $from } 到 { $to }。
notice-loop-off = 已關閉循環播放。

## Errors, shown in place of what could not play

error-needs-yt-dlp = 這看起來是網頁。plaa 使用 yt-dlp 播放網頁上的影片，但尚未安裝 yt-dlp。
error-empty-list = 此列表中沒有可播放的內容。

## Settings

settings-title = 設定
settings-tab-appearance = 外觀
settings-tab-playback = 播放
settings-tab-network = 網路
settings-tab-language = 語言
settings-tab-agents = 代理程式
settings-version = plaa { $version }（{ $build }）
settings-version-development = plaa { $version }（開發版，{ $build }）
settings-choose = 選擇…

settings-appearance-system = 依照系統
settings-appearance-light = 淺色
settings-appearance-dark = 深色
settings-colors = 顏色
settings-system-accent = 使用系統強調色
settings-omarchy-note = 顏色是根據「{ $theme }」的強調色產生。
settings-system-accent-note = 顏色是根據系統的強調色產生。
settings-system-accent-none = 系統沒有強調色，因此 plaa 會使用下方選擇的顏色。
settings-accent-chosen-note = 顏色是根據下方選擇的顏色產生。
settings-controls = 控制項
settings-overlay = 浮動控制項
settings-overlay-note = 工具列和播放控制項會浮在畫面或封面上方，指標停住不動或移到視窗外時會滑出隱藏。
settings-overlay-delay = 隱藏前等待
settings-overlay-delay-note = 指標在播放內容上停住多久後，控制項才會滑出隱藏。
settings-seconds = { $seconds } 秒
settings-transparency = 透明度
settings-transparency-note = 透過浮動控制項能看到多少畫面內容。
settings-percent = { $percent }%
settings-corner-radius = 圓角半徑
settings-corner-radius-note = 用於對話框和浮動控制項。
settings-pixels = { $pixels } px
settings-animations = 動畫
settings-animations-note = 滑動的列與面板、展開的對話框和有彈性的按鈕。
settings-animations-reduced = 系統要求減少動態效果時會關閉。

settings-hardware = 使用顯示卡
settings-hardware-note = 以顯示卡解碼影片和編碼輸出內容：更省電，大型影片播放更順暢，輸出也更快。對之後開啟的檔案生效。
settings-audio-device = 聲音輸出
settings-audio-device-system = 系統預設
settings-resume = 從上次停止處繼續
settings-resume-note = 檔案會從上次關閉時的位置開始播放。
settings-folder-playlists = 播放資料夾中的其他檔案
settings-folder-playlists-note = 開啟一個檔案時，會列出同一資料夾中的其他檔案，並接著播放。
settings-streams = 串流與網路影片
settings-stream-quality = 畫質
settings-stream-quality-note = 用於提供多種畫質的串流和網路影片。對之後開啟的內容生效。
settings-quality-auto = 自動，與螢幕同高
settings-quality-best = 最佳
settings-web-pages = 使用 yt-dlp 播放網路影片
settings-web-pages-found = yt-dlp 能在 YouTube 等網頁上找到影片。
settings-web-pages-missing = 尚未安裝 yt-dlp。安裝後即可播放 YouTube 等網頁上的影片。
settings-folders = 資料夾
settings-recordings = 錄製內容
settings-screenshots = 已儲存的影格

settings-interface-language = 介面語言
# The first entry of the language list.
settings-language-system = 依照系統
settings-track-languages = 軌道語言
settings-track-languages-note = 語言代碼（例如 en 或 fra），最想要的排在最前面，以逗號分隔。檔案有這些語言的音訊和字幕時，plaa 會選用它們。
settings-audio-languages = 音訊
settings-subtitle-languages = 字幕

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB 介面
settings-avb-interface-note = plaa 會在此有線連接埠上尋找 AVB 和 Milan 的傳送端與接收端。傳送需要具備 PTP 硬體時鐘的連接埠，例如 Intel I210 的連接埠。
settings-avb-off = 關閉
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface }（僅列出：沒有 PTP 時鐘）
settings-avb-off-mac = { $interface }（僅列出：AVB 已關閉）
settings-avb-interface-note-mac = plaa 會在此有線連接埠上尋找 AVB 和 Milan 的傳送端與接收端。傳送會透過這部 Mac 本身的 AVB 實體進行，因此需要在「音訊 MIDI 設定」的「網路裝置瀏覽器」中為此連接埠開啟 AVB。
settings-avb-talker-note-mac = plaa 會透過這部 Mac 本身的 AVB 實體，將最後播放的視窗的聲音傳送給與其連接的接收端。
settings-avb-talker = 傳送正在播放的內容
settings-avb-talker-note = plaa 會成為 Milan 傳送端，將最後播放的視窗的聲音傳送給與其連接的接收端。
settings-avb-name = 網路上的名稱
settings-avb-name-note = 控制器會將 plaa 顯示為「{ $name }」。按下 Enter 即可重新命名。
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } 上的 plaa
avb-could-not-start = 無法使用 { $interface }：{ $error }
network-title = 網路音訊
network-off = 在「設定」中開啟「傳送正在播放的內容」，即可在網路上播放。
network-blocked = 未傳送：{ $reason }
network-waiting = 已準備好傳送，正在等待接收端連線。
network-sending = 正在傳送給已連線的接收端。
network-looking = 正在尋找接收端…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = 正在啟動 ptp4l，它負責維持 plaa 傳送串流所依據的網路時間…
network-ptp-elsewhere = ptp4l 在另一個介面上執行，因此 plaa 的串流在這裡無法與網路時間同步，接收端可能無法播放。請在「設定」中選擇該介面，或在此介面上執行 ptp4l。
network-ptp-missing = ptp4l 未在執行，因此 plaa 的串流無法與網路時間同步，接收端可能無法播放。請隨 plaa 的套件安裝 linuxptp，或安裝會執行 ptp4l 的 triib。
network-ptp-failed = ptp4l 未在執行，因此 plaa 的串流無法與網路時間同步，接收端可能無法播放。plaa 無法啟動 ptp4l：{ $reason }
network-no-listeners = 網路上沒有能播放 plaa 聲音的接收端。
# A listener with several inputs: its name and the input's.
network-stream = { $entity }：{ $stream }
network-play-on = 在 { $listener } 上播放
network-stop-on = 停止在 { $listener } 上播放
network-choose-format = 選擇要在 { $listener } 上播放的格式
network-failed = 無法連線：{ $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = 請更新 triib 以在此播放。
network-choose = 選擇一個接收端，查看它如何接收串流。

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = 狀態
network-not-connected = 未連線
network-from = 來自 { $stream }
network-from-receiving = 來自 { $stream }，正在接收
network-from-waiting = 來自 { $stream }，正在等待傳送端
network-from-failed = 來自 { $stream }，傳送端的預留失敗：{ $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = 媒體時鐘
network-locked = 已鎖定至串流
network-not-locked = 未鎖定
network-format = 格式
# The formats both the listener and plaa's stream can be in.
network-formats = 可從 plaa 接收的格式
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = 串流 ID
# The MAC address the stream is sent to.
network-destination = 目的地
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = 延遲
network-microseconds = { $time } µs
network-frames = 已接收的訊框
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = 順序錯亂的訊框
# How often the stream stopped coming while it was taken.
network-interrupted = 中斷次數
network-entity-id = 實體 ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = 頻寬不足
msrp-failure-2 = 橋接器資源不足
msrp-failure-3 = 流量類別頻寬不足
msrp-failure-4 = 串流 ID 已被其他傳送端使用
msrp-failure-5 = 目的地位址已在使用中
msrp-failure-6 = 被更高等級的串流搶占
msrp-failure-7 = 回報的延遲已變更
msrp-failure-8 = 輸出連接埠不支援 AVB
msrp-failure-9 = 需使用其他目的地位址
msrp-failure-10 = MSRP 資源耗盡
msrp-failure-11 = MMRP 資源耗盡
msrp-failure-12 = 無法儲存目的地位址
msrp-failure-13 = 優先順序不是 SR 類別的優先順序
msrp-failure-14 = 訊框對媒介而言過大
msrp-failure-15 = 已達扇入連接埠上限
msrp-failure-16 = 已註冊串流的首個值已變更
msrp-failure-17 = VLAN 在輸出連接埠上遭封鎖
msrp-failure-18 = 輸出連接埠已停用 VLAN 標記
msrp-failure-19 = SR 類別優先順序不符
msrp-failure-unknown = 未知原因
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }，位於橋接器 { $bridge }

## The macOS menu bar

menu-about = 關於 plaa
menu-settings = 設定…
menu-services = 服務
menu-hide = 隱藏 plaa
menu-hide-others = 隱藏其他
menu-show-all = 顯示全部
menu-quit = 結束 plaa
menu-file = 檔案
menu-open = 打開…
menu-open-address = 打開位址…
menu-close = 關閉視窗
menu-edit = 編輯
menu-cut = 剪下
menu-copy = 拷貝
menu-paste = 貼上
menu-select-all = 全選
menu-undo = 還原
menu-redo = 重做
menu-trim = 修剪…
menu-split = 分割剪輯片段
menu-rotate-left = 向左旋轉
menu-rotate-right = 向右旋轉
menu-flip-horizontal = 水平翻轉
menu-flip-vertical = 垂直翻轉
menu-revert = 回復成原始版本
menu-export = 輸出…
menu-view = 顯示方式
menu-playback = 播放
menu-play-pause = 播放或暫停
menu-to-start = 前往開頭
menu-previous-chapter = 上一個章節
menu-next-chapter = 下一個章節
menu-faster = 加快
menu-slower = 減慢
menu-normal-speed = 正常速度
menu-ab-loop = 在兩點之間循環播放
menu-volume-up = 調高音量
menu-volume-down = 調低音量
menu-mute = 靜音或取消靜音
menu-next-subtitles = 下一個字幕
menu-next-audio = 下一個音軌
menu-window = 視窗
menu-minimize = 縮到最小
menu-zoom = 縮放
menu-bring-all-to-front = 將此程式所有視窗移至最前

## Editing

action-edit = 編輯
title-edited = { $name }（已編輯）
edit-trim = 修剪
edit-trim-apply = 修剪
edit-split = 分割剪輯片段
edit-remove-clip = 移除剪輯片段
edit-done = 完成
edit-rotate-left = 向左旋轉
edit-rotate-right = 向右旋轉
edit-flip-horizontal = 水平翻轉
edit-flip-vertical = 垂直翻轉
edit-undo = 還原
edit-redo = 重做
edit-revert = 回復成原始版本
edit-export = 輸出…

## Exporting

export-title = 輸出
export-format = 格式
export-size = 大小
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = 僅音訊
export-size-original = { $lines }p，保持原樣
export-on-graphics-card = 以顯示卡編碼（{ $api }）
export-on-processor = 以處理器編碼，需要較長時間
export-no-encoder = 此處無法開啟此格式的編碼器
export-checking = 正在尋找編碼器…
export-sound-format = 音訊格式
export-quality = 音質
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = 壓縮格式；幾乎在任何地方都能播放，包括 Apple 的裝置
export-mp3-note = 壓縮格式；在任何地方都能播放
export-opus-note = 壓縮格式；同等大小下音質最佳，但並非所有播放器都支援
export-flac-note = 無損：完整保留原本的聲音，採用開放格式
export-alac-note = 無損：完整保留原本的聲音，採用 Apple 裝置播放的格式
export-wav-note = 未壓縮：完整保留原本的聲音，檔案最大
export-kilobits = { $rate } kb/s
export-bits = { $bits } 位元
export-rate-lower = 以 { $rate } kHz 儲存，低於檔案的 { $file } kHz
export-choose = 輸出…
export-name = { $name }（已輸出）
export-name-edited = { $name }（已編輯）
export-progress = 正在輸出「{ $name }」
export-stop = 停止輸出
notice-exported = 已輸出「{ $name }」
notice-export-stopped = 已停止輸出
notice-export-failed = 無法輸出：{ $error }
close-title = 要關閉而不輸出嗎？
close-unexported = 如果不輸出，對「{ $name }」的更改將會遺失。
close-exporting = 「{ $name }」的輸出將會停止，目前已寫入的內容將會遺失。
close-anyway = 關閉
close-quit-anyway = 結束

## Recording

record-title = 錄音
record-file-name = 錄音
record-start = 錄製
record-stop = 停止錄製
record-no-input = 沒有麥克風
record-microphone = 麥克風
record-default-microphone = 系統預設
record-playing = 電腦正在播放的聲音
record-mono = 單聲道
record-stereo = 立體聲
record-channels = { $count } 聲道
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }，以 { $saved } kHz 儲存
action-record-sound = 錄音
open-record-sound = 新增錄音
start-record-sound = 錄音
menu-record-sound = 新增錄音
record-screen-title = 螢幕錄製
record-screen-file-name = 螢幕錄製
record-screen-whole = 螢幕
record-screen-number = 螢幕 { $number }
record-screen-main = 螢幕 { $number }（主螢幕）
record-screens = 螢幕
record-windows = 視窗
record-screen-what = 錄製內容
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = 每個影格
record-rate-tip = 每秒影格數
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · 已丟棄 { $count } 格
record-screen-window = 視窗
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = 已停止分享
record-screen-again = 重新選擇
record-screen-failed = 無法錄製螢幕
record-sound = 聲音
record-no-sound = 無聲音
# How much louder the input is taken, for a quiet microphone.
record-gain = 增益
record-gain-value = +{ $decibels } dB
record-level-automatically = 自動調整音量
record-hide-title = plaa 會在錄製時隱藏
record-hide-tray = 若要停止錄製，請按一下 plaa 放在系統匣中的紅點，或再次開啟 plaa。
record-hide-reopen = 若要停止錄製，請再次開啟 plaa。
record-hide-start = 開始錄製
record-tray-title = plaa 正在錄製螢幕
record-tray-tip = 按一下以停止錄製
action-record-screen = 錄製螢幕
open-record-screen = 新增螢幕錄製
start-record-screen = 錄製螢幕
menu-record-screen = 新增螢幕錄製
record-camera-title = 相機錄製
record-camera-file-name = 相機錄製
record-camera = 相機
record-camera-failed = 無法開啟相機
action-record-camera = 錄製相機
open-record-camera = 新增相機錄製
start-record-camera = 錄製相機
menu-record-camera = 新增相機錄製
notice-no-camera = 沒有可錄製的相機
notice-recorded = 已將「{ $name }」儲存到 { $folder }
notice-record-failed = 無法錄製：{ $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = 允許外部控制
settings-allow-outside-control-note = Claude Code 等 AI 代理程式可以透過 plaa --mcp 查看 plaa 正在播放的內容並加以控制。每個新的代理程式接入前，plaa 都會先詢問。
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = 已允許：{ $agents }
settings-forget-agents = 忘記
settings-ask-before-note = 代理程式執行以下操作前先詢問：
settings-ask-reading = 查看正在播放的內容或畫面
settings-ask-playing = 控制播放、聲音或全螢幕
settings-ask-opening = 開啟檔案或關閉視窗
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = 要允許 { $agent } 控制 plaa 嗎？
agent-prompt-body = { $agent } 要求使用 plaa 的外部控制，以查看正在播放的內容並加以控制。你可以在「設定」中關閉外部控制。
agent-prompt-allow = 允許
agent-prompt-deny = 不允許
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = 要允許 { $agent } 查看這裡正在播放的內容嗎？
agent-ask-play = 要允許 { $agent } 控制這裡的播放嗎？
agent-ask-open = 要允許 { $agent } 開啟或關閉視窗嗎？
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } 要求使用「{ $tool }」。你可以在「設定」中選擇 plaa 要詢問哪些操作。
