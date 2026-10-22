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
language-name = ไทย

## Common

app-name = plaa
common-cancel = ยกเลิก
common-close = ปิด
common-more = เพิ่มเติม

## Command line

usage-help =
    วิธีใช้: plaa [FILE OR ADDRESS]...

    เล่นไฟล์วิดีโอและเสียง รวมถึงสตรีมจากที่อยู่ เช่น
    https://example.com/live.m3u8 แต่ละรายการจะเปิดในหน้าต่างของ plaa
    ที่กำลังทำงานอยู่ ซึ่งจะเริ่มทำงานเองหากจำเป็น

    ตัวเลือก:
      -h, --help        แสดงวิธีใช้นี้
      -V, --version     แสดงเวอร์ชัน
          --interfaces  แสดงอินเทอร์เฟซแบบมีสาย และบอกว่า plaa ส่งสตรีม AVB
                        บนอินเทอร์เฟซเหล่านั้นได้หรือไม่
          --mcp         ให้บริการ MCP ผ่าน stdin และ stdout เพื่อให้เอเจนต์ AI
                        ดูว่า plaa ที่กำลังทำงานอยู่เล่นอะไร และควบคุม plaa ได้

## Start window

start-hint = เปิดไฟล์วิดีโอหรือเสียง สตรีม หรือวิดีโอบนเว็บ
start-open = เปิด
start-open-address = เปิดที่อยู่
# Opens a list of the talkers on the AVB network to listen to.
start-connect = เชื่อมต่อ

## Actions, as buttons, tooltips and menus name them

action-open = เปิด
action-open-address = เปิดที่อยู่
open-file = เปิดไฟล์…
open-address = เปิดที่อยู่…
action-settings = การตั้งค่า
action-info = ตัวตรวจสอบ
action-loop = วนซ้ำ
action-screenshot = บันทึกเฟรมเป็นรูปภาพ
action-play = เล่น
action-pause = หยุดชั่วคราว
# $seconds is how far the button moves, such as 10.
action-back = ย้อนกลับ { $seconds } วินาที
action-forward = ไปข้างหน้า { $seconds } วินาที
action-mute = ปิดเสียง
action-unmute = เปิดเสียง
action-speed = ความเร็ว
action-tracks = เสียงและคำบรรยาย
action-full-screen = เต็มหน้าจอ
action-playlist = เพลย์ลิสต์
action-chapters = บท
action-picture-in-picture = ภาพซ้อนภาพ
action-previous = ก่อนหน้า
action-next = ถัดไป
action-exit-full-screen = ออกจากโหมดเต็มหน้าจอ
action-quality = คุณภาพ
action-visualizer = การแสดงภาพเสียง
action-network = เสียงผ่านเครือข่าย
action-cast = แคสต์
action-connect = เชื่อมต่อ
action-assistant = ผู้ช่วย

## What a sound file shows under its cover

visualizer-spectrum = สเปกตรัม
visualizer-waveform = รูปคลื่น
visualizer-prism = ปริซึม
visualizer-cover = ภาพปกเท่านั้น

## The floating controls

overlay-keep-shown = ตรึงตัวควบคุมไว้กับที่
overlay-auto-hide = ให้ตัวควบคุมลอยเหนือภาพ

## Audio and subtitle tracks

