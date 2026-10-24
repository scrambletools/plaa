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
language-name = Kiswahili

## Common

app-name = plaa
common-cancel = Ghairi
common-close = Funga
common-more = Zaidi

## Command line

usage-help =
    Matumizi: plaa [FILE OR ADDRESS]...

    Cheza faili za video na sauti, na mitiririko kwenye anwani kama
    https://example.com/live.m3u8. Kila moja hufunguka katika dirisha la
    plaa inayoendeshwa, ambayo huanza ikihitajika.

    Chaguo:
      -h, --help        Onyesha msaada huu
      -V, --version     Onyesha toleo
          --interfaces  Onyesha violesura vya waya, na kama plaa inaweza
                        kutuma mitiririko ya AVB kupitia hivyo
          --mcp         Toa MCP kupitia stdin na stdout, ili mawakala wa AI
                        waone kile ambacho plaa inayoendeshwa inacheza na
                        waidhibiti

## Start window

start-hint = Fungua faili ya video au sauti, mtiririko au video ya wavuti.
start-open = Fungua
start-open-address = Fungua Anwani
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Unganisha

## Actions, as buttons, tooltips and menus name them

action-open = Fungua
action-open-address = Fungua Anwani
open-file = Fungua faili…
open-address = Fungua anwani…
action-settings = Mipangilio
action-info = Kikaguzi
action-loop = Kitanzi
action-screenshot = Hifadhi Fremu kama Picha
action-play = Cheza
action-pause = Sitisha
# $seconds is how far the button moves, such as 10.
action-back = Rudi nyuma sekunde { $seconds }
action-forward = Songa mbele sekunde { $seconds }
action-mute = Zima sauti
action-unmute = Washa sauti
action-speed = Kasi
action-tracks = Sauti na Manukuu
action-full-screen = Skrini Nzima
action-playlist = Orodha ya Kucheza
action-chapters = Sura
action-picture-in-picture = Picha Ndani ya Picha
action-previous = Iliyotangulia
action-next = Inayofuata
action-exit-full-screen = Toka kwenye Skrini Nzima
action-quality = Ubora
action-visualizer = Taswira ya Sauti
action-network = Sauti ya Mtandao
action-cast = Tuma
action-connect = Unganisha
action-assistant = Msaidizi

## What a sound file shows under its cover

visualizer-spectrum = Wigo
visualizer-waveform = Umbo la wimbi
visualizer-prism = Prizimu
visualizer-cover = Jalada pekee

## The floating controls

overlay-keep-shown = Weka vidhibiti mahali pake
overlay-auto-hide = Vidhibiti vielee juu ya picha

## Audio and subtitle tracks

