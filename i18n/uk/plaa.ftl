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
language-name = Українська

## Common

app-name = plaa
common-cancel = Скасувати
common-close = Закрити
common-more = Ще

## Command line

usage-help =
    Використання: plaa [ФАЙЛ АБО АДРЕСА]...

    Відтворення відео- та аудіофайлів, а також потоків за адресами на зразок
    https://example.com/live.m3u8. Кожен відкривається у вікні запущеного
    plaa; якщо його не запущено, він запуститься.

    Параметри:
      -h, --help        Показати цю довідку
      -V, --version     Показати версію
          --interfaces  Показати дротові інтерфейси та чи може plaa
                        надсилати ними потоки AVB
          --mcp         Надати MCP через stdin і stdout, щоб ШІ-агенти бачили,
                        що відтворює запущений plaa, і керували ним

## Start window

start-hint = Відкрийте відео- чи аудіофайл, потік або вебвідео.
start-open = Відкрити
start-open-address = Відкрити адресу
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Підключитися

## Actions, as buttons, tooltips and menus name them

action-open = Відкрити
action-open-address = Відкрити адресу
open-file = Відкрити файл…
open-address = Відкрити адресу…
action-settings = Параметри
action-info = Інспектор
action-loop = Повтор
action-screenshot = Зберегти кадр як зображення
action-play = Відтворити
action-pause = Пауза
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Назад на { $seconds } секунду
    [few] Назад на { $seconds } секунди
    [many] Назад на { $seconds } секунд
   *[other] Назад на { $seconds } секунди
}
action-forward = { $seconds ->
    [one] Вперед на { $seconds } секунду
    [few] Вперед на { $seconds } секунди
    [many] Вперед на { $seconds } секунд
   *[other] Вперед на { $seconds } секунди
}
action-mute = Вимкнути звук
action-unmute = Увімкнути звук
action-speed = Швидкість
action-tracks = Аудіо та субтитри
action-full-screen = Повноекранний режим
action-playlist = Список відтворення
action-chapters = Розділи
action-picture-in-picture = Картинка в картинці
action-previous = Попередній
action-next = Наступний
action-exit-full-screen = Вийти з повноекранного режиму
action-quality = Якість
action-visualizer = Візуалізація
action-network = Мережеве аудіо
action-cast = Транслювати
action-connect = Підключитися
action-assistant = Асистент

## What a sound file shows under its cover

visualizer-spectrum = Спектр
visualizer-waveform = Форма хвилі
visualizer-prism = Призма
visualizer-cover = Лише обкладинка

## The floating controls

overlay-keep-shown = Закріпити елементи керування
overlay-auto-hide = Показувати елементи керування поверх зображення

## Audio and subtitle tracks

