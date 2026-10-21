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
language-name = 日本語

## Common

app-name = plaa
common-cancel = キャンセル
common-close = 閉じる
common-more = その他

## Command line

usage-help =
    使い方：plaa [FILE OR ADDRESS]...

    ビデオやオーディオのファイルと、https://example.com/live.m3u8などの
    アドレスにあるストリームを再生します。それぞれ実行中のplaaのウインドウで
    開きます。plaaが起動していない場合は起動します。

    オプション：
      -h, --help        このヘルプを表示
      -V, --version     バージョンを表示
          --interfaces  有線インターフェイスと、それぞれでplaaがAVBストリームを
                        送信できるかどうかを表示
          --mcp         stdinとstdoutでMCPを提供し、AIエージェントが
                        実行中のplaaの再生内容を確認して操作できるようにします

## Start window

start-hint = ビデオファイル、オーディオファイル、ストリーム、Web動画を開いてください。
start-open = 開く
start-open-address = アドレスを開く

## Actions, as buttons, tooltips and menus name them

action-open = 開く
action-open-address = アドレスを開く
open-file = ファイルを開く…
open-address = アドレスを開く…
action-settings = 設定
action-info = インスペクタ
action-loop = ループ
action-screenshot = フレームを画像として保存
action-play = 再生
action-pause = 一時停止
# $seconds is how far the button moves, such as 10.
action-back = { $seconds }秒戻る
action-forward = { $seconds }秒進む
action-mute = ミュート
action-unmute = ミュートを解除
action-speed = 再生速度
action-tracks = オーディオと字幕
action-full-screen = フルスクリーン
action-playlist = プレイリスト
action-chapters = チャプター
action-picture-in-picture = ピクチャ・イン・ピクチャ
action-previous = 前へ
action-next = 次へ
action-exit-full-screen = フルスクリーンを解除
action-quality = 画質
action-visualizer = ビジュアライザ
action-network = ネットワークオーディオ
action-cast = キャスト

## What a sound file shows under its cover

visualizer-spectrum = スペクトラム
visualizer-waveform = 波形
visualizer-prism = プリズム
visualizer-cover = アートワークのみ

## The floating controls

overlay-keep-shown = コントロールを固定表示
overlay-auto-hide = コントロールを映像の上に浮かべる

## Audio and subtitle tracks