tracks-audio = เสียง
tracks-subtitles = คำบรรยาย
tracks-off = ปิด
# A track the file gives no name or language, by its number.
tracks-numbered = แทร็ก { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = เสียงเท่านั้น

## Inspector

info-title = ตัวตรวจสอบ
playlist-title = เพลย์ลิสต์
chapters-numbered = บทที่ { $number }
info-file = ไฟล์
info-name = ชื่อ
info-address = ที่อยู่
info-folder = โฟลเดอร์
info-size = ขนาด
info-format = รูปแบบ
info-duration = ความยาว
info-bit-rate = บิตเรต
info-video = วิดีโอ
info-audio = เสียง
info-codec = โคเดก
info-dimensions = ขนาดภาพ
info-frame-rate = อัตราเฟรม
info-frames-per-second = { $rate } เฟรมต่อวินาที
info-pixel-format = รูปแบบพิกเซล
info-decoding = การถอดรหัส
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = การ์ดแสดงผล ({ $api })
info-decoding-software = โปรเซสเซอร์
info-sample-rate = อัตราสุ่มตัวอย่าง
info-hertz = { $rate } Hz
info-channels = ช่องสัญญาณ
info-language = ภาษา
info-no-sound = ไม่มีเสียง
info-tags = แท็ก

## Dialogs

dialog-open = เปิด
dialog-filter-media = วิดีโอและเสียง
dialog-filter-all = ไฟล์ทั้งหมด

address-title = เปิดที่อยู่
address-note = หน้าเว็บที่มีวิดีโอ เช่น หน้าวิดีโอ YouTube หรือสตรีมหรือไฟล์บนเครือข่าย เช่น https://example.com/live.m3u8
address-label = ที่อยู่
address-open = เปิด
address-recent = ล่าสุด
address-forget = เอาออกจากรายการล่าสุด

## Notices

notice-dialog-failed = เปิดกล่องโต้ตอบไฟล์ไม่ได้: { $error }
notice-screenshot-saved = บันทึก “{ $name }” แล้ว
notice-screenshot-failed = บันทึกรูปภาพไม่ได้: { $error }
notice-no-sound = กำลังเล่นโดยไม่มีเสียง: { $reason }
notice-loop-from = วนซ้ำจาก { $time } กดอีกครั้งที่จุดที่ต้องการให้สิ้นสุด
notice-loop-between = กำลังวนซ้ำจาก { $from } ถึง { $to }
notice-loop-off = ปิดการวนซ้ำแล้ว

## Errors, shown in place of what could not play

error-needs-yt-dlp = ดูเหมือนว่านี่เป็นหน้าเว็บ plaa เล่นวิดีโอบนหน้าเว็บด้วย yt-dlp ซึ่งยังไม่ได้ติดตั้ง
error-empty-list = รายการนี้ไม่มีอะไรให้เล่น

## Settings

settings-title = การตั้งค่า
settings-tab-appearance = ลักษณะที่ปรากฏ
settings-tab-playback = การเล่น
settings-tab-network = เครือข่าย
settings-tab-language = ภาษา
settings-tab-agents = เอเจนต์
settings-tab-assistant = ผู้ช่วย
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (บิลด์สำหรับการพัฒนา, { $build })
settings-choose = เลือก…

settings-appearance-system = ตามระบบ
settings-appearance-light = สว่าง
settings-appearance-dark = มืด
settings-colors = สี
settings-system-accent = ใช้สีเน้นของระบบ
settings-omarchy-note = สีสร้างจากสีเน้นของ “{ $theme }”
settings-system-accent-note = สีสร้างจากสีเน้นของระบบ
settings-system-accent-none = ระบบไม่มีสีเน้น plaa จึงใช้สีที่เลือกไว้ด้านล่าง
settings-accent-chosen-note = สีสร้างจากสีที่เลือกไว้ด้านล่าง
settings-controls = ตัวควบคุม
settings-overlay = ตัวควบคุมแบบลอย
settings-overlay-note = แถบเครื่องมือและตัวควบคุมการเล่นจะลอยอยู่เหนือภาพหรือภาพปก และเลื่อนหายไปเมื่อตัวชี้หยุดนิ่งหรืออยู่นอกหน้าต่าง
settings-overlay-delay = ซ่อนหลังจาก
settings-overlay-delay-note = ระยะเวลาที่ตัวชี้หยุดนิ่งอยู่เหนือสิ่งที่กำลังเล่น ก่อนที่ตัวควบคุมจะเลื่อนหายไป
settings-seconds = { $seconds } วินาที
settings-transparency = ความโปร่งใส
settings-transparency-note = ภาพจะมองเห็นผ่านตัวควบคุมแบบลอยได้มากเพียงใด
settings-percent = { $percent }%
settings-corner-radius = รัศมีมุม
settings-corner-radius-note = สำหรับกล่องโต้ตอบและตัวควบคุมแบบลอย
settings-pixels = { $pixels } px
settings-animations = ภาพเคลื่อนไหว
settings-animations-note = แถบและแผงที่เลื่อนเข้าออก กล่องโต้ตอบที่ขยายขึ้น และปุ่มที่เด้งได้
settings-animations-reduced = ปิดอยู่ขณะที่ระบบขอให้ลดการเคลื่อนไหว

settings-hardware = ใช้การ์ดแสดงผล
settings-hardware-note = ถอดรหัสวิดีโอและเข้ารหัสงานส่งออกด้วยการ์ดแสดงผล ซึ่งใช้พลังงานน้อยลง เล่นวิดีโอขนาดใหญ่ได้ลื่นไหล และส่งออกได้เร็วขึ้น มีผลกับไฟล์ที่เปิดหลังจากนี้
settings-audio-device = เอาต์พุตเสียง
settings-audio-device-system = ค่าเริ่มต้นของระบบ
settings-resume = เล่นต่อจากที่ค้างไว้
settings-resume-note = ไฟล์จะเริ่มเล่นจากจุดที่ปิดไว้ครั้งล่าสุด
settings-folder-playlists = เล่นไฟล์อื่นในโฟลเดอร์
settings-folder-playlists-note = เมื่อเปิดไฟล์หนึ่ง ไฟล์อื่นในโฟลเดอร์เดียวกันจะแสดงในรายการและเล่นต่อจากไฟล์นั้น
settings-streams = สตรีมและวิดีโอบนเว็บ
settings-stream-quality = คุณภาพ
settings-stream-quality-note = สำหรับสตรีมและวิดีโอบนเว็บที่มีให้เลือกหลายคุณภาพ มีผลกับสิ่งที่เปิดหลังจากนี้
settings-quality-auto = อัตโนมัติ สูงเท่ากับหน้าจอ
settings-quality-best = ดีที่สุด
settings-web-pages = เล่นวิดีโอบนเว็บด้วย yt-dlp
settings-web-pages-found = yt-dlp จะค้นหาวิดีโอบนหน้าเว็บ เช่น หน้าของ YouTube
settings-web-pages-missing = ยังไม่ได้ติดตั้ง yt-dlp ติดตั้งเพื่อเล่นวิดีโอบนหน้าเว็บ เช่น หน้าของ YouTube
settings-folders = โฟลเดอร์
settings-recordings = การบันทึก
settings-screenshots = เฟรมที่บันทึกไว้

settings-interface-language = ภาษาของอินเทอร์เฟซ
# The first entry of the language list.
settings-language-system = ตามระบบ
settings-track-languages = ภาษาของแทร็ก
settings-track-languages-note = รหัสภาษา เช่น en หรือ fra เรียงจากที่ต้องการมากที่สุดก่อน คั่นด้วยจุลภาค plaa จะเลือกเสียงและคำบรรยายในภาษาเหล่านี้เมื่อไฟล์มีให้
settings-audio-languages = เสียง
settings-subtitle-languages = คำบรรยาย

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = อินเทอร์เฟซ AVB
settings-avb-interface-note = พอร์ตแบบมีสายที่ plaa ใช้ค้นหา Talker และ Listener ของ AVB และ Milan การส่งต้องใช้พอร์ตที่มีคล็อกฮาร์ดแวร์ PTP เช่น พอร์ตของ Intel I210
settings-avb-off = ปิด
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (แสดงรายการเท่านั้น: ไม่มีคล็อก PTP)
settings-avb-off-mac = { $interface } (แสดงรายการเท่านั้น: AVB ปิดอยู่)
settings-avb-interface-note-mac = พอร์ตแบบมีสายที่ plaa ใช้ค้นหา Talker และ Listener ของ AVB และ Milan การส่งจะผ่านเอนทิตี AVB ของ Mac เครื่องนี้เอง ซึ่งต้องเปิด AVB สำหรับพอร์ตนั้นใน Network Device Browser ของ Audio MIDI Setup
settings-avb-talker-note-mac = plaa ส่งเสียงของหน้าต่างที่เล่นล่าสุดผ่านเอนทิตี AVB ของ Mac เครื่องนี้เอง ไปยัง Listener ที่เชื่อมต่ออยู่กับเอนทิตีนั้น
settings-avb-talker = ส่งสิ่งที่กำลังเล่น
settings-avb-talker-note = plaa จะเป็น Talker ของ Milan และส่งเสียงของหน้าต่างที่เล่นล่าสุดไปยัง Listener ที่เชื่อมต่ออยู่
settings-avb-name = ชื่อบนเครือข่าย
settings-avb-name-note = คอนโทรลเลอร์จะแสดง plaa เป็น “{ $name }” กด Enter เพื่อเปลี่ยนชื่อ
# plaa's name as a Milan entity, by default.
avb-default-name = plaa บน { $computer }
avb-could-not-start = ใช้ { $interface } ไม่ได้: { $error }
network-title = เสียงผ่านเครือข่าย
network-off = เปิด “ส่งสิ่งที่กำลังเล่น” ในการตั้งค่าเพื่อเล่นบนเครือข่าย
network-blocked = ไม่ได้ส่ง: { $reason }
network-waiting = พร้อมส่ง กำลังรอให้ Listener เชื่อมต่อ
network-sending = กำลังส่งไปยัง Listener ที่เชื่อมต่ออยู่
network-looking = กำลังค้นหา Listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = กำลังเริ่ม ptp4l ซึ่งรักษาเวลาเครือข่ายที่สตรีมของ plaa ใช้ในการส่ง…
network-ptp-elsewhere = ptp4l ทำงานบนอินเทอร์เฟซอื่น สตรีมของ plaa จึงไม่มีเวลาเครือข่ายที่นี่ และ Listener อาจเล่นไม่ได้ เลือกอินเทอร์เฟซนั้นในการตั้งค่า หรือเรียกใช้ ptp4l บนอินเทอร์เฟซนี้
network-ptp-missing = ptp4l ไม่ได้ทำงาน สตรีมของ plaa จึงไม่มีเวลาเครือข่าย และ Listener อาจเล่นไม่ได้ ติดตั้ง linuxptp พร้อมกับแพ็กเกจของ plaa หรือติดตั้ง triib ซึ่งจะเรียกใช้ ptp4l ให้
network-ptp-failed = ptp4l ไม่ได้ทำงาน สตรีมของ plaa จึงไม่มีเวลาเครือข่าย และ Listener อาจเล่นไม่ได้ plaa เริ่ม ptp4l ไม่ได้: { $reason }
network-no-listeners = ไม่มี Listener บนเครือข่ายที่เล่นเสียงของ plaa ได้
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = เล่นบน { $listener }
network-stop-on = หยุดเล่นบน { $listener }
network-choose-format = เลือกรูปแบบที่จะใช้เล่นบน { $listener }
network-failed = เชื่อมต่อไม่ได้: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = อัปเดต triib เพื่อเล่นที่นี่
network-choose = เลือก Listener เพื่อดูว่ารับสตรีมอย่างไร
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = ฟังที่นี่
network-no-sources = ไม่มี Talker บนเครือข่ายที่ส่งในรูปแบบที่ plaa เล่น หรือเปลี่ยนเป็นรูปแบบนั้นได้
# $talker is a talker's name, or its name and its output's.
network-listen-to = ฟัง { $talker }
network-listen-stop = หยุดฟัง { $talker }
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = เปลี่ยน { $talker } เป็น { $format } แล้วฟัง
network-listen-change-cuts-off = เปลี่ยน { $talker } เป็น { $format } แล้วฟัง โดยตัด { $count ->
       *[other] Listener อื่นอีก { $count } ตัว
    }
network-listen-frames = ได้รับ { $frames } เฟรม
network-listen-update-triib = อัปเดต triib เพื่อฟังที่นี่: plaa เล่นเฉพาะสิ่งที่มาถึงคอมพิวเตอร์เครื่องนี้อยู่แล้ว
network-listen-lite = พอร์ตใช้ AVB Lite ซึ่งไม่จองสตรีม: plaa เล่นเฉพาะสิ่งที่มาถึงคอมพิวเตอร์เครื่องนี้อยู่แล้ว

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = กำลังค้นหา Talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = เปิด “ส่งสิ่งที่กำลังเล่น” ในการตั้งค่าเพื่อฟัง Talker ที่นี่
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa ฟังที่นี่ไม่ได้: { $reason }
connect-sending = ส่งในรูปแบบที่ plaa เล่น
connect-changing = เปลี่ยนเป็นรูปแบบที่ plaa เล่น
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker } ในรูปแบบ { $format }
connect-change-cuts-off = { $talker } ในรูปแบบ { $format } โดยตัด { $count ->
       *[other] Listener อื่นอีก { $count } ตัว
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = สถานะ
network-not-connected = ไม่ได้เชื่อมต่อ
network-from = จาก { $stream }
network-from-receiving = จาก { $stream } กำลังรับ
network-from-waiting = จาก { $stream } กำลังรอ Talker
network-from-failed = จาก { $stream } การจองของ Talker ล้มเหลว: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = มีเดียคล็อก
network-locked = ล็อกกับสตรีมแล้ว
network-not-locked = ไม่ได้ล็อก
network-format = รูปแบบ
# The formats both the listener and plaa's stream can be in.
network-formats = รูปแบบที่รับจาก plaa ได้
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID สตรีม
# The MAC address the stream is sent to.
network-destination = ปลายทาง
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = ความหน่วง
network-microseconds = { $time } µs
network-frames = เฟรมที่รับ
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = เฟรมผิดลำดับ
# How often the stream stopped coming while it was taken.
network-interrupted = การหยุดชะงัก
network-entity-id = ID เอนทิตี

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = แบนด์วิดท์ไม่พอ
msrp-failure-2 = ทรัพยากรของสวิตช์ไม่พอ
msrp-failure-3 = แบนด์วิดท์ไม่พอสำหรับคลาสทราฟฟิกนี้
msrp-failure-4 = ID สตรีมถูก Talker อื่นใช้อยู่
msrp-failure-5 = ที่อยู่ปลายทางถูกใช้อยู่แล้ว
msrp-failure-6 = ถูกสตรีมที่มีลำดับสูงกว่าแย่งสิทธิ์
msrp-failure-7 = ความหน่วงที่รายงานเปลี่ยนไป
msrp-failure-8 = พอร์ตขาออกไม่รองรับ AVB
msrp-failure-9 = ใช้ที่อยู่ปลายทางอื่น
msrp-failure-10 = ทรัพยากร MSRP หมด
msrp-failure-11 = ทรัพยากร MMRP หมด
msrp-failure-12 = จัดเก็บที่อยู่ปลายทางไม่ได้
msrp-failure-13 = ลำดับความสำคัญไม่ใช่ลำดับความสำคัญของคลาส SR
msrp-failure-14 = เฟรมใหญ่เกินไปสำหรับสื่อกลาง
msrp-failure-15 = ถึงขีดจำกัดพอร์ต fan-in แล้ว
msrp-failure-16 = ค่าแรกเปลี่ยนไปสำหรับสตรีมที่ลงทะเบียนแล้ว
msrp-failure-17 = VLAN ถูกบล็อกที่พอร์ตขาออก
msrp-failure-18 = การแท็ก VLAN ถูกปิดที่พอร์ตขาออก
msrp-failure-19 = ลำดับความสำคัญของคลาส SR ไม่ตรงกัน
msrp-failure-unknown = ไม่ทราบสาเหตุ
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason } ที่สวิตช์ { $bridge }

