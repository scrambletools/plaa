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
language-name = Bahasa Melayu

## Common

app-name = plaa
common-cancel = Batal
common-close = Tutup
common-more = Lagi

## Command line

usage-help =
    Penggunaan: plaa [FILE OR ADDRESS]...

    Main fail video dan audio, serta strim di alamat seperti
    https://example.com/live.m3u8. Setiap satu dibuka dalam tetingkap plaa
    yang sedang berjalan, yang akan dimulakan jika perlu.

    Pilihan:
      -h, --help        Tunjukkan bantuan ini
      -V, --version     Tunjukkan versi
          --interfaces  Tunjukkan antara muka berwayar, dan sama ada plaa
                        boleh menghantar strim AVB padanya
          --mcp         Sajikan MCP pada stdin dan stdout, supaya ejen AI
                        dapat melihat apa yang dimainkan oleh plaa yang sedang
                        berjalan dan mengawalnya

## Start window

start-hint = Buka fail video atau audio, strim atau video web.
start-open = Buka
start-open-address = Buka Alamat
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Sambung

## Actions, as buttons, tooltips and menus name them

action-open = Buka
action-open-address = Buka Alamat
open-file = Buka fail…
open-address = Buka alamat…
action-settings = Tetapan
action-info = Pemeriksa
action-loop = Gelung
action-screenshot = Simpan Bingkai sebagai Gambar
action-play = Main
action-pause = Jeda
# $seconds is how far the button moves, such as 10.
action-back = Undur { $seconds } saat
action-forward = Maju { $seconds } saat
action-mute = Senyapkan
action-unmute = Nyahsenyap
action-speed = Kelajuan
action-tracks = Audio dan Sari Kata
action-full-screen = Skrin Penuh
action-playlist = Senarai Main
action-chapters = Bab
action-picture-in-picture = Gambar dalam Gambar
action-previous = Sebelumnya
action-next = Seterusnya
action-exit-full-screen = Keluar Skrin Penuh
action-quality = Kualiti
action-visualizer = Pengvisualan
action-network = Audio Rangkaian
action-cast = Hantar
action-connect = Sambung
action-assistant = Pembantu

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Bentuk gelombang
visualizer-prism = Prisma
visualizer-cover = Kulit album sahaja

## The floating controls

