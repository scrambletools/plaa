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
language-name = Bahasa Indonesia

## Common

app-name = plaa
common-cancel = Batal
common-close = Tutup
common-more = Lainnya

## Command line

usage-help =
    Penggunaan: plaa [FILE OR ADDRESS]...

    Putar berkas video dan audio, serta stream di alamat seperti
    https://example.com/live.m3u8. Masing-masing dibuka di jendela plaa
    yang sedang berjalan, yang akan dijalankan bila perlu.

    Opsi:
      -h, --help        Tampilkan bantuan ini
      -V, --version     Tampilkan versi
          --interfaces  Tampilkan antarmuka kabel, dan apakah plaa dapat
                        mengirim stream AVB melaluinya
          --mcp         Sajikan MCP di stdin dan stdout, agar agen AI dapat
                        melihat yang diputar plaa yang sedang berjalan dan
                        mengendalikannya

## Start window

start-hint = Buka berkas video atau audio, stream, atau video web.
start-open = Buka
start-open-address = Buka Alamat
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Hubungkan

## Actions, as buttons, tooltips and menus name them

action-open = Buka
action-open-address = Buka Alamat
open-file = Buka berkas…
open-address = Buka alamat…
action-settings = Pengaturan
action-info = Inspektur
action-loop = Putar Berulang
action-screenshot = Simpan Bingkai sebagai Gambar
action-play = Putar
action-pause = Jeda
# $seconds is how far the button moves, such as 10.
action-back = Mundur { $seconds } detik
action-forward = Maju { $seconds } detik
action-mute = Bisukan
action-unmute = Bunyikan
action-speed = Kecepatan
action-tracks = Audio dan Subtitel
action-full-screen = Layar Penuh
action-playlist = Daftar Putar
action-chapters = Bab
action-picture-in-picture = Gambar dalam Gambar
action-previous = Sebelumnya
action-next = Berikutnya
action-exit-full-screen = Keluar dari Layar Penuh
action-quality = Kualitas
action-visualizer = Visualisasi
action-network = Audio Jaringan
action-cast = Transmisikan
action-connect = Hubungkan
action-assistant = Asisten

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Bentuk gelombang
visualizer-prism = Prisma
visualizer-cover = Sampul saja

## The floating controls