tracks-audio = Аудіо
tracks-subtitles = Субтитри
tracks-off = Вимкнено
# A track the file gives no name or language, by its number.
tracks-numbered = Доріжка { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Лише аудіо

## Inspector

info-title = Інспектор
playlist-title = Список відтворення
chapters-numbered = Розділ { $number }
info-file = Файл
info-name = Назва
info-address = Адреса
info-folder = Папка
info-size = Розмір
info-format = Формат
info-duration = Тривалість
info-bit-rate = Бітрейт
info-video = Відео
info-audio = Аудіо
info-codec = Кодек
info-dimensions = Розміри
info-frame-rate = Частота кадрів
info-frames-per-second = { $rate } кадр/с
info-pixel-format = Формат пікселів
info-decoding = Декодування
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Відеокарта ({ $api })
info-decoding-software = Процесор
info-sample-rate = Частота дискретизації
info-hertz = { $rate } Hz
info-channels = Канали
info-language = Мова
info-no-sound = Без звуку
info-tags = Теги
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Передавач
info-stream = Потік
# The talker's listeners other than plaa.
info-other-listeners = Інші приймачі
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Клас SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Новий формат відключить { $count ->
        [one] { $count } інший приймач передавача.
        [few] { $count } інші приймачі передавача.
        [many] { $count } інших приймачів передавача.
       *[other] { $count } іншого приймача передавача.
    }

## Dialogs

dialog-open = Відкрити
dialog-filter-media = Відео та аудіо
dialog-filter-all = Усі файли

address-title = Відкрити адресу
address-note = Вебсторінка з відео, наприклад з YouTube, або потік чи файл у мережі, наприклад https://example.com/live.m3u8.
address-label = Адреса
address-open = Відкрити
address-recent = Нещодавні
address-forget = Видалити з нещодавніх

## Notices

notice-dialog-failed = Не вдалося відкрити вікно вибору файлу: { $error }
notice-screenshot-saved = Збережено «{ $name }».
notice-screenshot-failed = Не вдалося зберегти зображення: { $error }
notice-no-sound = Відтворення без звуку: { $reason }
notice-loop-from = Повтор від { $time }. Натисніть ще раз там, де він має закінчитися.
notice-loop-between = Повтор від { $from } до { $to }.
notice-loop-off = Повтор вимкнено.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Схоже, це вебсторінка. plaa відтворює відео з вебсторінок за допомогою yt-dlp, але його не встановлено.
error-empty-list = У цьому списку немає нічого для відтворення.

## Settings

settings-title = Параметри
settings-tab-appearance = Оформлення
settings-tab-playback = Відтворення
settings-tab-storage = Зберігання
settings-tab-network = Мережа
settings-tab-language = Мова
settings-tab-agents = Агенти
settings-tab-assistant = Асистент
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (збірка для розробки, { $build })
settings-choose = Вибрати…

settings-appearance-system = Як у системі
settings-appearance-light = Світле
settings-appearance-dark = Темне
settings-colors = Кольори
settings-system-accent = Використовувати акцентний колір системи
settings-omarchy-note = Кольори побудовано на основі акцентного кольору теми «{ $theme }».
settings-system-accent-note = Кольори побудовано на основі акцентного кольору системи.
settings-system-accent-none = У системі немає акцентного кольору, тому plaa використовує колір, вибраний нижче.
settings-accent-chosen-note = Кольори побудовано на основі кольору, вибраного нижче.
settings-controls = Елементи керування
settings-overlay = Плаваючі елементи керування
settings-overlay-note = Панель інструментів і елементи керування відтворенням розташовуються поверх зображення чи обкладинки й ховаються, коли вказівник нерухомий або поза вікном.
settings-overlay-delay = Ховати через
settings-overlay-delay-note = Скільки вказівник має залишатися нерухомим над вмістом, перш ніж елементи керування сховаються.
settings-seconds = { $seconds } с
settings-transparency = Прозорість
settings-transparency-note = Наскільки зображення просвічує крізь плаваючі елементи керування.
settings-percent = { $percent }%
settings-corner-radius = Радіус заокруглення
settings-corner-radius-note = Для діалогів і плаваючих елементів керування.
settings-pixels = { $pixels } px
settings-animations = Анімація
settings-animations-note = Висувні панелі, діалоги, що розгортаються, і пружні кнопки.
settings-animations-reduced = Вимкнено, поки в системі ввімкнено зменшення руху.

settings-hardware = Використовувати відеокарту
settings-hardware-note = На ній декодується відео й кодується експорт: менше споживання енергії, плавне відтворення великих відео та швидший експорт. Діє для файлів, відкритих після цього.
settings-audio-device = Виведення звуку
settings-audio-device-system = Системний за замовчуванням
settings-resume = Продовжувати з місця зупинки
settings-resume-note = Файли починаються з того місця, де їх востаннє закрили.
settings-folder-playlists = Відтворювати інші файли з папки
settings-folder-playlists-note = Коли відкрито один файл, до списку додаються інші файли з його папки, які відтворюються після нього.
settings-streams = Потоки та вебвідео
settings-stream-quality = Якість
settings-stream-quality-note = Для потоків і вебвідео з кількома варіантами якості. Діє для того, що буде відкрито після цього.
settings-quality-auto = Автоматично, за висотою екрана
settings-quality-best = Найкраща
settings-web-pages = Відтворювати вебвідео через yt-dlp
settings-web-pages-found = yt-dlp знаходить відео на таких сторінках, як YouTube.
settings-web-pages-missing = yt-dlp не встановлено. Установіть його, щоб відтворювати відео з таких сторінок, як YouTube.
settings-default-app = Застосунок за замовчуванням
settings-default-app-label = Відкривати файли в plaa
settings-default-app-note = Зробити plaa застосунком, що відкриває відео- та звукові файли.
settings-default-app-note-windows = У Windows застосунки за замовчуванням вибирають лише в її власних параметрах. Кнопка відкриває там сторінку plaa.
settings-default-app-note-macos = macOS просить підтвердити кожен тип: MP4, MOV, MKV, WebM, MP3, M4A, FLAC і WAV.
settings-default-app-status = Типи файлів, що відкриваються в plaa: { $set } з { $total }.
settings-default-app-button = Зробити за замовчуванням
settings-default-app-button-windows = Відкрити параметри
settings-default-app-no-entry = Ярлик робочого столу plaa не встановлено, тому система не може відкривати ним файли. Установіть plaa з пакета.
settings-default-app-no-bundle = Відкрийте plaa з plaa.app, щоб зробити його застосунком за замовчуванням.
settings-default-app-failed = Не вдалося зробити plaa застосунком за замовчуванням: { $error }
settings-folders = Папки
settings-sound-recordings = Звукозаписи
settings-video-recordings = Відеозаписи
settings-screenshots = Збережені кадри

settings-interface-language = Мова інтерфейсу
# The first entry of the language list.
settings-language-system = Як у системі
settings-track-languages = Мови доріжок
settings-track-languages-note = Коди мов, як-от en або fra, через кому, у порядку пріоритету. plaa вибирає аудіо та субтитри цими мовами, якщо вони є у файлі.
settings-audio-languages = Аудіо
settings-subtitle-languages = Субтитри

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Інтерфейс AVB
settings-avb-interface-note = Дротовий порт, на якому plaa шукає передавачі й приймачі AVB і Milan. Для надсилання потрібен порт з апаратним годинником PTP, як-от в Intel I210.
settings-avb-off = Вимкнено
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (лише перегляд: немає годинника PTP)
settings-avb-off-mac = { $interface } (лише перегляд: AVB вимкнено)
settings-avb-interface-note-mac = Дротовий порт, на якому plaa шукає передавачі й приймачі AVB і Milan. Надсилання йде через власну сутність AVB цього Mac, для якої потрібно ввімкнути AVB на цьому порту в браузері мережевих пристроїв програми «Налаштування Audio MIDI».
settings-avb-talker-note-mac = plaa надсилає звук вікна, яке відтворювало останнім, через власну сутність AVB цього Mac до підключених до неї приймачів.
settings-avb-talker = Надсилати те, що відтворюється
settings-avb-talker-note = plaa стає передавачем Milan і надсилає звук вікна, яке відтворювало останнім, підключеним до нього приймачам.
settings-avb-name = Назва в мережі
settings-avb-name-note = Контролери показують plaa як «{ $name }». Натисніть Enter, щоб перейменувати.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa на { $computer }
avb-could-not-start = Не вдалося використати { $interface }: { $error }
network-title = Мережеве аудіо
network-off = Щоб відтворювати в мережі, увімкніть «Надсилати те, що відтворюється» в параметрах.
network-blocked = Не надсилається: { $reason }
network-waiting = Готово до надсилання, очікування підключення приймача.
network-sending = Надсилання підключеним приймачам.
network-looking = Пошук приймачів…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Запуск ptp4l, який підтримує мережевий час, за яким надсилається потік plaa…
network-ptp-elsewhere = ptp4l працює на іншому інтерфейсі, тому потік plaa не має тут мережевого часу і приймачі можуть не відтворити його. Виберіть той інтерфейс у параметрах або запустіть ptp4l на цьому.
network-ptp-missing = ptp4l не запущено, тому потік plaa не має мережевого часу і приймачі можуть не відтворити його. Установіть linuxptp разом із пакетом plaa або triib, який його запускає.
network-ptp-failed = ptp4l не запущено, тому потік plaa не має мережевого часу і приймачі можуть не відтворити його. plaa не вдалося його запустити: { $reason }
network-no-listeners = У мережі немає приймача, який може відтворити звук plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Відтворити на пристрої { $listener }
network-stop-on = Зупинити відтворення на пристрої { $listener }
network-choose-format = Виберіть формат для відтворення на пристрої { $listener }
network-failed = Не вдалося підключитися: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Оновіть triib, щоб відтворювати тут.
network-choose = Виберіть приймач, щоб побачити, як він отримує потік.
network-no-sources = У мережі немає передавача, що надсилає у форматі, який відтворює plaa, або може на нього перемкнутися.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Перестати слухати передавач { $talker }
network-listen-update-triib = Оновіть triib, щоб слухати тут: plaa відтворює лише те, що й так доходить до цього комп’ютера.
network-listen-lite = Порт працює в режимі AVB Lite, який не резервує потоків: plaa відтворює лише те, що й так доходить до цього комп’ютера.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Пошук передавачів…
# plaa listens through its entity, which exists only while it sends.
connect-off = Щоб слухати тут передавачі, увімкніть «Надсилати те, що відтворюється» в параметрах.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa не може слухати тут: { $reason }
connect-sending = Надсилають у форматі, який відтворює plaa
connect-changing = Перемкнуться на формат, який відтворює plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, у форматі { $format }
connect-change-cuts-off = { $talker }, у форматі { $format }, з відключенням { $count ->
        [one] { $count } іншого приймача
        [few] { $count } інших приймачів
        [many] { $count } інших приймачів
       *[other] { $count } іншого приймача
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Стан
network-not-connected = Не підключено
network-from = Джерело: { $stream }
network-from-receiving = Джерело: { $stream }, отримання
network-from-waiting = Джерело: { $stream }, очікування передавача
network-from-failed = Джерело: { $stream }, збій резервування передавача: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Медіагодинник
network-locked = Синхронізовано з потоком
network-not-locked = Не синхронізовано
network-format = Формат
# The formats both the listener and plaa's stream can be in.
network-formats = Формати, які він може приймати від plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID потоку
# The MAC address the stream is sent to.
network-destination = Адреса призначення
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Затримка
network-microseconds = { $time } µs
network-frames = Отримано кадрів
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Кадри поза послідовністю
# How often the stream stopped coming while it was taken.
network-interrupted = Переривання
# Frames that came after, or long before, their time to be played.
network-late = Запізнілі кадри
network-early = Передчасні кадри
network-entity-id = ID сутності

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = недостатня пропускна здатність
msrp-failure-2 = недостатньо ресурсів мосту
msrp-failure-3 = недостатня пропускна здатність для класу трафіку
msrp-failure-4 = ID потоку використовується іншим передавачем
msrp-failure-5 = адреса призначення вже використовується
msrp-failure-6 = витіснено потоком вищого рангу
msrp-failure-7 = повідомлена затримка змінилася
msrp-failure-8 = вихідний порт не підтримує AVB
msrp-failure-9 = використайте іншу адресу призначення
msrp-failure-10 = вичерпано ресурси MSRP
msrp-failure-11 = вичерпано ресурси MMRP
msrp-failure-12 = неможливо зберегти адресу призначення
msrp-failure-13 = пріоритет не є пріоритетом класу SR
msrp-failure-14 = кадри завеликі для середовища передавання
msrp-failure-15 = досягнуто обмеження fan-in для порту
msrp-failure-16 = змінилося перше значення для зареєстрованого потоку
msrp-failure-17 = VLAN заблоковано на вихідному порту
msrp-failure-18 = тегування VLAN вимкнено на вихідному порту
msrp-failure-19 = невідповідність пріоритету класу SR
msrp-failure-unknown = невідома причина
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, на мосту { $bridge }

## The macOS menu bar

menu-about = Про plaa
menu-settings = Параметри…
menu-services = Послуги
menu-hide = Сховати plaa
menu-hide-others = Сховати інші
menu-show-all = Показати всі
menu-quit = Вийти з plaa
menu-file = Файл
menu-open = Відкрити…
menu-open-address = Відкрити адресу…
menu-close = Закрити вікно
menu-edit = Редагування
menu-cut = Вирізати
menu-copy = Скопіювати
menu-paste = Вставити
menu-select-all = Вибрати все
menu-undo = Відмінити
menu-redo = Повторити
menu-trim = Обрізати…
menu-split = Розділити кліп
menu-rotate-left = Повернути ліворуч
menu-rotate-right = Повернути праворуч
menu-flip-horizontal = Віддзеркалити горизонтально
menu-flip-vertical = Віддзеркалити вертикально
menu-revert = Повернути до оригіналу
menu-export = Експортувати…
menu-view = Вигляд
menu-playback = Відтворення
menu-play-pause = Відтворити або призупинити
menu-to-start = Перейти на початок
menu-previous-chapter = Попередній розділ
menu-next-chapter = Наступний розділ
menu-faster = Швидше
menu-slower = Повільніше
menu-normal-speed = Звичайна швидкість
menu-ab-loop = Повтор між двома точками
menu-volume-up = Гучніше
menu-volume-down = Тихіше
menu-mute = Вимкнути або ввімкнути звук
menu-next-subtitles = Наступні субтитри
menu-next-audio = Наступна аудіодоріжка
menu-window = Вікно
menu-minimize = Згорнути
menu-zoom = Масштаб
menu-bring-all-to-front = Усі на передній план

## Editing

action-edit = Редагувати
title-edited = { $name } (змінено)
edit-trim = Обрізати
edit-trim-apply = Обрізати
edit-split = Розділити кліп
edit-remove-clip = Видалити кліп
edit-done = Готово
edit-rotate-left = Повернути ліворуч
edit-rotate-right = Повернути праворуч
edit-flip-horizontal = Віддзеркалити горизонтально
edit-flip-vertical = Віддзеркалити вертикально
edit-undo = Відмінити
edit-redo = Повторити
edit-revert = Повернути до оригіналу
edit-export = Експортувати…

## Exporting

export-title = Експорт
export-format = Формат
export-size = Розмір
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Лише звук
export-size-original = { $lines }p, як є
export-on-graphics-card = Кодування на відеокарті ({ $api })
export-on-processor = Кодування на процесорі, це триває довше
export-no-encoder = Тут немає доступного кодувальника для цього формату
export-checking = Пошук кодувальника…
export-sound-format = Формат звуку
export-quality = Якість
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Зі стисненням; відтворюється майже всюди, зокрема на пристроях Apple
export-mp3-note = Зі стисненням; відтворюється всюди
export-opus-note = Зі стисненням; найкращий звук для свого розміру, хоча не кожен програвач його підтримує
export-flac-note = Без втрат: звук точно як є, у відкритому форматі
export-alac-note = Без втрат: звук точно як є, у форматі, який відтворюють пристрої Apple
export-wav-note = Без стиснення: звук точно як є, у найбільших файлах
export-kilobits = { $rate } kb/s
export-bits = { $bits } біт
export-rate-lower = Буде збережено з частотою { $rate } kHz, нижчою за { $file } kHz у файлі
export-choose = Експортувати…
export-name = { $name } (експорт)
export-name-edited = { $name } (змінено)
export-progress = Експорт «{ $name }»
export-stop = Зупинити експорт
notice-exported = Експортовано «{ $name }»
notice-export-stopped = Експорт зупинено
notice-export-failed = Не вдалося експортувати: { $error }
close-title = Закрити без експорту?
close-unexported = Зміни у файлі «{ $name }» буде втрачено, якщо їх не експортувати.
close-exporting = Експорт «{ $name }» буде зупинено, а вже записане буде втрачено.
close-anyway = Закрити
close-quit-anyway = Вийти

## Recording

record-title = Запис звуку
record-file-name = Запис звуку
record-start = Записати
record-stop = Зупинити запис
record-no-input = Немає мікрофона
record-microphone = Мікрофон
record-default-microphone = Системний за замовчуванням
record-playing = Те, що відтворює комп’ютер
record-mono = моно
record-stereo = стерео
record-channels = { $count ->
    [one] { $count } канал
    [few] { $count } канали
    [many] { $count } каналів
   *[other] { $count } каналу
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, зберігається з частотою { $saved } kHz
action-record-sound = Записати звук
open-record-sound = Новий запис звуку
start-record-sound = Записати звук
menu-record-sound = Новий запис звуку
record-screen-title = Запис екрана
record-screen-file-name = Запис екрана
record-screen-whole = Екран
record-screen-number = Екран { $number }
record-screen-main = Екран { $number } (основний)
record-screens = Екрани
record-windows = Вікна
record-screen-what = Що записувати
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Кожен кадр
record-rate-tip = Кадрів за секунду
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] пропущено { $count } кадр
    [few] пропущено { $count } кадри
    [many] пропущено { $count } кадрів
   *[other] пропущено { $count } кадру
}
record-screen-window = Вікно
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Спільний доступ припинено
record-screen-again = Вибрати знову
record-screen-failed = Не вдалося записати екран
record-sound = Звук
record-no-sound = Без звуку
# How much louder the input is taken, for a quiet microphone.
record-gain = Підсилення
record-gain-value = +{ $decibels } dB
record-level-automatically = Автоматично регулювати рівень
record-hide-title = plaa ховається під час запису
record-hide-tray = Щоб зупинити запис, клацніть червону крапку, яку plaa показує в системному треї, або знову відкрийте plaa.
record-hide-reopen = Щоб зупинити запис, знову відкрийте plaa.
record-hide-start = Почати запис
record-tray-title = plaa записує екран
record-tray-tip = Клацніть, щоб зупинити запис
action-record-screen = Записати екран
open-record-screen = Новий запис екрана
start-record-screen = Записати екран
menu-record-screen = Новий запис екрана
record-camera-title = Запис з камери
record-camera-file-name = Запис з камери
record-camera = Камера
record-camera-failed = Не вдалося відкрити камеру
record-name-title = Назвіть запис
record-name-note = Його збережено як «{ $name }» у папці { $folder }.
record-name-label = Назва
record-name-keep = Залишити
record-name-rename = Перейменувати
record-name-taken = Там уже є файл «{ $name }».
record-name-invalid = Назва не може містити жодного з цих символів: / \ : * ? " < > |
record-name-failed = Не вдалося перейменувати: { $error }
action-record-camera = Записати з камери
open-record-camera = Новий запис з камери
start-record-camera = Записати з камери
menu-record-camera = Новий запис з камери
notice-no-camera = Немає камери для запису
notice-recorded = Збережено «{ $name }» у папці { $folder }
notice-record-failed = Не вдалося записати: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Дозволити зовнішнє керування
settings-allow-outside-control-note = ШІ-агенти, як-от Claude Code, можуть бачити, що відтворюється в plaa, і керувати ним через plaa --mcp. plaa питає перед кожним новим агентом.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Дозволено: { $agents }
settings-forget-agents = Забути
settings-ask-before-note = Питати, перш ніж агент:
settings-ask-reading = Побачить, що відтворюється, або зображення
settings-ask-playing = Керуватиме відтворенням, звуком або повноекранним режимом
settings-ask-opening = Відкриє файл або закриє вікно
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Дозволити агентові { $agent } керувати plaa?
agent-prompt-body = { $agent } просить доступ до зовнішнього керування plaa, щоб бачити, що відтворюється, і керувати ним. Зовнішнє керування можна вимкнути в параметрах.
agent-prompt-allow = Дозволити
agent-prompt-deny = Не дозволяти
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Дозволити агентові { $agent } бачити, що тут відтворюється?
agent-ask-play = Дозволити агентові { $agent } керувати відтворенням тут?
agent-ask-open = Дозволити агентові { $agent } відкрити або закрити вікно?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } просить використати «{ $tool }». У параметрах можна вибрати, про що питає plaa.