tracks-audio = オーディオ
tracks-subtitles = 字幕
tracks-off = オフ
# A track the file gives no name or language, by its number.
tracks-numbered = トラック{ $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = オーディオのみ

## Inspector

info-title = インスペクタ
playlist-title = プレイリスト
chapters-numbered = チャプター{ $number }
info-file = ファイル
info-name = 名前
info-address = アドレス
info-folder = フォルダ
info-size = サイズ
info-format = フォーマット
info-duration = 長さ
info-bit-rate = ビットレート
info-video = ビデオ
info-audio = オーディオ
info-codec = コーデック
info-dimensions = 寸法
info-frame-rate = フレームレート
info-frames-per-second = 毎秒{ $rate }フレーム
info-pixel-format = ピクセルフォーマット
info-decoding = デコード
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = グラフィックカード（{ $api }）
info-decoding-software = プロセッサ
info-sample-rate = サンプルレート
info-hertz = { $rate } Hz
info-channels = チャンネル
info-language = 言語
info-no-sound = サウンドなし
info-tags = タグ

## Dialogs

dialog-open = 開く
dialog-filter-media = ビデオとオーディオ
dialog-filter-all = すべてのファイル

address-title = アドレスを開く
address-note = YouTubeの動画ページのような、動画のあるWebページ。または、https://example.com/live.m3u8のような、ネットワーク上のストリームやファイル。
address-label = アドレス
address-open = 開く
address-recent = 最近開いた項目
address-forget = 最近開いた項目から削除

## Notices

notice-dialog-failed = ファイルダイアログを開けませんでした：{ $error }
notice-screenshot-saved = 「{ $name }」を保存しました。
notice-screenshot-failed = 画像を保存できませんでした：{ $error }
notice-no-sound = サウンドなしで再生しています：{ $reason }
notice-loop-from = { $time }からループします。終了する位置でもう一度押してください。
notice-loop-between = { $from }から{ $to }までをループしています。
notice-loop-off = ループをオフにしました。

## Errors, shown in place of what could not play

error-needs-yt-dlp = これはWebページのようです。plaaはWebページの動画をyt-dlpで再生しますが、yt-dlpがインストールされていません。
error-empty-list = このリストには再生するものがありません。

## Settings

settings-title = 設定
settings-tab-appearance = 外観
settings-tab-playback = 再生
settings-tab-network = ネットワーク
settings-tab-language = 言語
settings-tab-agents = エージェント
settings-version = plaa { $version }（{ $build }）
settings-version-development = plaa { $version }（開発版、{ $build }）
settings-choose = 選択…

settings-appearance-system = システムに従う
settings-appearance-light = ライト
settings-appearance-dark = ダーク
settings-colors = カラー
settings-system-accent = システムのアクセントカラーを使用
settings-omarchy-note = 「{ $theme }」のアクセントからカラーを作成しています。
settings-system-accent-note = システムのアクセントカラーからカラーを作成しています。
settings-system-accent-none = システムにアクセントカラーがないため、plaaは下で選択したカラーを使用しています。
settings-accent-chosen-note = 下で選択したカラーからカラーを作成しています。
settings-controls = コントロール
settings-overlay = コントロールをフローティング表示
settings-overlay-note = ツールバーと再生コントロールは映像やアートワークの上に浮かび、ポインタが止まっているあいだや、ウインドウの外にあるあいだはスライドして隠れます。
settings-overlay-delay = 隠すまでの時間
settings-overlay-delay-note = 再生中の映像の上でポインタが止まってから、コントロールがスライドして隠れるまでの時間です。
settings-seconds = { $seconds }秒
settings-transparency = 透明度
settings-transparency-note = フローティングコントロール越しに映像が透けて見える度合いです。
settings-percent = { $percent }%
settings-corner-radius = 角の半径
settings-corner-radius-note = ダイアログとフローティングコントロールに適用されます。
settings-pixels = { $pixels } px
settings-animations = アニメーション
settings-animations-note = バーやパネルのスライド、ダイアログの拡大、弾むボタン。
settings-animations-reduced = システムで動きを減らす設定がオンのあいだはオフになります。

settings-hardware = グラフィックカードを使用
settings-hardware-note = ビデオのデコードと書き出し時のエンコードをグラフィックカードで行います。消費電力が減り、大きなビデオも滑らかに再生でき、書き出しも速くなります。このあと開いたファイルから有効になります。
settings-audio-device = サウンド出力
settings-audio-device-system = システムのデフォルト
settings-resume = 前回の続きから再生
settings-resume-note = ファイルは前回閉じたときの位置から始まります。
settings-folder-playlists = フォルダ内のほかのファイルも再生
settings-folder-playlists-note = ファイルを1つ開くと、同じフォルダのほかのファイルが一覧に加わり、続けて再生されます。
settings-streams = ストリームとWeb動画
settings-stream-quality = 画質
settings-stream-quality-note = 複数の画質があるストリームやWeb動画に適用されます。このあと開くものから有効になります。
settings-quality-auto = 自動（画面の高さに合わせる）
settings-quality-best = 最高
settings-web-pages = yt-dlpでWeb動画を再生
settings-web-pages-found = yt-dlpがYouTubeなどのページから動画を見つけます。
settings-web-pages-missing = yt-dlpがインストールされていません。YouTubeなどのページの動画を再生するには、yt-dlpをインストールしてください。
settings-folders = フォルダ
settings-recordings = 収録
settings-screenshots = 保存したフレーム

settings-interface-language = 表示言語
# The first entry of the language list.
settings-language-system = システムに従う
settings-track-languages = トラックの言語
settings-track-languages-note = enやfraなどの言語コードを、優先するものから順にカンマで区切って入力します。ファイルにこれらの言語のオーディオや字幕があれば、plaaがそれを選びます。
settings-audio-languages = オーディオ
settings-subtitle-languages = 字幕

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVBインターフェイス
settings-avb-interface-note = plaaがAVBやMilanのトーカーとリスナーを探す有線ポートです。送信には、Intel I210のようなPTPハードウェアクロックを備えたポートが必要です。
settings-avb-off = オフ
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface }（一覧のみ：PTPクロックなし）
settings-avb-off-mac = { $interface }（一覧のみ：AVBがオフ）
settings-avb-interface-note-mac = plaaがAVBやMilanのトーカーとリスナーを探す有線ポートです。送信はこのMac自身のAVBエンティティを通じて行われるため、「Audio MIDI設定」の「ネットワークデバイスブラウザ」で、このポートのAVBをオンにする必要があります。
settings-avb-talker-note-mac = plaaは最後に再生したウインドウのサウンドを、このMac自身のAVBエンティティを通じて、接続されているリスナーに送信します。
settings-avb-talker = 再生中のサウンドを送信
settings-avb-talker-note = plaaがMilanトーカーになり、最後に再生したウインドウのサウンドを、接続されているリスナーに送信します。
settings-avb-name = ネットワーク上の名前
settings-avb-name-note = コントローラにはplaaが「{ $name }」と表示されます。Enterキーを押すと名前が変わります。
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer }のplaa
avb-could-not-start = { $interface }を使用できませんでした：{ $error }
network-title = ネットワークオーディオ
network-off = ネットワークで再生するには、「設定」で「再生中のサウンドを送信」をオンにしてください。
network-blocked = 送信していません：{ $reason }
network-waiting = 送信の準備ができました。リスナーの接続を待っています。
network-sending = 接続されているリスナーに送信しています。
network-looking = リスナーを探しています…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = plaaのストリームの送信基準となるネットワーク時刻を管理するptp4lを起動しています…
network-ptp-elsewhere = ptp4lは別のインターフェイスで動作しているため、ここではplaaのストリームがネットワーク時刻に同期できず、リスナーが再生できない場合があります。「設定」でそのインターフェイスを選ぶか、このインターフェイスでptp4lを実行してください。
network-ptp-missing = ptp4lが動作していないため、plaaのストリームがネットワーク時刻に同期できず、リスナーが再生できない場合があります。plaaのパッケージと一緒にlinuxptpをインストールするか、ptp4lを実行するtriibをインストールしてください。
network-ptp-failed = ptp4lが動作していないため、plaaのストリームがネットワーク時刻に同期できず、リスナーが再生できない場合があります。plaaはptp4lを起動できませんでした：{ $reason }
network-no-listeners = ネットワーク上に、plaaのサウンドを再生できるリスナーがありません。
# A listener with several inputs: its name and the input's.
network-stream = { $entity }：{ $stream }
network-play-on = { $listener }で再生
network-stop-on = { $listener }での再生を停止
network-choose-format = { $listener }で再生するフォーマットを選択
network-failed = 接続できませんでした：{ $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = ここで再生するには、triibをアップデートしてください。
network-choose = リスナーを選ぶと、ストリームの受け取り状況が表示されます。
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = ここで聴く
network-no-talkers = ネットワーク上に、plaaで再生できるフォーマットで送信しているトーカーがありません。
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker }を聴く
network-listen-stop = { $talker }を聴くのを停止
network-listen-frames = { $frames }フレームを受信
network-listen-update-triib = ここで聴くには、triibをアップデートしてください：plaaは、もともとこのコンピュータに届くものだけを再生します。
network-listen-lite = このポートは、ストリームを予約しないAVB Liteで動作しています：plaaは、もともとこのコンピュータに届くものだけを再生します。

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = 状態
network-not-connected = 未接続
network-from = { $stream }から
network-from-receiving = { $stream }から受信中
network-from-waiting = { $stream }から、トーカーを待機中
network-from-failed = { $stream }から、トーカーの予約に失敗しました：{ $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = メディアクロック
network-locked = ストリームにロック済み
network-not-locked = 未ロック
network-format = フォーマット
# The formats both the listener and plaa's stream can be in.
network-formats = plaaから受け取れるフォーマット
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ストリームID
# The MAC address the stream is sent to.
network-destination = 宛先
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = レイテンシ
network-microseconds = { $time } µs
network-frames = 受信したフレーム
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = 順序が乱れたフレーム
# How often the stream stopped coming while it was taken.
network-interrupted = 中断
network-entity-id = エンティティID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = 帯域幅の不足
msrp-failure-2 = ブリッジのリソース不足
msrp-failure-3 = トラフィッククラスの帯域幅の不足
msrp-failure-4 = ストリームIDがほかのトーカーで使用中
msrp-failure-5 = 宛先アドレスがすでに使用中
msrp-failure-6 = より上位のストリームが優先された
msrp-failure-7 = 報告されたレイテンシが変化した
msrp-failure-8 = 出力ポートがAVBに非対応
msrp-failure-9 = 別の宛先アドレスが必要
msrp-failure-10 = MSRPのリソース不足
msrp-failure-11 = MMRPのリソース不足
msrp-failure-12 = 宛先アドレスを保存できない
msrp-failure-13 = 優先度がSRクラスの優先度ではない
msrp-failure-14 = フレームが媒体に対して大きすぎる
msrp-failure-15 = ファンインポートの上限に到達
msrp-failure-16 = 登録済みストリームの最初の値が変化した
msrp-failure-17 = 出力ポートでVLANがブロックされている
msrp-failure-18 = 出力ポートでVLANタグ付けが無効
msrp-failure-19 = SRクラスの優先度の不一致
msrp-failure-unknown = 不明な理由
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }（ブリッジ{ $bridge }）

