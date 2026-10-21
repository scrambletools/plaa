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
language-name = Türkçe

## Common

app-name = plaa
common-cancel = Vazgeç
common-close = Kapat
common-more = Diğer

## Command line

usage-help =
    Kullanım: plaa [DOSYA VEYA ADRES]...

    Video ve ses dosyalarını ve https://example.com/live.m3u8 gibi
    adreslerdeki akışları oynatın. Her biri, çalışan plaa'nın bir
    penceresinde açılır; plaa gerekirse başlatılır.

    Seçenekler:
      -h, --help        Bu yardımı göster
      -V, --version     Sürümü göster
          --interfaces  Kablolu arabirimleri ve plaa'nın bunlar üzerinden
                        AVB akışı gönderip gönderemeyeceğini göster
          --mcp         MCP'yi stdin ve stdout üzerinden sun; yapay zekâ ajanları
                        çalışan plaa'nın ne oynattığını görüp onu denetleyebilsin

## Start window

start-hint = Bir video veya ses dosyası, bir akış ya da bir web videosu açın.
start-open = Aç
start-open-address = Adres Aç

## Actions, as buttons, tooltips and menus name them

action-open = Aç
action-open-address = Adres Aç
open-file = Dosya aç…
open-address = Adres aç…
action-settings = Ayarlar
action-info = Denetçi
action-loop = Döngü
action-screenshot = Kareyi Resim Olarak Kaydet
action-play = Oynat
action-pause = Duraklat
# $seconds is how far the button moves, such as 10.
action-back = { $seconds } saniye geri
action-forward = { $seconds } saniye ileri
action-mute = Sesi Kapat
action-unmute = Sesi Aç
action-speed = Hız
action-tracks = Ses ve Altyazılar
action-full-screen = Tam Ekran
action-playlist = Oynatma Listesi
action-chapters = Bölümler
action-picture-in-picture = Resim İçinde Resim
action-previous = Önceki
action-next = Sonraki
action-exit-full-screen = Tam Ekrandan Çık
action-quality = Kalite
action-visualizer = Görselleştirici
action-network = Ağ Sesi
action-cast = Yayınla

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Dalga biçimi
visualizer-prism = Prizma
visualizer-cover = Yalnızca kapak

## The floating controls

overlay-keep-shown = Denetimleri sabit tut
overlay-auto-hide = Denetimleri görüntünün üzerinde yüzdür

## Audio and subtitle tracks

