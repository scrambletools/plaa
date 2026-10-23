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
language-name = Русский

## Common

app-name = plaa
common-cancel = Отменить
common-close = Закрыть
common-more = Ещё

## Command line

usage-help =
    Использование: plaa [ФАЙЛ ИЛИ АДРЕС]...

    Воспроизведение видео- и аудиофайлов, а также потоков по адресам вроде
    https://example.com/live.m3u8. Каждый из них открывается в окне
    запущенного plaa; если он не запущен, он запустится.

    Параметры:
      -h, --help        Показать эту справку
      -V, --version     Показать версию
          --interfaces  Показать проводные интерфейсы и то, может ли plaa
                        отправлять через них потоки AVB
          --mcp         Предоставить MCP через stdin и stdout, чтобы ИИ-агенты
                        видели, что воспроизводит запущенный plaa, и управляли им

## Start window

start-hint = Откройте видео- или аудиофайл, поток или веб-видео.
start-open = Открыть
start-open-address = Открыть адрес
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Подключиться

## Actions, as buttons, tooltips and menus name them

action-open = Открыть
action-open-address = Открыть адрес
open-file = Открыть файл…
open-address = Открыть адрес…
action-settings = Настройки
action-info = Инспектор
action-loop = Повтор
action-screenshot = Сохранить кадр как изображение
action-play = Воспроизвести
action-pause = Пауза
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Назад на { $seconds } секунду
    [few] Назад на { $seconds } секунды
    [many] Назад на { $seconds } секунд
   *[other] Назад на { $seconds } секунды
}
action-forward = { $seconds ->
    [one] Вперёд на { $seconds } секунду
    [few] Вперёд на { $seconds } секунды
    [many] Вперёд на { $seconds } секунд
   *[other] Вперёд на { $seconds } секунды
}
action-mute = Выключить звук
action-unmute = Включить звук
action-speed = Скорость
action-tracks = Аудио и субтитры
action-full-screen = Полноэкранный режим
action-playlist = Плейлист
action-chapters = Главы
action-picture-in-picture = Картинка в картинке
action-previous = Предыдущий
action-next = Следующий
action-exit-full-screen = Выйти из полноэкранного режима
action-quality = Качество
action-visualizer = Визуализация
action-network = Сетевое аудио
action-cast = Трансляция
action-connect = Подключиться
action-assistant = Ассистент

## What a sound file shows under its cover

visualizer-spectrum = Спектр
visualizer-waveform = Форма волны
visualizer-prism = Призма
visualizer-cover = Только обложка

## The floating controls

overlay-keep-shown = Закрепить элементы управления
overlay-auto-hide = Показывать элементы управления поверх изображения

## Audio and subtitle tracks