## The macOS menu bar

menu-about = plaaについて
menu-settings = 設定…
menu-services = サービス
menu-hide = plaaを非表示
menu-hide-others = ほかを非表示
menu-show-all = すべてを表示
menu-quit = plaaを終了
menu-file = ファイル
menu-open = 開く…
menu-open-address = アドレスを開く…
menu-close = ウインドウを閉じる
menu-edit = 編集
menu-cut = カット
menu-copy = コピー
menu-paste = ペースト
menu-select-all = すべてを選択
menu-undo = 取り消す
menu-redo = やり直す
menu-trim = トリム…
menu-split = クリップを分割
menu-rotate-left = 左に回転
menu-rotate-right = 右に回転
menu-flip-horizontal = 左右反転
menu-flip-vertical = 上下反転
menu-revert = オリジナルに戻す
menu-export = 書き出す…
menu-view = 表示
menu-playback = 再生
menu-play-pause = 再生／一時停止
menu-to-start = 先頭に移動
menu-previous-chapter = 前のチャプター
menu-next-chapter = 次のチャプター
menu-faster = 速く
menu-slower = 遅く
menu-normal-speed = 標準の速度
menu-ab-loop = 2点間をループ
menu-volume-up = 音量を上げる
menu-volume-down = 音量を下げる
menu-mute = ミュート／ミュート解除
menu-next-subtitles = 次の字幕
menu-next-audio = 次のオーディオトラック
menu-window = ウインドウ
menu-minimize = しまう
menu-zoom = 拡大／縮小
menu-bring-all-to-front = すべてを手前に移動

