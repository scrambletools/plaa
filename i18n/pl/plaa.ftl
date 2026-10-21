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
language-name = Polski

## Common

app-name = plaa
common-cancel = Anuluj
common-close = Zamknij
common-more = Więcej

## Command line

usage-help =
    Użycie: plaa [PLIK LUB ADRES]...

    Odtwarzanie plików wideo i audio oraz strumieni spod adresów takich jak
    https://example.com/live.m3u8. Każdy z nich otwiera się w oknie
    działającej aplikacji plaa, która w razie potrzeby się uruchomi.

    Opcje:
      -h, --help        Pokaż tę pomoc
      -V, --version     Pokaż wersję
          --interfaces  Pokaż interfejsy przewodowe i to, czy plaa może
                        wysyłać przez nie strumienie AVB
          --mcp         Udostępnij MCP przez stdin i stdout, by agenci AI widzieli,
                        co odtwarza działająca aplikacja plaa, i mogli nią sterować

## Start window

start-hint = Otwórz plik wideo lub audio, strumień albo film z internetu.
start-open = Otwórz
start-open-address = Otwórz adres

## Actions, as buttons, tooltips and menus name them

action-open = Otwórz
action-open-address = Otwórz adres
open-file = Otwórz plik…
open-address = Otwórz adres…
action-settings = Ustawienia
action-info = Inspektor
action-loop = Pętla
action-screenshot = Zapisz klatkę jako obraz
action-play = Odtwórz
action-pause = Wstrzymaj
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Do tyłu o { $seconds } sekundę
    [few] Do tyłu o { $seconds } sekundy
    [many] Do tyłu o { $seconds } sekund
   *[other] Do tyłu o { $seconds } sekundy
}
action-forward = { $seconds ->
    [one] Do przodu o { $seconds } sekundę
    [few] Do przodu o { $seconds } sekundy
    [many] Do przodu o { $seconds } sekund
   *[other] Do przodu o { $seconds } sekundy
}
action-mute = Wycisz
action-unmute = Wyłącz wyciszenie
action-speed = Szybkość
action-tracks = Dźwięk i napisy
action-full-screen = Pełny ekran
action-playlist = Lista odtwarzania
action-chapters = Rozdziały
action-picture-in-picture = Obraz w obrazie
action-previous = Poprzedni
action-next = Następny
action-exit-full-screen = Wyłącz pełny ekran
action-quality = Jakość
action-visualizer = Wizualizacja
action-network = Dźwięk sieciowy

## What a sound file shows under its cover

visualizer-spectrum = Widmo
visualizer-waveform = Przebieg fali
visualizer-prism = Pryzmat
visualizer-cover = Tylko okładka

## The floating controls

overlay-keep-shown = Przypnij elementy sterujące
overlay-auto-hide = Wyświetlaj elementy sterujące nad obrazem

## Audio and subtitle tracks