tracks-audio = Аудио
tracks-subtitles = Субтитры
tracks-off = Выкл.
# A track the file gives no name or language, by its number.
tracks-numbered = Дорожка { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Только звук

## Inspector

info-title = Инспектор
playlist-title = Плейлист
chapters-numbered = Глава { $number }
info-file = Файл
info-name = Имя
info-address = Адрес
info-folder = Папка
info-size = Размер
info-format = Формат
info-duration = Длительность
info-bit-rate = Битрейт
info-video = Видео
info-audio = Аудио
info-codec = Кодек
info-dimensions = Размеры
info-frame-rate = Частота кадров
info-frames-per-second = { $rate } кадр/с
info-pixel-format = Формат пикселей
info-decoding = Декодирование
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Видеокарта ({ $api })
info-decoding-software = Процессор
info-sample-rate = Частота дискретизации
info-hertz = { $rate } Hz
info-channels = Каналы
info-language = Язык
info-no-sound = Без звука
info-tags = Теги
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Передатчик
info-stream = Поток
# The talker's listeners other than plaa.
info-other-listeners = Другие приёмники
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Класс SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Новый формат отключит { $count ->
        [one] { $count } другой приёмник передатчика.
        [few] { $count } других приёмника передатчика.
        [many] { $count } других приёмников передатчика.
       *[other] { $count } другого приёмника передатчика.
    }

## Dialogs

dialog-open = Открыть
dialog-filter-media = Видео и аудио
dialog-filter-all = Все файлы

address-title = Открыть адрес
address-note = Веб-страница с видео, например с YouTube, либо поток или файл в сети, например https://example.com/live.m3u8.
address-label = Адрес
address-open = Открыть
address-recent = Недавние
address-forget = Удалить из недавних

## Notices

notice-dialog-failed = Не удалось открыть окно выбора файла: { $error }
notice-screenshot-saved = Файл «{ $name }» сохранён.
notice-screenshot-failed = Не удалось сохранить изображение: { $error }
notice-no-sound = Воспроизведение без звука: { $reason }
notice-loop-from = Повтор с { $time }. Нажмите ещё раз там, где он должен закончиться.
notice-loop-between = Повтор с { $from } до { $to }.
notice-loop-off = Повтор выключен.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Похоже, это веб-страница. plaa воспроизводит видео с веб-страниц с помощью yt-dlp, но эта программа не установлена.
error-empty-list = В этом списке нечего воспроизводить.

## Settings

settings-title = Настройки
settings-tab-appearance = Оформление
settings-tab-playback = Воспроизведение
settings-tab-storage = Хранение
settings-tab-network = Сеть
settings-tab-language = Язык
settings-tab-agents = Агенты
settings-tab-assistant = Ассистент
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (сборка для разработки, { $build })
settings-choose = Выбрать…

settings-appearance-system = Как в системе
settings-appearance-light = Светлое
settings-appearance-dark = Тёмное
settings-colors = Цвета
settings-system-accent = Использовать акцентный цвет системы
settings-omarchy-note = Цвета строятся на основе акцентного цвета темы «{ $theme }».
settings-system-accent-note = Цвета строятся на основе акцентного цвета системы.
settings-system-accent-none = В системе нет акцентного цвета, поэтому plaa использует цвет, выбранный ниже.
settings-accent-chosen-note = Цвета строятся на основе цвета, выбранного ниже.
settings-controls = Элементы управления
settings-overlay = Плавающие элементы управления
settings-overlay-note = Панель инструментов и элементы управления воспроизведением располагаются поверх изображения или обложки и скрываются, когда указатель неподвижен или находится вне окна.
settings-overlay-delay = Скрывать через
settings-overlay-delay-note = Сколько указатель должен пробыть неподвижно над видео или обложкой, прежде чем элементы управления скроются.
settings-seconds = { $seconds } с
settings-transparency = Прозрачность
settings-transparency-note = Насколько изображение просвечивает сквозь плавающие элементы управления.
settings-percent = { $percent }%
settings-corner-radius = Радиус скругления
settings-corner-radius-note = Для диалогов и плавающих элементов управления.
settings-pixels = { $pixels } px
settings-animations = Анимация
settings-animations-note = Выдвижные полосы и панели, раскрывающиеся диалоги и пружинящие кнопки.
settings-animations-reduced = Отключена, пока в системе включено уменьшение движения.

settings-hardware = Использовать видеокарту
settings-hardware-note = На ней декодируется видео и кодируется экспорт: это экономит энергию, большие видео воспроизводятся плавно, а экспорт идёт быстрее. Действует для файлов, открытых после этого.
settings-audio-device = Вывод звука
settings-audio-device-system = Системное устройство по умолчанию
settings-resume = Продолжать с места остановки
settings-resume-note = Файлы начинаются с того места, на котором их закрыли в последний раз.
settings-folder-playlists = Воспроизводить другие файлы папки
settings-folder-playlists-note = При открытии одного файла в список попадают остальные файлы из его папки, и они воспроизводятся после него.
settings-streams = Потоки и веб-видео
settings-stream-quality = Качество
settings-stream-quality-note = Для потоков и веб-видео, доступных в нескольких вариантах качества. Действует для того, что откроется после этого.
settings-quality-auto = Автоматически, по высоте экрана
settings-quality-best = Наилучшее
settings-web-pages = Воспроизводить веб-видео с помощью yt-dlp
settings-web-pages-found = yt-dlp находит видео на страницах вроде YouTube.
settings-web-pages-missing = yt-dlp не установлен. Установите его, чтобы воспроизводить видео на страницах вроде YouTube.
settings-folders = Папки
settings-sound-recordings = Звукозаписи
settings-video-recordings = Видеозаписи
settings-screenshots = Сохранённые кадры

settings-interface-language = Язык интерфейса
# The first entry of the language list.
settings-language-system = Как в системе
settings-track-languages = Языки дорожек
settings-track-languages-note = Коды языков, например en или fra, через запятую в порядке предпочтения. plaa выбирает аудио и субтитры на этих языках, если они есть в файле.
settings-audio-languages = Аудио
settings-subtitle-languages = Субтитры

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Интерфейс AVB
settings-avb-interface-note = Проводной порт, на котором plaa ищет передатчики и приёмники AVB и Milan. Для отправки нужен порт с аппаратными часами PTP, например у Intel I210.
settings-avb-off = Выкл.
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (только просмотр: нет часов PTP)
settings-avb-off-mac = { $interface } (только просмотр: AVB выключен)
settings-avb-interface-note-mac = Проводной порт, на котором plaa ищет передатчики и приёмники AVB и Milan. Отправка идёт через собственную сущность AVB этого Mac, а для этого нужно включить AVB для порта в браузере сетевых устройств программы «Настройка Audio-MIDI».
settings-avb-talker-note-mac = plaa отправляет звук окна, в котором последним шло воспроизведение, через собственную сущность AVB этого Mac подключённым к ней приёмникам.
settings-avb-talker = Отправлять воспроизводимый звук
settings-avb-talker-note = plaa становится передатчиком Milan и отправляет звук окна, в котором последним шло воспроизведение, подключённым к нему приёмникам.
settings-avb-name = Имя в сети
settings-avb-name-note = Контроллеры показывают plaa как «{ $name }». Нажмите Enter, чтобы переименовать.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa на { $computer }
avb-could-not-start = Не удалось использовать { $interface }: { $error }
network-title = Сетевое аудио
network-off = Чтобы воспроизводить в сети, включите «Отправлять воспроизводимый звук» в настройках.
network-blocked = Звук не отправляется: { $reason }
network-waiting = Всё готово к отправке, ожидание подключения приёмника.
network-sending = Идёт отправка подключённым приёмникам.
network-looking = Поиск приёмников…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Запуск ptp4l, который поддерживает сетевое время для отправки потока plaa…
network-ptp-elsewhere = ptp4l работает на другом интерфейсе, поэтому здесь у потока plaa нет сетевого времени и приёмники могут его не воспроизвести. Выберите тот интерфейс в настройках или запустите ptp4l на этом.
network-ptp-missing = ptp4l не запущен, поэтому у потока plaa нет сетевого времени и приёмники могут его не воспроизвести. Установите linuxptp вместе с пакетом plaa или установите triib, который его запускает.
network-ptp-failed = ptp4l не запущен, поэтому у потока plaa нет сетевого времени и приёмники могут его не воспроизвести. plaa не удалось его запустить: { $reason }
network-no-listeners = В сети нет приёмников, которые могут воспроизвести звук plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Воспроизвести на приёмнике { $listener }
network-stop-on = Остановить воспроизведение на приёмнике { $listener }
network-choose-format = Выберите формат для воспроизведения на приёмнике { $listener }
network-failed = Не удалось подключиться: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Обновите triib, чтобы воспроизводить здесь.
network-choose = Выберите приёмник, чтобы увидеть, как он принимает поток.
network-no-sources = В сети нет передатчиков с форматом, который воспроизводит plaa, или способных переключиться на такой формат.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Перестать слушать передатчик { $talker }
network-listen-update-triib = Обновите triib, чтобы слушать здесь: plaa воспроизводит только то, что и так доходит до этого компьютера.
network-listen-lite = Порт работает в режиме AVB Lite, который не резервирует потоки: plaa воспроизводит только то, что и так доходит до этого компьютера.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Поиск передатчиков…
# plaa listens through its entity, which exists only while it sends.
connect-off = Чтобы слушать здесь передатчики, включите «Отправлять воспроизводимый звук» в настройках.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa не может слушать здесь: { $reason }
connect-sending = Отправляют в формате, который воспроизводит plaa
connect-changing = Переключатся на формат, который воспроизводит plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, в формате { $format }
connect-change-cuts-off = { $talker }, в формате { $format }, с отключением { $count ->
        [one] { $count } другого приёмника
        [few] { $count } других приёмников
        [many] { $count } других приёмников
       *[other] { $count } другого приёмника
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Состояние
network-not-connected = Не подключено
network-from = Источник: { $stream }
network-from-receiving = Источник: { $stream }, идёт приём
network-from-waiting = Источник: { $stream }, ожидание передатчика
network-from-failed = Источник: { $stream }, сбой резервирования у передатчика: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Медиачасы
network-locked = Синхронизированы с потоком
network-not-locked = Не синхронизированы
network-format = Формат
# The formats both the listener and plaa's stream can be in.
network-formats = Форматы, которые он принимает от plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID потока
# The MAC address the stream is sent to.
network-destination = Адрес назначения
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Задержка
network-microseconds = { $time } µs
network-frames = Получено кадров
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Кадры не по порядку
# How often the stream stopped coming while it was taken.
network-interrupted = Прерывания
# Frames that came after, or long before, their time to be played.
network-late = Опоздавшие кадры
network-early = Преждевременные кадры
network-entity-id = ID сущности

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = недостаточная пропускная способность
msrp-failure-2 = недостаточно ресурсов моста
msrp-failure-3 = недостаточная пропускная способность для класса трафика
msrp-failure-4 = ID потока используется другим передатчиком
msrp-failure-5 = адрес назначения уже используется
msrp-failure-6 = вытеснение потоком более высокого ранга
msrp-failure-7 = изменилась заявленная задержка
msrp-failure-8 = выходной порт не поддерживает AVB
msrp-failure-9 = используйте другой адрес назначения
msrp-failure-10 = недостаточно ресурсов MSRP
msrp-failure-11 = недостаточно ресурсов MMRP
msrp-failure-12 = не удаётся сохранить адрес назначения
msrp-failure-13 = приоритет не является приоритетом класса SR
msrp-failure-14 = кадры слишком велики для среды передачи
msrp-failure-15 = достигнут предел fan-in для порта
msrp-failure-16 = изменилось первое значение зарегистрированного потока
msrp-failure-17 = VLAN заблокирована на выходном порту
msrp-failure-18 = тегирование VLAN отключено на выходном порту
msrp-failure-19 = несоответствие приоритета класса SR
msrp-failure-unknown = неизвестная причина
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, на мосту { $bridge }

## The macOS menu bar

menu-about = О программе plaa
menu-settings = Настройки…
menu-services = Службы
menu-hide = Скрыть plaa
menu-hide-others = Скрыть остальные
menu-show-all = Показать все
menu-quit = Завершить plaa
menu-file = Файл
menu-open = Открыть…
menu-open-address = Открыть адрес…
menu-close = Закрыть окно
menu-edit = Правка
menu-cut = Вырезать
menu-copy = Скопировать
menu-paste = Вставить
menu-select-all = Выбрать все
menu-undo = Отменить
menu-redo = Повторить
menu-trim = Обрезать…
menu-split = Разделить клип
menu-rotate-left = Повернуть влево
menu-rotate-right = Повернуть вправо
menu-flip-horizontal = Отразить по горизонтали
menu-flip-vertical = Отразить по вертикали
menu-revert = Вернуть к исходному
menu-export = Экспортировать…
menu-view = Вид
menu-playback = Воспроизведение
menu-play-pause = Воспроизвести или приостановить
menu-to-start = Перейти в начало
menu-previous-chapter = Предыдущая глава
menu-next-chapter = Следующая глава
menu-faster = Быстрее
menu-slower = Медленнее
menu-normal-speed = Обычная скорость
menu-ab-loop = Повтор между двумя точками
menu-volume-up = Громче
menu-volume-down = Тише
menu-mute = Выключить или включить звук
menu-next-subtitles = Следующие субтитры
menu-next-audio = Следующая аудиодорожка
menu-window = Окно
menu-minimize = Свернуть
menu-zoom = Изменить масштаб
menu-bring-all-to-front = Все окна на передний план

## Editing

action-edit = Редактировать
title-edited = { $name } (изменено)
edit-trim = Обрезать
edit-trim-apply = Обрезать
edit-split = Разделить клип
edit-remove-clip = Удалить клип
edit-done = Готово
edit-rotate-left = Повернуть влево
edit-rotate-right = Повернуть вправо
edit-flip-horizontal = Отразить по горизонтали
edit-flip-vertical = Отразить по вертикали
edit-undo = Отменить
edit-redo = Повторить
edit-revert = Вернуть к исходному
edit-export = Экспортировать…

## Exporting

export-title = Экспорт
export-format = Формат
export-size = Размер
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Только звук
export-size-original = { $lines }p, без изменений
export-on-graphics-card = Кодирование на видеокарте ({ $api })
export-on-processor = Кодирование на процессоре, что занимает больше времени
export-no-encoder = Не удаётся открыть кодировщик для этого формата
export-checking = Поиск кодировщика…
export-sound-format = Формат звука
export-quality = Качество
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Со сжатием; воспроизводится почти везде, включая устройства Apple
export-mp3-note = Со сжатием; воспроизводится везде
export-opus-note = Со сжатием; лучший звук для своего размера, хотя не все плееры его поддерживают
export-flac-note = Без потерь: звук в точности как есть, в открытом формате
export-alac-note = Без потерь: звук в точности как есть, в формате устройств Apple
export-wav-note = Без сжатия: звук в точности как есть, но файлы самые большие
export-kilobits = { $rate } kb/s
export-bits = { $bits } бит
export-rate-lower = Сохраняется с частотой { $rate } kHz, ниже исходной частоты файла ({ $file } kHz)
export-choose = Экспортировать…
export-name = { $name } (экспорт)
export-name-edited = { $name } (изменено)
export-progress = Экспорт «{ $name }»
export-stop = Остановить экспорт
notice-exported = Файл «{ $name }» экспортирован
notice-export-stopped = Экспорт остановлен
notice-export-failed = Не удалось экспортировать: { $error }
close-title = Закрыть без экспорта?
close-unexported = Изменения в файле «{ $name }» будут потеряны, если их не экспортировать.
close-exporting = Экспорт «{ $name }» будет остановлен, а всё, что уже записано, будет потеряно.
close-anyway = Закрыть
close-quit-anyway = Завершить

## Recording

record-title = Запись звука
record-file-name = Запись звука
record-start = Записать
record-stop = Остановить запись
record-no-input = Нет микрофона
record-microphone = Микрофон
record-default-microphone = Системный по умолчанию
record-playing = Звук, воспроизводимый компьютером
record-mono = моно
record-stereo = стерео
record-channels = { $count ->
    [one] { $count } канал
    [few] { $count } канала
    [many] { $count } каналов
   *[other] { $count } канала
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, сохраняется в { $saved } kHz
action-record-sound = Записать звук
open-record-sound = Новая запись звука
start-record-sound = Записать звук
menu-record-sound = Новая запись звука
record-screen-title = Запись экрана
record-screen-file-name = Запись экрана
record-screen-whole = Экран
record-screen-number = Экран { $number }
record-screen-main = Экран { $number } (основной)
record-screens = Экраны
record-windows = Окна
record-screen-what = Что записывать
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Все кадры
record-rate-tip = Кадров в секунду
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] пропущен { $count } кадр
    [few] пропущено { $count } кадра
    [many] пропущено { $count } кадров
   *[other] пропущено { $count } кадра
}
record-screen-window = Окно
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Общий доступ прекращён
record-screen-again = Выбрать снова
record-screen-failed = Не удалось записать экран
record-sound = Звук
record-no-sound = Без звука
# How much louder the input is taken, for a quiet microphone.
record-gain = Усиление
record-gain-value = +{ $decibels } dB
record-level-automatically = Регулировать уровень автоматически
record-hide-title = plaa скрывается во время записи
record-hide-tray = Чтобы остановить запись, нажмите красную точку, которую plaa показывает в системном трее, или снова откройте plaa.
record-hide-reopen = Чтобы остановить запись, снова откройте plaa.
record-hide-start = Начать запись
record-tray-title = plaa записывает экран
record-tray-tip = Нажмите, чтобы остановить запись
action-record-screen = Записать экран
open-record-screen = Новая запись экрана
start-record-screen = Записать экран
menu-record-screen = Новая запись экрана
record-camera-title = Запись с камеры
record-camera-file-name = Запись с камеры
record-camera = Камера
record-camera-failed = Не удалось открыть камеру
record-name-title = Назовите запись
record-name-note = Она сохранена как «{ $name }» в папке { $folder }.
record-name-label = Имя
record-name-keep = Оставить
record-name-rename = Переименовать
record-name-taken = Там уже есть файл «{ $name }».
record-name-invalid = Имя не может содержать ни один из этих символов: / \ : * ? " < > |
record-name-failed = Не удалось переименовать: { $error }
action-record-camera = Записать с камеры
open-record-camera = Новая запись с камеры
start-record-camera = Записать с камеры
menu-record-camera = Новая запись с камеры
notice-no-camera = Нет камеры для записи
notice-recorded = Файл «{ $name }» сохранён в папке { $folder }
notice-record-failed = Не удалось записать: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Разрешить внешнее управление
settings-allow-outside-control-note = ИИ-агенты, например Claude Code, могут видеть, что воспроизводится в plaa, и управлять им через plaa --mcp. plaa спрашивает перед каждым новым агентом.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Разрешено: { $agents }
settings-forget-agents = Забыть
settings-ask-before-note = Спрашивать, прежде чем агент:
settings-ask-reading = Увидит, что воспроизводится, или изображение
settings-ask-playing = Будет управлять воспроизведением, звуком или полноэкранным режимом
settings-ask-opening = Откроет файл или закроет окно
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Разрешить агенту { $agent } управлять plaa?
agent-prompt-body = { $agent } просит доступ к внешнему управлению plaa, чтобы видеть, что воспроизводится, и управлять им. Внешнее управление можно отключить в настройках.
agent-prompt-allow = Разрешить
agent-prompt-deny = Не разрешать
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Разрешить агенту { $agent } видеть, что здесь воспроизводится?
agent-ask-play = Разрешить агенту { $agent } управлять воспроизведением здесь?
agent-ask-open = Разрешить агенту { $agent } открыть или закрыть окно?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } просит использовать «{ $tool }». В настройках можно выбрать, о чём спрашивает plaa.