tracks-audio = Ses
tracks-subtitles = Altyazılar
tracks-off = Kapalı
# A track the file gives no name or language, by its number.
tracks-numbered = Parça { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Yalnızca ses

## Inspector

info-title = Denetçi
playlist-title = Oynatma Listesi
chapters-numbered = Bölüm { $number }
info-file = Dosya
info-name = Ad
info-address = Adres
info-folder = Klasör
info-size = Boyut
info-format = Biçim
info-duration = Süre
info-bit-rate = Bit hızı
info-video = Video
info-audio = Ses
info-codec = Kodek
info-dimensions = Boyutlar
info-frame-rate = Kare hızı
info-frames-per-second = Saniyede { $rate } kare
info-pixel-format = Piksel biçimi
info-decoding = Kod çözme
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Ekran kartı ({ $api })
info-decoding-software = İşlemci
info-sample-rate = Örnekleme hızı
info-hertz = { $rate } Hz
info-channels = Kanallar
info-language = Dil
info-no-sound = Ses yok
info-tags = Etiketler

## Dialogs

dialog-open = Aç
dialog-filter-media = Video ve ses
dialog-filter-all = Tüm dosyalar

address-title = Adres Aç
address-note = YouTube videosu gibi video içeren bir web sayfası ya da https://example.com/live.m3u8 gibi ağdaki bir akış veya dosya.
address-label = Adres
address-open = Aç
address-recent = Son kullanılanlar
address-forget = Son kullanılanlardan kaldır

## Notices

notice-dialog-failed = Dosya iletişim kutusu açılamadı: { $error }
notice-screenshot-saved = “{ $name }” kaydedildi.
notice-screenshot-failed = Resim kaydedilemedi: { $error }
notice-no-sound = Ses olmadan oynatılıyor: { $reason }
notice-loop-from = Döngü başlangıcı: { $time }. Bitiş noktasında yeniden basın.
notice-loop-between = Döngü: { $from } ile { $to } arası.
notice-loop-off = Döngü kapalı.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Bu bir web sayfasına benziyor. plaa web sayfalarındaki videoları yt-dlp ile oynatır, ancak yt-dlp yüklü değil.
error-empty-list = Bu listede oynatılacak bir şey yok.

## Settings

settings-title = Ayarlar
settings-tab-appearance = Görünüm
settings-tab-playback = Oynatma
settings-tab-network = Ağ
settings-tab-language = Dil
settings-tab-agents = Ajanlar
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (geliştirme sürümü, { $build })
settings-choose = Seç…

settings-appearance-system = Sisteme uy
settings-appearance-light = Açık
settings-appearance-dark = Koyu
settings-colors = Renkler
settings-system-accent = Sistem vurgu rengini kullan
settings-omarchy-note = Renkler “{ $theme }” temasının vurgu renginden oluşturulur.
settings-system-accent-note = Renkler sistemin vurgu renginden oluşturulur.
settings-system-accent-none = Sistemin vurgu rengi yok, bu yüzden plaa aşağıda seçilen rengi kullanıyor.
settings-accent-chosen-note = Renkler aşağıda seçilen renkten oluşturulur.
settings-controls = Denetimler
settings-overlay = Yüzen denetimler
settings-overlay-note = Araç çubuğu ve oynatma denetimleri görüntünün veya kapağın üzerinde durur; işaretçi hareketsiz kaldığında ya da pencerenin dışındayken kayarak gizlenir.
settings-overlay-delay = Gizleme süresi
settings-overlay-delay-note = Denetimler kayarak gizlenmeden önce işaretçinin oynatılan içeriğin üzerinde ne kadar hareketsiz kalacağı.
settings-seconds = { $seconds } sn
settings-transparency = Saydamlık
settings-transparency-note = Yüzen denetimlerin arkasından görüntünün ne kadarının görüneceği.
settings-percent = %{ $percent }
settings-corner-radius = Köşe yarıçapı
settings-corner-radius-note = İletişim kutuları ve yüzen denetimler için.
settings-pixels = { $pixels } px
settings-animations = Animasyonlar
settings-animations-note = Kayan çubuklar ve paneller, büyüyen iletişim kutuları ve yaylanan düğmeler.
settings-animations-reduced = Sistem hareketin azaltılmasını istediğinde kapalıdır.

settings-hardware = Ekran kartını kullan
settings-hardware-note = Videoların kodunu onunla çözer ve dışa aktarmaları onunla kodlar: daha az güç harcar, büyük videoları akıcı oynatır ve daha hızlı dışa aktarır. Bundan sonra açılan dosyalarda geçerli olur.
settings-audio-device = Ses çıkışı
settings-audio-device-system = Sistem varsayılanı
settings-resume = Kaldığınız yerden devam et
settings-resume-note = Dosyalar en son kapatıldıkları yerden başlar.
settings-folder-playlists = Klasördeki diğer dosyaları oynat
settings-folder-playlists-note = Bir dosyayı açmak, klasöründeki diğer dosyaları da listeler; bunlar ondan sonra oynatılır.
settings-streams = Akışlar ve web videoları
settings-stream-quality = Kalite
settings-stream-quality-note = Birkaç kalitede sunulan akışlar ve web videoları için. Bundan sonra açılanlarda geçerli olur.
settings-quality-auto = Otomatik, ekran yüksekliğinde
settings-quality-best = En iyi
settings-web-pages = Web videolarını yt-dlp ile oynat
settings-web-pages-found = yt-dlp, YouTube gibi sayfalardaki videoyu bulur.
settings-web-pages-missing = yt-dlp yüklü değil. YouTube gibi sayfalardaki videoları oynatmak için yükleyin.
settings-folders = Klasörler
settings-recordings = Kayıtlar
settings-screenshots = Kaydedilen kareler

settings-interface-language = Arayüz dili
# The first entry of the language list.
settings-language-system = Sisteme uy
settings-track-languages = Parça dilleri
settings-track-languages-note = en veya fra gibi dil kodları, en çok istenen önce olmak üzere virgülle ayrılmış. Bir dosyada bu dillerde ses ve altyazı varsa plaa onları seçer.
settings-audio-languages = Ses
settings-subtitle-languages = Altyazılar

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB arabirimi
settings-avb-interface-note = plaa'nın AVB ve Milan konuşmacılarını ve dinleyicilerini bulduğu kablolu bağlantı noktası. Göndermek için Intel I210 gibi PTP donanım saati olan bir bağlantı noktası gerekir.
settings-avb-off = Kapalı
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (yalnızca listeler: PTP saati yok)
settings-avb-off-mac = { $interface } (yalnızca listeler: AVB kapalı)
settings-avb-interface-note-mac = plaa'nın AVB ve Milan konuşmacılarını ve dinleyicilerini bulduğu kablolu bağlantı noktası. Gönderme bu Mac'in kendi AVB varlığı üzerinden yapılır; bunun için Audio MIDI Ayarları'ndaki Ağ Aygıtı Tarayıcısı'nda bu bağlantı noktası için AVB'nin açılması gerekir.
settings-avb-talker-note-mac = plaa, en son oynatılan pencerenin sesini bu Mac'in kendi AVB varlığı üzerinden ona bağlı dinleyicilere gönderir.
settings-avb-talker = Oynatılanı gönder
settings-avb-talker-note = plaa bir Milan konuşmacısı olur ve en son oynatılan pencerenin sesini ona bağlı dinleyicilere gönderir.
settings-avb-name = Ağdaki ad
settings-avb-name-note = Denetleyiciler plaa'yı “{ $name }” olarak gösterir. Yeniden adlandırmak için Enter'a basın.
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } üzerindeki plaa
avb-could-not-start = { $interface } kullanılamadı: { $error }
network-title = Ağ Sesi
network-off = Ağda oynatmak için Ayarlar'da Oynatılanı gönder seçeneğini açın.
network-blocked = Gönderilmiyor: { $reason }
network-waiting = Göndermeye hazır, bir dinleyicinin bağlanması bekleniyor.
network-sending = Bağlı dinleyicilere gönderiliyor.
network-looking = Dinleyiciler aranıyor…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = plaa'nın akışının gönderildiği ağ saatini tutan ptp4l başlatılıyor…
network-ptp-elsewhere = ptp4l başka bir arabirimde çalışıyor, bu yüzden plaa'nın akışı burada ağ saatine bağlı değil ve dinleyiciler onu oynatamayabilir. Ayarlar'dan o arabirimi seçin ya da bu arabirimde ptp4l çalıştırın.
network-ptp-missing = ptp4l çalışmıyor, bu yüzden plaa'nın akışı ağ saatine bağlı değil ve dinleyiciler onu oynatamayabilir. linuxptp paketini plaa'nın paketiyle birlikte yükleyin ya da bunu kendisi çalıştıran triib'i yükleyin.
network-ptp-failed = ptp4l çalışmıyor, bu yüzden plaa'nın akışı ağ saatine bağlı değil ve dinleyiciler onu oynatamayabilir. plaa onu başlatamadı: { $reason }
network-no-listeners = Ağda plaa'nın sesini oynatabilecek bir dinleyici yok.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener } üzerinde oynat
network-stop-on = { $listener } üzerinde oynatmayı durdur
network-choose-format = { $listener } üzerinde oynatmak için bir biçim seçin
network-failed = Bağlanılamadı: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Burada oynatmak için triib'i güncelleyin.
network-choose = Akışını nasıl aldığını görmek için bir dinleyici seçin.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Burada dinle
network-no-talkers = Ağda plaa'nın oynatabileceği bir biçimde gönderen konuşmacı yok.
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker } konuşmacısını dinle
network-listen-stop = { $talker } konuşmacısını dinlemeyi durdur
network-listen-frames = Alınan çerçeveler: { $frames }
network-listen-update-triib = Burada dinlemek için triib'i güncelleyin: plaa yalnızca bu bilgisayara zaten ulaşanı oynatır.
network-listen-lite = Bağlantı noktası, hiçbir akışı rezerve etmeyen AVB Lite ile çalışıyor: plaa yalnızca bu bilgisayara zaten ulaşanı oynatır.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Durum
network-not-connected = Bağlı değil
network-from = Kaynak: { $stream }
network-from-receiving = Kaynak: { $stream }, alınıyor
network-from-waiting = Kaynak: { $stream }, konuşmacı bekleniyor
network-from-failed = Kaynak: { $stream }, konuşmacının rezervasyonu başarısız oldu: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Medya saati
network-locked = Akışa kilitli
network-not-locked = Kilitli değil
network-format = Biçim
# The formats both the listener and plaa's stream can be in.
network-formats = plaa'dan alabileceği biçimler
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Akış kimliği
# The MAC address the stream is sent to.
network-destination = Hedef
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Gecikme
network-microseconds = { $time } µs
network-frames = Alınan çerçeveler
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Sırası bozuk çerçeveler
# How often the stream stopped coming while it was taken.
network-interrupted = Kesintiler
network-entity-id = Varlık kimliği

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = yetersiz bant genişliği
msrp-failure-2 = yetersiz köprü kaynakları
msrp-failure-3 = trafik sınıfı için yetersiz bant genişliği
msrp-failure-4 = akış kimliği başka bir konuşmacı tarafından kullanılıyor
msrp-failure-5 = hedef adres zaten kullanımda
msrp-failure-6 = daha yüksek dereceli bir akış öne geçti
msrp-failure-7 = bildirilen gecikme değişti
msrp-failure-8 = çıkış bağlantı noktası AVB uyumlu değil
msrp-failure-9 = farklı bir hedef adres kullanın
msrp-failure-10 = MSRP kaynakları tükendi
msrp-failure-11 = MMRP kaynakları tükendi
msrp-failure-12 = hedef adres saklanamıyor
msrp-failure-13 = öncelik bir SR sınıfı önceliği değil
msrp-failure-14 = çerçeveler ortam için çok büyük
msrp-failure-15 = fan-in bağlantı noktası sınırına ulaşıldı
msrp-failure-16 = kayıtlı bir akış için ilk değer değişti
msrp-failure-17 = VLAN, çıkış bağlantı noktasında engellendi
msrp-failure-18 = çıkış bağlantı noktasında VLAN etiketleme devre dışı
msrp-failure-19 = SR sınıfı önceliği uyuşmuyor
msrp-failure-unknown = bilinmeyen neden
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, { $bridge } köprüsünde