## The macOS menu bar

menu-about = เกี่ยวกับ plaa
menu-settings = การตั้งค่า…
menu-services = บริการ
menu-hide = ซ่อน plaa
menu-hide-others = ซ่อนแอปอื่น
menu-show-all = แสดงทั้งหมด
menu-quit = ออกจาก plaa
menu-file = ไฟล์
menu-open = เปิด…
menu-open-address = เปิดที่อยู่…
menu-close = ปิดหน้าต่าง
menu-edit = แก้ไข
menu-cut = ตัด
menu-copy = คัดลอก
menu-paste = วาง
menu-select-all = เลือกทั้งหมด
menu-undo = เลิกทำ
menu-redo = ทำซ้ำ
menu-trim = ตัดแต่ง…
menu-split = แยกคลิป
menu-rotate-left = หมุนซ้าย
menu-rotate-right = หมุนขวา
menu-flip-horizontal = พลิกแนวนอน
menu-flip-vertical = พลิกแนวตั้ง
menu-revert = ย้อนกลับเป็นต้นฉบับ
menu-export = ส่งออก…
menu-view = มุมมอง
menu-playback = การเล่น
menu-play-pause = เล่นหรือหยุดชั่วคราว
menu-to-start = ไปที่จุดเริ่มต้น
menu-previous-chapter = บทก่อนหน้า
menu-next-chapter = บทถัดไป
menu-faster = เร็วขึ้น
menu-slower = ช้าลง
menu-normal-speed = ความเร็วปกติ
menu-ab-loop = วนซ้ำระหว่างสองจุด
menu-volume-up = ดังขึ้น
menu-volume-down = เบาลง
menu-mute = ปิดหรือเปิดเสียง
menu-next-subtitles = คำบรรยายถัดไป
menu-next-audio = แทร็กเสียงถัดไป
menu-window = หน้าต่าง
menu-minimize = ย่อ
menu-zoom = ซูม
menu-bring-all-to-front = นำทั้งหมดมาไว้ด้านหน้า

