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
language-name = Magyar

## Common

app-name = plaa
common-cancel = Mégse
common-close = Bezárás
common-more = Továbbiak

## Command line

usage-help =
    Használat: plaa [FÁJL VAGY CÍM]...

    Videó- és hangfájlok, valamint olyan címeken elérhető adatfolyamok
    lejátszása, mint a https://example.com/live.m3u8. Mindegyik a futó
    plaa egy ablakában nyílik meg; a plaa szükség esetén elindul.

    Kapcsolók:
      -h, --help        A súgó megjelenítése
      -V, --version     A verzió megjelenítése
          --interfaces  A vezetékes csatolók megjelenítése, és hogy a plaa
                        küldhet-e rajtuk AVB-adatfolyamokat
          --mcp         MCP kiszolgálása stdin és stdout útján, hogy MI-ügynökök
                        lássák, mit játszik le a futó plaa, és vezérelhessék

## Start window

start-hint = Nyisson meg egy videó- vagy hangfájlt, egy adatfolyamot vagy egy webes videót.
start-open = Megnyitás
start-open-address = Cím megnyitása
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Csatlakozás

## Actions, as buttons, tooltips and menus name them

action-open = Megnyitás
action-open-address = Cím megnyitása
open-file = Fájl megnyitása…
open-address = Cím megnyitása…
action-settings = Beállítások
action-info = Felügyelő
action-loop = Szakaszismétlés
action-screenshot = Képkocka mentése képként
action-play = Lejátszás
action-pause = Szünet
# $seconds is how far the button moves, such as 10.
action-back = Vissza { $seconds } másodperccel
action-forward = Előre { $seconds } másodperccel
action-mute = Némítás
action-unmute = Némítás feloldása
action-speed = Sebesség
action-tracks = Hang és feliratok
action-full-screen = Teljes képernyő
action-playlist = Lejátszási lista
action-chapters = Fejezetek
action-picture-in-picture = Kép a képben
action-previous = Előző
action-next = Következő
action-exit-full-screen = Kilépés a teljes képernyőből
action-quality = Minőség
action-visualizer = Vizualizáció
action-network = Hálózati hang
action-cast = Átküldés
action-connect = Csatlakozás
action-assistant = Asszisztens

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Hullámforma
visualizer-prism = Prizma
visualizer-cover = Csak a borító

## The floating controls

overlay-keep-shown = A vezérlők rögzítése
overlay-auto-hide = Lebegő vezérlők a kép fölött

## Audio and subtitle tracks

tracks-audio = Hang
tracks-subtitles = Feliratok
tracks-off = Ki
# A track the file gives no name or language, by its number.
tracks-numbered = { $number }. sáv

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Csak hang

## Inspector

info-title = Felügyelő
playlist-title = Lejátszási lista
chapters-numbered = { $number }. fejezet
info-file = Fájl
info-name = Név
info-address = Cím
info-folder = Mappa
info-size = Méret
info-format = Formátum
info-duration = Időtartam
info-bit-rate = Bitráta
info-video = Videó
info-audio = Hang
info-codec = Kodek
info-dimensions = Méretek
info-frame-rate = Képkockasebesség
info-frames-per-second = { $rate } képkocka másodpercenként
info-pixel-format = Képpontformátum
info-decoding = Dekódolás
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Videokártya ({ $api })
info-decoding-software = Processzor
info-sample-rate = Mintavételi frekvencia
info-hertz = { $rate } Hz
info-channels = Csatornák
info-language = Nyelv
info-no-sound = Nincs hang
info-tags = Címkék
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Adó
info-stream = Adatfolyam
# The talker's listeners other than plaa.
info-other-listeners = Többi vevő
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR-osztály
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Az új formátum leválasztja { $count ->
        [one] az adó másik vevőjét.
       *[other] az adó többi { $count } vevőjét.
    }

## Dialogs

dialog-open = Megnyitás
dialog-filter-media = Videó és hang
dialog-filter-all = Minden fájl

