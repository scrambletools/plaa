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
language-name = Čeština

## Common

app-name = plaa
common-cancel = Zrušit
common-close = Zavřít
common-more = Další

## Command line

usage-help =
    Použití: plaa [SOUBOR NEBO ADRESA]...

    Přehrávání videí, zvukových souborů a streamů z adres, jako je
    https://example.com/live.m3u8. Každý se otevře v okně spuštěné
    aplikace plaa, která se v případě potřeby spustí.

    Možnosti:
      -h, --help        Zobrazí tuto nápovědu
      -V, --version     Zobrazí verzi
          --interfaces  Zobrazí kabelová síťová rozhraní a to, zda na nich
                        plaa může odesílat streamy AVB
          --mcp         Poskytuje MCP přes stdin a stdout, aby agenti AI viděli,
                        co spuštěná aplikace plaa přehrává, a mohli ji ovládat

## Start window

start-hint = Otevřete video, zvukový soubor, stream nebo video z webu.
start-open = Otevřít
start-open-address = Otevřít adresu

## Actions, as buttons, tooltips and menus name them

action-open = Otevřít
action-open-address = Otevřít adresu
open-file = Otevřít soubor…
open-address = Otevřít adresu…
action-settings = Nastavení
action-info = Inspektor
action-loop = Smyčka
action-screenshot = Uložit snímek jako obrázek
action-play = Přehrát
action-pause = Pozastavit
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Zpět o { $seconds } sekundu
    [few] Zpět o { $seconds } sekundy
    [many] Zpět o { $seconds } sekundy
   *[other] Zpět o { $seconds } sekund
}
action-forward = { $seconds ->
    [one] Vpřed o { $seconds } sekundu
    [few] Vpřed o { $seconds } sekundy
    [many] Vpřed o { $seconds } sekundy
   *[other] Vpřed o { $seconds } sekund
}
action-mute = Ztlumit
action-unmute = Zrušit ztlumení
action-speed = Rychlost
action-tracks = Zvuk a titulky
action-full-screen = Celá obrazovka
action-playlist = Playlist
action-chapters = Kapitoly
action-picture-in-picture = Obraz v obraze
action-previous = Předchozí
action-next = Další
action-exit-full-screen = Ukončit režim celé obrazovky
action-quality = Kvalita
action-visualizer = Vizualizace
action-network = Síťový zvuk
action-cast = Odeslat

## What a sound file shows under its cover

visualizer-spectrum = Spektrum
visualizer-waveform = Vlnový průběh
visualizer-prism = Hranol
visualizer-cover = Jen obal

## The floating controls

overlay-keep-shown = Ukotvit ovládací prvky
overlay-auto-hide = Zobrazovat ovládací prvky nad obrazem

## Audio and subtitle tracks

