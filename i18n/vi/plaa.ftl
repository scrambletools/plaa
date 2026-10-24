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
language-name = Tiếng Việt

## Common

app-name = plaa
common-cancel = Hủy
common-close = Đóng
common-more = Thêm

## Command line

usage-help =
    Cách dùng: plaa [FILE OR ADDRESS]...

    Phát tệp video, tệp âm thanh và luồng tại các địa chỉ như
    https://example.com/live.m3u8. Mỗi mục mở trong một cửa sổ của plaa
    đang chạy, và plaa sẽ khởi động nếu cần.

    Tùy chọn:
      -h, --help        Hiển thị trợ giúp này
      -V, --version     Hiển thị phiên bản
          --interfaces  Hiển thị các giao diện mạng có dây, và plaa có thể
                        gửi luồng AVB trên đó hay không
          --mcp         Cung cấp MCP qua stdin và stdout, để các tác nhân AI
                        xem plaa đang chạy phát gì và điều khiển nó

## Start window

start-hint = Mở tệp video hoặc âm thanh, luồng hoặc video trên web.
start-open = Mở
start-open-address = Mở địa chỉ
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Kết nối

## Actions, as buttons, tooltips and menus name them

action-open = Mở
action-open-address = Mở địa chỉ
open-file = Mở tệp…
open-address = Mở địa chỉ…
action-settings = Cài đặt
action-info = Bộ kiểm tra
action-loop = Lặp lại
action-screenshot = Lưu khung hình thành ảnh
action-play = Phát
action-pause = Tạm dừng
# $seconds is how far the button moves, such as 10.
action-back = Lùi { $seconds } giây
action-forward = Tới { $seconds } giây
action-mute = Tắt tiếng
action-unmute = Bật tiếng
action-speed = Tốc độ
action-tracks = Âm thanh và phụ đề
action-full-screen = Toàn màn hình
action-playlist = Danh sách phát
action-chapters = Chương
action-picture-in-picture = Hình trong hình
action-previous = Trước
action-next = Tiếp theo
action-exit-full-screen = Thoát chế độ toàn màn hình
action-quality = Chất lượng
action-visualizer = Hiệu ứng hình ảnh
action-network = Âm thanh qua mạng
action-cast = Truyền
action-connect = Kết nối
action-assistant = Trợ lý

## What a sound file shows under its cover

visualizer-spectrum = Phổ tần
visualizer-waveform = Dạng sóng
visualizer-prism = Lăng kính
visualizer-cover = Chỉ ảnh bìa

## The floating controls

overlay-keep-shown = Giữ cố định các nút điều khiển
overlay-auto-hide = Cho các nút điều khiển nổi trên hình

## Audio and subtitle tracks