address-title = Cím megnyitása
address-note = Videót tartalmazó weboldal, például egy YouTube-videó oldala, vagy egy hálózaton elérhető adatfolyam vagy fájl, például https://example.com/live.m3u8.
address-label = Cím
address-open = Megnyitás
address-recent = Legutóbbiak
address-forget = Eltávolítás a legutóbbiak közül

## Notices

notice-dialog-failed = Nem sikerült megjeleníteni a fájlválasztó ablakot: { $error }
notice-screenshot-saved = Mentve: „{ $name }”.
notice-screenshot-failed = Nem sikerült menteni a képet: { $error }
notice-no-sound = Lejátszás hang nélkül: { $reason }
notice-loop-from = Szakaszismétlés innen: { $time }. Nyomja meg újra ott, ahol véget kell érnie.
notice-loop-between = Ismétlés { $from } és { $to } között.
notice-loop-off = Szakaszismétlés kikapcsolva.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Ez weboldalnak tűnik. A plaa a weboldalakon lévő videókat a yt-dlp segítségével játssza le, de az nincs telepítve.
error-empty-list = Ebben a listában nincs mit lejátszani.

## Settings

settings-title = Beállítások
settings-tab-appearance = Megjelenés
settings-tab-playback = Lejátszás
settings-tab-storage = Tárhely
settings-tab-network = Hálózat
settings-tab-language = Nyelv
settings-tab-agents = Ügynökök
settings-tab-assistant = Asszisztens
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (fejlesztői változat, { $build })
settings-choose = Tallózás…

settings-appearance-system = A rendszer szerint
settings-appearance-light = Világos
settings-appearance-dark = Sötét
settings-colors = Színek
settings-system-accent = A rendszer kiemelőszínének használata
settings-omarchy-note = A színek az Omarchy-téma („{ $theme }”) kiemelőszínéből készülnek.
settings-system-accent-note = A színek a rendszer kiemelőszínéből készülnek.
settings-system-accent-none = A rendszernek nincs kiemelőszíne, ezért a plaa az alább kiválasztott színt használja.
settings-accent-chosen-note = A színek az alább kiválasztott színből készülnek.
settings-controls = Vezérlők
settings-overlay = Lebegő vezérlők
settings-overlay-note = Az eszköztár és a lejátszásvezérlők a kép vagy a borító fölött lebegnek, és eltűnnek, amíg a mutató nem mozdul, vagy az ablakon kívül van.
settings-overlay-delay = Elrejtés késleltetése
settings-overlay-delay-note = Mennyi ideig kell a mutatónak mozdulatlanul állnia a lejátszott tartalom fölött, mielőtt a vezérlők eltűnnek.
settings-seconds = { $seconds } mp
settings-transparency = Átlátszóság
settings-transparency-note = Mennyire látszik át a kép a lebegő vezérlőkön.
settings-percent = { $percent }%
settings-corner-radius = Sarkok lekerekítése
settings-corner-radius-note = A párbeszédpanelekhez és a lebegő vezérlőkhöz.
settings-pixels = { $pixels } px
settings-animations = Animációk
settings-animations-note = Becsúszó sávok és panelek, kinyíló párbeszédpanelek és rugalmas gombok.
settings-animations-reduced = Kikapcsolva, amíg a rendszer csökkentett mozgást kér.