## Casting to a TV or speaker on the network

cast-looking = Поиск телевизоров и колонок…
cast-none = Не найдено телевизоров, колонок или устройств Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Остановить воспроизведение на устройстве { $renderer }
notice-casting = Воспроизведение на устройстве { $renderer }.
notice-cast-failed = Не удалось воспроизвести на устройстве { $renderer }: { $reason }
cast-web-page = Веб-страницы пока нельзя транслировать: вместо этого откройте адрес самого видео.

## The assistant panel, and its models in Settings

assistant-add-another = Добавить модель…
assistant-add-model = Ассистенту нужна модель: облачная модель с ключом API или локальная модель.
assistant-ask = Спросите о том, что воспроизводится
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Чат становился слишком длинным, чтобы { $model } мог прочитать его целиком, поэтому его начало было кратко изложено. Ассистент может снова взглянуть на то, что видел.
assistant-compacting = Подводится итог начала чата
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } на этом компьютере
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } по адресу { $host }
assistant-new-chat = Новый чат
assistant-no-model = Сначала добавьте модель в настройках.
assistant-open-settings = Добавить модель
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Разговор больше не помещается в объём, который модель { $model } может прочитать за раз. Начните новый чат или выберите модель, которая читает больше.
assistant-problem-credit = Поставщик { $provider } сообщает, что на балансе учётной записи нет средств. Для новой учётной записи сначала нужно пополнить баланс на сайте поставщика { $provider }, только тогда ключ заработает; затем повторите попытку.
assistant-problem-key = Поставщик { $provider } отклонил ключ API. Проверьте его в настройках.
assistant-problem-model = У поставщика { $provider } нет модели с именем { $model }. Проверьте её имя в настройках.
# $message is what the provider said, untranslated.
assistant-problem-other = Модель { $model } не ответила: { $message }
assistant-problem-rate = Поставщик { $provider } просит снизить частоту запросов. Повторите попытку чуть позже.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = Поставщик { $provider } просит снизить частоту запросов: { $message }
assistant-problem-refused = Модель { $model } отказалась отвечать.
assistant-problem-unavailable = Поставщик { $provider } перегружен или испытывает проблемы. Повторите попытку чуть позже.
assistant-problem-unreachable = plaa не удалось связаться с поставщиком { $provider }. Проверьте подключение или работает ли сервер.
assistant-running = Выполняется…
assistant-send = Отправить
assistant-shortened = Чат был слишком длинным, чтобы { $model } мог прочитать его целиком, поэтому прежние результаты инструментов опущены. Ассистент может снова взглянуть на то, что видел.
assistant-stop = Остановить
assistant-stopped = Остановлено.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Теперь вы общаетесь с моделью { $model }.
assistant-thinking = Думает…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Размышления
assistant-title = Ассистент
settings-assistant-add = Добавить модель
settings-assistant-add-button = Добавить
# $model is the model's name.
settings-assistant-added = Модель { $model } ответила и добавлена.
settings-assistant-added-tag = Добавлена
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Адрес, например { $example }
settings-assistant-change-key = Изменить ключ
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Сервер, совместимый с OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Контекст
settings-assistant-context-note = Чем больше контекст, тем большую часть чата ассистент сможет учитывать, но модель займёт больше памяти и может отвечать медленнее.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } тыс. токенов
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Открыть страницу
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = Сервер { $provider } работает по адресу { $address }.
settings-assistant-found-none = На сервере { $provider } пока нет моделей. Скачайте модель с его помощью и поищите снова.
settings-assistant-get-key = Получить ключ API
settings-assistant-get-server = Скачать { $provider }
settings-assistant-in-use = Используется
settings-assistant-key = Ключ API
# $error is what the keychain said.
settings-assistant-key-failed = Не удалось сохранить ключ в связке ключей: { $error }
settings-assistant-key-kept = Ваш ключ для поставщика { $provider } хранится в системной связке ключей.
settings-assistant-key-needed = Этой модели нужен ключ API.
settings-assistant-key-refused = Поставщик { $provider } отклонил этот ключ. Проверьте, что он скопирован целиком и из нужной учётной записи.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Создайте ключ на сайте поставщика { $provider } и вставьте его сюда.
settings-assistant-look-again = Искать снова
settings-assistant-looking = Поиск моделей…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Модель, например { $example }
settings-assistant-no-server = На этом компьютере не найден работающий сервер { $provider }. Запустите его или укажите его адрес ниже.
settings-assistant-no-tools = Не умеет использовать инструменты, нужные ассистенту
settings-assistant-none = Моделей пока нет. Добавьте модель ниже: локальная модель, например в Ollama, остаётся на этом компьютере; облачной модели нужен ключ API от её поставщика.
settings-assistant-note = Модели, с которыми может общаться панель ассистента. Ключи хранятся в системной связке ключей.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Использовать другой адрес
settings-assistant-recommended = Рекомендуется
settings-assistant-remove = Удалить
settings-assistant-sees = Видит изображения
settings-assistant-trying = Проверка…
settings-assistant-use = Использовать
settings-assistant-use-key = Продолжить
settings-assistant-uses-tools = Использует инструменты