## Editing

action-edit = แก้ไข
title-edited = { $name } (แก้ไขแล้ว)
edit-trim = ตัดแต่ง
edit-trim-apply = ตัดแต่ง
edit-split = แยกคลิป
edit-remove-clip = เอาคลิปออก
edit-done = เสร็จสิ้น
edit-rotate-left = หมุนซ้าย
edit-rotate-right = หมุนขวา
edit-flip-horizontal = พลิกแนวนอน
edit-flip-vertical = พลิกแนวตั้ง
edit-undo = เลิกทำ
edit-redo = ทำซ้ำ
edit-revert = ย้อนกลับเป็นต้นฉบับ
edit-export = ส่งออก…

## Exporting

export-title = ส่งออก
export-format = รูปแบบ
export-size = ขนาด
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = เสียงเท่านั้น
export-size-original = { $lines }p ตามต้นฉบับ
export-on-graphics-card = เข้ารหัสบนการ์ดแสดงผล ({ $api })
export-on-processor = เข้ารหัสบนโปรเซสเซอร์ ซึ่งใช้เวลานานกว่า
export-no-encoder = ไม่มีตัวเข้ารหัสสำหรับรูปแบบนี้ที่ใช้ได้ที่นี่
export-checking = กำลังค้นหาตัวเข้ารหัส…
export-sound-format = รูปแบบเสียง
export-quality = คุณภาพ
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = บีบอัด: เล่นได้แทบทุกที่ รวมถึงอุปกรณ์ของ Apple
export-mp3-note = บีบอัด: เล่นได้ทุกที่
export-opus-note = บีบอัด: ให้เสียงดีที่สุดเมื่อเทียบกับขนาด แต่ไม่ใช่ทุกเครื่องเล่นที่รองรับ
export-flac-note = ไม่สูญเสียคุณภาพ: เสียงตรงตามต้นฉบับทุกประการ ในรูปแบบเปิด
export-alac-note = ไม่สูญเสียคุณภาพ: เสียงตรงตามต้นฉบับทุกประการ ในรูปแบบที่อุปกรณ์ของ Apple เล่นได้
export-wav-note = ไม่บีบอัด: เสียงตรงตามต้นฉบับทุกประการ แต่ไฟล์มีขนาดใหญ่ที่สุด
export-kilobits = { $rate } kb/s
export-bits = { $bits } บิต
export-rate-lower = บันทึกที่ { $rate } kHz ต่ำกว่า { $file } kHz ของไฟล์
export-choose = ส่งออก…
export-name = { $name } ฉบับส่งออก
export-name-edited = { $name } ฉบับแก้ไข
export-progress = กำลังส่งออก “{ $name }”
export-stop = หยุดการส่งออก
notice-exported = ส่งออก “{ $name }” แล้ว
notice-export-stopped = หยุดการส่งออกแล้ว
notice-export-failed = ส่งออกไม่ได้: { $error }
close-title = ปิดโดยไม่ส่งออกหรือไม่
close-unexported = การเปลี่ยนแปลงใน “{ $name }” จะหายไปหากคุณไม่ส่งออก
close-exporting = การส่งออก “{ $name }” จะหยุดลง และสิ่งที่เขียนไปแล้วจะหายไป
close-anyway = ปิด
close-quit-anyway = ออก