tracks-audio = Zvuk
tracks-subtitles = Titulky
tracks-off = Vypnuto
# A track the file gives no name or language, by its number.
tracks-numbered = Stopa { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Jen zvuk

## Inspector

info-title = Inspektor
playlist-title = Playlist
chapters-numbered = Kapitola { $number }
info-file = Soubor
info-name = Název
info-address = Adresa
info-folder = Složka
info-size = Velikost
info-format = Formát
info-duration = Délka
info-bit-rate = Datový tok
info-video = Video
info-audio = Zvuk
info-codec = Kodek
info-dimensions = Rozměry
info-frame-rate = Snímková frekvence
info-frames-per-second = { $rate } snímků/s
info-pixel-format = Formát pixelů
info-decoding = Dekódování
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Grafická karta ({ $api })
info-decoding-software = Procesor
info-sample-rate = Vzorkovací frekvence
info-hertz = { $rate } Hz
info-channels = Kanály
info-language = Jazyk
info-no-sound = Bez zvuku
info-tags = Štítky

## Dialogs

dialog-open = Otevřít
dialog-filter-media = Video a zvuk
dialog-filter-all = Všechny soubory

address-title = Otevřít adresu
address-note = Webová stránka s videem, například stránka videa na YouTube, nebo stream či soubor v síti, například https://example.com/live.m3u8.
address-label = Adresa
address-open = Otevřít
address-recent = Nedávné
address-forget = Odebrat z nedávných

## Notices

notice-dialog-failed = Dialog souborů se nepodařilo zobrazit: { $error }
notice-screenshot-saved = Soubor „{ $name }“ byl uložen.
notice-screenshot-failed = Obrázek se nepodařilo uložit: { $error }
notice-no-sound = Přehrávání bez zvuku: { $reason }
notice-loop-from = Smyčka od { $time }. Stiskněte znovu v místě, kde má skončit.
notice-loop-between = Přehrává se ve smyčce od { $from } do { $to }.
notice-loop-off = Smyčka vypnuta.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Toto vypadá jako webová stránka. plaa přehrává videa z webových stránek pomocí nástroje yt-dlp, který není nainstalovaný.
error-empty-list = V tomto seznamu není nic k přehrání.

## Settings

settings-title = Nastavení
settings-tab-appearance = Vzhled
settings-tab-playback = Přehrávání
settings-tab-network = Síť
settings-tab-language = Jazyk
settings-tab-agents = Agenti
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (vývojové sestavení, { $build })
settings-choose = Vybrat…

settings-appearance-system = Podle systému
settings-appearance-light = Světlý
settings-appearance-dark = Tmavý
settings-colors = Barvy
settings-system-accent = Použít zvýrazňující barvu systému
settings-omarchy-note = Barvy vycházejí ze zvýrazňující barvy motivu „{ $theme }“.
settings-system-accent-note = Barvy vycházejí ze zvýrazňující barvy systému.
settings-system-accent-none = Systém nemá žádnou zvýrazňující barvu, proto plaa používá barvu vybranou níže.
settings-accent-chosen-note = Barvy vycházejí z barvy vybrané níže.
settings-controls = Ovládací prvky
settings-overlay = Plovoucí ovládací prvky
settings-overlay-note = Panel nástrojů a ovládání přehrávání se vznášejí nad obrazem nebo obalem a zasunou se, když se ukazatel nehýbe nebo je mimo okno.
settings-overlay-delay = Skrýt po
settings-overlay-delay-note = Jak dlouho musí ukazatel nehybně zůstat nad přehrávaným obsahem, než se ovládací prvky zasunou.
settings-seconds = { $seconds } s
settings-transparency = Průhlednost
settings-transparency-note = Nakolik obraz prosvítá plovoucími ovládacími prvky.
settings-percent = { $percent } %
settings-corner-radius = Zaoblení rohů
settings-corner-radius-note = Pro dialogy a plovoucí ovládací prvky.
settings-pixels = { $pixels } px
settings-animations = Animace
settings-animations-note = Vysouvací lišty a panely, zvětšující se dialogy a pružná tlačítka.
settings-animations-reduced = Vypnuto, dokud systém požaduje omezení pohybu.

settings-hardware = Používat grafickou kartu
settings-hardware-note = Video se dekóduje a exporty kódují na grafické kartě: spotřebuje se méně energie, velká videa se přehrávají plynule a export je rychlejší. Projeví se u souborů otevřených poté.
settings-audio-device = Zvukový výstup
settings-audio-device-system = Výchozí systémové zařízení
settings-resume = Pokračovat tam, kde jste skončili
settings-resume-note = Soubory se spustí od místa, kde byly naposledy zavřeny.
settings-folder-playlists = Přehrát ostatní soubory ve složce
settings-folder-playlists-note = Po otevření souboru se zobrazí i ostatní soubory z jeho složky a přehrají se po něm.
settings-streams = Streamy a webová videa
settings-stream-quality = Kvalita
settings-stream-quality-note = Pro streamy a webová videa dostupná ve více kvalitách. Projeví se u toho, co otevřete poté.
settings-quality-auto = Automaticky, podle výšky obrazovky
settings-quality-best = Nejlepší
settings-web-pages = Přehrávat webová videa pomocí yt-dlp
settings-web-pages-found = yt-dlp najde video na stránkách, jako je YouTube.
settings-web-pages-missing = yt-dlp není nainstalovaný. Nainstalujte ho, abyste mohli přehrávat videa ze stránek, jako je YouTube.
settings-folders = Složky
settings-recordings = Nahrávky
settings-screenshots = Uložené snímky

settings-interface-language = Jazyk rozhraní
# The first entry of the language list.
settings-language-system = Podle systému
settings-track-languages = Jazyky stop
settings-track-languages-note = Kódy jazyků, například en nebo fra, seřazené od nejžádanějšího a oddělené čárkami. plaa vybere zvuk a titulky v těchto jazycích, pokud je soubor obsahuje.
settings-audio-languages = Zvuk
settings-subtitle-languages = Titulky

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Rozhraní AVB
settings-avb-interface-note = Kabelový port, na kterém plaa hledá vysílače a přijímače AVB a Milan. Odesílání vyžaduje port s hardwarovými hodinami PTP, například port karty Intel I210.
settings-avb-off = Vypnuto
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (jen výpis: bez hodin PTP)
settings-avb-off-mac = { $interface } (jen výpis: AVB je vypnuté)
settings-avb-interface-note-mac = Kabelový port, na kterém plaa hledá vysílače a přijímače AVB a Milan. Odesílá se přes vlastní entitu AVB tohoto Macu, která vyžaduje zapnout AVB pro daný port v Prohlížeči síťových zařízení v aplikaci Nastavení Audio MIDI.
settings-avb-talker-note-mac = plaa odesílá zvuk okna, které přehrávalo naposledy, přes vlastní entitu AVB tohoto Macu přijímačům, které jsou k ní připojené.
settings-avb-talker = Odesílat přehrávaný zvuk
settings-avb-talker-note = plaa se stane vysílačem Milan a odesílá zvuk okna, které přehrávalo naposledy, přijímačům, které jsou k němu připojené.
settings-avb-name = Název v síti
settings-avb-name-note = Řídicí aplikace zobrazují plaa jako „{ $name }“. Stisknutím klávesy Enter ho přejmenujete.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa na { $computer }
avb-could-not-start = Rozhraní { $interface } se nepodařilo použít: { $error }
network-title = Síťový zvuk
network-off = Chcete-li přehrávat v síti, zapněte v Nastavení volbu Odesílat přehrávaný zvuk.
network-blocked = Neodesílá se: { $reason }
network-waiting = Připraveno k odesílání, čeká se na připojení přijímače.
network-sending = Odesílá se připojeným přijímačům.
network-looking = Hledání přijímačů…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Spouští se ptp4l, který udržuje síťový čas, podle kterého se odesílá stream plaa…
network-ptp-elsewhere = ptp4l běží na jiném rozhraní, takže stream plaa zde neudržuje síťový čas a přijímače ho nemusí přehrát. Vyberte v Nastavení to rozhraní, nebo spusťte ptp4l na tomto.
network-ptp-missing = ptp4l neběží, takže stream plaa neudržuje síťový čas a přijímače ho nemusí přehrát. Nainstalujte linuxptp s balíčkem plaa, nebo triib, který ho spouští.
network-ptp-failed = ptp4l neběží, takže stream plaa neudržuje síťový čas a přijímače ho nemusí přehrát. ptp4l se nepodařilo spustit: { $reason }
network-no-listeners = Žádný přijímač v síti nemůže přehrát zvuk z plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Přehrát na přijímači { $listener }
network-stop-on = Zastavit přehrávání na přijímači { $listener }
network-choose-format = Vyberte formát pro přehrávání na přijímači { $listener }
network-failed = Nepodařilo se připojit: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Chcete-li přehrávat zde, aktualizujte triib.
network-choose = Vyberte přijímač a uvidíte, jak přijímá svůj stream.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Poslouchat zde
network-no-talkers = Žádný vysílač v síti neodesílá ve formátu, který plaa dokáže přehrát.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Poslouchat vysílač { $talker }
network-listen-stop = Přestat poslouchat vysílač { $talker }
network-listen-frames = Přijaté rámce: { $frames }
network-listen-update-triib = Chcete-li poslouchat zde, aktualizujte triib: plaa přehrává jen to, co k tomuto počítači dorazí i tak.
network-listen-lite = Port používá AVB Lite, který nerezervuje žádné streamy: plaa přehrává jen to, co k tomuto počítači dorazí i tak.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Stav
network-not-connected = Nepřipojeno
network-from = Zdroj: { $stream }
network-from-receiving = Zdroj: { $stream }, příjem probíhá
network-from-waiting = Zdroj: { $stream }, čeká se na vysílač
network-from-failed = Zdroj: { $stream }, rezervace vysílače selhala: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Mediální hodiny
network-locked = Synchronizováno se streamem
network-not-locked = Nesynchronizováno
network-format = Formát
# The formats both the listener and plaa's stream can be in.
network-formats = Formáty, které může přijímat z plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID streamu
# The MAC address the stream is sent to.
network-destination = Cíl
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latence
network-microseconds = { $time } µs
network-frames = Přijaté rámce
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Rámce mimo pořadí
# How often the stream stopped coming while it was taken.
network-interrupted = Přerušení
network-entity-id = ID entity

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = nedostatečná šířka pásma
msrp-failure-2 = nedostatečné prostředky mostu
msrp-failure-3 = nedostatečná šířka pásma pro třídu provozu
msrp-failure-4 = ID streamu používá jiný vysílač
msrp-failure-5 = cílová adresa se již používá
msrp-failure-6 = vytlačeno streamem s vyšší prioritou
msrp-failure-7 = hlášená latence se změnila
msrp-failure-8 = výstupní port nepodporuje AVB
msrp-failure-9 = použijte jinou cílovou adresu
msrp-failure-10 = došly prostředky MSRP
msrp-failure-11 = došly prostředky MMRP
msrp-failure-12 = cílovou adresu nelze uložit
msrp-failure-13 = priorita není prioritou třídy SR
msrp-failure-14 = rámce jsou pro médium příliš velké
msrp-failure-15 = dosažen limit vstupních portů (fan-in)
msrp-failure-16 = u registrovaného streamu se změnila první hodnota
msrp-failure-17 = síť VLAN je na výstupním portu blokována
msrp-failure-18 = značkování VLAN je na výstupním portu vypnuté
msrp-failure-19 = neshoda priority třídy SR
msrp-failure-unknown = neznámý důvod
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, na mostu { $bridge }

## The macOS menu bar

menu-about = O aplikaci plaa
menu-settings = Nastavení…
menu-services = Služby
menu-hide = Skrýt plaa
menu-hide-others = Skrýt ostatní
menu-show-all = Zobrazit vše
menu-quit = Ukončit plaa
menu-file = Soubor
menu-open = Otevřít…
menu-open-address = Otevřít adresu…
menu-close = Zavřít okno
menu-edit = Úpravy
menu-cut = Vyjmout
menu-copy = Kopírovat
menu-paste = Vložit
menu-select-all = Vybrat vše
menu-undo = Zpět
menu-redo = Znovu
menu-trim = Oříznout…
menu-split = Rozdělit klip
menu-rotate-left = Otočit doleva
menu-rotate-right = Otočit doprava
menu-flip-horizontal = Převrátit vodorovně
menu-flip-vertical = Převrátit svisle
menu-revert = Vrátit k originálu
menu-export = Exportovat…
menu-view = Zobrazení
menu-playback = Přehrávání
menu-play-pause = Přehrát nebo pozastavit
menu-to-start = Přejít na začátek
menu-previous-chapter = Předchozí kapitola
menu-next-chapter = Další kapitola
menu-faster = Rychleji
menu-slower = Pomaleji
menu-normal-speed = Normální rychlost
menu-ab-loop = Smyčka mezi dvěma body
menu-volume-up = Hlasitěji
menu-volume-down = Tišeji
menu-mute = Ztlumit nebo zrušit ztlumení
menu-next-subtitles = Další titulky
menu-next-audio = Další zvuková stopa
menu-window = Okno
menu-minimize = Minimalizovat
menu-zoom = Zvětšit/zmenšit
menu-bring-all-to-front = Přenést vše do popředí

## Editing

action-edit = Upravit
title-edited = { $name } (upraveno)
edit-trim = Oříznout
edit-trim-apply = Oříznout
edit-split = Rozdělit klip
edit-remove-clip = Odstranit klip
edit-done = Hotovo
edit-rotate-left = Otočit doleva
edit-rotate-right = Otočit doprava
edit-flip-horizontal = Převrátit vodorovně
edit-flip-vertical = Převrátit svisle
edit-undo = Zpět
edit-redo = Znovu
edit-revert = Vrátit k originálu
edit-export = Exportovat…

## Exporting

export-title = Exportovat
export-format = Formát
export-size = Velikost
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Jen zvuk
export-size-original = { $lines }p, beze změny
export-on-graphics-card = Kóduje se na grafické kartě ({ $api })
export-on-processor = Kóduje se na procesoru, což trvá déle
export-no-encoder = Pro tento formát zde není dostupný žádný kodér
export-checking = Hledání kodéru…
export-sound-format = Formát zvuku
export-quality = Kvalita
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Komprimovaný; přehraje se téměř všude, včetně zařízení Apple
export-mp3-note = Komprimovaný; přehraje se všude
export-opus-note = Komprimovaný; nejlepší zvuk vzhledem k velikosti, ale ne každý přehrávač ho podporuje
export-flac-note = Bezeztrátový: zvuk přesně takový, jaký je, v otevřeném formátu
export-alac-note = Bezeztrátový: zvuk přesně takový, jaký je, ve formátu zařízení Apple
export-wav-note = Nekomprimovaný: zvuk přesně takový, jaký je, v největších souborech
export-kilobits = { $rate } kb/s
export-bits = { $bits } bitů
export-rate-lower = Uloží se se vzorkovací frekvencí { $rate } kHz, nižší než původních { $file } kHz
export-choose = Exportovat…
export-name = { $name } (export)
export-name-edited = { $name } (upraveno)
export-progress = Exportování souboru „{ $name }“
export-stop = Zastavit export
notice-exported = Soubor „{ $name }“ byl exportován
notice-export-stopped = Export byl zastaven
notice-export-failed = Nepodařilo se exportovat: { $error }
close-title = Zavřít bez exportu?
close-unexported = Změny souboru „{ $name }“ budou ztraceny, pokud je neexportujete.
close-exporting = Export souboru „{ $name }“ se zastaví a to, co se dosud zapsalo, bude ztraceno.
close-anyway = Zavřít
close-quit-anyway = Ukončit

## Recording

record-title = Nahrávání zvuku
record-file-name = Zvuková nahrávka
record-start = Nahrávat
record-stop = Zastavit nahrávání
record-no-input = Žádný mikrofon
record-microphone = Mikrofon
record-default-microphone = Výchozí mikrofon systému
record-playing = Zvuk přehrávaný počítačem
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanál
    [few] { $count } kanály
    [many] { $count } kanálu
   *[other] { $count } kanálů
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, ukládá se jako { $saved } kHz
action-record-sound = Nahrát zvuk
open-record-sound = Nová zvuková nahrávka
start-record-sound = Nahrát zvuk
menu-record-sound = Nová zvuková nahrávka
record-screen-title = Nahrávání obrazovky
record-screen-file-name = Záznam obrazovky
record-screen-whole = Obrazovka
record-screen-number = Obrazovka { $number }
record-screen-main = Obrazovka { $number } (hlavní)
record-screens = Obrazovky
record-windows = Okna
record-screen-what = Co nahrávat
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Každý snímek
record-rate-tip = Snímky za sekundu
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
        [one] { $count } zahozený snímek
        [few] { $count } zahozené snímky
        [many] { $count } zahozeného snímku
       *[other] { $count } zahozených snímků
    }