overlay-keep-shown = Kekalkan kawalan di tempatnya
overlay-auto-hide = Kawalan terapung di atas gambar

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Sari kata
tracks-off = Mati
# A track the file gives no name or language, by its number.
tracks-numbered = Trek { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Audio sahaja

## Inspector

info-title = Pemeriksa
playlist-title = Senarai Main
chapters-numbered = Bab { $number }
info-file = Fail
info-name = Nama
info-address = Alamat
info-folder = Folder
info-size = Saiz
info-format = Format
info-duration = Tempoh
info-bit-rate = Kadar bit
info-video = Video
info-audio = Audio
info-codec = Kodek
info-dimensions = Dimensi
info-frame-rate = Kadar bingkai
info-frames-per-second = { $rate } bingkai sesaat
info-pixel-format = Format piksel
info-decoding = Penyahkodan
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Kad grafik ({ $api })
info-decoding-software = Pemproses
info-sample-rate = Kadar sampel
info-hertz = { $rate } Hz
info-channels = Saluran
info-language = Bahasa
info-no-sound = Tiada bunyi
info-tags = Tag
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Strim
# The talker's listeners other than plaa.
info-other-listeners = Listener lain
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Kelas SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Format baharu memutuskan { $count ->
       *[other] { $count } listener lain milik talker.
    }

## Dialogs

dialog-open = Buka
dialog-filter-media = Video dan audio
dialog-filter-all = Semua fail

address-title = Buka Alamat
address-note = Halaman web dengan video, seperti halaman video YouTube, atau strim atau fail pada rangkaian, seperti https://example.com/live.m3u8.
address-label = Alamat
address-open = Buka
address-recent = Terkini
address-forget = Alih keluar daripada senarai terkini

## Notices

notice-dialog-failed = Dialog fail tidak dapat dibuka: { $error }
notice-screenshot-saved = “{ $name }” telah disimpan.
notice-screenshot-failed = Gambar tidak dapat disimpan: { $error }
notice-no-sound = Memainkan tanpa bunyi: { $reason }
notice-loop-from = Gelung dari { $time }. Tekan sekali lagi di tempat ia patut berakhir.
notice-loop-between = Menggelung { $from } hingga { $to }.
notice-loop-off = Gelung dimatikan.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Ini kelihatan seperti halaman web. plaa memainkan video pada halaman web dengan yt-dlp, yang tidak dipasang.
error-empty-list = Senarai ini tiada apa-apa untuk dimainkan.

## Settings

settings-title = Tetapan
settings-tab-appearance = Penampilan
settings-tab-playback = Main balik
settings-tab-network = Rangkaian
settings-tab-language = Bahasa
settings-tab-agents = Ejen
settings-tab-assistant = Pembantu
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (binaan pembangunan, { $build })
settings-choose = Pilih…

settings-appearance-system = Ikut sistem
settings-appearance-light = Cerah
settings-appearance-dark = Gelap
settings-colors = Warna
settings-system-accent = Guna warna aksen sistem
settings-omarchy-note = Warna dibina daripada aksen “{ $theme }”.
settings-system-accent-note = Warna dibina daripada warna aksen sistem.
settings-system-accent-none = Sistem tiada warna aksen, jadi plaa menggunakan warna yang dipilih di bawah.
settings-accent-chosen-note = Warna dibina daripada warna yang dipilih di bawah.
settings-controls = Kawalan
settings-overlay = Kawalan terapung
settings-overlay-note = Bar alat dan kawalan main balik terapung di atas gambar atau kulit album, dan menggelongsor pergi apabila penuding diam atau berada di luar tetingkap.
settings-overlay-delay = Sembunyikan selepas
settings-overlay-delay-note = Berapa lama penuding diam di atas apa yang dimainkan sebelum kawalan menggelongsor pergi.
settings-seconds = { $seconds } saat
settings-transparency = Kelutsinaran
settings-transparency-note = Sejauh mana gambar kelihatan menembusi kawalan terapung.
settings-percent = { $percent }%
settings-corner-radius = Jejari bucu
settings-corner-radius-note = Untuk dialog dan kawalan terapung.
settings-pixels = { $pixels } px
settings-animations = Animasi
settings-animations-note = Bar dan panel yang menggelongsor, dialog yang membesar dan butang yang melantun.
settings-animations-reduced = Dimatikan selagi sistem meminta pergerakan dikurangkan.

settings-hardware = Guna kad grafik
settings-hardware-note = Menyahkod video dan mengekod eksport pada kad grafik: menggunakan kurang kuasa, memainkan video besar dengan lancar dan mengeksport lebih pantas. Berkuat kuasa untuk fail yang dibuka selepas itu.
settings-audio-device = Output bunyi
settings-audio-device-system = Lalai sistem
settings-resume = Sambung dari tempat anda berhenti
settings-resume-note = Fail bermula di tempat ia berada semasa kali terakhir ditutup.
settings-folder-playlists = Mainkan fail lain dalam folder
settings-folder-playlists-note = Membuka satu fail akan menyenaraikan fail lain dalam foldernya, yang dimainkan selepasnya.
settings-streams = Strim dan video web
settings-stream-quality = Kualiti
settings-stream-quality-note = Untuk strim dan video web yang tersedia dalam beberapa kualiti. Berkuat kuasa untuk apa yang dibuka selepas itu.
settings-quality-auto = Automatik, setinggi skrin
settings-quality-best = Terbaik
settings-web-pages = Mainkan video web dengan yt-dlp
settings-web-pages-found = yt-dlp menemui video pada halaman seperti YouTube.
settings-web-pages-missing = yt-dlp tidak dipasang. Pasangkannya untuk memainkan video pada halaman seperti YouTube.
settings-folders = Folder
settings-recordings = Rakaman
settings-screenshots = Bingkai disimpan

settings-interface-language = Bahasa antara muka
# The first entry of the language list.
settings-language-system = Ikut sistem
settings-track-languages = Bahasa trek
settings-track-languages-note = Kod bahasa seperti en atau fra, yang paling dikehendaki dahulu, dipisahkan dengan koma. plaa memilih audio dan sari kata dalam bahasa ini apabila fail memilikinya.
settings-audio-languages = Audio
settings-subtitle-languages = Sari kata

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Antara muka AVB
settings-avb-interface-note = Port berwayar tempat plaa menemui talker dan listener AVB dan Milan. Penghantaran memerlukan port dengan jam perkakasan PTP, seperti port Intel I210.
settings-avb-off = Mati
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (senarai sahaja: tiada jam PTP)
settings-avb-off-mac = { $interface } (senarai sahaja: AVB dimatikan)
settings-avb-interface-note-mac = Port berwayar tempat plaa menemui talker dan listener AVB dan Milan. Penghantaran melalui entiti AVB milik Mac ini sendiri, yang memerlukan AVB dihidupkan untuk port itu dalam Network Device Browser pada Audio MIDI Setup.
settings-avb-talker-note-mac = plaa menghantar bunyi tetingkap yang terakhir dimainkan melalui entiti AVB milik Mac ini sendiri, kepada listener yang disambungkan kepadanya.
settings-avb-talker = Hantar apa yang dimainkan
settings-avb-talker-note = plaa menjadi talker Milan, yang menghantar bunyi tetingkap yang terakhir dimainkan kepada listener yang disambungkan kepadanya.
settings-avb-name = Nama pada rangkaian
settings-avb-name-note = Pengawal menunjukkan plaa sebagai “{ $name }”. Tekan Enter untuk menamakannya semula.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa pada { $computer }
avb-could-not-start = Tidak dapat menggunakan { $interface }: { $error }
network-title = Audio Rangkaian
network-off = Hidupkan “Hantar apa yang dimainkan” dalam Tetapan untuk bermain pada rangkaian.
network-blocked = Tidak menghantar: { $reason }
network-waiting = Sedia untuk menghantar, menunggu listener menyambung.
network-sending = Menghantar kepada listener yang disambungkan.
network-looking = Mencari listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Memulakan ptp4l, yang menjaga masa rangkaian yang menjadi rujukan penghantaran strim plaa…
network-ptp-elsewhere = ptp4l berjalan pada antara muka lain, jadi strim plaa tiada masa rangkaian di sini dan listener mungkin tidak dapat memainkannya. Pilih antara muka itu dalam Tetapan, atau jalankan ptp4l pada antara muka ini.
network-ptp-missing = ptp4l tidak berjalan, jadi strim plaa tiada masa rangkaian dan listener mungkin tidak dapat memainkannya. Pasang linuxptp bersama pakej plaa, atau triib, yang menjalankannya.
network-ptp-failed = ptp4l tidak berjalan, jadi strim plaa tiada masa rangkaian dan listener mungkin tidak dapat memainkannya. plaa tidak dapat memulakannya: { $reason }
network-no-listeners = Tiada listener pada rangkaian yang dapat memainkan bunyi plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Main pada { $listener }
network-stop-on = Henti bermain pada { $listener }
network-choose-format = Pilih format untuk dimainkan pada { $listener }
network-failed = Tidak dapat menyambung: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Kemas kini triib untuk bermain di sini.
network-choose = Pilih listener untuk melihat cara ia menerima strimnya.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Dengar di sini
network-no-sources = Tiada talker pada rangkaian yang menghantar dalam format yang dimainkan oleh plaa, atau boleh bertukar kepada format itu.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Dengar { $talker }
network-listen-stop = Henti mendengar { $talker }
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = Tukar { $talker } kepada { $format } dan dengar
network-listen-change-cuts-off = Tukar { $talker } kepada { $format } dan dengar, memutuskan { $count ->
       *[other] { $count } listener lain
    }
network-listen-frames = { $frames } bingkai diterima
network-listen-update-triib = Kemas kini triib untuk mendengar di sini: plaa hanya memainkan apa yang memang sampai ke komputer ini.
network-listen-lite = Port menjalankan AVB Lite, yang tidak menempah strim: plaa hanya memainkan apa yang memang sampai ke komputer ini.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Mencari talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = Hidupkan “Hantar apa yang dimainkan” dalam Tetapan untuk mendengar talker di sini.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa tidak dapat mendengar di sini: { $reason }
connect-sending = Menghantar dalam format yang dimainkan oleh plaa
connect-changing = Bertukar kepada format yang dimainkan oleh plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, dalam { $format }
connect-change-cuts-off = { $talker }, dalam { $format }, memutuskan { $count ->
       *[other] { $count } listener lain
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Tidak disambungkan
network-from = Daripada { $stream }
network-from-receiving = Daripada { $stream }, menerima
network-from-waiting = Daripada { $stream }, menunggu talker
network-from-failed = Daripada { $stream }, tempahan talker gagal: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Jam media
network-locked = Terkunci pada strim
network-not-locked = Tidak terkunci
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Format yang boleh diterimanya daripada plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID strim
# The MAC address the stream is sent to.
network-destination = Destinasi
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Kependaman
network-microseconds = { $time } µs
network-frames = Bingkai diterima
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Bingkai tidak mengikut urutan
# How often the stream stopped coming while it was taken.
network-interrupted = Gangguan
# Frames that came after, or long before, their time to be played.
network-late = Bingkai lewat
network-early = Bingkai terlalu awal
network-entity-id = ID entiti

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = lebar jalur tidak mencukupi
msrp-failure-2 = sumber suis tidak mencukupi
msrp-failure-3 = lebar jalur tidak mencukupi untuk kelas trafik ini
msrp-failure-4 = ID strim digunakan oleh talker lain
msrp-failure-5 = alamat destinasi sudah digunakan
msrp-failure-6 = didahului oleh strim yang lebih tinggi kedudukannya
msrp-failure-7 = kependaman yang dilaporkan telah berubah
msrp-failure-8 = port egress tidak menyokong AVB
msrp-failure-9 = gunakan alamat destinasi lain
msrp-failure-10 = kehabisan sumber MSRP
msrp-failure-11 = kehabisan sumber MMRP
msrp-failure-12 = tidak dapat menyimpan alamat destinasi
msrp-failure-13 = keutamaan bukan keutamaan kelas SR
msrp-failure-14 = bingkai terlalu besar untuk medium
msrp-failure-15 = had port fan-in dicapai
msrp-failure-16 = nilai pertama berubah bagi strim berdaftar
msrp-failure-17 = VLAN disekat pada port egress
msrp-failure-18 = pengetegan VLAN dinyahdayakan pada port egress
msrp-failure-19 = keutamaan kelas SR tidak sepadan
msrp-failure-unknown = sebab tidak diketahui
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, pada suis { $bridge }

## The macOS menu bar

menu-about = Perihal plaa
menu-settings = Tetapan…
menu-services = Perkhidmatan
menu-hide = Sembunyikan plaa
menu-hide-others = Sembunyikan Yang Lain
menu-show-all = Tunjukkan Semua
menu-quit = Keluar plaa
menu-file = Fail
menu-open = Buka…
menu-open-address = Buka Alamat…
menu-close = Tutup Tetingkap
menu-edit = Edit
menu-cut = Potong
menu-copy = Salin
menu-paste = Tampal
menu-select-all = Pilih Semua
menu-undo = Buat Asal
menu-redo = Buat Semula
menu-trim = Pangkas…
menu-split = Pisahkan Klip
menu-rotate-left = Putar ke Kiri
menu-rotate-right = Putar ke Kanan
menu-flip-horizontal = Balikkan Mendatar
menu-flip-vertical = Balikkan Menegak
menu-revert = Kembalikan kepada Asal
menu-export = Eksport…
menu-view = Paparan
menu-playback = Main Balik
menu-play-pause = Main atau Jeda
menu-to-start = Pergi ke Permulaan
menu-previous-chapter = Bab Sebelumnya
menu-next-chapter = Bab Seterusnya
menu-faster = Lebih Laju
menu-slower = Lebih Perlahan
menu-normal-speed = Kelajuan Biasa
menu-ab-loop = Gelung Antara Dua Titik
menu-volume-up = Lebih Kuat
menu-volume-down = Lebih Senyap
menu-mute = Senyapkan atau Nyahsenyap
menu-next-subtitles = Sari Kata Seterusnya
menu-next-audio = Trek Audio Seterusnya
menu-window = Tetingkap
menu-minimize = Minimumkan
menu-zoom = Zum
menu-bring-all-to-front = Bawa Semua ke Hadapan

## Editing

action-edit = Edit
title-edited = { $name } (diedit)
edit-trim = Pangkas
edit-trim-apply = Pangkas
edit-split = Pisahkan klip
edit-remove-clip = Alih keluar klip
edit-done = Selesai
edit-rotate-left = Putar ke kiri
edit-rotate-right = Putar ke kanan
edit-flip-horizontal = Balikkan mendatar
edit-flip-vertical = Balikkan menegak
edit-undo = Buat asal
edit-redo = Buat semula
edit-revert = Kembalikan kepada asal
edit-export = Eksport…

## Exporting

export-title = Eksport
export-format = Format
export-size = Saiz
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Bunyi sahaja
export-size-original = { $lines }p, seperti asal
export-on-graphics-card = Dikodkan pada kad grafik ({ $api })
export-on-processor = Dikodkan pada pemproses, yang mengambil masa lebih lama
export-no-encoder = Tiada pengekod untuk format ini yang dapat dibuka di sini
export-checking = Mencari pengekod…
export-sound-format = Format bunyi
export-quality = Kualiti
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Dimampatkan; boleh dimainkan hampir di mana-mana, termasuk pada peranti Apple
export-mp3-note = Dimampatkan; boleh dimainkan di mana-mana
export-opus-note = Dimampatkan; bunyi terbaik bagi saiznya, walaupun bukan semua pemain menyokongnya
export-flac-note = Tanpa kehilangan kualiti: bunyi tepat seperti asalnya, dalam format terbuka
export-alac-note = Tanpa kehilangan kualiti: bunyi tepat seperti asalnya, sebagaimana peranti Apple memainkannya
export-wav-note = Tidak dimampatkan: bunyi tepat seperti asalnya, dalam fail paling besar
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Disimpan pada { $rate } kHz, di bawah { $file } kHz fail asal
export-choose = Eksport…
export-name = { $name } eksport
export-name-edited = { $name } diedit
export-progress = Mengeksport “{ $name }”
export-stop = Henti mengeksport
notice-exported = “{ $name }” telah dieksport
notice-export-stopped = Pengeksportan dihentikan
notice-export-failed = Tidak dapat mengeksport: { $error }
close-title = Tutup tanpa mengeksport?
close-unexported = Perubahan pada “{ $name }” akan hilang melainkan anda mengeksportnya.
close-exporting = Pengeksportan “{ $name }” akan berhenti, dan apa yang telah ditulis setakat ini akan hilang.
close-anyway = Tutup
close-quit-anyway = Keluar

## Recording

record-title = Rakaman bunyi
record-file-name = Rakaman bunyi
record-start = Rakam
record-stop = Henti merakam
record-no-input = Tiada mikrofon
record-microphone = Mikrofon
record-default-microphone = Lalai sistem
record-playing = Apa yang dimainkan oleh komputer
record-mono = mono
record-stereo = stereo
record-channels = { $count } saluran
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, disimpan pada { $saved } kHz
action-record-sound = Rakam bunyi
open-record-sound = Rakaman bunyi baharu
start-record-sound = Rakam Bunyi
menu-record-sound = Rakaman Bunyi Baharu
record-screen-title = Rakaman skrin
record-screen-file-name = Rakaman skrin
record-screen-whole = Skrin
record-screen-number = Skrin { $number }
record-screen-main = Skrin { $number } (utama)
record-screens = Skrin
record-windows = Tetingkap
record-screen-what = Apa yang hendak dirakam
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Setiap bingkai
record-rate-tip = Bingkai sesaat
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } tercicir
record-screen-window = Tetingkap
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Tidak lagi dikongsi
record-screen-again = Pilih semula
record-screen-failed = Tidak dapat merakam skrin
record-sound = Bunyi
record-no-sound = Tiada bunyi
# How much louder the input is taken, for a quiet microphone.
record-gain = Lebih kuat
record-gain-value = +{ $decibels } dB
record-level-automatically = Laraskan paras secara automatik
record-hide-title = plaa bersembunyi semasa merakam
record-hide-tray = Untuk berhenti merakam, klik titik merah yang diletakkan oleh plaa dalam dulang sistem, atau buka plaa semula.
record-hide-reopen = Untuk berhenti merakam, buka plaa semula.
record-hide-start = Mula merakam
record-tray-title = plaa sedang merakam skrin
record-tray-tip = Klik untuk berhenti merakam
action-record-screen = Rakam skrin
open-record-screen = Rakaman skrin baharu
start-record-screen = Rakam Skrin
menu-record-screen = Rakaman Skrin Baharu
record-camera-title = Rakaman kamera
record-camera-file-name = Rakaman kamera
record-camera = Kamera
record-camera-failed = Tidak dapat membuka kamera
action-record-camera = Rakam kamera
open-record-camera = Rakaman kamera baharu
start-record-camera = Rakam Kamera
menu-record-camera = Rakaman Kamera Baharu
notice-no-camera = Tiada kamera untuk dirakam
notice-recorded = “{ $name }” disimpan dalam { $folder }
notice-record-failed = Tidak dapat merakam: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Benarkan kawalan luar
settings-allow-outside-control-note = Ejen AI seperti Claude Code boleh melihat apa yang dimainkan dalam plaa dan mengawalnya, melalui plaa --mcp. plaa akan bertanya sebelum setiap ejen baharu.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Dibenarkan: { $agents }
settings-forget-agents = Lupakan
settings-ask-before-note = Tanya dahulu sebelum ejen:
settings-ask-reading = Melihat apa yang dimainkan, atau gambarnya
settings-ask-playing = Mengawal main balik, bunyi atau skrin penuh
settings-ask-opening = Membuka fail atau menutup tetingkap
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Benarkan { $agent } mengawal plaa?
agent-prompt-body = { $agent } meminta untuk menggunakan kawalan luar plaa, untuk melihat apa yang dimainkan dan mengawalnya. Anda boleh mematikan kawalan luar dalam Tetapan.
agent-prompt-allow = Benarkan
agent-prompt-deny = Jangan benarkan
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Benarkan { $agent } melihat apa yang dimainkan di sini?
agent-ask-play = Benarkan { $agent } mengawal main balik di sini?
agent-ask-open = Benarkan { $agent } membuka atau menutup tetingkap?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } meminta untuk menggunakan “{ $tool }”. Tetapan menentukan perkara yang ditanya oleh plaa.