tracks-audio = Âm thanh
tracks-subtitles = Phụ đề
tracks-off = Tắt
# A track the file gives no name or language, by its number.
tracks-numbered = Rãnh { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Chỉ âm thanh

## Inspector

info-title = Bộ kiểm tra
playlist-title = Danh sách phát
chapters-numbered = Chương { $number }
info-file = Tệp
info-name = Tên
info-address = Địa chỉ
info-folder = Thư mục
info-size = Kích thước
info-format = Định dạng
info-duration = Thời lượng
info-bit-rate = Tốc độ bit
info-video = Video
info-audio = Âm thanh
info-codec = Codec
info-dimensions = Kích thước khung hình
info-frame-rate = Tốc độ khung hình
info-frames-per-second = { $rate } khung hình/giây
info-pixel-format = Định dạng pixel
info-decoding = Giải mã
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Card đồ họa ({ $api })
info-decoding-software = Bộ xử lý
info-sample-rate = Tần số lấy mẫu
info-hertz = { $rate } Hz
info-channels = Kênh
info-language = Ngôn ngữ
info-no-sound = Không có âm thanh
info-tags = Thẻ
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Luồng
# The talker's listeners other than plaa.
info-other-listeners = Listener khác
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Lớp SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Định dạng mới sẽ ngắt { $count ->
       *[other] { $count } Listener khác của Talker.
    }

## Dialogs

dialog-open = Mở
dialog-filter-media = Video và âm thanh
dialog-filter-all = Tất cả các tệp

address-title = Mở địa chỉ
address-note = Một trang web có video, chẳng hạn trang của một video YouTube, hoặc một luồng hay tệp trên mạng, chẳng hạn https://example.com/live.m3u8.
address-label = Địa chỉ
address-open = Mở
address-recent = Gần đây
address-forget = Xóa khỏi danh sách gần đây

## Notices

notice-dialog-failed = Không thể mở hộp thoại tệp: { $error }
notice-screenshot-saved = Đã lưu “{ $name }”.
notice-screenshot-failed = Không thể lưu ảnh: { $error }
notice-no-sound = Đang phát không có âm thanh: { $reason }
notice-loop-from = Lặp từ { $time }. Nhấn lần nữa tại điểm cần kết thúc.
notice-loop-between = Đang lặp từ { $from } đến { $to }.
notice-loop-off = Đã tắt lặp.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Đây có vẻ là một trang web. plaa phát video trên trang web bằng yt-dlp, nhưng yt-dlp chưa được cài đặt.
error-empty-list = Danh sách này không có gì để phát.

## Settings

settings-title = Cài đặt
settings-tab-appearance = Giao diện
settings-tab-playback = Phát lại
settings-tab-storage = Lưu trữ
settings-tab-network = Mạng
settings-tab-language = Ngôn ngữ
settings-tab-agents = Tác nhân
settings-tab-assistant = Trợ lý
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (bản phát triển, { $build })
settings-choose = Chọn…

settings-appearance-system = Theo hệ thống
settings-appearance-light = Sáng
settings-appearance-dark = Tối
settings-colors = Màu sắc
settings-system-accent = Dùng màu nhấn của hệ thống
settings-omarchy-note = Màu sắc được tạo từ màu nhấn của “{ $theme }”.
settings-system-accent-note = Màu sắc được tạo từ màu nhấn của hệ thống.
settings-system-accent-none = Hệ thống không có màu nhấn nên plaa dùng màu được chọn bên dưới.
settings-accent-chosen-note = Màu sắc được tạo từ màu được chọn bên dưới.
settings-controls = Nút điều khiển
settings-overlay = Nút điều khiển nổi
settings-overlay-note = Thanh công cụ và các nút điều khiển phát nổi phía trên hình hoặc ảnh bìa, và trượt đi khi con trỏ đứng yên hoặc ở ngoài cửa sổ.
settings-overlay-delay = Ẩn sau
settings-overlay-delay-note = Thời gian con trỏ đứng yên trên nội dung đang phát trước khi các nút điều khiển trượt đi.
settings-seconds = { $seconds } giây
settings-transparency = Độ trong suốt
settings-transparency-note = Mức độ hình hiện xuyên qua các nút điều khiển nổi.
settings-percent = { $percent }%
settings-corner-radius = Bán kính góc
settings-corner-radius-note = Cho hộp thoại và các nút điều khiển nổi.
settings-pixels = { $pixels } px
settings-animations = Hiệu ứng động
settings-animations-note = Thanh và bảng trượt, hộp thoại phóng to và nút bấm nảy.
settings-animations-reduced = Tắt khi hệ thống yêu cầu giảm chuyển động.

settings-hardware = Dùng card đồ họa
settings-hardware-note = Giải mã video và mã hóa tệp xuất bằng card đồ họa: tốn ít điện hơn, phát video lớn mượt mà và xuất nhanh hơn. Áp dụng cho các tệp được mở sau đó.
settings-audio-device = Đầu ra âm thanh
settings-audio-device-system = Mặc định của hệ thống
settings-resume = Tiếp tục từ chỗ đã dừng
settings-resume-note = Tệp bắt đầu từ vị trí lúc được đóng lần gần nhất.
settings-folder-playlists = Phát các tệp khác trong thư mục
settings-folder-playlists-note = Khi mở một tệp, các tệp khác trong thư mục của tệp đó được đưa vào danh sách và phát tiếp sau nó.
settings-streams = Luồng và video trên web
settings-stream-quality = Chất lượng
settings-stream-quality-note = Cho các luồng và video trên web có nhiều mức chất lượng. Áp dụng cho nội dung được mở sau đó.
settings-quality-auto = Tự động, cao bằng màn hình
settings-quality-best = Tốt nhất
settings-web-pages = Phát video trên web bằng yt-dlp
settings-web-pages-found = yt-dlp tìm video trên các trang như YouTube.
settings-web-pages-missing = yt-dlp chưa được cài đặt. Hãy cài đặt yt-dlp để phát video trên các trang như YouTube.
settings-default-app = Ứng dụng mặc định
settings-default-app-label = Mở tệp bằng plaa
settings-default-app-note = Đặt plaa làm ứng dụng mở tệp video và âm thanh.
settings-default-app-note-windows = Windows chỉ cho phép chọn ứng dụng mặc định trong phần Cài đặt của chính nó. Nút này sẽ mở trang của plaa ở đó.
settings-default-app-note-macos = macOS yêu cầu bạn xác nhận từng loại: MP4, MOV, MKV, WebM, MP3, M4A, FLAC và WAV.
settings-default-app-status = { $set } trên { $total } loại tệp được mở bằng plaa.
settings-default-app-button = Đặt làm mặc định
settings-default-app-button-windows = Mở Cài đặt
settings-default-app-no-entry = Tệp .desktop của plaa chưa được cài đặt nên hệ thống không thể mở tệp bằng plaa. Hãy cài đặt plaa từ một gói.
settings-default-app-no-bundle = Mở plaa từ plaa.app để đặt làm mặc định.
settings-default-app-failed = Không thể đặt plaa làm mặc định: { $error }
settings-folders = Thư mục
settings-sound-recordings = Bản ghi âm
settings-video-recordings = Bản ghi video
settings-screenshots = Khung hình đã lưu

settings-interface-language = Ngôn ngữ giao diện
# The first entry of the language list.
settings-language-system = Theo hệ thống
settings-track-languages = Ngôn ngữ của rãnh
settings-track-languages-note = Mã ngôn ngữ như en hoặc fra, ưu tiên cao nhất đứng trước, phân tách bằng dấu phẩy. plaa chọn âm thanh và phụ đề bằng các ngôn ngữ này khi tệp có.
settings-audio-languages = Âm thanh
settings-subtitle-languages = Phụ đề

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Giao diện AVB
settings-avb-interface-note = Cổng có dây mà plaa dùng để tìm Talker và Listener AVB và Milan. Để gửi, cần một cổng có đồng hồ phần cứng PTP, chẳng hạn cổng của Intel I210.
settings-avb-off = Tắt
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (chỉ liệt kê: không có đồng hồ PTP)
settings-avb-off-mac = { $interface } (chỉ liệt kê: AVB đang tắt)
settings-avb-interface-note-mac = Cổng có dây mà plaa dùng để tìm Talker và Listener AVB và Milan. Việc gửi đi qua thực thể AVB riêng của máy Mac này, và cần bật AVB cho cổng đó trong Trình duyệt thiết bị mạng của Thiết lập Audio MIDI.
settings-avb-talker-note-mac = plaa gửi âm thanh của cửa sổ phát gần nhất qua thực thể AVB riêng của máy Mac này, tới các Listener đã kết nối với nó.
settings-avb-talker = Gửi nội dung đang phát
settings-avb-talker-note = plaa trở thành một Talker Milan, gửi âm thanh của cửa sổ phát gần nhất tới các Listener đã kết nối với nó.
settings-avb-name = Tên trên mạng
settings-avb-name-note = Các bộ điều khiển hiển thị plaa là “{ $name }”. Nhấn Enter để đổi tên.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa trên { $computer }
avb-could-not-start = Không thể dùng { $interface }: { $error }
network-title = Âm thanh qua mạng
network-off = Hãy bật “Gửi nội dung đang phát” trong Cài đặt để phát qua mạng.
network-blocked = Không gửi: { $reason }
network-waiting = Sẵn sàng gửi, đang chờ một Listener kết nối.
network-sending = Đang gửi tới các Listener đã kết nối.
network-looking = Đang tìm Listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Đang khởi động ptp4l, chương trình giữ thời gian mạng mà luồng của plaa dựa vào để gửi…
network-ptp-elsewhere = ptp4l đang chạy trên một giao diện khác, nên luồng của plaa không có thời gian mạng ở đây và các Listener có thể không phát được. Hãy chọn giao diện đó trong Cài đặt, hoặc chạy ptp4l trên giao diện này.
network-ptp-missing = ptp4l không chạy, nên luồng của plaa không có thời gian mạng và các Listener có thể không phát được. Hãy cài đặt linuxptp cùng gói của plaa, hoặc cài triib, ứng dụng sẽ chạy ptp4l.
network-ptp-failed = ptp4l không chạy, nên luồng của plaa không có thời gian mạng và các Listener có thể không phát được. plaa không thể khởi động ptp4l: { $reason }
network-no-listeners = Không có Listener nào trên mạng có thể phát âm thanh của plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Phát trên { $listener }
network-stop-on = Dừng phát trên { $listener }
network-choose-format = Chọn định dạng để phát trên { $listener }
network-failed = Không thể kết nối: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Hãy cập nhật triib để phát ở đây.
network-choose = Chọn một Listener để xem cách nó nhận luồng.
network-no-sources = Không có Talker nào trên mạng gửi ở định dạng mà plaa phát, hoặc có thể chuyển sang định dạng đó.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Dừng nghe { $talker }
network-listen-update-triib = Hãy cập nhật triib để nghe ở đây: plaa chỉ phát những gì vốn đã đến được máy tính này.
network-listen-lite = Cổng đang chạy AVB Lite, không đặt trước luồng nào: plaa chỉ phát những gì vốn đã đến được máy tính này.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Đang tìm Talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = Hãy bật “Gửi nội dung đang phát” trong Cài đặt để nghe các Talker tại đây.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa không thể nghe tại đây: { $reason }
connect-sending = Đang gửi ở định dạng mà plaa phát
connect-changing = Chuyển sang định dạng mà plaa phát
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, ở { $format }
connect-change-cuts-off = { $talker }, ở { $format }, ngắt { $count ->
       *[other] { $count } Listener khác của nó
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Trạng thái
network-not-connected = Chưa kết nối
network-from = Từ { $stream }
network-from-receiving = Từ { $stream }, đang nhận
network-from-waiting = Từ { $stream }, đang chờ Talker
network-from-failed = Từ { $stream }, đặt trước của Talker thất bại: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Đồng hồ media
network-locked = Đã khóa theo luồng
network-not-locked = Chưa khóa
network-format = Định dạng
# The formats both the listener and plaa's stream can be in.
network-formats = Các định dạng có thể nhận từ plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID luồng
# The MAC address the stream is sent to.
network-destination = Đích
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Độ trễ
network-microseconds = { $time } µs
network-frames = Khung đã nhận
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Khung sai thứ tự
# How often the stream stopped coming while it was taken.
network-interrupted = Số lần gián đoạn
# Frames that came after, or long before, their time to be played.
network-late = Khung đến trễ
network-early = Khung đến sớm
network-entity-id = ID thực thể

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = không đủ băng thông
msrp-failure-2 = không đủ tài nguyên switch
msrp-failure-3 = không đủ băng thông cho lớp lưu lượng
msrp-failure-4 = ID luồng đang được một Talker khác dùng
msrp-failure-5 = địa chỉ đích đã được dùng
msrp-failure-6 = bị một luồng có thứ hạng cao hơn giành quyền
msrp-failure-7 = độ trễ được báo cáo đã thay đổi
msrp-failure-8 = cổng ra không hỗ trợ AVB
msrp-failure-9 = hãy dùng địa chỉ đích khác
msrp-failure-10 = hết tài nguyên MSRP
msrp-failure-11 = hết tài nguyên MMRP
msrp-failure-12 = không thể lưu địa chỉ đích
msrp-failure-13 = mức ưu tiên không phải mức ưu tiên của lớp SR
msrp-failure-14 = khung quá lớn so với môi trường truyền
msrp-failure-15 = đã đạt giới hạn cổng fan-in
msrp-failure-16 = giá trị đầu tiên của một luồng đã đăng ký bị thay đổi
msrp-failure-17 = VLAN bị chặn trên cổng ra
msrp-failure-18 = gắn thẻ VLAN bị tắt trên cổng ra
msrp-failure-19 = mức ưu tiên lớp SR không khớp
msrp-failure-unknown = lý do không xác định
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, tại switch { $bridge }

## The macOS menu bar

menu-about = Giới thiệu về plaa
menu-settings = Cài đặt…
menu-services = Dịch vụ
menu-hide = Ẩn plaa
menu-hide-others = Ẩn các ứng dụng khác
menu-show-all = Hiển thị tất cả
menu-quit = Thoát plaa
menu-file = Tệp
menu-open = Mở…
menu-open-address = Mở địa chỉ…
menu-close = Đóng cửa sổ
menu-edit = Sửa
menu-cut = Cắt
menu-copy = Sao chép
menu-paste = Dán
menu-select-all = Chọn tất cả
menu-undo = Hoàn tác
menu-redo = Làm lại
menu-trim = Cắt bớt…
menu-split = Tách clip
menu-rotate-left = Xoay trái
menu-rotate-right = Xoay phải
menu-flip-horizontal = Lật ngang
menu-flip-vertical = Lật dọc
menu-revert = Khôi phục bản gốc
menu-export = Xuất…
menu-view = Xem
menu-playback = Phát lại
menu-play-pause = Phát hoặc tạm dừng
menu-to-start = Về đầu
menu-previous-chapter = Chương trước
menu-next-chapter = Chương tiếp theo
menu-faster = Nhanh hơn
menu-slower = Chậm hơn
menu-normal-speed = Tốc độ bình thường
menu-ab-loop = Lặp giữa hai điểm
menu-volume-up = To hơn
menu-volume-down = Nhỏ hơn
menu-mute = Tắt hoặc bật tiếng
menu-next-subtitles = Phụ đề tiếp theo
menu-next-audio = Rãnh âm thanh tiếp theo
menu-window = Cửa sổ
menu-minimize = Thu nhỏ cửa sổ
menu-zoom = Thu phóng
menu-bring-all-to-front = Đưa tất cả ra trước

## Editing

action-edit = Sửa
title-edited = { $name } (đã sửa)
edit-trim = Cắt bớt
edit-trim-apply = Cắt bớt
edit-split = Tách clip
edit-remove-clip = Xóa clip
edit-done = Xong
edit-rotate-left = Xoay trái
edit-rotate-right = Xoay phải
edit-flip-horizontal = Lật ngang
edit-flip-vertical = Lật dọc
edit-undo = Hoàn tác
edit-redo = Làm lại
edit-revert = Khôi phục bản gốc
edit-export = Xuất…

## Exporting

export-title = Xuất
export-format = Định dạng
export-size = Kích thước
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Chỉ âm thanh
export-size-original = { $lines }p, giữ nguyên
export-on-graphics-card = Mã hóa trên card đồ họa ({ $api })
export-on-processor = Mã hóa trên bộ xử lý, mất nhiều thời gian hơn
export-no-encoder = Không mở được bộ mã hóa nào cho định dạng này
export-checking = Đang tìm bộ mã hóa…
export-sound-format = Định dạng âm thanh
export-quality = Chất lượng
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Có nén; phát được hầu như ở mọi nơi, kể cả trên thiết bị của Apple
export-mp3-note = Có nén; phát được ở mọi nơi
export-opus-note = Có nén; âm thanh tốt nhất so với dung lượng, dù không phải trình phát nào cũng hỗ trợ
export-flac-note = Không mất dữ liệu: âm thanh đúng như bản gốc, ở định dạng mở
export-alac-note = Không mất dữ liệu: âm thanh đúng như bản gốc, ở định dạng thiết bị của Apple phát được
export-wav-note = Không nén: âm thanh đúng như bản gốc, với tệp lớn nhất
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Lưu ở { $rate } kHz, thấp hơn mức { $file } kHz của tệp
export-choose = Xuất…
export-name = { $name } bản xuất
export-name-edited = { $name } đã sửa
export-progress = Đang xuất “{ $name }”
export-stop = Dừng xuất
notice-exported = Đã xuất “{ $name }”
notice-export-stopped = Đã dừng xuất
notice-export-failed = Không thể xuất: { $error }
close-title = Đóng mà không xuất?
close-unexported = Các thay đổi của “{ $name }” sẽ mất nếu bạn không xuất chúng.
close-exporting = Việc xuất “{ $name }” sẽ dừng, và phần đã ghi đến lúc đó sẽ mất.
close-anyway = Đóng
close-quit-anyway = Thoát

## Recording

record-title = Ghi âm
record-file-name = Bản ghi âm
record-start = Ghi
record-stop = Dừng ghi
record-no-input = Không có micrô
record-microphone = Micrô
record-default-microphone = Mặc định của hệ thống
record-playing = Âm thanh máy tính đang phát
record-mono = mono
record-stereo = stereo
record-channels = { $count } kênh
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, lưu ở { $saved } kHz
action-record-sound = Ghi âm
open-record-sound = Bản ghi âm mới
start-record-sound = Ghi âm
menu-record-sound = Bản ghi âm mới
record-screen-title = Ghi màn hình
record-screen-file-name = Bản ghi màn hình
record-screen-whole = Màn hình
record-screen-number = Màn hình { $number }
record-screen-main = Màn hình { $number } (chính)
record-screens = Màn hình
record-windows = Cửa sổ
record-screen-what = Nội dung cần ghi
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Mọi khung hình
record-rate-tip = Số khung hình mỗi giây
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } khung bị bỏ
record-screen-window = Cửa sổ
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Không còn được chia sẻ
record-screen-again = Chọn lại
record-screen-failed = Không thể ghi màn hình
record-sound = Âm thanh
record-no-sound = Không có âm thanh
# How much louder the input is taken, for a quiet microphone.
record-gain = Tăng âm
record-gain-value = +{ $decibels } dB
record-level-automatically = Tự động cân chỉnh mức âm
record-hide-title = plaa ẩn đi trong khi ghi
record-hide-tray = Để dừng ghi, hãy nhấp vào chấm đỏ mà plaa đặt trong khay hệ thống, hoặc mở lại plaa.
record-hide-reopen = Để dừng ghi, hãy mở lại plaa.
record-hide-start = Bắt đầu ghi
record-tray-title = plaa đang ghi màn hình
record-tray-tip = Nhấp để dừng ghi
action-record-screen = Ghi màn hình
open-record-screen = Bản ghi màn hình mới
start-record-screen = Ghi màn hình
menu-record-screen = Bản ghi màn hình mới
record-camera-title = Ghi hình camera
record-camera-file-name = Bản ghi camera
record-camera = Camera
record-camera-failed = Không thể mở camera
record-name-title = Đặt tên cho bản ghi
record-name-note = Đã lưu với tên “{ $name }” trong { $folder }.
record-name-label = Tên
record-name-keep = Giữ nguyên
record-name-rename = Đổi tên
record-name-taken = Đã có một tệp tên “{ $name }” ở đó.
record-name-invalid = Tên không được chứa bất kỳ ký tự nào sau: / \ : * ? " < > |
record-name-failed = Không thể đổi tên: { $error }
action-record-camera = Ghi hình camera
open-record-camera = Bản ghi camera mới
start-record-camera = Ghi hình camera
menu-record-camera = Bản ghi camera mới
notice-no-camera = Không có camera nào để ghi
notice-recorded = Đã lưu “{ $name }” trong { $folder }
notice-record-failed = Không thể ghi: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Cho phép điều khiển từ bên ngoài
settings-allow-outside-control-note = Các tác nhân AI như Claude Code có thể xem nội dung đang phát trong plaa và điều khiển nó, thông qua plaa --mcp. plaa sẽ hỏi trước mỗi tác nhân mới.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Đã cho phép: { $agents }
settings-forget-agents = Quên
settings-ask-before-note = Hỏi trước khi tác nhân:
settings-ask-reading = Xem nội dung đang phát hoặc hình ảnh
settings-ask-playing = Điều khiển việc phát, âm thanh hoặc chế độ toàn màn hình
settings-ask-opening = Mở tệp hoặc đóng cửa sổ
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Cho phép { $agent } điều khiển plaa?
agent-prompt-body = { $agent } yêu cầu dùng tính năng điều khiển từ bên ngoài của plaa để xem nội dung đang phát và điều khiển nó. Bạn có thể tắt điều khiển từ bên ngoài trong Cài đặt.
agent-prompt-allow = Cho phép
agent-prompt-deny = Không cho phép
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Cho phép { $agent } xem nội dung đang phát ở đây?
agent-ask-play = Cho phép { $agent } điều khiển việc phát ở đây?
agent-ask-open = Cho phép { $agent } mở hoặc đóng cửa sổ?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } yêu cầu dùng “{ $tool }”. Bạn có thể chọn những gì plaa sẽ hỏi trong Cài đặt.