tracks-audio = Dźwięk
tracks-subtitles = Napisy
tracks-off = Wyłączone
# A track the file gives no name or language, by its number.
tracks-numbered = Ścieżka { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Tylko dźwięk

## Inspector

info-title = Inspektor
playlist-title = Lista odtwarzania
chapters-numbered = Rozdział { $number }
info-file = Plik
info-name = Nazwa
info-address = Adres
info-folder = Folder
info-size = Rozmiar
info-format = Format
info-duration = Czas trwania
info-bit-rate = Przepływność
info-video = Wideo
info-audio = Dźwięk
info-codec = Kodek
info-dimensions = Wymiary
info-frame-rate = Liczba klatek na sekundę
info-frames-per-second = { $rate } kl./s
info-pixel-format = Format pikseli
info-decoding = Dekodowanie
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Karta graficzna ({ $api })
info-decoding-software = Procesor
info-sample-rate = Częstotliwość próbkowania
info-hertz = { $rate } Hz
info-channels = Kanały
info-language = Język
info-no-sound = Brak dźwięku
info-tags = Tagi

## Dialogs

dialog-open = Otwórz
dialog-filter-media = Wideo i audio
dialog-filter-all = Wszystkie pliki

address-title = Otwórz adres
address-note = Strona internetowa z filmem, np. strona filmu w YouTube, albo strumień lub plik w sieci, np. https://example.com/live.m3u8.
address-label = Adres
address-open = Otwórz
address-recent = Ostatnie
address-forget = Usuń z ostatnich

## Notices

notice-dialog-failed = Nie można wyświetlić okna wyboru pliku: { $error }
notice-screenshot-saved = Zapisano „{ $name }”.
notice-screenshot-failed = Nie można zapisać obrazu: { $error }
notice-no-sound = Odtwarzanie bez dźwięku: { $reason }
notice-loop-from = Pętla od { $time }. Naciśnij ponownie w miejscu, w którym ma się kończyć.
notice-loop-between = Pętla od { $from } do { $to }.
notice-loop-off = Pętla wyłączona.

## Errors, shown in place of what could not play

error-needs-yt-dlp = To wygląda na stronę internetową. plaa odtwarza filmy ze stron internetowych za pomocą yt-dlp, który nie jest zainstalowany.
error-empty-list = Na tej liście nie ma nic do odtworzenia.

## Settings

settings-title = Ustawienia
settings-tab-appearance = Wygląd
settings-tab-playback = Odtwarzanie
settings-tab-network = Sieć
settings-tab-language = Język
settings-tab-agents = Agenci
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (wersja deweloperska, { $build })
settings-choose = Wybierz…

settings-appearance-system = Systemowy
settings-appearance-light = Jasny
settings-appearance-dark = Ciemny
settings-colors = Kolory
settings-system-accent = Użyj koloru akcentu systemu
settings-omarchy-note = Kolory są tworzone na podstawie akcentu motywu „{ $theme }”.
settings-system-accent-note = Kolory są tworzone na podstawie koloru akcentu systemu.
settings-system-accent-none = System nie ma koloru akcentu, więc plaa używa koloru wybranego poniżej.
settings-accent-chosen-note = Kolory są tworzone na podstawie koloru wybranego poniżej.
settings-controls = Elementy sterujące
settings-overlay = Pływające elementy sterujące
settings-overlay-note = Pasek narzędzi i elementy sterujące odtwarzaniem unoszą się nad obrazem lub okładką i chowają się, gdy wskaźnik się nie porusza lub jest poza oknem.
settings-overlay-delay = Ukryj po
settings-overlay-delay-note = Jak długo wskaźnik musi pozostać nieruchomo nad odtwarzanym materiałem, zanim elementy sterujące się schowają.
settings-seconds = { $seconds } s
settings-transparency = Przezroczystość
settings-transparency-note = Na ile obraz prześwituje przez pływające elementy sterujące.
settings-percent = { $percent }%
settings-corner-radius = Promień narożników
settings-corner-radius-note = Dla okien dialogowych i pływających elementów sterujących.
settings-pixels = { $pixels } px
settings-animations = Animacje
settings-animations-note = Wysuwane paski i panele, rozwijane okna dialogowe i sprężyste przyciski.
settings-animations-reduced = Wyłączone, gdy system prosi o ograniczenie ruchu.

settings-hardware = Używaj karty graficznej
settings-hardware-note = Karta graficzna dekoduje wideo i koduje eksportowane pliki, co zmniejsza zużycie energii, zapewnia płynne odtwarzanie dużych filmów i przyspiesza eksport. Dotyczy plików otwartych później.
settings-audio-device = Wyjście dźwięku
settings-audio-device-system = Domyślne systemowe
settings-resume = Wznawiaj od miejsca przerwania
settings-resume-note = Pliki zaczynają się tam, gdzie były przy ostatnim zamknięciu.
settings-folder-playlists = Odtwarzaj pozostałe pliki z folderu
settings-folder-playlists-note = Po otwarciu jednego pliku pozostałe pliki z jego folderu trafiają na listę i są odtwarzane po nim.
settings-streams = Strumienie i filmy z internetu
settings-stream-quality = Jakość
settings-stream-quality-note = Dla strumieni i filmów z internetu dostępnych w kilku jakościach. Dotyczy tego, co zostanie otwarte później.
settings-quality-auto = Automatyczna, na wysokość ekranu
settings-quality-best = Najlepsza
settings-web-pages = Odtwarzaj filmy z internetu za pomocą yt-dlp
settings-web-pages-found = yt-dlp znajduje film na stronach takich jak YouTube.
settings-web-pages-missing = yt-dlp nie jest zainstalowany. Zainstaluj go, aby odtwarzać filmy ze stron takich jak YouTube.
settings-folders = Foldery
settings-recordings = Nagrania
settings-screenshots = Zapisane klatki

settings-interface-language = Język interfejsu
# The first entry of the language list.
settings-language-system = Zgodnie z systemem
settings-track-languages = Języki ścieżek
settings-track-languages-note = Kody języków, np. pl lub eng, od najbardziej pożądanego, oddzielone przecinkami. plaa wybiera dźwięk i napisy w tych językach, jeśli plik je zawiera.
settings-audio-languages = Dźwięk
settings-subtitle-languages = Napisy

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interfejs AVB
settings-avb-interface-note = Port przewodowy, na którym plaa wyszukuje nadajniki i odbiorniki AVB i Milan. Do wysyłania potrzebny jest port ze sprzętowym zegarem PTP, np. w karcie Intel I210.
settings-avb-off = Wyłączony
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (tylko wykrywanie: brak zegara PTP)
settings-avb-off-mac = { $interface } (tylko wykrywanie: AVB jest wyłączone)
settings-avb-interface-note-mac = Port przewodowy, na którym plaa wyszukuje nadajniki i odbiorniki AVB i Milan. Wysyłanie odbywa się przez własną jednostkę AVB tego Maca, która wymaga włączenia AVB dla tego portu w przeglądarce urządzeń sieciowych w Konfiguracji audio-MIDI.
settings-avb-talker-note-mac = plaa wysyła dźwięk okna odtwarzanego jako ostatnie przez własną jednostkę AVB tego Maca do podłączonych do niej odbiorników.
settings-avb-talker = Wysyłaj odtwarzany dźwięk
settings-avb-talker-note = plaa staje się nadajnikiem Milan i wysyła dźwięk okna odtwarzanego jako ostatnie do podłączonych do niego odbiorników.
settings-avb-name = Nazwa w sieci
settings-avb-name-note = Kontrolery pokazują plaa jako „{ $name }”. Naciśnij Enter, aby zmienić nazwę.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa na { $computer }
avb-could-not-start = Nie można użyć interfejsu { $interface }: { $error }
network-title = Dźwięk sieciowy
network-off = Włącz „Wysyłaj odtwarzany dźwięk” w Ustawieniach, aby odtwarzać w sieci.
network-blocked = Nie można wysyłać: { $reason }
network-waiting = Gotowe do wysyłania, oczekiwanie na podłączenie odbiornika.
network-sending = Wysyłanie do podłączonych odbiorników.
network-looking = Wyszukiwanie odbiorników…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Uruchamianie ptp4l, który utrzymuje czas sieciowy, według którego wysyłany jest strumień plaa…
network-ptp-elsewhere = ptp4l działa na innym interfejsie, więc strumień plaa nie ma tutaj czasu sieciowego i odbiorniki mogą go nie odtworzyć. Wybierz tamten interfejs w Ustawieniach albo uruchom ptp4l na tym.
network-ptp-missing = ptp4l nie działa, więc strumień plaa nie ma czasu sieciowego i odbiorniki mogą go nie odtworzyć. Zainstaluj linuxptp razem z pakietem plaa albo zainstaluj triib, który go uruchamia.
network-ptp-failed = ptp4l nie działa, więc strumień plaa nie ma czasu sieciowego i odbiorniki mogą go nie odtworzyć. plaa nie może go uruchomić: { $reason }
network-no-listeners = Żaden odbiornik w sieci nie może odtworzyć dźwięku z plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Odtwarzaj na { $listener }
network-stop-on = Zatrzymaj odtwarzanie na { $listener }
network-choose-format = Wybierz format odtwarzania na { $listener }
network-failed = Nie można nawiązać połączenia: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Zaktualizuj triib, aby odtwarzać tutaj.
network-choose = Wybierz odbiornik, aby zobaczyć, jak odbiera strumień.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Stan
network-not-connected = Niepołączony
network-from = Źródło: { $stream }
network-from-receiving = Źródło: { $stream }, odbieranie
network-from-waiting = Źródło: { $stream }, oczekiwanie na nadajnik
network-from-failed = Źródło: { $stream }, rezerwacja nadajnika nie powiodła się: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Zegar mediów
network-locked = Zsynchronizowany ze strumieniem
network-not-locked = Niezsynchronizowany
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formaty, które może przyjąć z plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID strumienia
# The MAC address the stream is sent to.
network-destination = Adres docelowy
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Opóźnienie
network-microseconds = { $time } µs
network-frames = Odebrane ramki
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Ramki poza kolejnością
# How often the stream stopped coming while it was taken.
network-interrupted = Przerwy
network-entity-id = ID jednostki

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = niewystarczająca przepustowość
msrp-failure-2 = niewystarczające zasoby mostu
msrp-failure-3 = niewystarczająca przepustowość dla klasy ruchu
msrp-failure-4 = identyfikator strumienia jest używany przez inny nadajnik
msrp-failure-5 = adres docelowy jest już używany
msrp-failure-6 = wywłaszczony przez strumień o wyższej randze
msrp-failure-7 = zgłoszone opóźnienie się zmieniło
msrp-failure-8 = port wyjściowy nie obsługuje AVB
msrp-failure-9 = użyj innego adresu docelowego
msrp-failure-10 = brak zasobów MSRP
msrp-failure-11 = brak zasobów MMRP
msrp-failure-12 = nie można zapisać adresu docelowego
msrp-failure-13 = priorytet nie jest priorytetem klasy SR
msrp-failure-14 = ramki są za duże dla medium
msrp-failure-15 = osiągnięto limit portów fan-in
msrp-failure-16 = zmieniła się pierwsza wartość zarejestrowanego strumienia
msrp-failure-17 = VLAN zablokowany na porcie wyjściowym
msrp-failure-18 = tagowanie VLAN wyłączone na porcie wyjściowym
msrp-failure-19 = niezgodny priorytet klasy SR
msrp-failure-unknown = nieznana przyczyna
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, na moście { $bridge }

## The macOS menu bar

menu-about = O aplikacji plaa
menu-settings = Ustawienia…
menu-services = Usługi
menu-hide = Ukryj plaa
menu-hide-others = Ukryj pozostałe
menu-show-all = Pokaż wszystko
menu-quit = Zakończ plaa
menu-file = Plik
menu-open = Otwórz…
menu-open-address = Otwórz adres…
menu-close = Zamknij okno
menu-edit = Edycja
menu-cut = Wytnij
menu-copy = Kopiuj
menu-paste = Wklej
menu-select-all = Zaznacz wszystko
menu-undo = Cofnij
menu-redo = Powtórz
menu-trim = Przytnij…
menu-split = Podziel klip
menu-rotate-left = Obróć w lewo
menu-rotate-right = Obróć w prawo
menu-flip-horizontal = Odbij w poziomie
menu-flip-vertical = Odbij w pionie
menu-revert = Przywróć oryginał
menu-export = Eksportuj…
menu-view = Widok
menu-playback = Odtwarzanie
menu-play-pause = Odtwórz lub wstrzymaj
menu-to-start = Przejdź na początek
menu-previous-chapter = Poprzedni rozdział
menu-next-chapter = Następny rozdział
menu-faster = Szybciej
menu-slower = Wolniej
menu-normal-speed = Normalna szybkość
menu-ab-loop = Pętla między dwoma punktami
menu-volume-up = Głośniej
menu-volume-down = Ciszej
menu-mute = Wycisz lub wyłącz wyciszenie
menu-next-subtitles = Następne napisy
menu-next-audio = Następna ścieżka dźwiękowa
menu-window = Okno
menu-minimize = Minimalizuj
menu-zoom = Zmień rozmiar
menu-bring-all-to-front = Przenieś wszystko na wierzch

## Editing

action-edit = Edytuj
title-edited = { $name } (po edycji)
edit-trim = Przytnij
edit-trim-apply = Przytnij
edit-split = Podziel klip
edit-remove-clip = Usuń klip
edit-done = Gotowe
edit-rotate-left = Obróć w lewo
edit-rotate-right = Obróć w prawo
edit-flip-horizontal = Odbij w poziomie
edit-flip-vertical = Odbij w pionie
edit-undo = Cofnij
edit-redo = Powtórz
edit-revert = Przywróć oryginał
edit-export = Eksportuj…

## Exporting

export-title = Eksportuj
export-format = Format
export-size = Rozmiar
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Tylko dźwięk
export-size-original = { $lines }p, bez zmian
export-on-graphics-card = Kodowanie na karcie graficznej ({ $api })
export-on-processor = Kodowanie na procesorze, co trwa dłużej
export-no-encoder = Brak dostępnego kodera dla tego formatu
export-checking = Wyszukiwanie kodera…
export-sound-format = Format dźwięku
export-quality = Jakość
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Skompresowany; odtwarzany niemal wszędzie, także na urządzeniach Apple
export-mp3-note = Skompresowany; odtwarzany wszędzie
export-opus-note = Skompresowany; najlepszy dźwięk przy danym rozmiarze, choć nie każdy odtwarzacz go obsługuje
export-flac-note = Bezstratny: dźwięk dokładnie taki, jaki jest, w otwartym formacie
export-alac-note = Bezstratny: dźwięk dokładnie taki, jaki jest, w formacie odtwarzanym przez urządzenia Apple
export-wav-note = Nieskompresowany: dźwięk dokładnie taki, jaki jest, w największych plikach
export-kilobits = { $rate } kb/s
export-bits = { $bits }-bitowy
export-rate-lower = Zapis z częstotliwością { $rate } kHz, niższą niż { $file } kHz w pliku
export-choose = Eksportuj…
export-name = { $name } (eksport)
export-name-edited = { $name } (po edycji)
export-progress = Eksportowanie „{ $name }”
export-stop = Zatrzymaj eksportowanie
notice-exported = Wyeksportowano „{ $name }”
notice-export-stopped = Eksportowanie zatrzymane
notice-export-failed = Nie można wyeksportować: { $error }
close-title = Zamknąć bez eksportowania?
close-unexported = Zmiany w pliku „{ $name }” zostaną utracone, jeśli ich nie wyeksportujesz.
close-exporting = Eksportowanie „{ $name }” zostanie przerwane, a to, co zostało już zapisane, przepadnie.
close-anyway = Zamknij
close-quit-anyway = Zakończ

## Recording

record-title = Nagrywanie dźwięku
record-file-name = Nagranie dźwięku
record-start = Nagrywaj
record-stop = Zatrzymaj nagrywanie
record-no-input = Brak mikrofonu
record-microphone = Mikrofon
record-default-microphone = Domyślny systemowy
record-playing = Dźwięk odtwarzany przez komputer
record-mono = mono
record-stereo = stereo
record-channels = { $count ->
    [one] { $count } kanał
    [few] { $count } kanały
    [many] { $count } kanałów
   *[other] { $count } kanału
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, zapis w { $saved } kHz
action-record-sound = Nagraj dźwięk
open-record-sound = Nowe nagranie dźwięku
start-record-sound = Nagraj dźwięk
menu-record-sound = Nowe nagranie dźwięku
record-screen-title = Nagrywanie ekranu
record-screen-file-name = Nagranie ekranu
record-screen-whole = Ekran
record-screen-number = Ekran { $number }
record-screen-main = Ekran { $number } (główny)
record-screens = Ekrany
record-windows = Okna
record-screen-what = Co nagrywać
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Każda klatka
record-rate-tip = Klatki na sekundę
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · pominięto { $count } klatkę
    [few] { $size } · pominięto { $count } klatki
    [many] { $size } · pominięto { $count } klatek
   *[other] { $size } · pominięto { $count } klatki
}
record-screen-window = Okno
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Udostępnianie zakończone
record-screen-again = Wybierz ponownie
record-screen-failed = Nie można nagrać ekranu
record-sound = Dźwięk
record-no-sound = Bez dźwięku
# How much louder the input is taken, for a quiet microphone.
record-gain = Wzmocnienie
record-gain-value = +{ $decibels } dB
record-level-automatically = Automatyczna regulacja poziomu
record-hide-title = plaa ukrywa się podczas nagrywania
record-hide-tray = Aby zatrzymać nagrywanie, kliknij czerwoną kropkę, którą plaa umieszcza w zasobniku, albo ponownie otwórz plaa.
record-hide-reopen = Aby zatrzymać nagrywanie, ponownie otwórz plaa.
record-hide-start = Rozpocznij nagrywanie
record-tray-title = plaa nagrywa ekran
record-tray-tip = Kliknij, aby zatrzymać nagrywanie
action-record-screen = Nagraj ekran
open-record-screen = Nowe nagranie ekranu
start-record-screen = Nagraj ekran
menu-record-screen = Nowe nagranie ekranu
record-camera-title = Nagrywanie z kamery
record-camera-file-name = Nagranie z kamery
record-camera = Kamera
record-camera-failed = Nie można otworzyć kamery
action-record-camera = Nagraj z kamery
open-record-camera = Nowe nagranie z kamery
start-record-camera = Nagraj z kamery
menu-record-camera = Nowe nagranie z kamery
notice-no-camera = Brak kamery do nagrywania
notice-recorded = Zapisano „{ $name }” w folderze { $folder }
notice-record-failed = Nie można nagrać: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Zezwalaj na sterowanie zewnętrzne
settings-allow-outside-control-note = Agenci AI, tacy jak Claude Code, mogą przez plaa --mcp widzieć, co jest odtwarzane w plaa, i sterować aplikacją. plaa pyta przed każdym nowym agentem.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Dozwolone: { $agents }
settings-forget-agents = Zapomnij
settings-ask-before-note = Pytaj, zanim agent:
settings-ask-reading = Zobaczy, co jest odtwarzane, lub obraz
settings-ask-playing = Będzie sterować odtwarzaniem, dźwiękiem lub pełnym ekranem
settings-ask-opening = Otworzy plik lub zamknie okno
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Zezwolić agentowi { $agent } na sterowanie plaa?
agent-prompt-body = { $agent } prosi o użycie sterowania zewnętrznego plaa, aby widzieć, co jest odtwarzane, i sterować aplikacją. Sterowanie zewnętrzne możesz wyłączyć w Ustawieniach.
agent-prompt-allow = Zezwól
agent-prompt-deny = Nie zezwalaj
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Zezwolić agentowi { $agent } na podgląd tego, co jest tu odtwarzane?
agent-ask-play = Zezwolić agentowi { $agent } na sterowanie odtwarzaniem tutaj?
agent-ask-open = Zezwolić agentowi { $agent } na otwarcie lub zamknięcie okna?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } prosi o użycie narzędzia „{ $tool }”. W Ustawieniach wybierzesz, o co pyta plaa.