## Casting to a TV or speaker on the network

cast-looking = Mencari TV dan pembesar suara…
cast-none = Tiada TV, pembesar suara atau Chromecast ditemui.
# $renderer is the TV's or speaker's name.
cast-stop = Henti bermain pada { $renderer }
notice-casting = Memainkan pada { $renderer }.
notice-cast-failed = Tidak dapat memainkan pada { $renderer }: { $reason }
cast-web-page = Halaman web belum boleh dihantar: buka alamat video itu sendiri.

## The assistant panel, and its models in Settings

assistant-add-another = Tambah model…
assistant-add-model = Pembantu memerlukan model: model awan dengan kunci API-nya, atau model setempat.
assistant-ask = Tanya tentang apa yang dimainkan
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Sembang menjadi terlalu panjang untuk dibaca oleh { $model } sekali gus, jadi bahagian awalnya telah diringkaskan. Pembantu mungkin melihat semula apa yang telah dilihatnya.
assistant-compacting = Meringkaskan bahagian awal sembang
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } pada komputer ini
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } di { $host }
assistant-new-chat = Sembang baharu
assistant-no-model = Tambah model dalam Tetapan dahulu.
assistant-open-settings = Tambah model
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Perbualan ini tidak lagi muat dalam jumlah yang boleh dibaca oleh { $model } sekali gus. Mulakan sembang baharu, atau pilih model yang boleh membaca lebih banyak.
assistant-problem-credit = { $provider } menyatakan akaun ini tiada kredit. Akaun baharu perlu membeli kredit di laman web { $provider } sebelum kuncinya berfungsi; kemudian cuba lagi.
assistant-problem-key = { $provider } menolak kunci API. Semak kunci itu dalam Tetapan.
assistant-problem-model = { $provider } tiada model bernama { $model }. Semak namanya dalam Tetapan.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } tidak menjawab: { $message }
assistant-problem-rate = { $provider } meminta anda memperlahankan permintaan. Cuba lagi sebentar nanti.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } meminta anda memperlahankan permintaan: { $message }
assistant-problem-refused = { $model } enggan menjawab.
assistant-problem-unavailable = { $provider } sedang sibuk atau menghadapi masalah. Cuba lagi sebentar nanti.
assistant-problem-unreachable = plaa tidak dapat menghubungi { $provider }. Semak sambungan anda, atau pastikan pelayan sedang berjalan.
assistant-running = Sedang berjalan…
assistant-send = Hantar
assistant-shortened = Sembang terlalu panjang untuk dibaca oleh { $model } sekali gus, jadi hasil alat yang terdahulu telah ditinggalkan. Pembantu mungkin melihat semula apa yang telah dilihatnya.
assistant-stop = Henti
assistant-stopped = Dihentikan.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Kini bercakap dengan { $model }.
assistant-thinking = Berfikir…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Pemikiran
assistant-title = Pembantu
settings-assistant-add = Tambah model
settings-assistant-add-button = Tambah
# $model is the model's name.
settings-assistant-added = { $model } telah menjawab dan ditambah.
settings-assistant-added-tag = Ditambah
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Alamat, contohnya { $example }
settings-assistant-change-key = Tukar kunci
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Pelayan yang serasi dengan OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Konteks
settings-assistant-context-note = Konteks yang lebih besar membolehkan pembantu mengingati lebih banyak bahagian sembang, tetapi model menggunakan lebih banyak memori dan mungkin menjawab dengan lebih perlahan.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Buka halaman
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } sedang berjalan di { $address }.
settings-assistant-found-none = { $provider } belum mempunyai model. Muat turun satu dengannya, kemudian cari semula.
settings-assistant-get-key = Dapatkan kunci API
settings-assistant-get-server = Dapatkan { $provider }
settings-assistant-in-use = Sedang digunakan
settings-assistant-key = Kunci API
# $error is what the keychain said.
settings-assistant-key-failed = Kunci tidak dapat disimpan dalam storan kelayakan: { $error }
settings-assistant-key-kept = Kunci { $provider } anda disimpan dalam storan kelayakan sistem.
settings-assistant-key-needed = Model ini memerlukan kunci API.
settings-assistant-key-refused = { $provider } menolak kunci ini. Pastikan kunci disalin sepenuhnya, daripada akaun yang betul.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Buat kunci di laman web { $provider } dan tampalkannya di sini.
settings-assistant-look-again = Cari semula
settings-assistant-looking = Mencari model…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, contohnya { $example }
settings-assistant-no-server = plaa tidak menemui { $provider } yang berjalan pada komputer ini. Mulakannya, atau berikan alamatnya di bawah.
settings-assistant-no-tools = Tidak boleh menggunakan alat, yang diperlukan oleh pembantu
settings-assistant-none = Belum ada model. Tambah satu di bawah: model setempat, seperti model Ollama, kekal pada komputer ini; model awan memerlukan kunci API daripada penyedianya.
settings-assistant-note = Model yang boleh dihubungi oleh panel pembantu. Kunci disimpan dalam storan kelayakan sistem.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Guna alamat lain
settings-assistant-recommended = Disyorkan
settings-assistant-remove = Alih keluar
settings-assistant-sees = Boleh melihat gambar
settings-assistant-trying = Mencuba…
settings-assistant-use = Guna
settings-assistant-use-key = Teruskan
settings-assistant-uses-tools = Menggunakan alat
