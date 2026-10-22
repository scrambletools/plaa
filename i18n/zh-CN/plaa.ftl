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
language-name = 简体中文

## Common

app-name = plaa
common-cancel = 取消
common-close = 关闭
common-more = 更多

## Command line

usage-help =
    用法：plaa [FILE OR ADDRESS]...

    播放视频和音频文件，以及 https://example.com/live.m3u8 等地址上的流。
    每一项都会在正在运行的 plaa 的窗口中打开，如有需要会先启动 plaa。

    选项：
      -h, --help        显示此帮助
      -V, --version     显示版本
          --interfaces  显示有线接口，以及 plaa 能否在这些接口上发送
                        AVB 流
          --mcp         通过 stdin 和 stdout 提供 MCP，让 AI 智能体查看
                        正在运行的 plaa 播放的内容并加以控制

## Start window

start-hint = 打开视频或音频文件、流或网络视频。
start-open = 打开
start-open-address = 打开地址
# Opens a list of the talkers on the AVB network to listen to.
start-connect = 连接

## Actions, as buttons, tooltips and menus name them

action-open = 打开
action-open-address = 打开地址
open-file = 打开文件…
open-address = 打开地址…
action-settings = 设置
action-info = 检查器
action-loop = 循环
action-screenshot = 将帧保存为图片
action-play = 播放
action-pause = 暂停
# $seconds is how far the button moves, such as 10.
action-back = 后退 { $seconds } 秒
action-forward = 前进 { $seconds } 秒
action-mute = 静音
action-unmute = 取消静音
action-speed = 播放速度
action-tracks = 音频和字幕
action-full-screen = 全屏幕
action-playlist = 播放列表
action-chapters = 章节
action-picture-in-picture = 画中画
action-previous = 上一个
action-next = 下一个
action-exit-full-screen = 退出全屏幕
action-quality = 画质
action-visualizer = 可视化效果
action-network = 网络音频
action-cast = 投放
action-connect = 连接
action-assistant = 助手

## What a sound file shows under its cover

visualizer-spectrum = 频谱
visualizer-waveform = 波形
visualizer-prism = 棱镜
visualizer-cover = 仅封面

## The floating controls

overlay-keep-shown = 固定控件
overlay-auto-hide = 让控件浮在画面上

## Audio and subtitle tracks