## Casting to a TV or speaker on the network

cast-looking = Пошук телевізорів і колонок…
cast-none = Не знайдено телевізора, колонки чи пристрою Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Зупинити відтворення на пристрої { $renderer }
notice-casting = Відтворення на пристрої { $renderer }.
notice-cast-failed = Не вдалося відтворити на пристрої { $renderer }: { $reason }
cast-web-page = Вебсторінки поки не можна транслювати: натомість відкрийте адресу самого відео.

## The assistant panel, and its models in Settings

assistant-add-another = Додати модель…
assistant-add-model = Асистентові потрібна модель: хмарна модель із ключем API або локальна модель.
assistant-ask = Запитайте про те, що відтворюється
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Чат ставав задовгим, щоб { $model } міг прочитати його повністю, тож його початок підсумовано. Асистент може знову поглянути на те, що бачив.
assistant-compacting = Підсумовується початок чату
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } на цьому комп’ютері
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } за адресою { $host }
assistant-new-chat = Новий чат
assistant-no-model = Спершу додайте модель у параметрах.
assistant-open-settings = Додати модель
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Розмова більше не вміщується в обсяг, який модель { $model } може прочитати за раз. Почніть новий чат або виберіть модель, яка читає більше.
assistant-problem-credit = Постачальник { $provider } повідомляє, що на балансі облікового запису немає коштів. Для нового облікового запису спершу потрібно поповнити баланс на сайті постачальника { $provider }, лише тоді ключ запрацює; потім спробуйте ще раз.
assistant-problem-key = Постачальник { $provider } відхилив ключ API. Перевірте його в параметрах.
assistant-problem-model = Постачальник { $provider } не має моделі з назвою { $model }. Перевірте її назву в параметрах.
# $message is what the provider said, untranslated.
assistant-problem-other = Модель { $model } не відповіла: { $message }
assistant-problem-rate = Постачальник { $provider } просить зменшити частоту запитів. Спробуйте ще раз трохи згодом.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = Постачальник { $provider } просить зменшити частоту запитів: { $message }
assistant-problem-refused = Модель { $model } відмовилася відповідати.
assistant-problem-unavailable = Постачальник { $provider } перевантажений або має проблеми. Спробуйте ще раз трохи згодом.
assistant-problem-unreachable = plaa не вдалося зв’язатися з постачальником { $provider }. Перевірте підключення або чи працює сервер.
assistant-running = Виконується…
assistant-send = Надіслати
assistant-shortened = Чат був задовгим, щоб { $model } міг прочитати його повністю, тож попередні результати інструментів пропущено. Асистент може знову поглянути на те, що бачив.
assistant-stop = Зупинити
assistant-stopped = Зупинено.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Тепер ви спілкуєтеся з моделлю { $model }.
assistant-thinking = Думає…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Міркування
assistant-title = Асистент
settings-assistant-add = Додати модель
settings-assistant-add-button = Додати
# $model is the model's name.
settings-assistant-added = Модель { $model } відповіла й додана.
settings-assistant-added-tag = Додано
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Адреса, наприклад { $example }
settings-assistant-change-key = Змінити ключ
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Сервер, сумісний з OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Контекст
settings-assistant-context-note = Що більший контекст, то більшу частину чату асистент зможе враховувати, але модель займе більше пам’яті й може відповідати повільніше.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } тис. токенів
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Відкрити сторінку
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = Сервер { $provider } працює за адресою { $address }.
settings-assistant-found-none = На сервері { $provider } ще немає моделей. Завантажте модель за його допомогою і пошукайте знову.
settings-assistant-get-key = Отримати ключ API
settings-assistant-get-server = Завантажити { $provider }
settings-assistant-in-use = Використовується
settings-assistant-key = Ключ API
# $error is what the keychain said.
settings-assistant-key-failed = Не вдалося зберегти ключ у в’язці ключів: { $error }
settings-assistant-key-kept = Ваш ключ для постачальника { $provider } зберігається в системній в’язці ключів.
settings-assistant-key-needed = Цій моделі потрібен ключ API.
settings-assistant-key-refused = Постачальник { $provider } відхилив цей ключ. Перевірте, чи його скопійовано повністю і з потрібного облікового запису.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Створіть ключ на сайті постачальника { $provider } і вставте його сюди.
settings-assistant-look-again = Шукати знову
settings-assistant-looking = Пошук моделей…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Модель, наприклад { $example }
settings-assistant-no-server = На цьому комп’ютері не знайдено запущеного сервера { $provider }. Запустіть його або вкажіть його адресу нижче.
settings-assistant-no-tools = Не вміє використовувати інструменти, потрібні асистентові
settings-assistant-none = Моделей ще немає. Додайте модель нижче: локальна модель, наприклад в Ollama, залишається на цьому комп’ютері; хмарній моделі потрібен ключ API від її постачальника.
settings-assistant-note = Моделі, з якими може спілкуватися панель асистента. Ключі зберігаються в системній в’язці ключів.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Використати іншу адресу
settings-assistant-recommended = Рекомендовано
settings-assistant-remove = Видалити
settings-assistant-sees = Бачить зображення
settings-assistant-trying = Перевірка…
settings-assistant-use = Використати
settings-assistant-use-key = Продовжити
settings-assistant-uses-tools = Використовує інструменти