tracks-audio = Sauti
tracks-subtitles = Manukuu
tracks-off = Imezimwa
# A track the file gives no name or language, by its number.
tracks-numbered = Mkondo { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Sauti pekee

## Inspector

info-title = Kikaguzi
playlist-title = Orodha ya Kucheza
chapters-numbered = Sura { $number }
info-file = Faili
info-name = Jina
info-address = Anwani
info-folder = Folda
info-size = Ukubwa
info-format = Umbizo
info-duration = Muda
info-bit-rate = Kasi ya biti
info-video = Video
info-audio = Sauti
info-codec = Kodeki
info-dimensions = Vipimo
info-frame-rate = Kasi ya fremu
info-frames-per-second = fremu { $rate } kwa sekunde
info-pixel-format = Umbizo la pikseli
info-decoding = Usimbuaji
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Kadi ya michoro ({ $api })
info-decoding-software = Kichakataji
info-sample-rate = Kasi ya sampuli
info-hertz = { $rate } Hz
info-channels = Chaneli
info-language = Lugha
info-no-sound = Hakuna sauti
info-tags = Lebo
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Mtiririko
# The talker's listeners other than plaa.
info-other-listeners = Listener nyingine
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Daraja la SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Umbizo jipya litatenganisha { $count ->
        [one] listener nyingine ya talker.
       *[other] listener nyingine { $count } za talker.
    }

## Dialogs

dialog-open = Fungua
dialog-filter-media = Video na sauti
dialog-filter-all = Faili zote

address-title = Fungua Anwani
address-note = Ukurasa wa wavuti wenye video, kama ukurasa wa video ya YouTube, au mtiririko au faili kwenye mtandao, kama https://example.com/live.m3u8.
address-label = Anwani
address-open = Fungua
address-recent = Za hivi karibuni
address-forget = Ondoa kwenye za hivi karibuni

## Notices

notice-dialog-failed = Kidirisha cha faili kimeshindwa kufunguka: { $error }
notice-screenshot-saved = “{ $name }” imehifadhiwa.
notice-screenshot-failed = Imeshindwa kuhifadhi picha: { $error }
notice-no-sound = Inacheza bila sauti: { $reason }
notice-loop-from = Kitanzi kuanzia { $time }. Bonyeza tena mahali kinapopaswa kuishia.
notice-loop-between = Kitanzi kutoka { $from } hadi { $to }.
notice-loop-off = Kitanzi kimezimwa.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Huu unaonekana kuwa ukurasa wa wavuti. plaa hucheza video za kurasa za wavuti kwa yt-dlp, ambayo haijasakinishwa.
error-empty-list = Orodha hii haina chochote cha kucheza.

## Settings

settings-title = Mipangilio
settings-tab-appearance = Mwonekano
settings-tab-playback = Uchezaji
settings-tab-storage = Hifadhi
settings-tab-network = Mtandao
settings-tab-language = Lugha
settings-tab-agents = Mawakala
settings-tab-assistant = Msaidizi
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (toleo la usanidi, { $build })
settings-choose = Chagua…

settings-appearance-system = Fuata mfumo
settings-appearance-light = Angavu
settings-appearance-dark = Giza
settings-colors = Rangi
settings-system-accent = Tumia rangi ya msisitizo ya mfumo
settings-omarchy-note = Rangi zinatokana na rangi ya msisitizo ya “{ $theme }”.
settings-system-accent-note = Rangi zinatokana na rangi ya msisitizo ya mfumo.
settings-system-accent-none = Mfumo hauna rangi ya msisitizo, kwa hivyo plaa inatumia rangi iliyochaguliwa hapa chini.
settings-accent-chosen-note = Rangi zinatokana na rangi iliyochaguliwa hapa chini.
settings-controls = Vidhibiti
settings-overlay = Vidhibiti vinavyoelea
settings-overlay-note = Upau wa zana na vidhibiti vya uchezaji huelea juu ya picha au jalada, na huteleza mbali kielekezi kinapotulia au kikiwa nje ya dirisha.
settings-overlay-delay = Ficha baada ya
settings-overlay-delay-note = Muda ambao kielekezi hutulia juu ya kinachocheza kabla vidhibiti havijateleza mbali.
settings-seconds = sekunde { $seconds }
settings-transparency = Uwazi
settings-transparency-note = Kiasi cha picha kinachoonekana kupitia vidhibiti vinavyoelea.
settings-percent = { $percent }%
settings-corner-radius = Mviringo wa pembe
settings-corner-radius-note = Kwa vidirisha na vidhibiti vinavyoelea.
settings-pixels = pikseli { $pixels }
settings-animations = Uhuishaji
settings-animations-note = Pau na paneli zinazoteleza, vidirisha vinavyokua na vitufe vinavyodunda.
settings-animations-reduced = Umezimwa wakati mfumo unaomba mwendo uliopunguzwa.

settings-hardware = Tumia kadi ya michoro
settings-hardware-note = Husimbua video na kusimba faili zinazohamishwa kwenye kadi hiyo: hutumia nishati kidogo, hucheza video kubwa kwa ulaini na huhamisha haraka zaidi. Inaanza kufanya kazi kwa faili zitakazofunguliwa baadaye.
settings-audio-device = Utoaji wa sauti
settings-audio-device-system = Chaguomsingi la mfumo
settings-resume = Endelea ulipoachia
settings-resume-note = Faili huanza mahali zilipokuwa zilipofungwa mara ya mwisho.
settings-folder-playlists = Cheza faili nyingine za folda
settings-folder-playlists-note = Kufungua faili moja huorodhesha nyingine zilizo katika folda yake, ambazo hucheza baada yake.
settings-streams = Mitiririko na video za wavuti
settings-stream-quality = Ubora
settings-stream-quality-note = Kwa mitiririko na video za wavuti zinazokuja kwa viwango kadhaa vya ubora. Inaanza kufanya kazi kwa kitakachofunguliwa baadaye.
settings-quality-auto = Otomatiki, urefu sawa na skrini
settings-quality-best = Bora zaidi
settings-web-pages = Cheza video za wavuti kwa yt-dlp
settings-web-pages-found = yt-dlp hupata video kwenye kurasa kama za YouTube.
settings-web-pages-missing = yt-dlp haijasakinishwa. Isakinishe ili kucheza video kwenye kurasa kama za YouTube.
settings-default-app = Programu chaguomsingi
settings-default-app-label = Fungua faili kwa plaa
settings-default-app-note = Fanya plaa iwe programu inayofungua faili za video na sauti.
settings-default-app-note-windows = Windows hukuruhusu kuchagua programu chaguomsingi katika Mipangilio yake tu. Kitufe hiki kinafungua ukurasa wa plaa huko.
settings-default-app-note-macos = macOS hukuomba uthibitishe kila aina: MP4, MOV, MKV, WebM, MP3, M4A, FLAC na WAV.
settings-default-app-status = Aina za faili { $set } kati ya { $total } hufunguka kwa plaa.
settings-default-app-button = Fanya Chaguomsingi
settings-default-app-button-windows = Fungua Mipangilio
settings-default-app-no-entry = Kiingizo cha eneo-kazi cha plaa hakijasakinishwa, kwa hivyo mfumo hauwezi kufungua faili kwacho. Sakinisha plaa kutoka kwa kifurushi.
settings-default-app-no-bundle = Fungua plaa kutoka plaa.app ili uifanye kuwa chaguomsingi.
settings-default-app-failed = Imeshindwa kufanya plaa kuwa chaguomsingi: { $error }
settings-folders = Folda
settings-sound-recordings = Rekodi za sauti
settings-video-recordings = Rekodi za video
settings-screenshots = Fremu zilizohifadhiwa

settings-interface-language = Lugha ya kiolesura
# The first entry of the language list.
settings-language-system = Fuata mfumo
settings-track-languages = Lugha za mikondo
settings-track-languages-note = Misimbo ya lugha kama en au fra, inayotakiwa zaidi kwanza, ikitenganishwa kwa koma. plaa huchagua sauti na manukuu katika lugha hizi faili inapokuwa nazo.
settings-audio-languages = Sauti
settings-subtitle-languages = Manukuu

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Kiolesura cha AVB
settings-avb-interface-note = Mlango wa waya ambao plaa hutumia kupata talker na listener za AVB na Milan. Kutuma kunahitaji mlango wenye saa ya maunzi ya PTP, kama ule wa Intel I210.
settings-avb-off = Imezimwa
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (huorodhesha tu: hakuna saa ya PTP)
settings-avb-off-mac = { $interface } (huorodhesha tu: AVB imezimwa)
settings-avb-interface-note-mac = Mlango wa waya ambao plaa hutumia kupata talker na listener za AVB na Milan. Kutuma hupitia huluki ya AVB ya Mac hii yenyewe, ambayo inahitaji AVB iwashwe kwa mlango huo katika Network Device Browser ya Audio MIDI Setup.
settings-avb-talker-note-mac = plaa hutuma sauti ya dirisha lililocheza mwisho kupitia huluki ya AVB ya Mac hii yenyewe, kwa listener zilizounganishwa nayo.
settings-avb-talker = Tuma kinachocheza
settings-avb-talker-note = plaa inakuwa talker ya Milan, ikituma sauti ya dirisha lililocheza mwisho kwa listener zilizounganishwa nayo.
settings-avb-name = Jina kwenye mtandao
settings-avb-name-note = Controller huonyesha plaa kama “{ $name }”. Bonyeza Enter ili kuipa jina jipya.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa kwenye { $computer }
avb-could-not-start = Haiwezi kutumia { $interface }: { $error }
network-title = Sauti ya Mtandao
network-off = Washa “Tuma kinachocheza” katika Mipangilio ili kucheza kwenye mtandao.
network-blocked = Haitumi: { $reason }
network-waiting = Iko tayari kutuma, inasubiri listener iunganishwe.
network-sending = Inatuma kwa listener zilizounganishwa.
network-looking = Inatafuta listener…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Inawasha ptp4l, inayotunza saa ya mtandao ambayo mtiririko wa plaa hutumwa kwa kuifuata…
network-ptp-elsewhere = ptp4l inaendeshwa kwenye kiolesura kingine, kwa hivyo mtiririko wa plaa hauna saa ya mtandao hapa na huenda listener zisiweze kuucheza. Chagua kiolesura hicho katika Mipangilio, au endesha ptp4l kwenye hiki.
network-ptp-missing = ptp4l haiendeshwi, kwa hivyo mtiririko wa plaa hauna saa ya mtandao na huenda listener zisiweze kuucheza. Sakinisha linuxptp pamoja na kifurushi cha plaa, au triib, inayoiendesha.
network-ptp-failed = ptp4l haiendeshwi, kwa hivyo mtiririko wa plaa hauna saa ya mtandao na huenda listener zisiweze kuucheza. plaa imeshindwa kuiwasha: { $reason }
network-no-listeners = Hakuna listener kwenye mtandao inayoweza kucheza sauti ya plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Cheza kwenye { $listener }
network-stop-on = Acha kucheza kwenye { $listener }
network-choose-format = Chagua umbizo la kucheza kwenye { $listener }
network-failed = Imeshindwa kuunganisha: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Sasisha triib ili kucheza hapa.
network-choose = Chagua listener ili kuona jinsi inavyopokea mtiririko wake.
network-no-sources = Hakuna talker kwenye mtandao inayotuma katika umbizo ambalo plaa hucheza, au inayoweza kubadilika kuwa umbizo hilo.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Acha kusikiliza { $talker }
network-listen-update-triib = Sasisha triib ili kusikiliza hapa: plaa hucheza tu kile ambacho hufika kwenye kompyuta hii vyovyote vile.
network-listen-lite = Mlango unatumia AVB Lite, ambayo haihifadhi mitiririko: plaa hucheza tu kile ambacho hufika kwenye kompyuta hii vyovyote vile.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Inatafuta talker…
# plaa listens through its entity, which exists only while it sends.
connect-off = Washa “Tuma kinachocheza” katika Mipangilio ili kusikiliza talker hapa.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa haiwezi kusikiliza hapa: { $reason }
connect-sending = Zinazotuma katika umbizo ambalo plaa hucheza
connect-changing = Zinazobadilika kuwa umbizo ambalo plaa hucheza
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, katika { $format }
connect-change-cuts-off = { $talker }, katika { $format }, ikitenganisha { $count ->
        [one] listener yake nyingine
       *[other] listener zake nyingine { $count }
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Hali
network-not-connected = Haijaunganishwa
network-from = Kutoka { $stream }
network-from-receiving = Kutoka { $stream }, inapokea
network-from-waiting = Kutoka { $stream }, inasubiri talker
network-from-failed = Kutoka { $stream }, uhifadhi wa talker umeshindwa: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Saa ya media
network-locked = Imepata lock ya mtiririko
network-not-locked = Haijapata lock
network-format = Umbizo
# The formats both the listener and plaa's stream can be in.
network-formats = Umbizo ambazo inaweza kupokea kutoka plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID ya mtiririko
# The MAC address the stream is sent to.
network-destination = Lengwa
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Ucheleweshaji
network-microseconds = { $time } µs
network-frames = Fremu zilizopokelewa
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Fremu zisizofuata mpangilio
# How often the stream stopped coming while it was taken.
network-interrupted = Mikatizo
# Frames that came after, or long before, their time to be played.
network-late = Fremu zilizochelewa
network-early = Fremu zilizofika mapema
network-entity-id = ID ya huluki

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = kipimo data hakitoshi
msrp-failure-2 = rasilimali za swichi hazitoshi
msrp-failure-3 = kipimo data hakitoshi kwa daraja hili la trafiki
msrp-failure-4 = ID ya mtiririko inatumiwa na talker nyingine
msrp-failure-5 = anwani lengwa tayari inatumika
msrp-failure-6 = imetanguliwa na mtiririko wa cheo cha juu zaidi
msrp-failure-7 = ucheleweshaji ulioripotiwa umebadilika
msrp-failure-8 = mlango wa egress hauna uwezo wa AVB
msrp-failure-9 = tumia anwani lengwa nyingine
msrp-failure-10 = rasilimali za MSRP zimeisha
msrp-failure-11 = rasilimali za MMRP zimeisha
msrp-failure-12 = haiwezi kuhifadhi anwani lengwa
msrp-failure-13 = kipaumbele si kipaumbele cha daraja la SR
msrp-failure-14 = fremu ni kubwa mno kwa njia ya upitishaji
msrp-failure-15 = kikomo cha milango ya fan-in kimefikiwa
msrp-failure-16 = thamani ya kwanza imebadilika kwa mtiririko uliosajiliwa
msrp-failure-17 = VLAN imezuiwa kwenye mlango wa egress
msrp-failure-18 = uwekaji lebo wa VLAN umezimwa kwenye mlango wa egress
msrp-failure-19 = kipaumbele cha daraja la SR hakilingani
msrp-failure-unknown = sababu haijulikani
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, kwenye swichi { $bridge }

## The macOS menu bar

menu-about = Kuhusu plaa
menu-settings = Mipangilio…
menu-services = Huduma
menu-hide = Ficha plaa
menu-hide-others = Ficha Nyingine
menu-show-all = Onyesha Zote
menu-quit = Ondoka kwenye plaa
menu-file = Faili
menu-open = Fungua…
menu-open-address = Fungua Anwani…
menu-close = Funga Dirisha
menu-edit = Hariri
menu-cut = Kata
menu-copy = Nakili
menu-paste = Bandika
menu-select-all = Chagua Zote
menu-undo = Tendua
menu-redo = Rudia
menu-trim = Fupisha…
menu-split = Gawanya Klipu
menu-rotate-left = Zungusha Kushoto
menu-rotate-right = Zungusha Kulia
menu-flip-horizontal = Geuza Mlalo
menu-flip-vertical = Geuza Wima
menu-revert = Rejesha Asili
menu-export = Hamisha…
menu-view = Mwonekano
menu-playback = Uchezaji
menu-play-pause = Cheza au Sitisha
menu-to-start = Nenda Mwanzo
menu-previous-chapter = Sura Iliyotangulia
menu-next-chapter = Sura Inayofuata
menu-faster = Kasi Zaidi
menu-slower = Polepole Zaidi
menu-normal-speed = Kasi ya Kawaida
menu-ab-loop = Kitanzi Kati ya Alama Mbili
menu-volume-up = Sauti Juu
menu-volume-down = Sauti Chini
menu-mute = Zima au Washa Sauti
menu-next-subtitles = Manukuu Yanayofuata
menu-next-audio = Mkondo wa Sauti Unaofuata
menu-window = Dirisha
menu-minimize = Punguza Dirisha
menu-zoom = Kuza
menu-bring-all-to-front = Leta Zote Mbele

## Editing

action-edit = Hariri
title-edited = { $name } (imehaririwa)
edit-trim = Fupisha
edit-trim-apply = Fupisha
edit-split = Gawanya klipu
edit-remove-clip = Ondoa klipu
edit-done = Nimemaliza
edit-rotate-left = Zungusha kushoto
edit-rotate-right = Zungusha kulia
edit-flip-horizontal = Geuza mlalo
edit-flip-vertical = Geuza wima
edit-undo = Tendua
edit-redo = Rudia
edit-revert = Rejesha asili
edit-export = Hamisha…

## Exporting

export-title = Hamisha
export-format = Umbizo
export-size = Ukubwa
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Sauti pekee
export-size-original = { $lines }p, kama ilivyo
export-on-graphics-card = Inasimbwa kwenye kadi ya michoro ({ $api })
export-on-processor = Inasimbwa kwenye kichakataji, jambo linalochukua muda zaidi
export-no-encoder = Hakuna kisimbaji cha umbizo hili kinachofunguka hapa
export-checking = Inatafuta kisimbaji…
export-sound-format = Umbizo la sauti
export-quality = Ubora
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Imebanwa; inacheza karibu kila mahali, vikiwemo vifaa vya Apple
export-mp3-note = Imebanwa; inacheza kila mahali
export-opus-note = Imebanwa; sauti bora zaidi kwa ukubwa wake, ingawa si kila kicheza kinaikubali
export-flac-note = Bila upotevu: sauti kama ilivyo kabisa, katika umbizo huria
export-alac-note = Bila upotevu: sauti kama ilivyo kabisa, kama vifaa vya Apple vinavyoicheza
export-wav-note = Bila kubanwa: sauti kama ilivyo kabisa, katika faili kubwa zaidi
export-kilobits = { $rate } kb/s
export-bits = biti { $bits }
export-rate-lower = Inahifadhiwa kwa { $rate } kHz, chini ya { $file } kHz ya faili
export-choose = Hamisha…
export-name = { $name } iliyohamishwa
export-name-edited = { $name } iliyohaririwa
export-progress = Inahamisha “{ $name }”
export-stop = Acha kuhamisha
notice-exported = “{ $name }” imehamishwa
notice-export-stopped = Kuhamisha kumesimamishwa
notice-export-failed = Imeshindwa kuhamisha: { $error }
close-title = Funga bila kuhamisha?
close-unexported = Mabadiliko ya “{ $name }” yatapotea usipoyahamisha.
close-exporting = Kuhamisha “{ $name }” kutasimama, na kilichoandikwa hadi sasa kitapotea.
close-anyway = Funga
close-quit-anyway = Ondoka

## Recording

record-title = Rekodi ya sauti
record-file-name = Rekodi ya sauti
record-start = Rekodi
record-stop = Acha kurekodi
record-no-input = Hakuna maikrofoni
record-microphone = Maikrofoni
record-default-microphone = Chaguomsingi la mfumo
record-playing = Kinachochezwa na kompyuta
record-mono = mono
record-stereo = stereo
record-channels = chaneli { $count }
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, inahifadhiwa kwa { $saved } kHz
action-record-sound = Rekodi sauti
open-record-sound = Rekodi mpya ya sauti
start-record-sound = Rekodi Sauti
menu-record-sound = Rekodi Mpya ya Sauti
record-screen-title = Rekodi ya skrini
record-screen-file-name = Rekodi ya skrini
record-screen-whole = Skrini
record-screen-number = Skrini { $number }
record-screen-main = Skrini { $number } (kuu)
record-screens = Skrini
record-windows = Madirisha
record-screen-what = Cha kurekodi
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Kila fremu
record-rate-tip = Fremu kwa sekunde
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } imerukwa
   *[other] { $size } · { $count } zimerukwa
}
record-screen-window = Dirisha
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Haishirikiwi tena
record-screen-again = Chagua tena
record-screen-failed = Imeshindwa kurekodi skrini
record-sound = Sauti
record-no-sound = Hakuna sauti
# How much louder the input is taken, for a quiet microphone.
record-gain = Ongeza sauti
record-gain-value = +{ $decibels } dB
record-level-automatically = Rekebisha kiwango kiotomatiki
record-hide-title = plaa hujificha inaporekodi
record-hide-tray = Ili kuacha kurekodi, bofya kitone chekundu ambacho plaa huweka kwenye trei ya mfumo, au fungua plaa tena.
record-hide-reopen = Ili kuacha kurekodi, fungua plaa tena.
record-hide-start = Anza kurekodi
record-tray-title = plaa inarekodi skrini
record-tray-tip = Bofya ili kuacha kurekodi
action-record-screen = Rekodi skrini
open-record-screen = Rekodi mpya ya skrini
start-record-screen = Rekodi Skrini
menu-record-screen = Rekodi Mpya ya Skrini
record-camera-title = Rekodi ya kamera
record-camera-file-name = Rekodi ya kamera
record-camera = Kamera
record-camera-failed = Imeshindwa kufungua kamera
record-name-title = Ipe rekodi jina
record-name-note = Imehifadhiwa kama “{ $name }” katika { $folder }.
record-name-label = Jina
record-name-keep = Bakiza
record-name-rename = Badilisha jina
record-name-taken = Tayari kuna faili inayoitwa “{ $name }” hapo.
record-name-invalid = Jina haliwezi kuwa na herufi hizi: / \ : * ? " < > |
record-name-failed = Imeshindwa kubadilisha jina: { $error }
action-record-camera = Rekodi kamera
open-record-camera = Rekodi mpya ya kamera
start-record-camera = Rekodi Kamera
menu-record-camera = Rekodi Mpya ya Kamera
notice-no-camera = Hakuna kamera ya kurekodi
notice-recorded = “{ $name }” imehifadhiwa katika { $folder }
notice-record-failed = Imeshindwa kurekodi: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Ruhusu udhibiti wa nje
settings-allow-outside-control-note = Mawakala wa AI kama Claude Code wanaweza kuona kinachocheza katika plaa na kuidhibiti, kupitia plaa --mcp. plaa huuliza kabla ya kila wakala mpya.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Wanaoruhusiwa: { $agents }
settings-forget-agents = Sahau
settings-ask-before-note = Uliza kwanza wakala anapotaka:
settings-ask-reading = Kuona kinachocheza, au picha
settings-ask-playing = Kudhibiti uchezaji, sauti au skrini nzima
settings-ask-opening = Kufungua faili au kufunga dirisha
settings-ask-editing = Kuhariri faili au kuihamisha
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Ruhusu { $agent } kudhibiti plaa?
agent-prompt-body = { $agent } anaomba kutumia udhibiti wa nje wa plaa, ili kuona kinachocheza na kukidhibiti. Unaweza kuzima udhibiti wa nje katika Mipangilio.
agent-prompt-allow = Ruhusu
agent-prompt-deny = Usiruhusu
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Ruhusu { $agent } kuona kinachocheza hapa?
agent-ask-play = Ruhusu { $agent } kudhibiti uchezaji hapa?
agent-ask-open = Ruhusu { $agent } kufungua au kufunga dirisha?
agent-ask-edit = Ruhusu { $agent } kuhariri au kuhamisha hapa?
agent-working = { $agent } inafanya kazi…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } anaomba kutumia “{ $tool }”. Katika Mipangilio unachagua mambo ambayo plaa huuliza.

