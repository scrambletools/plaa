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
language-name = Suomi

## Common

app-name = plaa
common-cancel = Peruuta
common-close = Sulje
common-more = Lisää

## Command line

usage-help =
    Käyttö: plaa [TIEDOSTO TAI OSOITE]...

    Toista video- ja äänitiedostoja sekä virtoja osoitteista, kuten
    https://example.com/live.m3u8. Kukin avautuu käynnissä olevan plaan
    ikkunaan, ja plaa käynnistyy tarvittaessa.

    Valitsimet:
      -h, --help        Näytä tämä ohje
      -V, --version     Näytä versio
          --interfaces  Näytä langalliset liitännät ja se, voiko plaa
                        lähettää niissä AVB-virtoja
          --mcp         Tarjoa MCP stdinissä ja stdoutissa, jotta tekoälyagentit
                        näkevät, mitä käynnissä oleva plaa toistaa, ja voivat
                        ohjata sitä

## Start window

start-hint = Avaa video- tai äänitiedosto, virta tai verkkovideo.
start-open = Avaa
start-open-address = Avaa osoite
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Yhdistä

## Actions, as buttons, tooltips and menus name them

action-open = Avaa
action-open-address = Avaa osoite
open-file = Avaa tiedosto…
open-address = Avaa osoite…
action-settings = Asetukset
action-info = Tiedot
action-loop = Silmukka
action-screenshot = Tallenna ruutu kuvana
action-play = Toista
action-pause = Keskeytä
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] { $seconds } sekunti taaksepäin
   *[other] { $seconds } sekuntia taaksepäin
}
action-forward = { $seconds ->
    [one] { $seconds } sekunti eteenpäin
   *[other] { $seconds } sekuntia eteenpäin
}
action-mute = Mykistä
action-unmute = Poista mykistys
action-speed = Nopeus
action-tracks = Ääni ja tekstitykset
action-full-screen = Koko näyttö
action-playlist = Soittolista
action-chapters = Luvut
action-picture-in-picture = Kuva kuvassa
action-previous = Edellinen
action-next = Seuraava
action-exit-full-screen = Poistu koko näytön tilasta
action-quality = Laatu
action-visualizer = Visualisointi
action-network = Verkkoääni
action-cast = Suoratoista
action-connect = Yhdistä
action-assistant = Avustaja

## What a sound file shows under its cover

visualizer-spectrum = Spektri
visualizer-waveform = Aaltomuoto
visualizer-prism = Prisma
visualizer-cover = Vain kansikuva

## The floating controls

overlay-keep-shown = Kiinnitä säätimet
overlay-auto-hide = Kelluta säätimiä kuvan päällä

## Audio and subtitle tracks