settings-hardware = A videokártya használata
settings-hardware-note = A videó dekódolása és az exportálás kódolása a videokártyán történik: kevesebb energiát fogyaszt, a nagy videók is akadás nélkül játszhatók le, és az exportálás gyorsabb. A később megnyitott fájlokra érvényes.
settings-audio-device = Hangkimenet
settings-audio-device-system = Rendszer alapértelmezése
settings-resume = Folytatás onnan, ahol abbahagyta
settings-resume-note = A fájlok onnan indulnak, ahol a legutóbbi bezáráskor tartottak.
settings-folder-playlists = A mappa többi fájljának lejátszása
settings-folder-playlists-note = Egy fájl megnyitásakor a mappájában lévő többi fájl is a listára kerül, és utána lejátszódik.
settings-streams = Adatfolyamok és webes videók
settings-stream-quality = Minőség
settings-stream-quality-note = A több minőségben elérhető adatfolyamokhoz és webes videókhoz. A később megnyitottakra érvényes.
settings-quality-auto = Automatikus, a képernyő magasságához igazítva
settings-quality-best = Legjobb
settings-web-pages = Webes videók lejátszása a yt-dlp segítségével
settings-web-pages-found = A yt-dlp megtalálja a videót az olyan oldalakon, mint a YouTube.
settings-web-pages-missing = A yt-dlp nincs telepítve. Telepítse, hogy lejátszhassa az olyan oldalakon lévő videókat, mint a YouTube.
settings-default-app = Alapértelmezett alkalmazás
settings-default-app-label = A fájlokat a plaa nyissa meg
settings-default-app-note = A plaa legyen az az alkalmazás, amely a videó- és hangfájlokat megnyitja.
settings-default-app-note-windows = A Windowsban az alapértelmezett alkalmazások csak a rendszer saját Beállítások alkalmazásában választhatók ki. Ez ott megnyitja a plaa oldalát.
settings-default-app-note-macos = A macOS minden típushoz megerősítést kér: MP4, MOV, MKV, WebM, MP3, M4A, FLAC és WAV.
settings-default-app-status = { $total } fájltípusból { $set } nyílik meg a plaa alkalmazással.
settings-default-app-button = Beállítás alapértelmezettként
settings-default-app-button-windows = Beállítások megnyitása
settings-default-app-no-entry = A plaa asztali bejegyzése (desktop entry) nincs telepítve, így a rendszer nem tud vele fájlokat megnyitni. A plaa csomagból telepíthető.
settings-default-app-no-bundle = Ahhoz, hogy a plaa alapértelmezett legyen, a plaa.app-ból indítsa el.
settings-default-app-failed = Nem sikerült a plaa alkalmazást alapértelmezetté tenni: { $error }
settings-folders = Mappák
settings-sound-recordings = Hangfelvételek
settings-video-recordings = Videófelvételek
settings-screenshots = Mentett képkockák