## The macOS menu bar

menu-about = plaa Hakkında
menu-settings = Ayarlar…
menu-services = Hizmetler
menu-hide = plaa'yı Gizle
menu-hide-others = Diğerlerini Gizle
menu-show-all = Tümünü Göster
menu-quit = plaa'dan Çık
menu-file = Dosya
menu-open = Aç…
menu-open-address = Adres Aç…
menu-close = Pencereyi Kapat
menu-edit = Düzen
menu-cut = Kes
menu-copy = Kopyala
menu-paste = Yapıştır
menu-select-all = Tümünü Seç
menu-undo = Geri Al
menu-redo = Yinele
menu-trim = Kırp…
menu-split = Klibi Böl
menu-rotate-left = Sola Döndür
menu-rotate-right = Sağa Döndür
menu-flip-horizontal = Yatay Çevir
menu-flip-vertical = Dikey Çevir
menu-revert = Orijinale Döndür
menu-export = Dışa Aktar…
menu-view = Görüntü
menu-playback = Oynatma
menu-play-pause = Oynat veya Duraklat
menu-to-start = Başa Git
menu-previous-chapter = Önceki Bölüm
menu-next-chapter = Sonraki Bölüm
menu-faster = Daha Hızlı
menu-slower = Daha Yavaş
menu-normal-speed = Normal Hız
menu-ab-loop = İki Nokta Arasında Döngü
menu-volume-up = Sesi Artır
menu-volume-down = Sesi Azalt
menu-mute = Sesi Kapat veya Aç
menu-next-subtitles = Sonraki Altyazı
menu-next-audio = Sonraki Ses Parçası
menu-window = Pencere
menu-minimize = Simge Durumuna Küçült
menu-zoom = Büyüt/Küçült
menu-bring-all-to-front = Tümünü Öne Getir