tracks-audio = Ääni
tracks-subtitles = Tekstitykset
tracks-off = Pois
# A track the file gives no name or language, by its number.
tracks-numbered = Raita { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Vain ääni

## Inspector

info-title = Tiedot
playlist-title = Soittolista
chapters-numbered = Luku { $number }
info-file = Tiedosto
info-name = Nimi
info-address = Osoite
info-folder = Kansio
info-size = Koko
info-format = Tiedostomuoto
info-duration = Kesto
info-bit-rate = Bittinopeus
info-video = Video
info-audio = Ääni
info-codec = Koodekki
info-dimensions = Mitat
info-frame-rate = Kuvataajuus
info-frames-per-second = { $rate } kuvaa sekunnissa
info-pixel-format = Pikselimuoto
info-decoding = Dekoodaus
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Näytönohjain ({ $api })
info-decoding-software = Suoritin
info-sample-rate = Näytteenottotaajuus
info-hertz = { $rate } Hz
info-channels = Kanavat
info-language = Kieli
info-no-sound = Ei ääntä
info-tags = Tunnisteet

## Dialogs

dialog-open = Avaa
dialog-filter-media = Video ja ääni
dialog-filter-all = Kaikki tiedostot

address-title = Avaa osoite
address-note = Verkkosivu, jolla on video, kuten YouTube-videon sivu, tai virta tai tiedosto verkossa, kuten https://example.com/live.m3u8.
address-label = Osoite
address-open = Avaa
address-recent = Viimeisimmät
address-forget = Poista viimeisimmistä

## Notices

notice-dialog-failed = Tiedostoikkunan avaaminen epäonnistui: { $error }
notice-screenshot-saved = Tallennettiin: ”{ $name }”.
notice-screenshot-failed = Kuvan tallentaminen epäonnistui: { $error }
notice-no-sound = Toistetaan ilman ääntä: { $reason }
notice-loop-from = Silmukka alkaa kohdasta { $time }. Paina uudelleen kohdassa, johon sen pitää päättyä.
notice-loop-between = Toistetaan silmukkana { $from } ja { $to } välillä.
notice-loop-off = Silmukka pois.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Tämä näyttää verkkosivulta. plaa toistaa verkkosivujen videot yt-dlp:n avulla, mutta sitä ei ole asennettu.
error-empty-list = Tässä luettelossa ei ole mitään toistettavaa.

## Settings

settings-title = Asetukset
settings-tab-appearance = Ulkoasu
settings-tab-playback = Toisto
settings-tab-network = Verkko
settings-tab-language = Kieli
settings-tab-agents = Agentit
settings-tab-assistant = Avustaja
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (kehitysversio, { $build })
settings-choose = Valitse…

settings-appearance-system = Seuraa järjestelmää
settings-appearance-light = Vaalea
settings-appearance-dark = Tumma
settings-colors = Värit
settings-system-accent = Käytä järjestelmän korostusväriä
settings-omarchy-note = Värit muodostetaan teeman ”{ $theme }” korostusväristä.
settings-system-accent-note = Värit muodostetaan järjestelmän korostusväristä.
settings-system-accent-none = Järjestelmässä ei ole korostusväriä, joten plaa käyttää alla valittua.
settings-accent-chosen-note = Värit muodostetaan alla valitusta väristä.
settings-controls = Säätimet
settings-overlay = Kelluvat säätimet
settings-overlay-note = Työkalupalkki ja toistosäätimet kelluvat kuvan tai kansikuvan päällä ja liukuvat pois, kun osoitin on paikallaan tai ikkunan ulkopuolella.
settings-overlay-delay = Piilotusviive
settings-overlay-delay-note = Kuinka kauan osoitin on paikallaan toistettavan päällä, ennen kuin säätimet liukuvat pois.
settings-seconds = { $seconds } s
settings-transparency = Läpinäkyvyys
settings-transparency-note = Kuinka paljon kuvasta näkyy kelluvien säätimien läpi.
settings-percent = { $percent } %
settings-corner-radius = Kulmien pyöristys
settings-corner-radius-note = Valintaikkunoille ja kelluville säätimille.
settings-pixels = { $pixels } px
settings-animations = Animaatiot
settings-animations-note = Liukuvat palkit ja paneelit, kasvavat valintaikkunat ja joustavat painikkeet.
settings-animations-reduced = Pois käytöstä, kun järjestelmä pyytää vähentämään liikettä.

settings-hardware = Käytä näytönohjainta
settings-hardware-note = Dekoodaa videon ja koodaa viennit näytönohjaimella: kuluttaa vähemmän virtaa, toistaa suuret videot sujuvasti ja vie nopeammin. Koskee tämän jälkeen avattavia tiedostoja.
settings-audio-device = Äänen ulostulo
settings-audio-device-system = Järjestelmän oletus
settings-resume = Jatka siitä, mihin jäit
settings-resume-note = Tiedostot alkavat kohdasta, jossa ne olivat, kun ne viimeksi suljettiin.
settings-folder-playlists = Toista kansion muut tiedostot
settings-folder-playlists-note = Kun avaat yhden tiedoston, saman kansion muut tiedostot lisätään luetteloon ja toistetaan sen jälkeen.
settings-streams = Virrat ja verkkovideot
settings-stream-quality = Laatu
settings-stream-quality-note = Virroille ja verkkovideoille, joita on saatavana useana laatuna. Koskee tämän jälkeen avattavia.
settings-quality-auto = Automaattinen, näytön korkuinen
settings-quality-best = Paras
settings-web-pages = Toista verkkovideot yt-dlp:llä
settings-web-pages-found = yt-dlp löytää videon esimerkiksi YouTuben sivuilta.
settings-web-pages-missing = yt-dlp:tä ei ole asennettu. Asenna se, niin voit toistaa videoita esimerkiksi YouTuben sivuilta.
settings-folders = Kansiot
settings-recordings = Tallenteet
settings-screenshots = Tallennetut ruudut

settings-interface-language = Käyttöliittymän kieli
# The first entry of the language list.
settings-language-system = Seuraa järjestelmää
settings-track-languages = Raitojen kielet
settings-track-languages-note = Kielikoodit, kuten en tai fra, halutuin ensin, pilkuilla erotettuina. plaa valitsee äänen ja tekstitykset näillä kielillä, kun tiedostossa on niitä.
settings-audio-languages = Ääni
settings-subtitle-languages = Tekstitykset

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB-liitäntä
settings-avb-interface-note = Langallinen portti, josta plaa etsii AVB- ja Milan-lähettimiä ja -vastaanottimia. Lähettäminen vaatii portin, jossa on PTP-laitteistokello, kuten Intel I210:ssä.
settings-avb-off = Pois
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (vain selaus: ei PTP-kelloa)
settings-avb-off-mac = { $interface } (vain selaus: AVB on pois päältä)
settings-avb-interface-note-mac = Langallinen portti, josta plaa etsii AVB- ja Milan-lähettimiä ja -vastaanottimia. Lähetys kulkee tämän Macin oman AVB-entiteetin kautta, ja se vaatii, että AVB on otettu portille käyttöön Audio-MIDI-asetusten verkkolaiteselaimessa.
settings-avb-talker-note-mac = plaa lähettää viimeksi toistaneen ikkunan äänen tämän Macin oman AVB-entiteetin kautta siihen yhdistetyille vastaanottimille.
settings-avb-talker = Lähetä toistettava ääni
settings-avb-talker-note = plaasta tulee Milan-lähetin, joka lähettää viimeksi toistaneen ikkunan äänen siihen yhdistetyille vastaanottimille.
settings-avb-name = Nimi verkossa
settings-avb-name-note = Ohjaimet näyttävät plaan nimellä ”{ $name }”. Nimeä uudelleen painamalla Enter.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa ({ $computer })
avb-could-not-start = Liitännän { $interface } käyttäminen epäonnistui: { $error }
network-title = Verkkoääni
network-off = Ota Asetuksissa käyttöön ”Lähetä toistettava ääni”, niin voit toistaa verkossa.
network-blocked = Ei lähetetä: { $reason }
network-waiting = Valmis lähettämään, odotetaan vastaanottimen yhdistämistä.
network-sending = Lähetetään yhdistetyille vastaanottimille.
network-looking = Etsitään vastaanottimia…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Käynnistetään ptp4l, joka ylläpitää verkon aikaa, jonka mukaan plaan virta lähetetään…
network-ptp-elsewhere = ptp4l on käynnissä toisessa liitännässä, joten plaan virralla ei ole täällä verkon aikaa, eivätkä vastaanottimet ehkä pysty toistamaan sitä. Valitse se liitäntä Asetuksissa tai käynnistä ptp4l tässä liitännässä.
network-ptp-missing = ptp4l ei ole käynnissä, joten plaan virralla ei ole verkon aikaa, eivätkä vastaanottimet ehkä pysty toistamaan sitä. Asenna linuxptp plaan paketin mukana tai asenna triib, joka käynnistää sen.
network-ptp-failed = ptp4l ei ole käynnissä, joten plaan virralla ei ole verkon aikaa, eivätkä vastaanottimet ehkä pysty toistamaan sitä. plaa ei voinut käynnistää sitä: { $reason }
network-no-listeners = Mikään verkon vastaanotin ei pysty toistamaan plaan ääntä.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Toista laitteella { $listener }
network-stop-on = Lopeta toisto laitteella { $listener }
network-choose-format = Valitse muoto toistoon laitteella { $listener }
network-failed = Yhdistäminen epäonnistui: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Päivitä triib, niin voit toistaa tässä.
network-choose = Valitse vastaanotin nähdäksesi, miten se ottaa virtansa vastaan.
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = Kuuntele tässä
network-no-sources = Mikään verkon lähetin ei lähetä muodossa, jota plaa toistaa, eikä voi vaihtaa sellaiseen.
# $talker is a talker's name, or its name and its output's.
network-listen-to = Kuuntele lähetintä { $talker }
network-listen-stop = Lopeta lähettimen { $talker } kuuntelu
# $format is a stream format, such as "AAF 48 kHz, 2 ch, 32-bit". The
# talker's other listeners stop, as they do not take that format.
network-listen-change = Vaihda lähetin { $talker } muotoon { $format } ja kuuntele
network-listen-change-cuts-off = Vaihda lähetin { $talker } muotoon { $format } ja kuuntele, jolloin { $count ->
        [one] sen toisen vastaanottimen yhteys katkeaa
       *[other] sen { $count } muun vastaanottimen yhteys katkeaa
    }
network-listen-frames = Vastaanotetut kehykset: { $frames }
network-listen-update-triib = Päivitä triib, niin voit kuunnella tässä: plaa toistaa vain sen, mikä tulee tälle tietokoneelle muutenkin.
network-listen-lite = Portissa on käytössä AVB Lite, joka ei varaa virtoja: plaa toistaa vain sen, mikä tulee tälle tietokoneelle muutenkin.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Etsitään lähettimiä…
# plaa listens through its entity, which exists only while it sends.
connect-off = Ota Asetuksissa käyttöön ”Lähetä toistettava ääni”, niin voit kuunnella lähettimiä tässä.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa ei voi kuunnella tässä: { $reason }
connect-sending = Lähettää muodossa, jota plaa toistaa
connect-changing = Vaihtaa muotoon, jota plaa toistaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, muodossa { $format }
connect-change-cuts-off = { $talker }, muodossa { $format }, jolloin { $count ->
        [one] sen toisen vastaanottimen yhteys katkeaa
       *[other] sen { $count } muun vastaanottimen yhteys katkeaa
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Tila
network-not-connected = Ei yhdistetty
network-from = Lähde: { $stream }
network-from-receiving = Lähde: { $stream }, vastaanotetaan
network-from-waiting = Lähde: { $stream }, odotetaan lähetintä
network-from-failed = Lähde: { $stream }, lähettimen varaus epäonnistui: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Mediakello
network-locked = Lukittu virtaan
network-not-locked = Ei lukittu
network-format = Muoto
# The formats both the listener and plaa's stream can be in.
network-formats = Muodot, joita se voi vastaanottaa plaalta
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Virran tunnus
# The MAC address the stream is sent to.
network-destination = Kohde
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Viive
network-microseconds = { $time } µs
network-frames = Vastaanotetut kehykset
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Kehykset väärässä järjestyksessä
# How often the stream stopped coming while it was taken.
network-interrupted = Katkokset
network-entity-id = Entiteetin tunnus

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = kaistanleveys ei riitä
msrp-failure-2 = kytkimen resurssit eivät riitä
msrp-failure-3 = kaistanleveys ei riitä liikenneluokalle
msrp-failure-4 = toinen lähetin käyttää virran tunnusta
msrp-failure-5 = kohdeosoite on jo käytössä
msrp-failure-6 = korkeamman tason virta syrjäytti
msrp-failure-7 = ilmoitettu viive on muuttunut
msrp-failure-8 = lähtevä portti ei tue AVB:tä
msrp-failure-9 = käytä toista kohdeosoitetta
msrp-failure-10 = MSRP-resurssit loppuivat
msrp-failure-11 = MMRP-resurssit loppuivat
msrp-failure-12 = kohdeosoitetta ei voi tallentaa
msrp-failure-13 = prioriteetti ei ole SR-luokan prioriteetti
msrp-failure-14 = kehykset ovat liian suuria siirtotielle
msrp-failure-15 = fan-in-porttien raja on saavutettu
msrp-failure-16 = rekisteröidyn virran ensimmäinen arvo muuttui
msrp-failure-17 = VLAN on estetty lähtevässä portissa
msrp-failure-18 = VLAN-tunnisteet on poistettu käytöstä lähtevässä portissa
msrp-failure-19 = SR-luokan prioriteetit eivät täsmää
msrp-failure-unknown = tuntematon syy
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, kytkimessä { $bridge }

## The macOS menu bar

menu-about = Tietoja: plaa
menu-settings = Asetukset…
menu-services = Palvelut
menu-hide = Kätke plaa
menu-hide-others = Kätke muut
menu-show-all = Näytä kaikki
menu-quit = Lopeta plaa
menu-file = Arkisto
menu-open = Avaa…
menu-open-address = Avaa osoite…
menu-close = Sulje ikkuna
menu-edit = Muokkaa
menu-cut = Leikkaa
menu-copy = Kopioi
menu-paste = Sijoita
menu-select-all = Valitse kaikki
menu-undo = Kumoa
menu-redo = Tee uudelleen
menu-trim = Rajaa…
menu-split = Jaa leike
menu-rotate-left = Kierrä vasemmalle
menu-rotate-right = Kierrä oikealle
menu-flip-horizontal = Peilaa vaakasuunnassa
menu-flip-vertical = Peilaa pystysuunnassa
menu-revert = Palauta alkuperäinen
menu-export = Vie…
menu-view = Näytä
menu-playback = Toisto
menu-play-pause = Toista tai keskeytä
menu-to-start = Siirry alkuun
menu-previous-chapter = Edellinen luku
menu-next-chapter = Seuraava luku
menu-faster = Nopeammin
menu-slower = Hitaammin
menu-normal-speed = Normaali nopeus
menu-ab-loop = Toista kahden kohdan väliä silmukkana
menu-volume-up = Kovemmalle
menu-volume-down = Hiljemmalle
menu-mute = Mykistä tai poista mykistys
menu-next-subtitles = Seuraavat tekstitykset
menu-next-audio = Seuraava ääniraita
menu-window = Ikkuna
menu-minimize = Pienennä
menu-zoom = Zoomaa
menu-bring-all-to-front = Tuo kaikki eteen

## Editing

action-edit = Muokkaa
title-edited = { $name } (muokattu)
edit-trim = Rajaa
edit-trim-apply = Rajaa
edit-split = Jaa leike
edit-remove-clip = Poista leike
edit-done = Valmis
edit-rotate-left = Kierrä vasemmalle
edit-rotate-right = Kierrä oikealle
edit-flip-horizontal = Peilaa vaakasuunnassa
edit-flip-vertical = Peilaa pystysuunnassa
edit-undo = Kumoa
edit-redo = Tee uudelleen
edit-revert = Palauta alkuperäinen
edit-export = Vie…

## Exporting

export-title = Vie
export-format = Tiedostomuoto
export-size = Koko
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Vain ääni
export-size-original = { $lines }p, sellaisenaan
export-on-graphics-card = Koodataan näytönohjaimella ({ $api })
export-on-processor = Koodataan suorittimella, mikä kestää kauemmin
export-no-encoder = Tälle muodolle ei voi avata kooderia täällä
export-checking = Etsitään kooderia…
export-sound-format = Äänimuoto
export-quality = Laatu
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Pakattu; toimii lähes kaikkialla, myös Applen laitteissa
export-mp3-note = Pakattu; toimii kaikkialla
export-opus-note = Pakattu; paras ääni kokoonsa nähden, mutta kaikki soittimet eivät tue sitä
export-flac-note = Häviötön: ääni täsmälleen sellaisenaan, avoimessa muodossa
export-alac-note = Häviötön: ääni täsmälleen sellaisenaan, muodossa, jota Applen laitteet toistavat
export-wav-note = Pakkaamaton: ääni täsmälleen sellaisenaan, suurimpina tiedostoina
export-kilobits = { $rate } kb/s
export-bits = { $bits }-bittinen
export-rate-lower = Tallennetaan taajuudella { $rate } kHz (tiedostossa { $file } kHz)
export-choose = Vie…
export-name = { $name } vienti
export-name-edited = { $name } muokattu
export-progress = Viedään: ”{ $name }”
export-stop = Lopeta vienti
notice-exported = Vietiin: ”{ $name }”
notice-export-stopped = Vienti keskeytettiin
notice-export-failed = Vieminen epäonnistui: { $error }
close-title = Suljetaanko viemättä?
close-unexported = Tiedostoon ”{ $name }” tehdyt muutokset menetetään, ellet vie niitä.
close-exporting = Tiedoston ”{ $name }” vienti keskeytyy, ja tähän mennessä kirjoitettu menetetään.
close-anyway = Sulje
close-quit-anyway = Lopeta

## Recording

record-title = Äänitallenne
record-file-name = Äänitallenne
record-start = Aloita tallennus
record-stop = Lopeta tallennus
record-no-input = Ei mikrofonia
record-microphone = Mikrofoni
record-default-microphone = Järjestelmän oletus
record-playing = Tietokoneen toistama ääni
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanava
   *[other] { $count } kanavaa
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, tallennetaan taajuudella { $saved } kHz
action-record-sound = Äänitä
open-record-sound = Uusi äänitallenne
start-record-sound = Äänitä
menu-record-sound = Uusi äänitallenne
record-screen-title = Näyttötallenne
record-screen-file-name = Näyttötallenne
record-screen-whole = Näyttö
record-screen-number = Näyttö { $number }
record-screen-main = Näyttö { $number } (päänäyttö)
record-screens = Näytöt
record-windows = Ikkunat
record-screen-what = Mitä tallennetaan
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Jokainen kuva
record-rate-tip = Kuvaa sekunnissa
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } ohitettu
   *[other] { $size } · { $count } ohitettua
}
record-screen-window = Ikkuna
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Ei enää jaettu
record-screen-again = Valitse uudelleen
record-screen-failed = Näytön tallentaminen epäonnistui
record-sound = Ääni
record-no-sound = Ei ääntä
# How much louder the input is taken, for a quiet microphone.
record-gain = Vahvistus
record-gain-value = +{ $decibels } dB
record-level-automatically = Säädä tasoa automaattisesti
record-hide-title = plaa piiloutuu tallennuksen ajaksi
record-hide-tray = Lopeta tallennus napsauttamalla punaista pistettä, jonka plaa lisää ilmaisinalueelle, tai avaa plaa uudelleen.
record-hide-reopen = Lopeta tallennus avaamalla plaa uudelleen.
record-hide-start = Aloita tallennus
record-tray-title = plaa tallentaa näyttöä
record-tray-tip = Lopeta tallennus napsauttamalla
action-record-screen = Tallenna näyttöä
open-record-screen = Uusi näyttötallenne
start-record-screen = Tallenna näyttöä
menu-record-screen = Uusi näyttötallenne
record-camera-title = Kameratallenne
record-camera-file-name = Kameratallenne
record-camera = Kamera
record-camera-failed = Kameran avaaminen epäonnistui
action-record-camera = Kuvaa kameralla
open-record-camera = Uusi kameratallenne
start-record-camera = Kuvaa kameralla
menu-record-camera = Uusi kameratallenne
notice-no-camera = Tallentamiseen ei ole kameraa
notice-recorded = Tallennettiin ”{ $name }” kansioon { $folder }
notice-record-failed = Tallentaminen epäonnistui: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Salli ulkoinen ohjaus
settings-allow-outside-control-note = Tekoälyagentit, kuten Claude Code, voivat nähdä, mitä plaassa toistetaan, ja ohjata sitä komennon plaa --mcp kautta. plaa kysyy ennen jokaista uutta agenttia.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Sallitut: { $agents }
settings-forget-agents = Unohda
settings-ask-before-note = Kysy ennen kuin agentti:
settings-ask-reading = Näkee, mitä toistetaan, tai kuvan
settings-ask-playing = Ohjaa toistoa, ääntä tai koko näytön tilaa
settings-ask-opening = Avaa tiedoston tai sulkee ikkunan
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Saako { $agent } ohjata plaata?
agent-prompt-body = { $agent } pyytää käyttää plaan ulkoista ohjausta nähdäkseen, mitä toistetaan, ja ohjatakseen sitä. Voit poistaa ulkoisen ohjauksen käytöstä Asetuksissa.
agent-prompt-allow = Salli
agent-prompt-deny = Älä salli
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Saako { $agent } nähdä, mitä täällä toistetaan?
agent-ask-play = Saako { $agent } ohjata toistoa täällä?
agent-ask-open = Saako { $agent } avata tai sulkea ikkunan?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } pyytää käyttää toimintoa ”{ $tool }”. Asetuksissa valitset, mistä plaa kysyy.