## Editing

action-edit = 編集
title-edited = { $name }（編集済み）
edit-trim = トリム
edit-trim-apply = トリム
edit-split = クリップを分割
edit-remove-clip = クリップを削除
edit-done = 完了
edit-rotate-left = 左に回転
edit-rotate-right = 右に回転
edit-flip-horizontal = 左右反転
edit-flip-vertical = 上下反転
edit-undo = 取り消す
edit-redo = やり直す
edit-revert = オリジナルに戻す
edit-export = 書き出す…

## Exporting

export-title = 書き出す
export-format = フォーマット
export-size = サイズ
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = サウンドのみ
export-size-original = { $lines }p（元のまま）
export-on-graphics-card = グラフィックカードでエンコード（{ $api }）
export-on-processor = プロセッサでエンコード（時間がかかります）
export-no-encoder = このフォーマットのエンコーダをここで開けません
export-checking = エンコーダを探しています…
export-sound-format = サウンドフォーマット
export-quality = 音質
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = 圧縮。Appleのデバイスを含め、ほぼどこでも再生できます
export-mp3-note = 圧縮。どこでも再生できます
export-opus-note = 圧縮。サイズの割に最高の音質ですが、再生できないプレーヤーもあります
export-flac-note = ロスレス。元のサウンドをそのまま、オープンなフォーマットで保存します
export-alac-note = ロスレス。元のサウンドをそのまま、Appleのデバイスで再生できる形式で保存します
export-wav-note = 非圧縮。元のサウンドをそのまま保存しますが、ファイルは最も大きくなります
export-kilobits = { $rate } kb/s
export-bits = { $bits }ビット
export-rate-lower = { $rate } kHzで保存されます（ファイルは{ $file } kHz）
export-choose = 書き出す…
export-name = { $name }（書き出し）
export-name-edited = { $name }（編集済み）
export-progress = 「{ $name }」を書き出しています
export-stop = 書き出しを停止
notice-exported = 「{ $name }」を書き出しました
notice-export-stopped = 書き出しを停止しました
notice-export-failed = 書き出せませんでした：{ $error }
close-title = 書き出さずに閉じますか？
close-unexported = 「{ $name }」への変更は、書き出さないと失われます。
close-exporting = 「{ $name }」の書き出しが停止し、それまでに書き出した内容は失われます。
close-anyway = 閉じる
close-quit-anyway = 終了