## Editing

action-edit = Düzenle
title-edited = { $name } (düzenlendi)
edit-trim = Kırp
edit-trim-apply = Kırp
edit-split = Klibi böl
edit-remove-clip = Klibi kaldır
edit-done = Bitti
edit-rotate-left = Sola döndür
edit-rotate-right = Sağa döndür
edit-flip-horizontal = Yatay çevir
edit-flip-vertical = Dikey çevir
edit-undo = Geri al
edit-redo = Yinele
edit-revert = Orijinale döndür
edit-export = Dışa aktar…

## Exporting

export-title = Dışa Aktar
export-format = Biçim
export-size = Boyut
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Yalnızca ses
export-size-original = { $lines }p, olduğu gibi
export-on-graphics-card = Ekran kartında kodlanır ({ $api })
export-on-processor = İşlemcide kodlanır, bu daha uzun sürer
export-no-encoder = Bu biçim için burada kullanılabilen bir kodlayıcı yok
export-checking = Kodlayıcı aranıyor…
export-sound-format = Ses biçimi
export-quality = Kalite
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Sıkıştırılmış; Apple aygıtları dahil hemen her yerde oynatılır
export-mp3-note = Sıkıştırılmış; her yerde oynatılır
export-opus-note = Sıkıştırılmış; boyutuna göre en iyi ses, ancak her oynatıcı desteklemez
export-flac-note = Kayıpsız: ses tam olduğu gibi, açık bir biçimde
export-alac-note = Kayıpsız: ses tam olduğu gibi, Apple aygıtlarının oynattığı biçimde
export-wav-note = Sıkıştırılmamış: ses tam olduğu gibi, en büyük dosyalarla
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = { $rate } kHz olarak kaydedilir, dosyanın { $file } kHz değerinin altında
export-choose = Dışa Aktar…
export-name = { $name } dışa aktarım
export-name-edited = { $name } düzenlenmiş
export-progress = “{ $name }” dışa aktarılıyor
export-stop = Dışa aktarmayı durdur
notice-exported = “{ $name }” dışa aktarıldı
notice-export-stopped = Dışa aktarma durduruldu
notice-export-failed = Dışa aktarılamadı: { $error }
close-title = Dışa aktarmadan kapatılsın mı?
close-unexported = “{ $name }” üzerindeki değişiklikler, dışa aktarmazsanız kaybolur.
close-exporting = “{ $name }” dışa aktarımı durur ve şimdiye kadar yazılanlar kaybolur.
close-anyway = Kapat
close-quit-anyway = Çık