## Casting to a TV or speaker on the network

cast-looking = Inatafuta TV na spika…
cast-none = Hakuna TV, spika au Chromecast iliyopatikana.
# $renderer is the TV's or speaker's name.
cast-stop = Acha kucheza kwenye { $renderer }
notice-casting = Inacheza kwenye { $renderer }.
notice-cast-failed = Imeshindwa kucheza kwenye { $renderer }: { $reason }
cast-web-page = Kurasa za wavuti haziwezi kutumwa bado: badala yake fungua anwani ya video yenyewe.

## The assistant panel, and its models in Settings

assistant-add-another = Ongeza modeli…
assistant-add-model = Msaidizi anahitaji modeli: modeli ya wingu pamoja na ufunguo wake wa API, au modeli ya ndani.
assistant-ask = Uliza kuhusu kinachocheza
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Gumzo lilikuwa linakuwa refu mno kwa { $model } kulisoma kwa mara moja, kwa hiyo sehemu yake ya awali imefupishwa. Msaidizi anaweza kuangalia tena alichokiona.
assistant-compacting = Inafupisha sehemu ya awali ya gumzo
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } kwenye kompyuta hii
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } kwenye { $host }
assistant-new-chat = Gumzo jipya
assistant-no-model = Ongeza modeli katika Mipangilio kwanza.
assistant-open-settings = Ongeza modeli
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Mazungumzo hayatoshi tena katika kiasi ambacho { $model } inaweza kusoma kwa wakati mmoja. Anzisha gumzo jipya, au chagua modeli inayoweza kusoma zaidi.
assistant-problem-credit = { $provider } inasema akaunti haina salio. Akaunti mpya inahitaji kununua salio kwenye tovuti ya { $provider } kabla ufunguo wake haujafanya kazi; kisha jaribu tena.
assistant-problem-key = { $provider } imekataa ufunguo wa API. Uangalie katika Mipangilio.
assistant-problem-model = { $provider } haina modeli inayoitwa { $model }. Angalia jina lake katika Mipangilio.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } haikujibu: { $message }
assistant-problem-rate = { $provider } inaomba upunguze kasi. Jaribu tena baada ya muda mfupi.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } inaomba upunguze kasi: { $message }
assistant-problem-refused = { $model } imekataa kujibu.
assistant-problem-unavailable = { $provider } ina shughuli nyingi au ina tatizo. Jaribu tena baada ya muda mfupi.
assistant-problem-unreachable = plaa haikuweza kufikia { $provider }. Angalia muunganisho wako, au kama seva inaendeshwa.
assistant-running = Inaendesha…
assistant-send = Tuma
assistant-shortened = Gumzo lilikuwa refu mno kwa { $model } kulisoma kwa mara moja, kwa hiyo matokeo ya awali ya zana yameachwa. Msaidizi anaweza kuangalia tena alichokiona.
assistant-stop = Simamisha
assistant-stopped = Imesimamishwa.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Sasa unazungumza na { $model }.
assistant-thinking = Inafikiri…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Mawazo
assistant-title = Msaidizi
settings-assistant-add = Ongeza modeli
settings-assistant-add-button = Ongeza
# $model is the model's name.
settings-assistant-added = { $model } imejibu na imeongezwa.
settings-assistant-added-tag = Imeongezwa
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Anwani, kwa mfano { $example }
settings-assistant-change-key = Badilisha ufunguo
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Seva inayooana na OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Muktadha
settings-assistant-context-note = Muktadha mkubwa zaidi humwezesha msaidizi kuzingatia sehemu kubwa zaidi ya gumzo, lakini modeli hutumia kumbukumbu zaidi na huenda ikajibu polepole zaidi.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = Tokeni { $thousands }K
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Fungua ukurasa
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } inaendeshwa kwenye { $address }.
settings-assistant-found-none = { $provider } bado haina modeli. Pakua moja kwa kuitumia, kisha utafute tena.
settings-assistant-get-key = Pata ufunguo wa API
settings-assistant-get-server = Pata { $provider }
settings-assistant-in-use = Inatumika
settings-assistant-key = Ufunguo wa API
# $error is what the keychain said.
settings-assistant-key-failed = Ufunguo haukuweza kuhifadhiwa katika hifadhi ya funguo: { $error }
settings-assistant-key-kept = Ufunguo wako wa { $provider } umehifadhiwa katika hifadhi ya funguo ya mfumo.
settings-assistant-key-needed = Modeli hii inahitaji ufunguo wa API.
settings-assistant-key-refused = { $provider } imekataa ufunguo huu. Hakikisha umenakiliwa wote, kutoka kwa akaunti sahihi.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Tengeneza ufunguo kwenye tovuti ya { $provider } kisha uubandike hapa.
settings-assistant-look-again = Tafuta tena
settings-assistant-looking = Inatafuta modeli…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modeli, kwa mfano { $example }
settings-assistant-no-server = plaa haikupata { $provider } ikiendeshwa kwenye kompyuta hii. Iwashe, au weka anwani yake hapa chini.
settings-assistant-no-tools = Haiwezi kutumia zana, ambazo msaidizi anahitaji
settings-assistant-none = Bado hakuna modeli. Ongeza moja hapa chini: modeli ya ndani, kama ile ya Ollama, hubaki kwenye kompyuta hii; modeli ya wingu inahitaji ufunguo wa API kutoka kwa mtoa huduma wake.
settings-assistant-note = Modeli ambazo kidirisha cha msaidizi kinaweza kuzungumza nazo. Funguo huhifadhiwa katika hifadhi ya funguo ya mfumo.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Tumia anwani nyingine
settings-assistant-recommended = Inapendekezwa
settings-assistant-remove = Ondoa
settings-assistant-sees = Huona picha
settings-assistant-trying = Inajaribu…
settings-assistant-use = Tumia
settings-assistant-use-key = Endelea
settings-assistant-uses-tools = Hutumia zana