overlay-keep-shown = Tambatkan kontrol
overlay-auto-hide = Kontrol melayang di atas gambar

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Subtitel
tracks-off = Mati
# A track the file gives no name or language, by its number.
tracks-numbered = Trek { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Audio saja

## Inspector

info-title = Inspektur
playlist-title = Daftar Putar
chapters-numbered = Bab { $number }
info-file = Berkas
info-name = Nama
info-address = Alamat
info-folder = Folder
info-size = Ukuran
info-format = Format
info-duration = Durasi
info-bit-rate = Laju bit
info-video = Video
info-audio = Audio
info-codec = Codec
info-dimensions = Dimensi
info-frame-rate = Laju bingkai
info-frames-per-second = { $rate } bingkai per detik
info-pixel-format = Format piksel
info-decoding = Pendekodean
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Kartu grafis ({ $api })
info-decoding-software = Prosesor
info-sample-rate = Laju sampel
info-hertz = { $rate } Hz
info-channels = Kanal
info-language = Bahasa
info-no-sound = Tanpa suara
info-tags = Tag
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = Listener lain
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Kelas SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Format baru memutus { $count ->
       *[other] { $count } listener lain milik talker.
    }

## Dialogs

dialog-open = Buka
dialog-filter-media = Video dan audio
dialog-filter-all = Semua berkas

address-title = Buka Alamat
address-note = Halaman web berisi video, seperti halaman video YouTube, atau stream atau berkas di jaringan, seperti https://example.com/live.m3u8.
address-label = Alamat
address-open = Buka
address-recent = Terbaru
address-forget = Hapus dari daftar terbaru

## Notices

notice-dialog-failed = Dialog berkas tidak dapat dibuka: { $error }
notice-screenshot-saved = “{ $name }” telah disimpan.
notice-screenshot-failed = Gambar tidak dapat disimpan: { $error }
notice-no-sound = Memutar tanpa suara: { $reason }
notice-loop-from = Putar berulang dari { $time }. Tekan lagi di titik akhirnya.
notice-loop-between = Memutar berulang dari { $from } sampai { $to }.
notice-loop-off = Putar berulang mati.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Ini tampaknya halaman web. plaa memutar video di halaman web dengan yt-dlp, yang belum terpasang.
error-empty-list = Daftar ini tidak berisi apa pun untuk diputar.

## Settings

settings-title = Pengaturan
settings-tab-appearance = Tampilan
settings-tab-playback = Pemutaran
settings-tab-storage = Penyimpanan
settings-tab-network = Jaringan
settings-tab-language = Bahasa
settings-tab-agents = Agen
settings-tab-assistant = Asisten
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (versi pengembangan, { $build })
settings-choose = Pilih…

settings-appearance-system = Ikuti sistem
settings-appearance-light = Terang
settings-appearance-dark = Gelap
settings-colors = Warna
settings-system-accent = Gunakan warna aksen sistem
settings-omarchy-note = Warna dibuat dari aksen “{ $theme }”.
settings-system-accent-note = Warna dibuat dari warna aksen sistem.
settings-system-accent-none = Sistem tidak memiliki warna aksen, sehingga plaa memakai warna yang dipilih di bawah.
settings-accent-chosen-note = Warna dibuat dari warna yang dipilih di bawah.
settings-controls = Kontrol
settings-overlay = Kontrol melayang
settings-overlay-note = Bilah alat dan kontrol pemutaran melayang di atas gambar atau sampul, lalu bergeser menyingkir saat penunjuk diam atau berada di luar jendela.
settings-overlay-delay = Sembunyikan setelah
settings-overlay-delay-note = Berapa lama penunjuk diam di atas yang sedang diputar sebelum kontrol bergeser menyingkir.
settings-seconds = { $seconds } dtk
settings-transparency = Transparansi
settings-transparency-note = Seberapa banyak gambar yang terlihat menembus kontrol melayang.
settings-percent = { $percent }%
settings-corner-radius = Radius sudut
settings-corner-radius-note = Untuk dialog dan kontrol melayang.
settings-pixels = { $pixels } px
settings-animations = Animasi
settings-animations-note = Bilah dan panel yang bergeser, dialog yang membesar, dan tombol yang memantul.
settings-animations-reduced = Nonaktif selama sistem meminta gerakan dikurangi.

settings-hardware = Gunakan kartu grafis
settings-hardware-note = Mendekode video dan mengodekan hasil ekspor dengan kartu grafis: lebih hemat daya, memutar video besar dengan lancar, dan mengekspor lebih cepat. Berlaku untuk berkas yang dibuka setelahnya.
settings-audio-device = Keluaran suara
settings-audio-device-system = Bawaan sistem
settings-resume = Lanjutkan dari posisi terakhir
settings-resume-note = Berkas dimulai dari posisinya saat terakhir ditutup.
settings-folder-playlists = Putar berkas lain di folder
settings-folder-playlists-note = Membuka satu berkas akan mendaftarkan berkas lain di foldernya, yang diputar setelahnya.
settings-streams = Stream dan video web
settings-stream-quality = Kualitas
settings-stream-quality-note = Untuk stream dan video web yang tersedia dalam beberapa kualitas. Berlaku untuk yang dibuka setelahnya.
settings-quality-auto = Otomatis, setinggi layar
settings-quality-best = Terbaik
settings-web-pages = Putar video web dengan yt-dlp
settings-web-pages-found = yt-dlp menemukan video di halaman seperti YouTube.
settings-web-pages-missing = yt-dlp belum terpasang. Pasang yt-dlp untuk memutar video di halaman seperti YouTube.
settings-default-app = Aplikasi bawaan
settings-default-app-label = Buka berkas dengan plaa
settings-default-app-note = Jadikan plaa sebagai aplikasi yang membuka berkas video dan suara.
settings-default-app-note-windows = Windows hanya mengizinkan pemilihan aplikasi bawaan di Pengaturannya sendiri. Tombol ini membuka halaman plaa di sana.
settings-default-app-note-macos = macOS meminta Anda mengonfirmasi setiap jenis: MP4, MOV, MKV, WebM, MP3, M4A, FLAC, dan WAV.
settings-default-app-status = { $set } dari { $total } jenis berkas dibuka dengan plaa.
settings-default-app-button = Jadikan Bawaan
settings-default-app-button-windows = Buka Pengaturan
settings-default-app-no-entry = Entri desktop plaa belum terpasang, sehingga sistem tidak dapat membuka berkas dengannya. Pasang plaa dari paket.
settings-default-app-no-bundle = Buka plaa dari plaa.app untuk menjadikannya aplikasi bawaan.
settings-default-app-failed = Tidak dapat menjadikan plaa sebagai aplikasi bawaan: { $error }
settings-folders = Folder
settings-sound-recordings = Rekaman suara
settings-video-recordings = Rekaman video
settings-screenshots = Bingkai tersimpan

settings-interface-language = Bahasa antarmuka
# The first entry of the language list.
settings-language-system = Ikuti sistem
settings-track-languages = Bahasa trek
settings-track-languages-note = Kode bahasa seperti en atau fra, yang paling diinginkan lebih dulu, dipisahkan koma. plaa memilih audio dan subtitel dalam bahasa ini jika berkas memilikinya.
settings-audio-languages = Audio
settings-subtitle-languages = Subtitel

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Antarmuka AVB
settings-avb-interface-note = Port kabel tempat plaa menemukan talker dan listener AVB dan Milan. Untuk mengirim, diperlukan port dengan clock hardware PTP, seperti milik Intel I210.
settings-avb-off = Mati
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (hanya daftar: tanpa clock PTP)
settings-avb-off-mac = { $interface } (hanya daftar: AVB mati)
settings-avb-interface-note-mac = Port kabel tempat plaa menemukan talker dan listener AVB dan Milan. Pengiriman berjalan melalui entitas AVB milik Mac ini sendiri, yang memerlukan AVB dinyalakan untuk port tersebut di Network Device Browser pada Audio MIDI Setup.
settings-avb-talker-note-mac = plaa mengirim suara dari jendela yang terakhir diputar melalui entitas AVB milik Mac ini sendiri, ke listener yang terhubung dengannya.
settings-avb-talker = Kirim yang sedang diputar
settings-avb-talker-note = plaa menjadi talker Milan, yang mengirim suara dari jendela yang terakhir diputar ke listener yang terhubung dengannya.
settings-avb-name = Nama di jaringan
settings-avb-name-note = Controller menampilkan plaa sebagai “{ $name }”. Tekan Enter untuk mengganti namanya.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa di { $computer }
avb-could-not-start = Tidak dapat memakai { $interface }: { $error }
network-title = Audio Jaringan
network-off = Nyalakan “Kirim yang sedang diputar” di Pengaturan untuk memutar di jaringan.
network-blocked = Tidak mengirim: { $reason }
network-waiting = Siap mengirim, menunggu listener terhubung.
network-sending = Mengirim ke listener yang terhubung.
network-looking = Mencari listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Menjalankan ptp4l, yang menjaga waktu jaringan yang menjadi acuan pengiriman stream plaa…
network-ptp-elsewhere = ptp4l berjalan di antarmuka lain, jadi stream plaa tidak memiliki waktu jaringan di sini dan listener mungkin tidak dapat memutarnya. Pilih antarmuka tersebut di Pengaturan, atau jalankan ptp4l di antarmuka ini.
network-ptp-missing = ptp4l tidak berjalan, jadi stream plaa tidak memiliki waktu jaringan dan listener mungkin tidak dapat memutarnya. Pasang linuxptp bersama paket plaa, atau triib, yang menjalankannya.
network-ptp-failed = ptp4l tidak berjalan, jadi stream plaa tidak memiliki waktu jaringan dan listener mungkin tidak dapat memutarnya. plaa tidak dapat menjalankannya: { $reason }
network-no-listeners = Tidak ada listener di jaringan yang dapat memutar suara plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Putar di { $listener }
network-stop-on = Hentikan pemutaran di { $listener }
network-choose-format = Pilih format untuk memutar di { $listener }
network-failed = Tidak dapat terhubung: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Perbarui triib untuk memutar di sini.
network-choose = Pilih listener untuk melihat cara ia menerima stream.
network-no-sources = Tidak ada talker di jaringan yang mengirim dalam format yang diputar plaa, atau dapat beralih ke format tersebut.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Berhenti mendengarkan { $talker }
network-listen-update-triib = Perbarui triib untuk mendengarkan di sini: plaa hanya memutar apa yang memang sampai ke komputer ini.
network-listen-lite = Port menjalankan AVB Lite, yang tidak melakukan reservasi stream: plaa hanya memutar apa yang memang sampai ke komputer ini.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Mencari talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = Nyalakan “Kirim yang sedang diputar” di Pengaturan untuk mendengarkan talker di sini.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa tidak dapat mendengarkan di sini: { $reason }
connect-sending = Mengirim dalam format yang diputar plaa
connect-changing = Beralih ke format yang diputar plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, dalam { $format }
connect-change-cuts-off = { $talker }, dalam { $format }, memutus { $count ->
       *[other] { $count } listener lainnya
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Status
network-not-connected = Tidak terhubung
network-from = Dari { $stream }
network-from-receiving = Dari { $stream }, menerima
network-from-waiting = Dari { $stream }, menunggu talker
network-from-failed = Dari { $stream }, reservasi talker gagal: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Media clock
network-locked = Terkunci ke stream
network-not-locked = Tidak terkunci
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Format yang dapat diterimanya dari plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID stream
# The MAC address the stream is sent to.
network-destination = Tujuan
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latensi
network-microseconds = { $time } µs
network-frames = Frame diterima
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Frame tidak berurutan
# How often the stream stopped coming while it was taken.
network-interrupted = Gangguan
# Frames that came after, or long before, their time to be played.
network-late = Frame terlambat
network-early = Frame terlalu awal
network-entity-id = ID entitas

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = bandwidth tidak cukup
msrp-failure-2 = sumber daya switch tidak cukup
msrp-failure-3 = bandwidth tidak cukup untuk kelas trafik ini
msrp-failure-4 = ID stream dipakai talker lain
msrp-failure-5 = alamat tujuan sudah dipakai
msrp-failure-6 = didahului stream berperingkat lebih tinggi
msrp-failure-7 = latensi yang dilaporkan berubah
msrp-failure-8 = port egress tidak mendukung AVB
msrp-failure-9 = gunakan alamat tujuan lain
msrp-failure-10 = sumber daya MSRP habis
msrp-failure-11 = sumber daya MMRP habis
msrp-failure-12 = tidak dapat menyimpan alamat tujuan
msrp-failure-13 = prioritas bukan prioritas kelas SR
msrp-failure-14 = frame terlalu besar untuk medium
msrp-failure-15 = batas port fan-in tercapai
msrp-failure-16 = nilai pertama berubah untuk stream terdaftar
msrp-failure-17 = VLAN diblokir di port egress
msrp-failure-18 = tagging VLAN dinonaktifkan di port egress
msrp-failure-19 = prioritas kelas SR tidak cocok
msrp-failure-unknown = alasan tidak diketahui
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, di switch { $bridge }

## The macOS menu bar

menu-about = Tentang plaa
menu-settings = Pengaturan…
menu-services = Layanan
menu-hide = Sembunyikan plaa
menu-hide-others = Sembunyikan Lainnya
menu-show-all = Tampilkan Semua
menu-quit = Keluar dari plaa
menu-file = Berkas
menu-open = Buka…
menu-open-address = Buka Alamat…
menu-close = Tutup Jendela
menu-edit = Edit
menu-cut = Potong
menu-copy = Salin
menu-paste = Tempel
menu-select-all = Pilih Semua
menu-undo = Urungkan
menu-redo = Ulangi
menu-trim = Pangkas…
menu-split = Pisahkan Klip
menu-rotate-left = Putar ke Kiri
menu-rotate-right = Putar ke Kanan
menu-flip-horizontal = Balik Horizontal
menu-flip-vertical = Balik Vertikal
menu-revert = Kembalikan ke Asli
menu-export = Ekspor…
menu-view = Tampilan
menu-playback = Pemutaran
menu-play-pause = Putar atau Jeda
menu-to-start = Ke Awal
menu-previous-chapter = Bab Sebelumnya
menu-next-chapter = Bab Berikutnya
menu-faster = Lebih Cepat
menu-slower = Lebih Lambat
menu-normal-speed = Kecepatan Normal
menu-ab-loop = Putar Berulang di Antara Dua Titik
menu-volume-up = Lebih Keras
menu-volume-down = Lebih Pelan
menu-mute = Bisukan atau Bunyikan
menu-next-subtitles = Subtitel Berikutnya
menu-next-audio = Trek Audio Berikutnya
menu-window = Jendela
menu-minimize = Minimalkan
menu-zoom = Maksimalkan
menu-bring-all-to-front = Bawa Semua ke Depan

## Editing

action-edit = Edit
title-edited = { $name } (diedit)
edit-trim = Pangkas
edit-trim-apply = Pangkas
edit-split = Pisahkan klip
edit-remove-clip = Hapus klip
edit-done = Selesai
edit-rotate-left = Putar ke kiri
edit-rotate-right = Putar ke kanan
edit-flip-horizontal = Balik horizontal
edit-flip-vertical = Balik vertikal
edit-undo = Urungkan
edit-redo = Ulangi
edit-revert = Kembalikan ke asli
edit-export = Ekspor…

## Exporting

export-title = Ekspor
export-format = Format
export-size = Ukuran
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Suara saja
export-size-original = { $lines }p, apa adanya
export-on-graphics-card = Dikodekan di kartu grafis ({ $api })
export-on-processor = Dikodekan di prosesor, yang memakan waktu lebih lama
export-no-encoder = Tidak ada encoder untuk format ini yang dapat dibuka di sini
export-checking = Mencari encoder…
export-sound-format = Format suara
export-quality = Kualitas
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Terkompresi; dapat diputar hampir di mana saja, termasuk di perangkat Apple
export-mp3-note = Terkompresi; dapat diputar di mana saja
export-opus-note = Terkompresi; suara terbaik untuk ukurannya, meski tidak semua pemutar mendukungnya
export-flac-note = Tanpa penurunan kualitas: suara persis seperti aslinya, dalam format terbuka
export-alac-note = Tanpa penurunan kualitas: suara persis seperti aslinya, sebagaimana diputar perangkat Apple
export-wav-note = Tanpa kompresi: suara persis seperti aslinya, dengan berkas paling besar
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Disimpan pada { $rate } kHz, di bawah { $file } kHz milik berkas
export-choose = Ekspor…
export-name = { $name } hasil ekspor
export-name-edited = { $name } diedit
export-progress = Mengekspor “{ $name }”
export-stop = Hentikan ekspor
notice-exported = “{ $name }” telah diekspor
notice-export-stopped = Ekspor dihentikan
notice-export-failed = Tidak dapat mengekspor: { $error }
close-title = Tutup tanpa mengekspor?
close-unexported = Perubahan pada “{ $name }” akan hilang kecuali Anda mengekspornya.
close-exporting = Ekspor “{ $name }” akan berhenti, dan yang sudah ditulis sejauh ini akan hilang.
close-anyway = Tutup
close-quit-anyway = Keluar

## Recording

record-title = Rekaman suara
record-file-name = Rekaman suara
record-start = Rekam
record-stop = Hentikan perekaman
record-no-input = Tidak ada mikrofon
record-microphone = Mikrofon
record-default-microphone = Bawaan sistem
record-playing = Yang diputar komputer
record-mono = mono
record-stereo = stereo
record-channels = { $count } kanal
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, disimpan pada { $saved } kHz
action-record-sound = Rekam suara
open-record-sound = Rekaman suara baru
start-record-sound = Rekam Suara
menu-record-sound = Rekaman Suara Baru
record-screen-title = Rekaman layar
record-screen-file-name = Rekaman layar
record-screen-whole = Layar
record-screen-number = Layar { $number }
record-screen-main = Layar { $number } (utama)
record-screens = Layar
record-windows = Jendela
record-screen-what = Yang direkam
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Setiap bingkai
record-rate-tip = Bingkai per detik
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } terlewat
record-screen-window = Jendela
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Tidak lagi dibagikan
record-screen-again = Pilih lagi
record-screen-failed = Tidak dapat merekam layar
record-sound = Suara
record-no-sound = Tanpa suara
# How much louder the input is taken, for a quiet microphone.
record-gain = Lebih keras
record-gain-value = +{ $decibels } dB
record-level-automatically = Atur level otomatis
record-hide-title = plaa bersembunyi selama merekam
record-hide-tray = Untuk menghentikan perekaman, klik titik merah yang ditaruh plaa di baki sistem, atau buka plaa lagi.
record-hide-reopen = Untuk menghentikan perekaman, buka plaa lagi.
record-hide-start = Mulai merekam
record-tray-title = plaa sedang merekam layar
record-tray-tip = Klik untuk menghentikan perekaman
action-record-screen = Rekam layar
open-record-screen = Rekaman layar baru
start-record-screen = Rekam Layar
menu-record-screen = Rekaman Layar Baru
record-camera-title = Rekaman kamera
record-camera-file-name = Rekaman kamera
record-camera = Kamera
record-camera-failed = Tidak dapat membuka kamera
record-name-title = Beri nama rekaman
record-name-note = Disimpan sebagai “{ $name }” di { $folder }.
record-name-label = Nama
record-name-keep = Pertahankan
record-name-rename = Ganti nama
record-name-taken = Sudah ada file bernama “{ $name }” di sana.
record-name-invalid = Nama tidak boleh berisi karakter berikut: / \ : * ? " < > |
record-name-failed = Tidak dapat mengganti nama: { $error }
action-record-camera = Rekam kamera
open-record-camera = Rekaman kamera baru
start-record-camera = Rekam Kamera
menu-record-camera = Rekaman Kamera Baru
notice-no-camera = Tidak ada kamera untuk merekam
notice-recorded = “{ $name }” disimpan di { $folder }
notice-record-failed = Tidak dapat merekam: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Izinkan kendali luar
settings-allow-outside-control-note = Agen AI seperti Claude Code dapat melihat yang diputar di plaa dan mengendalikannya melalui plaa --mcp. plaa akan bertanya sebelum setiap agen baru.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Diizinkan: { $agents }
settings-forget-agents = Lupakan
settings-ask-before-note = Tanya dulu sebelum agen:
settings-ask-reading = Melihat yang diputar, atau gambarnya
settings-ask-playing = Mengendalikan pemutaran, suara, atau layar penuh
settings-ask-opening = Membuka berkas atau menutup jendela
settings-ask-editing = Mengedit berkas atau mengekspornya
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Izinkan { $agent } mengendalikan plaa?
agent-prompt-body = { $agent } meminta untuk memakai kendali luar plaa, untuk melihat yang diputar dan mengendalikannya. Anda dapat menonaktifkan kendali luar di Pengaturan.
agent-prompt-allow = Izinkan
agent-prompt-deny = Jangan izinkan
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Izinkan { $agent } melihat yang diputar di sini?
agent-ask-play = Izinkan { $agent } mengendalikan pemutaran di sini?
agent-ask-open = Izinkan { $agent } membuka atau menutup jendela?
agent-ask-edit = Izinkan { $agent } mengedit atau mengekspor di sini?
agent-working = { $agent } sedang bekerja…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } meminta untuk memakai “{ $tool }”. Pengaturan menentukan apa saja yang ditanyakan plaa.