tracks-audio = 音频
tracks-subtitles = 字幕
tracks-off = 关闭
# A track the file gives no name or language, by its number.
tracks-numbered = 轨道 { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = 仅音频

## Inspector

info-title = 检查器
playlist-title = 播放列表
chapters-numbered = 第 { $number } 章
info-file = 文件
info-name = 名称
info-address = 地址
info-folder = 文件夹
info-size = 大小
info-format = 格式
info-duration = 时长
info-bit-rate = 比特率
info-video = 视频
info-audio = 音频
info-codec = 编解码器
info-dimensions = 尺寸
info-frame-rate = 帧速率
info-frames-per-second = 每秒 { $rate } 帧
info-pixel-format = 像素格式
info-decoding = 解码
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = 显卡（{ $api }）
info-decoding-software = 处理器
info-sample-rate = 采样率
info-hertz = { $rate } Hz
info-channels = 声道
info-language = 语言
info-no-sound = 无声音
info-tags = 标签

## Dialogs

dialog-open = 打开
dialog-filter-media = 视频和音频
dialog-filter-all = 所有文件

address-title = 打开地址
address-note = 含有视频的网页（例如 YouTube 视频的页面），或网络上的流或文件（例如 https://example.com/live.m3u8）。
address-label = 地址
address-open = 打开
address-recent = 最近使用
address-forget = 从最近使用中移除

## Notices

notice-dialog-failed = 无法打开文件对话框：{ $error }
notice-screenshot-saved = 已保存“{ $name }”。
notice-screenshot-failed = 无法保存图片：{ $error }
notice-no-sound = 正在无声播放：{ $reason }
notice-loop-from = 从 { $time } 开始循环。请在结束位置再按一次。
notice-loop-between = 正在循环播放 { $from } 到 { $to }。
notice-loop-off = 循环已关闭。

## Errors, shown in place of what could not play

error-needs-yt-dlp = 这看起来是一个网页。plaa 使用 yt-dlp 播放网页上的视频，但尚未安装 yt-dlp。
error-empty-list = 此列表中没有可播放的内容。

## Settings

settings-title = 设置
settings-tab-appearance = 外观
settings-tab-playback = 播放
settings-tab-network = 网络
settings-tab-language = 语言
settings-tab-agents = 智能体
settings-tab-assistant = 助手
settings-version = plaa { $version }（{ $build }）
settings-version-development = plaa { $version }（开发版，{ $build }）
settings-choose = 选择…

settings-appearance-system = 跟随系统
settings-appearance-light = 浅色
settings-appearance-dark = 深色
settings-colors = 颜色
settings-system-accent = 使用系统强调色
settings-omarchy-note = 颜色根据“{ $theme }”的强调色生成。
settings-system-accent-note = 颜色根据系统的强调色生成。
settings-system-accent-none = 系统没有强调色，因此 plaa 使用下方选择的颜色。
settings-accent-chosen-note = 颜色根据下方选择的颜色生成。
settings-controls = 控件
settings-overlay = 浮动控件
settings-overlay-note = 工具栏和播放控件浮在画面或封面上方，指针停住不动或位于窗口外时会滑出隐藏。
settings-overlay-delay = 隐藏前等待
settings-overlay-delay-note = 指针在播放内容上停住多久后，控件才滑出隐藏。
settings-seconds = { $seconds } 秒
settings-transparency = 透明度
settings-transparency-note = 透过浮动控件能看到多少画面内容。
settings-percent = { $percent }%
settings-corner-radius = 圆角半径
settings-corner-radius-note = 用于对话框和浮动控件。
settings-pixels = { $pixels } px
settings-animations = 动画
settings-animations-note = 滑动的栏和面板、展开的对话框以及有弹性的按钮。
settings-animations-reduced = 系统要求减弱动态效果时关闭。

settings-hardware = 使用显卡
settings-hardware-note = 用显卡解码视频和编码导出内容：更省电，大视频播放更流畅，导出也更快。对之后打开的文件生效。
settings-audio-device = 声音输出
settings-audio-device-system = 系统默认
settings-resume = 从上次停止处继续
settings-resume-note = 文件从上次关闭时的位置开始播放。
settings-folder-playlists = 播放文件夹中的其他文件
settings-folder-playlists-note = 打开一个文件时，会列出同一文件夹中的其他文件，并接着播放。
settings-streams = 流和网络视频
settings-stream-quality = 画质
settings-stream-quality-note = 用于提供多种画质的流和网络视频。对之后打开的内容生效。
settings-quality-auto = 自动，与屏幕同高
settings-quality-best = 最佳
settings-web-pages = 使用 yt-dlp 播放网络视频
settings-web-pages-found = yt-dlp 能在 YouTube 等网页上找到视频。
settings-web-pages-missing = 未安装 yt-dlp。安装后即可播放 YouTube 等网页上的视频。
settings-folders = 文件夹
settings-recordings = 录制内容
settings-screenshots = 已保存的帧

settings-interface-language = 界面语言
# The first entry of the language list.
settings-language-system = 跟随系统
settings-track-languages = 轨道语言
settings-track-languages-note = 语言代码（例如 en 或 fra），最想要的排在最前，以逗号分隔。文件中有这些语言的音频和字幕时，plaa 会选择它们。
settings-audio-languages = 音频
settings-subtitle-languages = 字幕

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB 接口
settings-avb-interface-note = plaa 在此有线端口上查找 AVB 和 Milan 的发送端与接收端。发送需要带有 PTP 硬件时钟的端口，例如 Intel I210 的端口。
settings-avb-off = 关闭
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface }（仅列出：没有 PTP 时钟）
settings-avb-off-mac = { $interface }（仅列出：AVB 已关闭）
settings-avb-interface-note-mac = plaa 在此有线端口上查找 AVB 和 Milan 的发送端与接收端。发送会通过这台 Mac 自身的 AVB 实体进行，因此需要在“音频 MIDI 设置”的“网络设备浏览器”中为该端口打开 AVB。
settings-avb-talker-note-mac = plaa 通过这台 Mac 自身的 AVB 实体，将最后播放的窗口的声音发送给与其连接的接收端。
settings-avb-talker = 发送正在播放的内容
settings-avb-talker-note = plaa 成为 Milan 发送端，将最后播放的窗口的声音发送给与其连接的接收端。
settings-avb-name = 网络上的名称
settings-avb-name-note = 控制器会将 plaa 显示为“{ $name }”。按 Enter 键即可重命名。
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } 上的 plaa
avb-could-not-start = 无法使用 { $interface }：{ $error }
network-title = 网络音频
network-off = 在“设置”中打开“发送正在播放的内容”，即可在网络上播放。
network-blocked = 未发送：{ $reason }
network-waiting = 已准备好发送，正在等待接收端连接。
network-sending = 正在发送给已连接的接收端。
network-looking = 正在查找接收端…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = 正在启动 ptp4l，它维护 plaa 发送流所依据的网络时间…
network-ptp-elsewhere = ptp4l 在另一个接口上运行，因此 plaa 的流在这里无法与网络时间同步，接收端可能无法播放。请在“设置”中选择那个接口，或在此接口上运行 ptp4l。
network-ptp-missing = ptp4l 未运行，因此 plaa 的流无法与网络时间同步，接收端可能无法播放。请随 plaa 的软件包安装 linuxptp，或安装会运行 ptp4l 的 triib。
network-ptp-failed = ptp4l 未运行，因此 plaa 的流无法与网络时间同步，接收端可能无法播放。plaa 无法启动 ptp4l：{ $reason }
network-no-listeners = 网络上没有能播放 plaa 声音的接收端。
# A listener with several inputs: its name and the input's.
network-stream = { $entity }：{ $stream }
network-play-on = 在 { $listener } 上播放
network-stop-on = 停止在 { $listener } 上播放
network-choose-format = 选择在 { $listener } 上播放的格式
network-failed = 无法连接：{ $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = 请更新 triib 以在此播放。
network-choose = 选择一个接收端，查看它如何接收流。
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = 在此收听
network-no-sources = 网络上没有以 plaa 能播放的格式发送、或能切换为这种格式的发送端。
# $talker is a talker's name, or its name and its output's.
network-listen-to = 收听 { $talker }
network-listen-stop = 停止收听 { $talker }
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = 将 { $talker } 切换为 { $format } 并收听
network-listen-change-cuts-off = 将 { $talker } 切换为 { $format } 并收听，{ $count ->
       *[other] 同时断开其他 { $count } 个接收端
    }
network-listen-frames = 已接收 { $frames } 帧
network-listen-update-triib = 请更新 triib 以在此收听：plaa 只会播放本来就到达这台电脑的内容。
network-listen-lite = 该端口运行 AVB Lite，不预留任何流：plaa 只会播放本来就到达这台电脑的内容。

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = 正在查找发送端…
# plaa listens through its entity, which exists only while it sends.
connect-off = 在“设置”中打开“发送正在播放的内容”，即可在此收听发送端。
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa 无法在此收听：{ $reason }
connect-sending = 以 plaa 能播放的格式发送
connect-changing = 切换为 plaa 能播放的格式
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }，采用 { $format }
connect-change-cuts-off = { $talker }，采用 { $format }，{ $count ->
       *[other] 断开其他 { $count } 个接收端
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = 状态
network-not-connected = 未连接
network-from = 来自 { $stream }
network-from-receiving = 来自 { $stream }，正在接收
network-from-waiting = 来自 { $stream }，正在等待发送端
network-from-failed = 来自 { $stream }，发送端的预留失败：{ $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = 媒体时钟
network-locked = 已锁定到流
network-not-locked = 未锁定
network-format = 格式
# The formats both the listener and plaa's stream can be in.
network-formats = 可从 plaa 接收的格式
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = 流 ID
# The MAC address the stream is sent to.
network-destination = 目标
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = 延迟
network-microseconds = { $time } µs
network-frames = 已接收的帧
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = 顺序错乱的帧
# How often the stream stopped coming while it was taken.
network-interrupted = 中断次数
network-entity-id = 实体 ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = 带宽不足
msrp-failure-2 = 网桥资源不足
msrp-failure-3 = 流量类别带宽不足
msrp-failure-4 = 流 ID 已被其他发送端使用
msrp-failure-5 = 目标地址已被使用
msrp-failure-6 = 被更高等级的流抢占
msrp-failure-7 = 报告的延迟已改变
msrp-failure-8 = 出口端口不支持 AVB
msrp-failure-9 = 需使用其他目标地址
msrp-failure-10 = MSRP 资源耗尽
msrp-failure-11 = MMRP 资源耗尽
msrp-failure-12 = 无法存储目标地址
msrp-failure-13 = 优先级不是 SR 类别优先级
msrp-failure-14 = 帧对介质而言过大
msrp-failure-15 = 已达到扇入端口上限
msrp-failure-16 = 已注册流的首个值已更改
msrp-failure-17 = VLAN 在出口端口上被阻止
msrp-failure-18 = 出口端口上已禁用 VLAN 标记
msrp-failure-19 = SR 类别优先级不匹配
msrp-failure-unknown = 未知原因
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }，位于网桥 { $bridge }

## The macOS menu bar

menu-about = 关于 plaa
menu-settings = 设置…
menu-services = 服务
menu-hide = 隐藏 plaa
menu-hide-others = 隐藏其他
menu-show-all = 全部显示
menu-quit = 退出 plaa
menu-file = 文件
menu-open = 打开…
menu-open-address = 打开地址…
menu-close = 关闭窗口
menu-edit = 编辑
menu-cut = 剪切
menu-copy = 拷贝
menu-paste = 粘贴
menu-select-all = 全选
menu-undo = 撤销
menu-redo = 重做
menu-trim = 修剪…
menu-split = 拆分剪辑
menu-rotate-left = 向左旋转
menu-rotate-right = 向右旋转
menu-flip-horizontal = 水平翻转
menu-flip-vertical = 垂直翻转
menu-revert = 复原到原始版本
menu-export = 导出…
menu-view = 显示
menu-playback = 播放
menu-play-pause = 播放或暂停
menu-to-start = 前往开头
menu-previous-chapter = 上一章节
menu-next-chapter = 下一章节
menu-faster = 加快
menu-slower = 减慢
menu-normal-speed = 正常速度
menu-ab-loop = 在两点之间循环
menu-volume-up = 调高音量
menu-volume-down = 调低音量
menu-mute = 静音或取消静音
menu-next-subtitles = 下一个字幕
menu-next-audio = 下一个音轨
menu-window = 窗口
menu-minimize = 最小化
menu-zoom = 缩放
menu-bring-all-to-front = 前置全部窗口

## Editing

action-edit = 编辑
title-edited = { $name }（已编辑）
edit-trim = 修剪
edit-trim-apply = 修剪
edit-split = 拆分剪辑
edit-remove-clip = 移除剪辑
edit-done = 完成
edit-rotate-left = 向左旋转
edit-rotate-right = 向右旋转
edit-flip-horizontal = 水平翻转
edit-flip-vertical = 垂直翻转
edit-undo = 撤销
edit-redo = 重做
edit-revert = 复原到原始版本
edit-export = 导出…

## Exporting

export-title = 导出
export-format = 格式
export-size = 大小
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = 仅音频
export-size-original = { $lines }p，保持原样
export-on-graphics-card = 在显卡上编码（{ $api }）
export-on-processor = 在处理器上编码，耗时更长
export-no-encoder = 此处无法打开此格式的编码器
export-checking = 正在查找编码器…
export-sound-format = 音频格式
export-quality = 音质
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = 压缩格式；几乎在任何地方都能播放，包括 Apple 的设备
export-mp3-note = 压缩格式；在任何地方都能播放
export-opus-note = 压缩格式；同等大小下音质最佳，但并非所有播放器都支持
export-flac-note = 无损：完整保留原本的声音，采用开放格式
export-alac-note = 无损：完整保留原本的声音，采用 Apple 设备播放的格式
export-wav-note = 未压缩：完整保留原本的声音，文件最大
export-kilobits = { $rate } kb/s
export-bits = { $bits } 位
export-rate-lower = 以 { $rate } kHz 保存，低于文件的 { $file } kHz
export-choose = 导出…
export-name = { $name }（已导出）
export-name-edited = { $name }（已编辑）
export-progress = 正在导出“{ $name }”
export-stop = 停止导出
notice-exported = 已导出“{ $name }”
notice-export-stopped = 导出已停止
notice-export-failed = 无法导出：{ $error }
close-title = 不导出就关闭吗？
close-unexported = 如果不导出，对“{ $name }”的更改将会丢失。
close-exporting = “{ $name }”的导出将会停止，目前已写入的内容将会丢失。
close-anyway = 关闭
close-quit-anyway = 退出

## Recording

record-title = 录音
record-file-name = 录音
record-start = 录制
record-stop = 停止录制
record-no-input = 没有麦克风
record-microphone = 麦克风
record-default-microphone = 系统默认
record-playing = 电脑正在播放的声音
record-mono = 单声道
record-stereo = 立体声
record-channels = { $count } 声道
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }，以 { $saved } kHz 保存
action-record-sound = 录音
open-record-sound = 新建录音
start-record-sound = 录音
menu-record-sound = 新建录音
record-screen-title = 屏幕录制
record-screen-file-name = 屏幕录制
record-screen-whole = 屏幕
record-screen-number = 屏幕 { $number }
record-screen-main = 屏幕 { $number }（主屏幕）
record-screens = 屏幕
record-windows = 窗口
record-screen-what = 录制内容
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = 每一帧
record-rate-tip = 每秒帧数
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · 丢弃 { $count } 帧
record-screen-window = 窗口
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = 已停止共享
record-screen-again = 重新选择
record-screen-failed = 无法录制屏幕
record-sound = 声音
record-no-sound = 无声音
# How much louder the input is taken, for a quiet microphone.
record-gain = 增益
record-gain-value = +{ $decibels } dB
record-level-automatically = 自动调节音量
record-hide-title = plaa 会在录制时隐藏
record-hide-tray = 要停止录制，请点击 plaa 放在托盘中的红点，或再次打开 plaa。
record-hide-reopen = 要停止录制，请再次打开 plaa。
record-hide-start = 开始录制
record-tray-title = plaa 正在录制屏幕
record-tray-tip = 点击以停止录制
action-record-screen = 录制屏幕
open-record-screen = 新建屏幕录制
start-record-screen = 录制屏幕
menu-record-screen = 新建屏幕录制
record-camera-title = 摄像头录制
record-camera-file-name = 摄像头录制
record-camera = 摄像头
record-camera-failed = 无法打开摄像头
action-record-camera = 录制摄像头
open-record-camera = 新建摄像头录制
start-record-camera = 录制摄像头
menu-record-camera = 新建摄像头录制
notice-no-camera = 没有可录制的摄像头
notice-recorded = 已将“{ $name }”保存到 { $folder }
notice-record-failed = 无法录制：{ $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = 允许外部控制
settings-allow-outside-control-note = Claude Code 等 AI 智能体可以通过 plaa --mcp 查看 plaa 正在播放的内容并加以控制。每个新智能体接入前，plaa 都会先询问。
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = 已允许：{ $agents }
settings-forget-agents = 忘记
settings-ask-before-note = 智能体执行以下操作前先询问：
settings-ask-reading = 查看正在播放的内容或画面
settings-ask-playing = 控制播放、声音或全屏幕
settings-ask-opening = 打开文件或关闭窗口
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = 允许 { $agent } 控制 plaa？
agent-prompt-body = { $agent } 请求使用 plaa 的外部控制，以查看正在播放的内容并加以控制。你可以在“设置”中关闭外部控制。
agent-prompt-allow = 允许
agent-prompt-deny = 不允许
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = 允许 { $agent } 查看这里正在播放的内容？
agent-ask-play = 允许 { $agent } 控制这里的播放？
agent-ask-open = 允许 { $agent } 打开或关闭窗口？
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } 请求使用“{ $tool }”。你可以在“设置”中选择 plaa 询问哪些操作。