## Recording

record-title = การบันทึกเสียง
record-file-name = การบันทึกเสียง
record-start = บันทึก
record-stop = หยุดบันทึก
record-no-input = ไม่มีไมโครโฟน
record-microphone = ไมโครโฟน
record-default-microphone = ค่าเริ่มต้นของระบบ
record-playing = เสียงที่คอมพิวเตอร์กำลังเล่น
record-mono = โมโน
record-stereo = สเตอริโอ
record-channels = { $count } ช่อง
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels } บันทึกที่ { $saved } kHz
action-record-sound = บันทึกเสียง
open-record-sound = การบันทึกเสียงใหม่
start-record-sound = บันทึกเสียง
menu-record-sound = การบันทึกเสียงใหม่
record-screen-title = การบันทึกหน้าจอ
record-screen-file-name = การบันทึกหน้าจอ
record-screen-whole = หน้าจอ
record-screen-number = หน้าจอ { $number }
record-screen-main = หน้าจอ { $number } (หลัก)
record-screens = หน้าจอ
record-windows = หน้าต่าง
record-screen-what = สิ่งที่จะบันทึก
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = ทุกเฟรม
record-rate-tip = เฟรมต่อวินาที
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · หลุด { $count } เฟรม
record-screen-window = หน้าต่าง
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = หยุดแชร์แล้ว
record-screen-again = เลือกอีกครั้ง
record-screen-failed = บันทึกหน้าจอไม่ได้
record-sound = เสียง
record-no-sound = ไม่มีเสียง
# How much louder the input is taken, for a quiet microphone.
record-gain = เพิ่มความดัง
record-gain-value = +{ $decibels } dB
record-level-automatically = ปรับระดับอัตโนมัติ
record-hide-title = plaa จะซ่อนตัวขณะบันทึก
record-hide-tray = หากต้องการหยุดบันทึก ให้คลิกจุดสีแดงที่ plaa วางไว้ในถาดระบบ หรือเปิด plaa อีกครั้ง
record-hide-reopen = หากต้องการหยุดบันทึก ให้เปิด plaa อีกครั้ง
record-hide-start = เริ่มบันทึก
record-tray-title = plaa กำลังบันทึกหน้าจอ
record-tray-tip = คลิกเพื่อหยุดบันทึก
action-record-screen = บันทึกหน้าจอ
open-record-screen = การบันทึกหน้าจอใหม่
start-record-screen = บันทึกหน้าจอ
menu-record-screen = การบันทึกหน้าจอใหม่
record-camera-title = การบันทึกจากกล้อง
record-camera-file-name = การบันทึกจากกล้อง
record-camera = กล้อง
record-camera-failed = เปิดกล้องไม่ได้
action-record-camera = บันทึกจากกล้อง
open-record-camera = การบันทึกจากกล้องใหม่
start-record-camera = บันทึกจากกล้อง
menu-record-camera = การบันทึกจากกล้องใหม่
notice-no-camera = ไม่มีกล้องให้บันทึก
notice-recorded = บันทึก “{ $name }” ไว้ใน { $folder } แล้ว
notice-record-failed = บันทึกไม่ได้: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = อนุญาตการควบคุมจากภายนอก
settings-allow-outside-control-note = เอเจนต์ AI เช่น Claude Code สามารถดูสิ่งที่กำลังเล่นใน plaa และควบคุม plaa ได้ผ่าน plaa --mcp โดย plaa จะถามก่อนทุกครั้งที่มีเอเจนต์ใหม่
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = อนุญาตแล้ว: { $agents }
settings-forget-agents = ลืม
settings-ask-before-note = ถามก่อนเมื่อเอเจนต์จะ:
settings-ask-reading = ดูสิ่งที่กำลังเล่นหรือภาพ
settings-ask-playing = ควบคุมการเล่น เสียง หรือโหมดเต็มหน้าจอ
settings-ask-opening = เปิดไฟล์หรือปิดหน้าต่าง
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = อนุญาตให้ { $agent } ควบคุม plaa หรือไม่
agent-prompt-body = { $agent } ขอใช้การควบคุมจากภายนอกของ plaa เพื่อดูสิ่งที่กำลังเล่นและควบคุม plaa คุณปิดการควบคุมจากภายนอกได้ในการตั้งค่า
agent-prompt-allow = อนุญาต
agent-prompt-deny = ไม่อนุญาต
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = อนุญาตให้ { $agent } ดูสิ่งที่กำลังเล่นที่นี่หรือไม่
agent-ask-play = อนุญาตให้ { $agent } ควบคุมการเล่นที่นี่หรือไม่
agent-ask-open = อนุญาตให้ { $agent } เปิดหรือปิดหน้าต่างหรือไม่
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } ขอใช้ “{ $tool }” คุณเลือกได้ในการตั้งค่าว่าจะให้ plaa ถามเรื่องใดบ้าง