## Casting to a TV or speaker on the network

cast-looking = Mencari TV dan speaker…
cast-none = Tidak ditemukan TV, speaker, atau Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Hentikan pemutaran di { $renderer }
notice-casting = Memutar di { $renderer }.
notice-cast-failed = Tidak dapat memutar di { $renderer }: { $reason }
cast-web-page = Halaman web belum dapat ditransmisikan: buka alamat video itu sendiri.

## The assistant panel, and its models in Settings

assistant-add-another = Tambahkan model…
assistant-add-model = Asisten memerlukan model: model cloud dengan kunci API-nya, atau model lokal.
assistant-ask = Tanyakan tentang yang sedang diputar
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Obrolan menjadi terlalu panjang untuk dibaca { $model } sekaligus, jadi bagian awalnya diringkas. Asisten mungkin melihat lagi apa yang telah dilihatnya.
assistant-compacting = Meringkas bagian awal obrolan
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } di komputer ini
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } di { $host }
assistant-new-chat = Obrolan baru
assistant-no-model = Tambahkan model di Pengaturan terlebih dahulu.
assistant-open-settings = Tambahkan model
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Percakapan ini tidak lagi muat dalam jumlah yang dapat dibaca { $model } sekaligus. Mulai obrolan baru, atau pilih model yang dapat membaca lebih banyak.
assistant-problem-credit = { $provider } menyatakan akun ini tidak memiliki kredit. Akun baru perlu membeli kredit di situs { $provider } sebelum kuncinya dapat dipakai; setelah itu coba lagi.
assistant-problem-key = { $provider } menolak kunci API. Periksa di Pengaturan.
assistant-problem-model = { $provider } tidak memiliki model bernama { $model }. Periksa namanya di Pengaturan.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } tidak menjawab: { $message }
assistant-problem-rate = { $provider } meminta Anda memperlambat. Coba lagi sebentar lagi.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } meminta Anda memperlambat: { $message }
assistant-problem-refused = { $model } menolak menjawab.
assistant-problem-unavailable = { $provider } sedang sibuk atau mengalami masalah. Coba lagi sebentar lagi.
assistant-problem-unreachable = plaa tidak dapat menghubungi { $provider }. Periksa koneksi Anda, atau pastikan server sedang berjalan.
assistant-running = Berjalan…
assistant-send = Kirim
assistant-shortened = Obrolan terlalu panjang untuk dibaca { $model } sekaligus, jadi hasil alat sebelumnya dihilangkan. Asisten mungkin melihat lagi apa yang telah dilihatnya.
assistant-stop = Hentikan
assistant-stopped = Dihentikan.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Sekarang berbicara dengan { $model }.
assistant-thinking = Berpikir…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Pemikiran
assistant-title = Asisten
settings-assistant-add = Tambahkan model
settings-assistant-add-button = Tambah
# $model is the model's name.
settings-assistant-added = { $model } menjawab dan telah ditambahkan.
settings-assistant-added-tag = Ditambahkan
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Alamat, misalnya { $example }
settings-assistant-change-key = Ganti kunci
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Server yang kompatibel dengan OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Konteks
settings-assistant-context-note = Konteks yang lebih besar membuat asisten dapat mengingat lebih banyak isi obrolan, tetapi model memakai lebih banyak memori dan mungkin menjawab lebih lambat.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Buka halaman
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } berjalan di { $address }.
settings-assistant-found-none = { $provider } belum memiliki model. Unduh satu model dengannya, lalu cari lagi.
settings-assistant-get-key = Dapatkan kunci API
settings-assistant-get-server = Dapatkan { $provider }
settings-assistant-in-use = Sedang dipakai
settings-assistant-key = Kunci API
# $error is what the keychain said.
settings-assistant-key-failed = Kunci tidak dapat disimpan di penyimpanan kredensial: { $error }
settings-assistant-key-kept = Kunci { $provider } Anda disimpan di penyimpanan kredensial sistem.
settings-assistant-key-needed = Model ini memerlukan kunci API.
settings-assistant-key-refused = { $provider } menolak kunci ini. Pastikan kunci disalin utuh, dari akun yang benar.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Buat kunci di situs { $provider } lalu tempelkan di sini.
settings-assistant-look-again = Cari lagi
settings-assistant-looking = Mencari model…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, misalnya { $example }
settings-assistant-no-server = plaa tidak menemukan { $provider } yang berjalan di komputer ini. Jalankan, atau masukkan alamatnya di bawah.
settings-assistant-no-tools = Tidak dapat memakai alat, yang dibutuhkan asisten
settings-assistant-none = Belum ada model. Tambahkan satu di bawah: model lokal, seperti milik Ollama, tetap berada di komputer ini; model cloud memerlukan kunci API dari penyedianya.
settings-assistant-note = Model yang dapat diajak bicara oleh panel asisten. Kunci disimpan di penyimpanan kredensial sistem.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Gunakan alamat lain
settings-assistant-recommended = Disarankan
settings-assistant-remove = Hapus
settings-assistant-sees = Dapat melihat gambar
settings-assistant-trying = Mencoba…
settings-assistant-use = Pakai
settings-assistant-use-key = Lanjutkan
settings-assistant-uses-tools = Memakai alat