## Casting to a TV or speaker on the network

cast-looking = 正在查找电视和音箱…
cast-none = 未找到电视、音箱或 Chromecast。
# $renderer is the TV's or speaker's name.
cast-stop = 停止在 { $renderer } 上播放
notice-casting = 正在 { $renderer } 上播放。
notice-cast-failed = 无法在 { $renderer } 上播放：{ $reason }
cast-web-page = 暂时无法投放网页：请改为打开视频本身的地址。

## The assistant panel, and its models in Settings

assistant-add-another = 添加模型…
assistant-add-model = 助手需要一个模型：带 API 密钥的云端模型，或本地模型。
assistant-ask = 就正在播放的内容提问
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = 对话越来越长，{ $model } 无法一次读完，因此已将之前的部分总结。助手可能会重新查看它看过的内容。
assistant-compacting = 正在总结之前的对话
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = 这台电脑上的 { $provider }
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $host } 上的 { $provider }
assistant-new-chat = 新对话
assistant-no-model = 请先在“设置”中添加模型。
assistant-open-settings = 添加模型
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = 对话内容已超出 { $model } 一次能读取的范围。请开始新对话，或选择能读取更多内容的模型。
assistant-problem-credit = { $provider } 表示账户没有余额。新账户需要先在 { $provider } 的网站上购买额度，密钥才能使用；购买后请再试一次。
assistant-problem-key = { $provider } 拒绝了 API 密钥。请在“设置”中检查。
assistant-problem-model = { $provider } 没有名为 { $model } 的模型。请在“设置”中检查名称。
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } 未回答：{ $message }
assistant-problem-rate = { $provider } 要求降低请求频率。请稍后再试。
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } 要求降低请求频率：{ $message }
assistant-problem-refused = { $model } 拒绝回答。
assistant-problem-unavailable = { $provider } 繁忙或出现问题。请稍后再试。
assistant-problem-unreachable = plaa 无法连接到 { $provider }。请检查网络连接，或确认服务器正在运行。
assistant-running = 正在运行…
assistant-send = 发送
assistant-shortened = 对话太长，{ $model } 无法一次读完，因此省略了之前的工具结果。助手可能会重新查看它看过的内容。
assistant-stop = 停止
assistant-stopped = 已停止。
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = 现在使用 { $model } 对话。
assistant-thinking = 正在思考…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = 思考过程
assistant-title = 助手
settings-assistant-add = 添加模型
settings-assistant-add-button = 添加
# $model is the model's name.
settings-assistant-added = { $model } 已应答，已添加。
settings-assistant-added-tag = 已添加
# $example is an address, such as http://localhost:11434.
settings-assistant-address = 地址，例如 { $example }
settings-assistant-change-key = 更改密钥
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = 兼容 OpenAI 的服务器
# The menu of how much a local model reads at once.
settings-assistant-context = 上下文
settings-assistant-context-note = 调大后，助手可以记住对话中更多的内容，但模型会占用更多内存，回答也可能变慢。
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K Token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = 打开页面
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } 正在 { $address } 运行。
settings-assistant-found-none = { $provider } 还没有模型。请先用它下载一个模型，然后重新查找。
settings-assistant-get-key = 获取 API 密钥
settings-assistant-get-server = 获取 { $provider }
settings-assistant-in-use = 使用中
settings-assistant-key = API 密钥
# $error is what the keychain said.
settings-assistant-key-failed = 无法将密钥保存到密钥环：{ $error }
settings-assistant-key-kept = 你的 { $provider } 密钥保存在系统密钥环中。
settings-assistant-key-needed = 此模型需要 API 密钥。
settings-assistant-key-refused = { $provider } 拒绝了此密钥。请检查是否从正确的账户完整复制了密钥。
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = 在 { $provider } 的网站上创建密钥，然后粘贴到这里。
settings-assistant-look-again = 重新查找
settings-assistant-looking = 正在查找模型…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = 模型，例如 { $example }
settings-assistant-no-server = plaa 未在这台电脑上找到正在运行的 { $provider }。请启动它，或在下方输入它的地址。
settings-assistant-no-tools = 无法使用工具，而助手需要工具
settings-assistant-none = 还没有模型。请在下方添加：本地模型（例如 Ollama 的模型）只在这台电脑上运行；云端模型需要其提供商的 API 密钥。
settings-assistant-note = 助手面板可以使用的模型。密钥保存在系统密钥环中。
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = 使用其他地址
settings-assistant-recommended = 推荐
settings-assistant-remove = 移除
settings-assistant-sees = 可识别图片
settings-assistant-trying = 正在尝试…
settings-assistant-use = 使用
settings-assistant-use-key = 继续
settings-assistant-uses-tools = 可使用工具