settings-interface-language = A felület nyelve
# The first entry of the language list.
settings-language-system = A rendszer szerint
settings-track-languages = Sávok nyelve
settings-track-languages-note = Nyelvkódok, például en vagy fra, fontossági sorrendben, vesszővel elválasztva. A plaa ezeken a nyelveken választ hangot és feliratot, ha a fájlban vannak ilyenek.
settings-audio-languages = Hang
settings-subtitle-languages = Feliratok

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-csatoló
settings-avb-interface-note = Az a vezetékes port, amelyen a plaa AVB- és Milan-adókat és -vevőket keres. A küldéshez PTP-hardverórával rendelkező port kell, például egy Intel I210-é.
settings-avb-off = Ki
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (csak listázás: nincs PTP-óra)
settings-avb-off-mac = { $interface } (csak listázás: az AVB ki van kapcsolva)
settings-avb-interface-note-mac = Az a vezetékes port, amelyen a plaa AVB- és Milan-adókat és -vevőket keres. A küldés ennek a Macnek a saját AVB-entitásán keresztül történik, ehhez az Audio MIDI beállítás Hálózati eszközböngészőjében be kell kapcsolni az AVB-t a porton.
settings-avb-talker-note-mac = A plaa a legutóbb lejátszó ablak hangját ennek a Macnek a saját AVB-entitásán keresztül küldi a hozzá csatlakozó vevőknek.
settings-avb-talker = Lejátszott hang küldése
settings-avb-talker-note = A plaa Milan-adóvá válik, és a legutóbb lejátszó ablak hangját a hozzá csatlakozó vevőknek küldi.
settings-avb-name = Név a hálózaton
settings-avb-name-note = A vezérlőkben a plaa „{ $name }” néven jelenik meg. Az átnevezéshez nyomja meg az Enter billentyűt.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa ({ $computer })
avb-could-not-start = A(z) { $interface } csatoló nem használható: { $error }
network-title = Hálózati hang
network-off = A hálózaton való lejátszáshoz kapcsolja be a Beállításokban a Lejátszott hang küldése lehetőséget.
network-blocked = Nincs küldés: { $reason }
network-waiting = Küldésre kész, várakozás egy vevő csatlakozására.
network-sending = Küldés a csatlakozott vevőknek.
network-looking = Vevők keresése…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = A ptp4l indítása; ez tartja a hálózati időt, amelyhez a plaa adatfolyama igazodik…
network-ptp-elsewhere = A ptp4l egy másik csatolón fut, így a plaa adatfolyama itt nem követ hálózati időt, és előfordulhat, hogy a vevők nem tudják lejátszani. Válassza ki azt a csatolót a Beállításokban, vagy futtassa a ptp4l programot ezen a csatolón.
network-ptp-missing = A ptp4l nem fut, így a plaa adatfolyama nem követ hálózati időt, és előfordulhat, hogy a vevők nem tudják lejátszani. Telepítse a linuxptp csomagot a plaa csomagjával együtt, vagy a triib alkalmazást, amely futtatja a ptp4l programot.
network-ptp-failed = A ptp4l nem fut, így a plaa adatfolyama nem követ hálózati időt, és előfordulhat, hogy a vevők nem tudják lejátszani. A plaa nem tudta elindítani: { $reason }
network-no-listeners = A hálózaton egyetlen vevő sem tudja lejátszani a plaa hangját.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Lejátszás itt: { $listener }
network-stop-on = Lejátszás leállítása itt: { $listener }
network-choose-format = Válasszon formátumot a lejátszáshoz itt: { $listener }
network-failed = Nem sikerült kapcsolódni: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Az itteni lejátszáshoz frissítse a triib alkalmazást.
network-choose = Válasszon egy vevőt, hogy lássa, hogyan fogadja az adatfolyamát.
network-no-sources = A hálózaton egyetlen adó sem küld olyan formátumban, amelyet a plaa lejátszik, és nem is tud ilyenre váltani.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Hallgatás leállítása: { $talker }
network-listen-update-triib = Az itteni hallgatáshoz frissítse a triib alkalmazást: a plaa csak azt játssza le, ami amúgy is eljut ehhez a számítógéphez.
network-listen-lite = A port AVB Lite módban fut, amely nem foglal adatfolyamokat: a plaa csak azt játssza le, ami amúgy is eljut ehhez a számítógéphez.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Adók keresése…
# plaa listens through its entity, which exists only while it sends.
connect-off = Az adók itteni hallgatásához kapcsolja be a Beállításokban a Lejátszott hang küldése lehetőséget.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = A plaa itt nem tud hallgatni: { $reason }
connect-sending = Olyan formátumban küld, amelyet a plaa lejátszik
connect-changing = Olyan formátumra vált, amelyet a plaa lejátszik
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, { $format } formátumban
connect-change-cuts-off = { $talker }, { $format } formátumban, ami leválasztja { $count ->
        [one] a másik vevőjét
       *[other] a többi { $count } vevőjét
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Állapot
network-not-connected = Nincs csatlakoztatva
network-from = Forrás: { $stream }
network-from-receiving = Forrás: { $stream }, vétel folyamatban
network-from-waiting = Forrás: { $stream }, várakozás az adóra
network-from-failed = Forrás: { $stream }, az adó foglalása sikertelen: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Médiaóra
network-locked = Szinkronban az adatfolyammal
network-not-locked = Nincs szinkronban
network-format = Formátum
# The formats both the listener and plaa's stream can be in.
network-formats = A plaa adatfolyamából fogadható formátumok
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Adatfolyam-azonosító
# The MAC address the stream is sent to.
network-destination = Cél
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Késleltetés
network-microseconds = { $time } µs
network-frames = Fogadott keretek
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Sorrenden kívüli keretek
# How often the stream stopped coming while it was taken.
network-interrupted = Megszakadások
# Frames that came after, or long before, their time to be played.
network-late = Késve érkezett keretek
network-early = Túl korán érkezett keretek
network-entity-id = Entitásazonosító

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = nincs elég sávszélesség
msrp-failure-2 = nincs elég erőforrás a hídon
msrp-failure-3 = nincs elég sávszélesség a forgalmi osztály számára
msrp-failure-4 = az adatfolyam-azonosítót egy másik adó használja
msrp-failure-5 = a célcím már használatban van
msrp-failure-6 = egy magasabb rangú adatfolyam kiszorította
msrp-failure-7 = a jelentett késleltetés megváltozott
msrp-failure-8 = a kimenő port nem AVB-képes
msrp-failure-9 = használjon másik célcímet
msrp-failure-10 = elfogytak az MSRP-erőforrások
msrp-failure-11 = elfogytak az MMRP-erőforrások
msrp-failure-12 = a célcím nem tárolható
msrp-failure-13 = a prioritás nem SR-osztály prioritása
msrp-failure-14 = a keretek túl nagyok az átviteli közeghez
msrp-failure-15 = a port fan-in korlátja betelt
msrp-failure-16 = egy regisztrált adatfolyam első értéke megváltozott
msrp-failure-17 = a VLAN blokkolva van a kimenő porton
msrp-failure-18 = a VLAN-címkézés le van tiltva a kimenő porton
msrp-failure-19 = az SR-osztály prioritása nem egyezik
msrp-failure-unknown = ismeretlen ok
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, a következő hídnál: { $bridge }

## The macOS menu bar

menu-about = A plaa névjegye
menu-settings = Beállítások…
menu-services = Szolgáltatások
menu-hide = plaa elrejtése
menu-hide-others = Többi elrejtése
menu-show-all = Az összes megjelenítése
menu-quit = Kilépés a plaa alkalmazásból
menu-file = Fájl
menu-open = Megnyitás…
menu-open-address = Cím megnyitása…
menu-close = Ablak bezárása
menu-edit = Szerkesztés
menu-cut = Kivágás
menu-copy = Másolás
menu-paste = Beillesztés
menu-select-all = Az összes kijelölése
menu-undo = Visszavonás
menu-redo = Ismétlés
menu-trim = Vágás…
menu-split = Klip felosztása
menu-rotate-left = Forgatás balra
menu-rotate-right = Forgatás jobbra
menu-flip-horizontal = Vízszintes tükrözés
menu-flip-vertical = Függőleges tükrözés
menu-revert = Visszaállítás az eredetire
menu-export = Exportálás…
menu-view = Nézet
menu-playback = Lejátszás
menu-play-pause = Lejátszás vagy szünet
menu-to-start = Ugrás az elejére
menu-previous-chapter = Előző fejezet
menu-next-chapter = Következő fejezet
menu-faster = Gyorsabban
menu-slower = Lassabban
menu-normal-speed = Normál sebesség
menu-ab-loop = Ismétlés két pont között
menu-volume-up = Hangosabban
menu-volume-down = Halkabban
menu-mute = Némítás be- vagy kikapcsolása
menu-next-subtitles = Következő felirat
menu-next-audio = Következő hangsáv
menu-window = Ablak
menu-minimize = Kis méret
menu-zoom = Nagyítás/kicsinyítés
menu-bring-all-to-front = Az összes előtérbe hozása

## Editing

action-edit = Szerkesztés
title-edited = { $name } (szerkesztve)
edit-trim = Vágás
edit-trim-apply = Vágás
edit-split = Klip felosztása
edit-remove-clip = Klip eltávolítása
edit-done = Kész
edit-rotate-left = Forgatás balra
edit-rotate-right = Forgatás jobbra
edit-flip-horizontal = Vízszintes tükrözés
edit-flip-vertical = Függőleges tükrözés
edit-undo = Visszavonás
edit-redo = Ismétlés
edit-revert = Visszaállítás az eredetire
edit-export = Exportálás…

## Exporting

export-title = Exportálás
export-format = Formátum
export-size = Méret
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Csak hang
export-size-original = { $lines }p, eredeti méret
export-on-graphics-card = Kódolás a videokártyán ({ $api })
export-on-processor = Kódolás a processzoron, ami tovább tart
export-no-encoder = Ehhez a formátumhoz itt nem érhető el kódoló
export-checking = Kódoló keresése…
export-sound-format = Hangformátum
export-quality = Minőség
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Tömörített; szinte mindenhol lejátszható, az Apple eszközein is
export-mp3-note = Tömörített; mindenhol lejátszható
export-opus-note = Tömörített; a méretéhez képest a legjobb hangzás, bár nem minden lejátszó támogatja
export-flac-note = Veszteségmentes: a hang pontosan úgy, ahogy van, nyílt formátumban
export-alac-note = Veszteségmentes: a hang pontosan úgy, ahogy van, ahogyan az Apple eszközei lejátsszák
export-wav-note = Tömörítetlen: a hang pontosan úgy, ahogy van, a legnagyobb fájlokban
export-kilobits = { $rate } kb/s
export-bits = { $bits } bites
export-rate-lower = Mentés { $rate } kHz-en, a fájl { $file } kHz-es értéke alatt
export-choose = Exportálás…
export-name = { $name } (exportált)
export-name-edited = { $name } (szerkesztett)
export-progress = Exportálás: „{ $name }”
export-stop = Exportálás leállítása
notice-exported = Exportálva: „{ $name }”
notice-export-stopped = Az exportálás leállt
notice-export-failed = Nem sikerült exportálni: { $error }
close-title = Bezárja exportálás nélkül?
close-unexported = A(z) „{ $name }” fájl módosításai elvesznek, ha nem exportálja őket.
close-exporting = A(z) „{ $name }” fájl exportálása leáll, és az eddig kiírt rész elvész.
close-anyway = Bezárás
close-quit-anyway = Kilépés

## Recording

record-title = Hangfelvétel
record-file-name = Hangfelvétel
record-start = Felvétel
record-stop = Felvétel leállítása
record-no-input = Nincs mikrofon
record-microphone = Mikrofon
record-default-microphone = A rendszer alapértelmezett mikrofonja
record-playing = Amit a számítógép lejátszik
record-mono = mono
record-stereo = sztereó
record-channels = { $count } csatorna
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, mentés { $saved } kHz-en
action-record-sound = Hang felvétele
open-record-sound = Új hangfelvétel
start-record-sound = Hang felvétele
menu-record-sound = Új hangfelvétel
record-screen-title = Képernyőfelvétel
record-screen-file-name = Képernyőfelvétel
record-screen-whole = Képernyő
record-screen-number = { $number }. képernyő
record-screen-main = { $number }. képernyő (fő)
record-screens = Képernyők
record-windows = Ablakok
record-screen-what = Mit rögzítsen
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Minden képkocka
record-rate-tip = Képkocka másodpercenként
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } kimaradt
record-screen-window = Ablak
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Már nincs megosztva
record-screen-again = Újraválasztás
record-screen-failed = Nem sikerült rögzíteni a képernyőt
record-sound = Hang
record-no-sound = Nincs hang
# How much louder the input is taken, for a quiet microphone.
record-gain = Erősítés
record-gain-value = +{ $decibels } dB
record-level-automatically = Automatikus szintszabályozás
record-hide-title = A plaa felvétel közben elrejtőzik
record-hide-tray = A felvétel leállításához kattintson a piros pontra, amelyet a plaa a rendszertálcára helyez, vagy nyissa meg újra a plaa alkalmazást.
record-hide-reopen = A felvétel leállításához nyissa meg újra a plaa alkalmazást.
record-hide-start = Felvétel indítása
record-tray-title = A plaa rögzíti a képernyőt
record-tray-tip = Kattintson a felvétel leállításához
action-record-screen = Képernyő felvétele
open-record-screen = Új képernyőfelvétel
start-record-screen = Képernyő felvétele
menu-record-screen = Új képernyőfelvétel
record-camera-title = Kamerafelvétel
record-camera-file-name = Kamerafelvétel
record-camera = Kamera
record-camera-failed = Nem sikerült megnyitni a kamerát
record-name-title = A felvétel elnevezése
record-name-note = „{ $name }” néven van mentve ide: { $folder }.
record-name-label = Név
record-name-keep = Megtartás
record-name-rename = Átnevezés
record-name-taken = Már van ott „{ $name }” nevű fájl.
record-name-invalid = A név nem tartalmazhatja ezeket: / \ : * ? " < > |
record-name-failed = Nem sikerült átnevezni: { $error }
action-record-camera = Felvétel kamerával
open-record-camera = Új kamerafelvétel
start-record-camera = Felvétel kamerával
menu-record-camera = Új kamerafelvétel
notice-no-camera = Nincs kamera, amellyel felvételt lehetne készíteni
notice-recorded = „{ $name }” mentve ide: { $folder }
notice-record-failed = Nem sikerült a felvétel: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Külső vezérlés engedélyezése
settings-allow-outside-control-note = Az MI-ügynökök, például a Claude Code, a plaa --mcp segítségével láthatják, mit játszik le a plaa, és vezérelhetik. A plaa minden új ügynök előtt rákérdez.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Engedélyezve: { $agents }
settings-forget-agents = Elfelejtés
settings-ask-before-note = Rákérdezés az ügynök alábbi műveletei előtt:
settings-ask-reading = A lejátszott tartalom vagy a kép megtekintése
settings-ask-playing = A lejátszás, a hang vagy a teljes képernyő vezérlése
settings-ask-opening = Fájl megnyitása vagy ablak bezárása
settings-ask-editing = Fájl szerkesztése vagy exportálása
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Engedélyezi, hogy { $agent } vezérelje a plaa alkalmazást?
agent-prompt-body = { $agent } szeretné használni a plaa külső vezérlését, hogy lássa a lejátszott tartalmat, és vezérelhesse az alkalmazást. A külső vezérlést a Beállításokban kapcsolhatja ki.
agent-prompt-allow = Engedélyezés
agent-prompt-deny = Tiltás
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Engedélyezi, hogy { $agent } lássa az itt lejátszott tartalmat?
agent-ask-play = Engedélyezi, hogy { $agent } vezérelje az itteni lejátszást?
agent-ask-open = Engedélyezi, hogy { $agent } megnyisson vagy bezárjon egy ablakot?
agent-ask-edit = Engedélyezi, hogy { $agent } itt szerkesszen vagy exportáljon?
agent-working = { $agent } dolgozik…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } szeretné használni ezt: „{ $tool }”. A Beállításokban választhatja ki, mire kérdez rá a plaa.