record-screen-window = Okno
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Již se nesdílí
record-screen-again = Vybrat znovu
record-screen-failed = Obrazovku se nepodařilo nahrát
record-sound = Zvuk
record-no-sound = Bez zvuku
# How much louder the input is taken, for a quiet microphone.
record-gain = Zesílení
record-gain-value = +{ $decibels } dB
record-level-automatically = Automaticky upravovat úroveň
record-hide-title = plaa se během nahrávání skryje
record-hide-tray = Chcete-li nahrávání zastavit, klikněte na červenou tečku, kterou plaa zobrazí v oznamovací oblasti, nebo plaa znovu otevřete.
record-hide-reopen = Chcete-li nahrávání zastavit, znovu otevřete plaa.
record-hide-start = Spustit nahrávání
record-tray-title = plaa nahrává obrazovku
record-tray-tip = Kliknutím zastavíte nahrávání
action-record-screen = Nahrát obrazovku
open-record-screen = Nový záznam obrazovky
start-record-screen = Nahrát obrazovku
menu-record-screen = Nový záznam obrazovky
record-camera-title = Nahrávání z kamery
record-camera-file-name = Záznam z kamery
record-camera = Kamera
record-camera-failed = Kameru se nepodařilo otevřít
action-record-camera = Nahrát z kamery
open-record-camera = Nový záznam z kamery
start-record-camera = Nahrát z kamery
menu-record-camera = Nový záznam z kamery
notice-no-camera = Není k dispozici žádná kamera, ze které by šlo nahrávat
notice-recorded = Soubor „{ $name }“ byl uložen do složky { $folder }
notice-record-failed = Nahrávání se nezdařilo: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Povolit vnější ovládání
settings-allow-outside-control-note = Agenti AI, například Claude Code, mohou přes plaa --mcp vidět, co plaa přehrává, a ovládat ho. Před každým novým agentem se plaa zeptá.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Povoleno: { $agents }
settings-forget-agents = Zapomenout
settings-ask-before-note = Zeptat se, než agent:
settings-ask-reading = Uvidí, co se přehrává, nebo obraz
settings-ask-playing = Bude ovládat přehrávání, zvuk nebo režim celé obrazovky
settings-ask-opening = Otevře soubor nebo zavře okno
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Povolit agentovi { $agent } ovládat plaa?
agent-prompt-body = { $agent } žádá o použití vnějšího ovládání plaa, aby viděl, co plaa přehrává, a mohl ho ovládat. Vnější ovládání můžete vypnout v Nastavení.
agent-prompt-allow = Povolit
agent-prompt-deny = Nepovolit
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Povolit agentovi { $agent } vidět, co se zde přehrává?
agent-ask-play = Povolit agentovi { $agent } ovládat zde přehrávání?
agent-ask-open = Povolit agentovi { $agent } otevřít nebo zavřít okno?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } žádá o použití nástroje „{ $tool }“. V Nastavení zvolíte, na co se plaa ptá.

## Casting to a TV or speaker on the network

cast-looking = Hledání televizorů a reproduktorů…
cast-none = Nebyl nalezen žádný televizor ani reproduktor s podporou DLNA.
# $renderer is the TV's or speaker's name.
cast-stop = Zastavit přehrávání na zařízení { $renderer }
notice-casting = Přehrávání na zařízení { $renderer }.
notice-cast-failed = Na zařízení { $renderer } se nepodařilo přehrát: { $reason }
cast-web-page = Webové stránky zatím nelze odesílat: otevřete místo toho vlastní adresu videa.