## Recording

record-title = Ses kaydı
record-file-name = Ses kaydı
record-start = Kayıt
record-stop = Kaydı durdur
record-no-input = Mikrofon yok
record-microphone = Mikrofon
record-default-microphone = Sistem varsayılanı
record-playing = Bilgisayarda çalan ses
record-mono = mono
record-stereo = stereo
record-channels = { $count } kanal
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, { $saved } kHz olarak kaydedilir
action-record-sound = Ses kaydet
open-record-sound = Yeni ses kaydı
start-record-sound = Ses Kaydet
menu-record-sound = Yeni Ses Kaydı
record-screen-title = Ekran kaydı
record-screen-file-name = Ekran kaydı
record-screen-whole = Ekran
record-screen-number = Ekran { $number }
record-screen-main = Ekran { $number } (ana)
record-screens = Ekranlar
record-windows = Pencereler
record-screen-what = Kaydedilecek içerik
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Her kare
record-rate-tip = Saniyedeki kare sayısı
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } kare atlandı
record-screen-window = Pencere
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Artık paylaşılmıyor
record-screen-again = Yeniden seç
record-screen-failed = Ekran kaydedilemedi
record-sound = Ses
record-no-sound = Ses yok
# How much louder the input is taken, for a quiet microphone.
record-gain = Yükseltme
record-gain-value = +{ $decibels } dB
record-level-automatically = Düzeyi otomatik ayarla
record-hide-title = plaa kayıt sırasında gizlenir
record-hide-tray = Kaydı durdurmak için plaa'nın sistem tepsisine koyduğu kırmızı noktaya tıklayın ya da plaa'yı yeniden açın.
record-hide-reopen = Kaydı durdurmak için plaa'yı yeniden açın.
record-hide-start = Kaydı başlat
record-tray-title = plaa ekranı kaydediyor
record-tray-tip = Kaydı durdurmak için tıklayın
action-record-screen = Ekranı kaydet
open-record-screen = Yeni ekran kaydı
start-record-screen = Ekranı Kaydet
menu-record-screen = Yeni Ekran Kaydı
record-camera-title = Kamera kaydı
record-camera-file-name = Kamera kaydı
record-camera = Kamera
record-camera-failed = Kamera açılamadı
action-record-camera = Kamerayla kaydet
open-record-camera = Yeni kamera kaydı
start-record-camera = Kamerayla Kaydet
menu-record-camera = Yeni Kamera Kaydı
notice-no-camera = Kayıt yapılacak bir kamera yok
notice-recorded = “{ $name }”, { $folder } klasörüne kaydedildi
notice-record-failed = Kaydedilemedi: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Dışarıdan denetime izin ver
settings-allow-outside-control-note = Claude Code gibi yapay zekâ ajanları, plaa --mcp aracılığıyla plaa'da neyin oynatıldığını görebilir ve onu denetleyebilir. plaa her yeni ajandan önce sorar.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = İzin verilenler: { $agents }
settings-forget-agents = Unut
settings-ask-before-note = Bir ajan şunları yapmadan önce sor:
settings-ask-reading = Oynatılanı veya görüntüyü görme
settings-ask-playing = Oynatmayı, sesi veya tam ekranı denetleme
settings-ask-opening = Dosya açma veya pencere kapatma
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent } plaa'yı denetleyebilsin mi?
agent-prompt-body = { $agent }, neyin oynatıldığını görmek ve onu denetlemek için plaa'nın dışarıdan denetimini kullanmak istiyor. Dışarıdan denetimi Ayarlar'dan kapatabilirsiniz.
agent-prompt-allow = İzin ver
agent-prompt-deny = İzin verme
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent } burada neyin oynatıldığını görebilsin mi?
agent-ask-play = { $agent } buradaki oynatmayı denetleyebilsin mi?
agent-ask-open = { $agent } bir pencere açabilsin veya kapatabilsin mi?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent }, “{ $tool }” aracını kullanmak istiyor. plaa'nın neleri soracağını Ayarlar'dan seçebilirsiniz.

## Casting to a TV or speaker on the network

cast-looking = TV'ler ve hoparlörler aranıyor…
cast-none = DLNA destekli TV veya hoparlör bulunamadı.
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer } üzerinde oynatmayı durdur
notice-casting = { $renderer } üzerinde oynatılıyor.
notice-cast-failed = { $renderer } üzerinde oynatılamadı: { $reason }
cast-web-page = Web sayfaları henüz yayınlanamıyor: bunun yerine videonun kendi adresini açın.