## Casting to a TV or speaker on the network

cast-looking = Đang tìm TV và loa…
cast-none = Không tìm thấy TV, loa hay Chromecast nào.
# $renderer is the TV's or speaker's name.
cast-stop = Dừng phát trên { $renderer }
notice-casting = Đang phát trên { $renderer }.
notice-cast-failed = Không thể phát trên { $renderer }: { $reason }
cast-web-page = Chưa thể truyền trang web: hãy mở địa chỉ riêng của video.

## The assistant panel, and its models in Settings

assistant-add-another = Thêm mô hình…
assistant-add-model = Trợ lý cần một mô hình: mô hình đám mây kèm khóa API của nó, hoặc mô hình cục bộ.
assistant-ask = Hỏi về nội dung đang phát
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Cuộc trò chuyện đã quá dài để { $model } đọc một lần, nên phần trước đã được tóm tắt. Trợ lý có thể xem lại những gì đã thấy.
assistant-compacting = Đang tóm tắt phần trò chuyện trước
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } trên máy tính này
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } tại { $host }
assistant-new-chat = Cuộc trò chuyện mới
assistant-no-model = Hãy thêm một mô hình trong Cài đặt trước.
assistant-open-settings = Thêm mô hình
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Cuộc trò chuyện đã vượt quá lượng nội dung mà { $model } có thể đọc cùng lúc. Hãy bắt đầu cuộc trò chuyện mới, hoặc chọn mô hình có thể đọc nhiều hơn.
assistant-problem-credit = { $provider } cho biết tài khoản không còn số dư. Tài khoản mới cần nạp tiền trên trang web của { $provider } thì khóa mới hoạt động; sau đó hãy thử lại.
assistant-problem-key = { $provider } đã từ chối khóa API. Hãy kiểm tra trong Cài đặt.
assistant-problem-model = { $provider } không có mô hình nào tên { $model }. Hãy kiểm tra tên mô hình trong Cài đặt.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } không trả lời: { $message }
assistant-problem-rate = { $provider } yêu cầu giảm tốc độ. Hãy thử lại sau giây lát.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } yêu cầu giảm tốc độ: { $message }
assistant-problem-refused = { $model } đã từ chối trả lời.
assistant-problem-unavailable = { $provider } đang bận hoặc gặp sự cố. Hãy thử lại sau giây lát.
assistant-problem-unreachable = plaa không thể kết nối tới { $provider }. Hãy kiểm tra kết nối mạng, hoặc xem máy chủ có đang chạy không.
assistant-running = Đang chạy…
assistant-send = Gửi
assistant-shortened = Cuộc trò chuyện quá dài để { $model } đọc một lần, nên các kết quả công cụ trước đó đã được bỏ bớt. Trợ lý có thể xem lại những gì đã thấy.
assistant-stop = Dừng
assistant-stopped = Đã dừng.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Đang trò chuyện với { $model }.
assistant-thinking = Đang suy nghĩ…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Suy nghĩ
assistant-title = Trợ lý
settings-assistant-add = Thêm mô hình
settings-assistant-add-button = Thêm
# $model is the model's name.
settings-assistant-added = { $model } đã trả lời và được thêm.
settings-assistant-added-tag = Đã thêm
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Địa chỉ, ví dụ { $example }
settings-assistant-change-key = Đổi khóa
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Máy chủ tương thích với OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Ngữ cảnh
settings-assistant-context-note = Ngữ cảnh lớn hơn cho phép trợ lý ghi nhớ được nhiều phần hơn của một cuộc trò chuyện, nhưng mô hình sẽ dùng nhiều bộ nhớ hơn và có thể trả lời chậm hơn.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Mở trang
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } đang chạy tại { $address }.
settings-assistant-found-none = { $provider } chưa có mô hình nào. Hãy tải một mô hình bằng { $provider }, rồi tìm lại.
settings-assistant-get-key = Lấy khóa API
settings-assistant-get-server = Tải { $provider }
settings-assistant-in-use = Đang dùng
settings-assistant-key = Khóa API
# $error is what the keychain said.
settings-assistant-key-failed = Không thể lưu khóa vào kho thông tin xác thực: { $error }
settings-assistant-key-kept = Khóa { $provider } của bạn được lưu trong kho thông tin xác thực của hệ thống.
settings-assistant-key-needed = Mô hình này cần khóa API.
settings-assistant-key-refused = { $provider } đã từ chối khóa này. Hãy kiểm tra xem khóa đã được sao chép đầy đủ và từ đúng tài khoản chưa.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Tạo khóa trên trang web của { $provider } rồi dán vào đây.
settings-assistant-look-again = Tìm lại
settings-assistant-looking = Đang tìm mô hình…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Mô hình, ví dụ { $example }
settings-assistant-no-server = plaa không tìm thấy { $provider } nào đang chạy trên máy tính này. Hãy khởi động nó, hoặc nhập địa chỉ của nó bên dưới.
settings-assistant-no-tools = Không dùng được công cụ, điều mà trợ lý cần
settings-assistant-none = Chưa có mô hình nào. Hãy thêm một mô hình bên dưới: mô hình cục bộ, như mô hình của Ollama, chỉ ở trên máy tính này; mô hình đám mây cần khóa API từ nhà cung cấp của nó.
settings-assistant-note = Các mô hình mà bảng trợ lý có thể trò chuyện. Khóa được lưu trong kho thông tin xác thực của hệ thống.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Dùng địa chỉ khác
settings-assistant-recommended = Đề xuất
settings-assistant-remove = Xóa
settings-assistant-sees = Xem được hình ảnh
settings-assistant-trying = Đang thử…
settings-assistant-use = Dùng
settings-assistant-use-key = Tiếp tục
settings-assistant-uses-tools = Dùng công cụ