## Casting to a TV or speaker on the network

cast-looking = Tévék és hangszórók keresése…
cast-none = Nem található tévé, hangszóró vagy Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Lejátszás leállítása itt: { $renderer }
notice-casting = Lejátszás itt: { $renderer }.
notice-cast-failed = A(z) { $renderer } eszközön nem sikerült a lejátszás: { $reason }
cast-web-page = Weboldalakat még nem lehet átküldeni: inkább a videó saját címét nyissa meg.

## The assistant panel, and its models in Settings

assistant-add-another = Modell hozzáadása…
assistant-add-model = Az asszisztensnek modell kell: felhőalapú modell az API-kulcsával, vagy helyi modell.
assistant-ask = Kérdezzen a lejátszott tartalomról
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = A beszélgetés túl hosszú kezdett lenni ahhoz, hogy a(z) { $model } egyszerre elolvassa, ezért a korábbi részét összefoglaltuk. Az asszisztens talán újra megnézi, amit látott.
assistant-compacting = A korábbi beszélgetés összefoglalása
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } ezen a számítógépen
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } itt: { $host }
assistant-new-chat = Új csevegés
assistant-no-model = Előbb adjon hozzá egy modellt a Beállításokban.
assistant-open-settings = Modell hozzáadása
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = A beszélgetés már nem fér bele abba, amit { $model } egyszerre el tud olvasni. Kezdjen új csevegést, vagy válasszon olyan modellt, amely többet tud olvasni.
assistant-problem-credit = { $provider } szerint a fióknak nincs kreditje. Új fiók esetén előbb kreditet kell vásárolni a szolgáltató webhelyén ({ $provider }), csak utána működik a kulcs; ezután próbálja újra.
assistant-problem-key = { $provider } elutasította az API-kulcsot. Ellenőrizze a Beállításokban.
assistant-problem-model = { $provider } nem ismer { $model } nevű modellt. Ellenőrizze a nevét a Beállításokban.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } nem válaszolt: { $message }
assistant-problem-rate = { $provider } lassítást kér. Próbálja újra egy kis idő múlva.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } lassítást kér: { $message }
assistant-problem-refused = { $model } nem volt hajlandó válaszolni.
assistant-problem-unavailable = { $provider } túlterhelt, vagy problémái vannak. Próbálja újra egy kis idő múlva.
assistant-problem-unreachable = A plaa nem tudott kapcsolódni a szolgáltatóhoz: { $provider }. Ellenőrizze a kapcsolatot, vagy hogy fut-e a kiszolgáló.
assistant-running = Fut…
assistant-send = Küldés
assistant-shortened = A beszélgetés túl hosszú volt ahhoz, hogy a(z) { $model } egyszerre elolvassa, ezért a korábbi eszközeredmények kimaradtak. Az asszisztens talán újra megnézi, amit látott.
assistant-stop = Leállítás
assistant-stopped = Leállítva.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Most ezzel a modellel beszélget: { $model }.
assistant-thinking = Gondolkodik…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Gondolatok
assistant-title = Asszisztens
settings-assistant-add = Modell hozzáadása
settings-assistant-add-button = Hozzáadás
# $model is the model's name.
settings-assistant-added = { $model } válaszolt, és hozzá lett adva.
settings-assistant-added-tag = Hozzáadva
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Cím, például { $example }
settings-assistant-change-key = Kulcs módosítása
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-kompatibilis kiszolgáló
# The menu of how much a local model reads at once.
settings-assistant-context = Kontextus
settings-assistant-context-note = Nagyobb kontextussal az asszisztens egy csevegésből többet tarthat észben, de a modell több memóriát foglal, és lassabban válaszolhat.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } ezer token
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Oldal megnyitása
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } fut ezen a címen: { $address }.
settings-assistant-found-none = { $provider }: még nincs modell. Töltsön le vele egyet, majd keressen újra.
settings-assistant-get-key = API-kulcs beszerzése
settings-assistant-get-server = { $provider } letöltése
settings-assistant-in-use = Használatban
settings-assistant-key = API-kulcs
# $error is what the keychain said.
settings-assistant-key-failed = A kulcsot nem sikerült a kulcstartóba menteni: { $error }
settings-assistant-key-kept = A kulcsát ({ $provider }) a rendszer kulcstartója őrzi.
settings-assistant-key-needed = Ehhez a modellhez API-kulcs kell.
settings-assistant-key-refused = { $provider } elutasította ezt a kulcsot. Ellenőrizze, hogy teljes egészében és a megfelelő fiókból másolta-e ki.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Hozzon létre egy kulcsot a szolgáltató webhelyén ({ $provider }), és illessze be ide.
settings-assistant-look-again = Keresés újra
settings-assistant-looking = Modellek keresése…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modell, például { $example }
settings-assistant-no-server = A plaa nem talált futó { $provider }-kiszolgálót ezen a számítógépen. Indítsa el, vagy adja meg lent a címét.
settings-assistant-no-tools = Nem tud eszközöket használni, pedig az asszisztensnek erre szüksége van
settings-assistant-none = Még nincs modell. Adjon hozzá egyet lent: a helyi modell, például egy Ollama-modell, ezen a számítógépen marad; a felhőalapú modellhez API-kulcs kell a szolgáltatójától.
settings-assistant-note = Azok a modellek, amelyekkel az asszisztenspanel beszélgethet. A kulcsokat a rendszer kulcstartója őrzi.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Másik cím használata
settings-assistant-recommended = Ajánlott
settings-assistant-remove = Eltávolítás
settings-assistant-sees = Képeket is lát
settings-assistant-trying = Próba…
settings-assistant-use = Használat
settings-assistant-use-key = Tovább
settings-assistant-uses-tools = Eszközöket használ