## Recording

record-title = 録音
record-file-name = 録音
record-start = 収録
record-stop = 収録を停止
record-no-input = マイクがありません
record-microphone = マイク
record-default-microphone = システムのデフォルト
record-playing = コンピュータで再生中のサウンド
record-mono = モノラル
record-stereo = ステレオ
record-channels = { $count }チャンネル
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }、{ $saved } kHzで保存
action-record-sound = 録音
open-record-sound = 新規録音
start-record-sound = 録音
menu-record-sound = 新規録音
record-screen-title = 画面収録
record-screen-file-name = 画面収録
record-screen-whole = 画面
record-screen-number = 画面{ $number }
record-screen-main = 画面{ $number }（メイン）
record-screens = 画面
record-windows = ウインドウ
record-screen-what = 収録する対象
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = すべてのフレーム
record-rate-tip = 1秒あたりのフレーム数
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count }フレーム欠落
record-screen-window = ウインドウ
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = 共有が終了しました
record-screen-again = 選び直す
record-screen-failed = 画面を収録できませんでした
record-sound = サウンド
record-no-sound = サウンドなし
# How much louder the input is taken, for a quiet microphone.
record-gain = 増幅
record-gain-value = +{ $decibels } dB
record-level-automatically = レベルを自動調整
record-hide-title = 収録中はplaaが隠れます
record-hide-tray = 収録を停止するには、plaaがトレイに表示する赤い点をクリックするか、plaaをもう一度開いてください。
record-hide-reopen = 収録を停止するには、plaaをもう一度開いてください。
record-hide-start = 収録を開始
record-tray-title = plaaが画面を収録しています
record-tray-tip = クリックすると収録を停止します
action-record-screen = 画面を収録
open-record-screen = 新規画面収録
start-record-screen = 画面を収録
menu-record-screen = 新規画面収録
record-camera-title = カメラ録画
record-camera-file-name = カメラ録画
record-camera = カメラ
record-camera-failed = カメラを開けませんでした
action-record-camera = カメラで録画
open-record-camera = 新規カメラ録画
start-record-camera = カメラで録画
menu-record-camera = 新規カメラ録画
notice-no-camera = 録画できるカメラがありません
notice-recorded = 「{ $name }」を{ $folder }に保存しました
notice-record-failed = 収録できませんでした：{ $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = 外部からの操作を許可
settings-allow-outside-control-note = Claude CodeなどのAIエージェントが、plaa --mcpを通じて、plaaで再生中の内容を確認し、操作できるようになります。新しいエージェントごとにplaaが確認します。
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = 許可済み：{ $agents }
settings-forget-agents = 削除
settings-ask-before-note = エージェントが次の操作を行う前に確認：
settings-ask-reading = 再生中の内容や映像の確認
settings-ask-playing = 再生、サウンド、フルスクリーンの操作
settings-ask-opening = ファイルを開く操作やウインドウを閉じる操作
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent }にplaaの操作を許可しますか？
agent-prompt-body = { $agent }が、再生中の内容を確認して操作するために、plaaの外部からの操作を使おうとしています。外部からの操作は「設定」でオフにできます。
agent-prompt-allow = 許可
agent-prompt-deny = 許可しない
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent }に、ここで再生中の内容の確認を許可しますか？
agent-ask-play = { $agent }に、ここでの再生の操作を許可しますか？
agent-ask-open = { $agent }に、ウインドウを開いたり閉じたりすることを許可しますか？
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent }が「{ $tool }」を使おうとしています。plaaが確認する操作は「設定」で選べます。

## Casting to a TV or speaker on the network

cast-looking = テレビとスピーカーを探しています…
cast-none = DLNAで再生できるテレビやスピーカーが見つかりませんでした。
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer }での再生を停止
notice-casting = { $renderer }で再生しています。
notice-cast-failed = { $renderer }で再生できませんでした：{ $reason }
cast-web-page = Webページはまだキャストできません：代わりに、動画そのもののアドレスを開いてください。