## Casting to a TV or speaker on the network

cast-looking = กำลังค้นหาทีวีและลำโพง…
cast-none = ไม่พบทีวี ลำโพง หรือ Chromecast
# $renderer is the TV's or speaker's name.
cast-stop = หยุดเล่นบน { $renderer }
notice-casting = กำลังเล่นบน { $renderer }
notice-cast-failed = เล่นบน { $renderer } ไม่ได้: { $reason }
cast-web-page = ยังแคสต์หน้าเว็บไม่ได้: ให้เปิดที่อยู่ของวิดีโอนั้นโดยตรงแทน

## The assistant panel, and its models in Settings

assistant-add-another = เพิ่มโมเดล…
assistant-add-model = ผู้ช่วยต้องใช้โมเดล ได้แก่ โมเดลบนคลาวด์พร้อมคีย์ API หรือโมเดลภายในเครื่อง
assistant-ask = ถามเกี่ยวกับสิ่งที่กำลังเล่น
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = แชทเริ่มยาวเกินกว่าที่ { $model } จะอ่านได้ในครั้งเดียว จึงสรุปส่วนก่อนหน้าไว้ ผู้ช่วยอาจกลับไปดูสิ่งที่เห็นอีกครั้ง
assistant-compacting = กำลังสรุปแชทก่อนหน้า
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } บนคอมพิวเตอร์เครื่องนี้
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } ที่ { $host }
assistant-new-chat = แชทใหม่
assistant-no-model = เพิ่มโมเดลในการตั้งค่าก่อน
assistant-open-settings = เพิ่มโมเดล
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = บทสนทนายาวเกินกว่าที่ { $model } จะอ่านได้ในครั้งเดียว เริ่มแชทใหม่ หรือเลือกโมเดลที่อ่านได้มากกว่า
assistant-problem-credit = { $provider } แจ้งว่าบัญชีไม่มีเครดิต บัญชีใหม่ต้องซื้อเครดิตบนเว็บไซต์ของ { $provider } ก่อนจึงจะใช้คีย์ได้ จากนั้นลองอีกครั้ง
assistant-problem-key = { $provider } ปฏิเสธคีย์ API ตรวจสอบคีย์ในการตั้งค่า
assistant-problem-model = { $provider } ไม่มีโมเดลชื่อ { $model } ตรวจสอบชื่อในการตั้งค่า
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } ไม่ได้ตอบ: { $message }
assistant-problem-rate = { $provider } ขอให้ส่งคำขอช้าลง ลองอีกครั้งในอีกสักครู่
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } ขอให้ส่งคำขอช้าลง: { $message }
assistant-problem-refused = { $model } ปฏิเสธที่จะตอบ
assistant-problem-unavailable = { $provider } ไม่ว่างหรือมีปัญหา ลองอีกครั้งในอีกสักครู่
assistant-problem-unreachable = plaa เชื่อมต่อกับ { $provider } ไม่ได้ ตรวจสอบการเชื่อมต่อ หรือตรวจสอบว่าเซิร์ฟเวอร์กำลังทำงานอยู่
assistant-running = กำลังทำงาน…
assistant-send = ส่ง
assistant-shortened = แชทยาวเกินกว่าที่ { $model } จะอ่านได้ในครั้งเดียว จึงตัดผลลัพธ์ของเครื่องมือก่อนหน้าออก ผู้ช่วยอาจกลับไปดูสิ่งที่เห็นอีกครั้ง
assistant-stop = หยุด
assistant-stopped = หยุดแล้ว
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = กำลังคุยกับ { $model }
assistant-thinking = กำลังคิด…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = ความคิด
assistant-title = ผู้ช่วย
settings-assistant-add = เพิ่มโมเดล
settings-assistant-add-button = เพิ่ม
# $model is the model's name.
settings-assistant-added = { $model } ตอบกลับแล้วและถูกเพิ่มแล้ว
settings-assistant-added-tag = เพิ่มแล้ว
# $example is an address, such as http://localhost:11434.
settings-assistant-address = ที่อยู่ เช่น { $example }
settings-assistant-change-key = เปลี่ยนคีย์
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = เซิร์ฟเวอร์ที่เข้ากันได้กับ OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = บริบท
settings-assistant-context-note = ค่าที่มากขึ้นทำให้ผู้ช่วยจดจำแชทได้มากขึ้น แต่โมเดลจะใช้หน่วยความจำมากขึ้นและอาจตอบช้าลง
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K โทเค็น
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = เปิดหน้าเว็บ
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } กำลังทำงานอยู่ที่ { $address }
settings-assistant-found-none = { $provider } ยังไม่มีโมเดล ดาวน์โหลดโมเดลด้วย { $provider } แล้วค้นหาอีกครั้ง
settings-assistant-get-key = รับคีย์ API
settings-assistant-get-server = รับ { $provider }
settings-assistant-in-use = กำลังใช้
settings-assistant-key = คีย์ API
# $error is what the keychain said.
settings-assistant-key-failed = ไม่สามารถเก็บคีย์ไว้ในพวงกุญแจได้: { $error }
settings-assistant-key-kept = คีย์ { $provider } ของคุณเก็บไว้ในพวงกุญแจของระบบ
settings-assistant-key-needed = โมเดลนี้ต้องใช้คีย์ API
settings-assistant-key-refused = { $provider } ปฏิเสธคีย์นี้ ตรวจสอบว่าคัดลอกคีย์มาครบถ้วนและมาจากบัญชีที่ถูกต้อง
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = สร้างคีย์บนเว็บไซต์ของ { $provider } แล้ววางที่นี่
settings-assistant-look-again = ค้นหาอีกครั้ง
settings-assistant-looking = กำลังค้นหาโมเดล…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = โมเดล เช่น { $example }
settings-assistant-no-server = plaa ไม่พบ { $provider } ที่ทำงานอยู่บนคอมพิวเตอร์เครื่องนี้ เริ่ม { $provider } หรือใส่ที่อยู่ด้านล่าง
settings-assistant-no-tools = ใช้เครื่องมือไม่ได้ ซึ่งผู้ช่วยจำเป็นต้องใช้
settings-assistant-none = ยังไม่มีโมเดล เพิ่มได้ด้านล่าง โมเดลภายในเครื่อง เช่น โมเดลของ Ollama จะทำงานอยู่บนคอมพิวเตอร์เครื่องนี้เท่านั้น ส่วนโมเดลบนคลาวด์ต้องใช้คีย์ API จากผู้ให้บริการ
settings-assistant-note = โมเดลที่แผงผู้ช่วยใช้งานได้ คีย์จะเก็บไว้ในพวงกุญแจของระบบ
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = ใช้ที่อยู่อื่น
settings-assistant-recommended = แนะนำ
settings-assistant-remove = เอาออก
settings-assistant-sees = ดูรูปภาพได้
settings-assistant-trying = กำลังลอง…
settings-assistant-use = ใช้
settings-assistant-use-key = ดำเนินการต่อ
settings-assistant-uses-tools = ใช้เครื่องมือได้