## Casting to a TV or speaker on the network

cast-looking = Etsitään televisioita ja kaiuttimia…
cast-none = Televisiota, kaiutinta tai Chromecast-laitetta ei löytynyt.
# $renderer is the TV's or speaker's name.
cast-stop = Lopeta toisto laitteella { $renderer }
notice-casting = Toistetaan laitteella { $renderer }.
notice-cast-failed = Toisto laitteella { $renderer } epäonnistui: { $reason }
cast-web-page = Verkkosivuja ei voi vielä suoratoistaa: avaa sen sijaan videon oma osoite.

## The assistant panel, and its models in Settings

assistant-add-another = Lisää malli…
assistant-add-model = Avustaja tarvitsee mallin: pilvimallin ja sen API-avaimen tai paikallisen mallin.
assistant-ask = Kysy siitä, mitä toistetaan
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Keskustelu oli venymässä liian pitkäksi, jotta { $model } voisi lukea sen kerralla, joten sen alkuosa tiivistettiin. Avustaja saattaa katsoa uudelleen, mitä se näki.
assistant-compacting = Tiivistetään keskustelun alkua
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } tällä tietokoneella
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } osoitteessa { $host }
assistant-new-chat = Uusi keskustelu
assistant-no-model = Lisää ensin malli Asetuksissa.
assistant-open-settings = Lisää malli
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Keskustelu ei enää mahdu siihen, mitä { $model } pystyy lukemaan kerralla. Aloita uusi keskustelu tai valitse malli, joka pystyy lukemaan enemmän.
assistant-problem-credit = { $provider } ilmoittaa, ettei tilillä ole saldoa. Uudelle tilille on ensin ostettava saldoa { $provider }-palvelun sivustolta, ennen kuin avain toimii; yritä sitten uudelleen.
assistant-problem-key = { $provider } hylkäsi API-avaimen. Tarkista se Asetuksissa.
assistant-problem-model = Palvelussa { $provider } ei ole mallia nimeltä { $model }. Tarkista nimi Asetuksissa.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } ei vastannut: { $message }
assistant-problem-rate = { $provider } pyytää hidastamaan. Yritä hetken kuluttua uudelleen.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } pyytää hidastamaan: { $message }
assistant-problem-refused = { $model } kieltäytyi vastaamasta.
assistant-problem-unavailable = { $provider } on ruuhkautunut tai siinä on ongelmia. Yritä hetken kuluttua uudelleen.
assistant-problem-unreachable = plaa ei saanut yhteyttä palveluun { $provider }. Tarkista yhteytesi tai se, että palvelin on käynnissä.
assistant-running = Käynnissä…
assistant-send = Lähetä
assistant-shortened = Keskustelu oli liian pitkä, jotta { $model } voisi lukea sen kerralla, joten aiemmat työkalujen tulokset jätettiin pois. Avustaja saattaa katsoa uudelleen, mitä se näki.
assistant-stop = Pysäytä
assistant-stopped = Pysäytetty.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Keskustelukumppanina on nyt { $model }.
assistant-thinking = Miettii…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Ajatukset
assistant-title = Avustaja
settings-assistant-add = Lisää malli
settings-assistant-add-button = Lisää
# $model is the model's name.
settings-assistant-added = { $model } vastasi, ja se on lisätty.
settings-assistant-added-tag = Lisätty
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Osoite, esimerkiksi { $example }
settings-assistant-change-key = Vaihda avain
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-yhteensopiva palvelin
# The menu of how much a local model reads at once.
settings-assistant-context = Konteksti
settings-assistant-context-note = Suurempi arvo antaa avustajan pitää mielessä enemmän keskustelusta, mutta malli vie enemmän muistia ja voi vastata hitaammin.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokenia
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Avaa sivu
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } on käynnissä osoitteessa { $address }.
settings-assistant-found-none = Palvelussa { $provider } ei ole vielä malleja. Lataa malli sen avulla ja etsi sitten uudelleen.
settings-assistant-get-key = Hanki API-avain
settings-assistant-get-server = Hanki { $provider }
settings-assistant-in-use = Käytössä
settings-assistant-key = API-avain
# $error is what the keychain said.
settings-assistant-key-failed = Avainta ei voitu tallentaa avainnippuun: { $error }
settings-assistant-key-kept = { $provider }-avaimesi säilytetään järjestelmän avainnipussa.
settings-assistant-key-needed = Tämä malli tarvitsee API-avaimen.
settings-assistant-key-refused = { $provider } hylkäsi tämän avaimen. Tarkista, että se kopioitiin kokonaan ja oikealta tililtä.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Luo avain { $provider }-palvelun sivustolla ja liitä se tähän.
settings-assistant-look-again = Etsi uudelleen
settings-assistant-looking = Etsitään malleja…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Malli, esimerkiksi { $example }
settings-assistant-no-server = plaa ei löytänyt tältä tietokoneelta käynnissä olevaa { $provider }-palvelinta. Käynnistä se tai anna sen osoite alla.
settings-assistant-no-tools = Ei osaa käyttää työkaluja, joita avustaja tarvitsee
settings-assistant-none = Ei vielä malleja. Lisää malli alla: paikallinen malli, kuten Ollaman, pysyy tällä tietokoneella; pilvimalli tarvitsee palveluntarjoajansa API-avaimen.
settings-assistant-note = Mallit, joiden kanssa avustaja voi keskustella. Avaimet säilytetään järjestelmän avainnipussa.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Käytä toista osoitetta
settings-assistant-recommended = Suositeltu
settings-assistant-remove = Poista
settings-assistant-sees = Näkee kuvat
settings-assistant-trying = Kokeillaan…
settings-assistant-use = Käytä
settings-assistant-use-key = Jatka
settings-assistant-uses-tools = Käyttää työkaluja
