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
language-name = فارسی

## Common

app-name = plaa
common-cancel = لغو
common-close = بستن
common-more = بیشتر

## Command line

usage-help =
    کاربرد: plaa [پرونده یا نشانی]...

    پخش پرونده‌های ویدیو و صدا، و جریان‌ها در نشانی‌هایی مانند
    https://example.com/live.m3u8. هر کدام در پنجره‌ای از plaa در حال اجرا
    باز می‌شود و اگر plaa در حال اجرا نباشد، اجرا می‌شود.

    گزینه‌ها:
      -h, --help        نمایش این راهنما
      -V, --version     نمایش نسخه
          --interfaces  نمایش رابط‌های سیمی و اینکه plaa می‌تواند
                        جریان‌های AVB را روی آن‌ها بفرستد یا نه
          --mcp         ارائهٔ MCP روی stdin و stdout تا عامل‌های هوش مصنوعی
                        ببینند plaa در حال اجرا چه پخش می‌کند و آن را کنترل کنند

## Start window

start-hint = یک پروندهٔ ویدیو یا صدا، یک جریان یا یک ویدیوی وب را باز کنید.
start-open = باز کردن
start-open-address = باز کردن نشانی

## Actions, as buttons, tooltips and menus name them

action-open = باز کردن
action-open-address = باز کردن نشانی
open-file = باز کردن پرونده…
open-address = باز کردن نشانی…
action-settings = تنظیمات
action-info = بازرس
action-loop = تکرار
action-screenshot = ذخیرهٔ فریم به‌صورت تصویر
action-play = پخش
action-pause = مکث
# $seconds is how far the button moves, such as 10.
action-back = { $seconds } ثانیه عقب
action-forward = { $seconds } ثانیه جلو
action-mute = قطع صدا
action-unmute = وصل صدا
action-speed = سرعت
action-tracks = صدا و زیرنویس
action-full-screen = تمام‌صفحه
action-playlist = فهرست پخش
action-chapters = فصل‌ها
action-picture-in-picture = تصویر در تصویر
action-previous = قبلی
action-next = بعدی
action-exit-full-screen = خروج از حالت تمام‌صفحه
action-quality = کیفیت
action-visualizer = نمایش بصری
action-network = صدای شبکه

## What a sound file shows under its cover

visualizer-spectrum = طیف
visualizer-waveform = شکل موج
visualizer-prism = منشور
visualizer-cover = فقط جلد

## The floating controls

overlay-keep-shown = ثابت نگه‌داشتن کنترل‌ها
overlay-auto-hide = شناور کردن کنترل‌ها روی تصویر

## Audio and subtitle tracks

tracks-audio = صدا
tracks-subtitles = زیرنویس‌ها
tracks-off = خاموش
# A track the file gives no name or language, by its number.
tracks-numbered = ترک { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = فقط صدا

## Inspector

info-title = بازرس
playlist-title = فهرست پخش
chapters-numbered = فصل { $number }
info-file = پرونده
info-name = نام
info-address = نشانی
info-folder = پوشه
info-size = اندازه
info-format = قالب
info-duration = مدت
info-bit-rate = نرخ بیت
info-video = ویدیو
info-audio = صدا
info-codec = کدک
info-dimensions = ابعاد
info-frame-rate = نرخ فریم
info-frames-per-second = { $rate } فریم در ثانیه
info-pixel-format = قالب پیکسل
info-decoding = کدگشایی
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = کارت گرافیک ({ $api })
info-decoding-software = پردازنده
info-sample-rate = نرخ نمونه‌برداری
info-hertz = { $rate } Hz
info-channels = کانال‌ها
info-language = زبان
info-no-sound = بدون صدا
info-tags = برچسب‌ها

## Dialogs

dialog-open = باز کردن
dialog-filter-media = ویدیو و صدا
dialog-filter-all = همهٔ پرونده‌ها

address-title = باز کردن نشانی
address-note = صفحهٔ وبی دارای ویدیو، مانند صفحهٔ یک ویدیوی YouTube، یا جریان یا پرونده‌ای روی شبکه، مانند https://example.com/live.m3u8.
address-label = نشانی
address-open = باز کردن
address-recent = موارد اخیر
address-forget = حذف از موارد اخیر

## Notices

notice-dialog-failed = پنجرهٔ انتخاب پرونده باز نشد: { $error }
notice-screenshot-saved = «{ $name }» ذخیره شد.
notice-screenshot-failed = ذخیرهٔ تصویر ممکن نشد: { $error }
notice-no-sound = پخش بدون صدا: { $reason }
notice-loop-from = تکرار از { $time }. در جایی که باید تمام شود، دوباره فشار دهید.
notice-loop-between = تکرار از { $from } تا { $to }.
notice-loop-off = تکرار خاموش شد.

## Errors, shown in place of what could not play

error-needs-yt-dlp = به نظر می‌رسد این یک صفحهٔ وب است. plaa ویدیوهای صفحه‌های وب را با yt-dlp پخش می‌کند که نصب نشده است.
error-empty-list = این فهرست چیزی برای پخش ندارد.

## Settings

settings-title = تنظیمات
settings-tab-appearance = ظاهر
settings-tab-playback = پخش
settings-tab-network = شبکه
settings-tab-language = زبان
settings-tab-agents = عامل‌ها
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (نسخهٔ توسعه، { $build })
settings-choose = انتخاب…

settings-appearance-system = پیروی از سیستم
settings-appearance-light = روشن
settings-appearance-dark = تیره
settings-colors = رنگ‌ها
settings-system-accent = استفاده از رنگ تأکیدی سیستم
settings-omarchy-note = رنگ‌ها از رنگ تأکیدی پوستهٔ «{ $theme }» ساخته می‌شوند.
settings-system-accent-note = رنگ‌ها از رنگ تأکیدی سیستم ساخته می‌شوند.
settings-system-accent-none = سیستم رنگ تأکیدی ندارد، پس plaa از رنگ انتخاب‌شده در پایین استفاده می‌کند.
settings-accent-chosen-note = رنگ‌ها از رنگ انتخاب‌شده در پایین ساخته می‌شوند.
settings-controls = کنترل‌ها
settings-overlay = کنترل‌های شناور
settings-overlay-note = نوار ابزار و کنترل‌های پخش روی تصویر یا جلد شناور می‌شوند و وقتی نشانگر بی‌حرکت بماند یا بیرون از پنجره باشد، کنار می‌روند.
settings-overlay-delay = پنهان شدن پس از
settings-overlay-delay-note = مدتی که نشانگر باید روی محتوای در حال پخش بی‌حرکت بماند تا کنترل‌ها کنار بروند.
settings-seconds = { $seconds } ثانیه
settings-transparency = شفافیت
settings-transparency-note = چه مقدار از تصویر از پشت کنترل‌های شناور دیده شود.
settings-percent = { $percent }%
settings-corner-radius = شعاع گوشه‌ها
settings-corner-radius-note = برای پنجره‌های گفتگو و کنترل‌های شناور.
settings-pixels = { $pixels } پیکسل
settings-animations = پویانمایی‌ها
settings-animations-note = نوارها و پنل‌های لغزان، پنجره‌های گفتگوی بازشونده و دکمه‌های فنری.
settings-animations-reduced = وقتی سیستم کاهش حرکت را درخواست کند، خاموش است.

settings-hardware = استفاده از کارت گرافیک
settings-hardware-note = کدگشایی ویدیو و کدگذاری خروجی‌ها روی کارت گرافیک انجام می‌شود: انرژی کمتری مصرف می‌شود، ویدیوهای بزرگ روان پخش می‌شوند و صادر کردن سریع‌تر است. برای پرونده‌هایی که از این پس باز می‌شوند اعمال می‌شود.
settings-audio-device = خروجی صدا
settings-audio-device-system = پیش‌فرض سیستم
settings-resume = ادامه از جای قبلی
settings-resume-note = پرونده‌ها از جایی شروع می‌شوند که آخرین بار بسته شدند.
settings-folder-playlists = پخش دیگر پرونده‌های پوشه
settings-folder-playlists-note = با باز کردن یک پرونده، دیگر پرونده‌های پوشه‌اش فهرست می‌شوند و پس از آن پخش می‌شوند.
settings-streams = جریان‌ها و ویدیوهای وب
settings-stream-quality = کیفیت
settings-stream-quality-note = برای جریان‌ها و ویدیوهای وب که با چند کیفیت عرضه می‌شوند. برای مواردی که از این پس باز می‌شوند اعمال می‌شود.
settings-quality-auto = خودکار، هم‌ارتفاع با صفحه‌نمایش
settings-quality-best = بهترین
settings-web-pages = پخش ویدیوهای وب با yt-dlp
settings-web-pages-found = yt-dlp ویدیو را در صفحه‌هایی مانند صفحه‌های YouTube پیدا می‌کند.
settings-web-pages-missing = yt-dlp نصب نشده است. برای پخش ویدیوهای صفحه‌هایی مانند صفحه‌های YouTube، آن را نصب کنید.
settings-folders = پوشه‌ها
settings-recordings = ضبط‌ها
settings-screenshots = فریم‌های ذخیره‌شده

settings-interface-language = زبان رابط کاربری
# The first entry of the language list.
settings-language-system = پیروی از سیستم
settings-track-languages = زبان ترک‌ها
settings-track-languages-note = کدهای زبان مانند en یا fra، به ترتیب اولویت و جداشده با ویرگول. وقتی پرونده‌ای صدا و زیرنویس به این زبان‌ها داشته باشد، plaa همان‌ها را انتخاب می‌کند.
settings-audio-languages = صدا
settings-subtitle-languages = زیرنویس‌ها

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = رابط AVB
settings-avb-interface-note = درگاه سیمی‌ای که plaa فرستنده‌ها و گیرنده‌های AVB و Milan را روی آن پیدا می‌کند. ارسال به درگاهی با ساعت سخت‌افزاری PTP نیاز دارد، مانند درگاه Intel I210.
settings-avb-off = خاموش
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (فقط فهرست: بدون ساعت PTP)
settings-avb-off-mac = { $interface } (فقط فهرست: AVB خاموش است)
settings-avb-interface-note-mac = درگاه سیمی‌ای که plaa فرستنده‌ها و گیرنده‌های AVB و Milan را روی آن پیدا می‌کند. ارسال از راه موجودیت AVB خود این Mac انجام می‌شود و برای آن باید AVB برای این درگاه در Network Device Browser برنامهٔ Audio MIDI Setup روشن باشد.
settings-avb-talker-note-mac = plaa صدای پنجره‌ای را که آخرین بار پخش شده، از راه موجودیت AVB خود این Mac به گیرنده‌های متصل به آن می‌فرستد.
settings-avb-talker = ارسال آنچه پخش می‌شود
settings-avb-talker-note = plaa یک فرستندهٔ Milan می‌شود و صدای پنجره‌ای را که آخرین بار پخش شده، به گیرنده‌های متصل به آن می‌فرستد.
settings-avb-name = نام روی شبکه
settings-avb-name-note = کنترل‌گرها plaa را با نام «{ $name }» نشان می‌دهند. برای تغییر نام، Enter را فشار دهید.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa روی { $computer }
avb-could-not-start = استفاده از { $interface } ممکن نشد: { $error }
network-title = صدای شبکه
network-off = برای پخش روی شبکه، «ارسال آنچه پخش می‌شود» را در تنظیمات روشن کنید.
network-blocked = ارسال انجام نمی‌شود: { $reason }
network-waiting = آمادهٔ ارسال، در انتظار اتصال یک گیرنده.
network-sending = در حال ارسال به گیرنده‌های متصل.
network-looking = در حال جستجوی گیرنده‌ها…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = در حال اجرای ptp4l، که زمان شبکه را برای ارسال جریان plaa نگه می‌دارد…
network-ptp-elsewhere = ptp4l روی رابط دیگری اجرا می‌شود، پس جریان plaa اینجا از زمان شبکه پیروی نمی‌کند و ممکن است گیرنده‌ها آن را پخش نکنند. آن رابط را در تنظیمات انتخاب کنید، یا ptp4l را روی همین رابط اجرا کنید.
network-ptp-missing = ptp4l در حال اجرا نیست، پس جریان plaa از زمان شبکه پیروی نمی‌کند و ممکن است گیرنده‌ها آن را پخش نکنند. linuxptp را همراه بستهٔ plaa نصب کنید، یا triib را که آن را اجرا می‌کند.
network-ptp-failed = ptp4l در حال اجرا نیست، پس جریان plaa از زمان شبکه پیروی نمی‌کند و ممکن است گیرنده‌ها آن را پخش نکنند. plaa نتوانست آن را اجرا کند: { $reason }
network-no-listeners = هیچ گیرنده‌ای روی شبکه نمی‌تواند صدای plaa را پخش کند.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = پخش روی { $listener }
network-stop-on = توقف پخش روی { $listener }
network-choose-format = قالبی برای پخش روی { $listener } انتخاب کنید
network-failed = اتصال ممکن نشد: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = برای پخش در اینجا، triib را به‌روز کنید.
network-choose = یک گیرنده انتخاب کنید تا ببینید جریانش را چگونه دریافت می‌کند.

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = وضعیت
network-not-connected = متصل نیست
network-from = از { $stream }
network-from-receiving = از { $stream }، در حال دریافت
network-from-waiting = از { $stream }، در انتظار فرستنده
network-from-failed = از { $stream }، رزرو فرستنده ناموفق بود: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = ساعت رسانه
network-locked = قفل‌شده به جریان
network-not-locked = قفل نشده
network-format = قالب
# The formats both the listener and plaa's stream can be in.
network-formats = قالب‌هایی که می‌تواند از plaa بگیرد
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = شناسهٔ جریان
# The MAC address the stream is sent to.
network-destination = مقصد
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = تأخیر
network-microseconds = { $time } µs
network-frames = فریم‌های دریافت‌شده
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = فریم‌های خارج از ترتیب
# How often the stream stopped coming while it was taken.
network-interrupted = وقفه‌ها
network-entity-id = شناسهٔ موجودیت

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = پهنای باند ناکافی
msrp-failure-2 = منابع ناکافی در پل
msrp-failure-3 = پهنای باند ناکافی برای کلاس ترافیک
msrp-failure-4 = فرستندهٔ دیگری از این شناسهٔ جریان استفاده می‌کند
msrp-failure-5 = نشانی مقصد از قبل در حال استفاده است
msrp-failure-6 = جریانی با رتبهٔ بالاتر جای آن را گرفت
msrp-failure-7 = تأخیر گزارش‌شده تغییر کرده است
msrp-failure-8 = درگاه خروجی از AVB پشتیبانی نمی‌کند
msrp-failure-9 = از نشانی مقصد دیگری استفاده کنید
msrp-failure-10 = منابع MSRP تمام شده است
msrp-failure-11 = منابع MMRP تمام شده است
msrp-failure-12 = نمی‌توان نشانی مقصد را ذخیره کرد
msrp-failure-13 = اولویت، اولویت کلاس SR نیست
msrp-failure-14 = فریم‌ها برای رسانهٔ انتقال بیش از حد بزرگ‌اند
msrp-failure-15 = به حد درگاه‌های ورودی هم‌گرا رسیده است
msrp-failure-16 = مقدار نخست برای جریانی ثبت‌شده تغییر کرد
msrp-failure-17 = VLAN روی درگاه خروجی مسدود است
msrp-failure-18 = برچسب‌گذاری VLAN روی درگاه خروجی غیرفعال است
msrp-failure-19 = ناهمخوانی اولویت کلاس SR
msrp-failure-unknown = دلیل نامشخص
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }، در پل { $bridge }

## The macOS menu bar

menu-about = دربارهٔ plaa
menu-settings = تنظیمات…
menu-services = خدمات
menu-hide = پنهان کردن plaa
menu-hide-others = پنهان کردن بقیه
menu-show-all = نمایش همه
menu-quit = خروج از plaa
menu-file = پرونده
menu-open = باز کردن…
menu-open-address = باز کردن نشانی…
menu-close = بستن پنجره
menu-edit = ویرایش
menu-cut = بریدن
menu-copy = کپی
menu-paste = چسباندن
menu-select-all = انتخاب همه
menu-undo = واگرد
menu-redo = ازنو
menu-trim = کوتاه کردن…
menu-split = تقسیم کلیپ
menu-rotate-left = چرخش به چپ
menu-rotate-right = چرخش به راست
menu-flip-horizontal = قرینهٔ افقی
menu-flip-vertical = قرینهٔ عمودی
menu-revert = بازگشت به نسخهٔ اصلی
menu-export = صادر کردن…
menu-view = نما
menu-playback = پخش
menu-play-pause = پخش یا مکث
menu-to-start = رفتن به ابتدا
menu-previous-chapter = فصل قبلی
menu-next-chapter = فصل بعدی
menu-faster = سریع‌تر
menu-slower = آهسته‌تر
menu-normal-speed = سرعت عادی
menu-ab-loop = تکرار بین دو نقطه
menu-volume-up = افزایش صدا
menu-volume-down = کاهش صدا
menu-mute = قطع یا وصل صدا
menu-next-subtitles = زیرنویس بعدی
menu-next-audio = ترک صوتی بعدی
menu-window = پنجره
menu-minimize = کمینه کردن
menu-zoom = بزرگ/کوچک کردن
menu-bring-all-to-front = آوردن همه به جلو

## Editing

action-edit = ویرایش
title-edited = { $name } (ویرایش‌شده)
edit-trim = کوتاه کردن
edit-trim-apply = کوتاه کردن
edit-split = تقسیم کلیپ
edit-remove-clip = حذف کلیپ
edit-done = انجام شد
edit-rotate-left = چرخش به چپ
edit-rotate-right = چرخش به راست
edit-flip-horizontal = قرینهٔ افقی
edit-flip-vertical = قرینهٔ عمودی
edit-undo = واگرد
edit-redo = ازنو
edit-revert = بازگشت به نسخهٔ اصلی
edit-export = صادر کردن…

## Exporting

export-title = صادر کردن
export-format = قالب
export-size = اندازه
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = فقط صدا
export-size-original = { $lines }p، بدون تغییر
export-on-graphics-card = کدگذاری روی کارت گرافیک ({ $api })
export-on-processor = کدگذاری روی پردازنده، که بیشتر طول می‌کشد
export-no-encoder = هیچ کدگذاری برای این قالب اینجا در دسترس نیست
export-checking = در حال جستجوی کدگذار…
export-sound-format = قالب صدا
export-quality = کیفیت
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = فشرده؛ تقریباً همه‌جا پخش می‌شود، از جمله در دستگاه‌های Apple
export-mp3-note = فشرده؛ همه‌جا پخش می‌شود
export-opus-note = فشرده؛ بهترین صدا نسبت به حجم، هرچند همهٔ پخش‌کننده‌ها آن را نمی‌پذیرند
export-flac-note = بدون افت کیفیت: صدا دقیقاً همان‌طور که هست، در قالبی باز
export-alac-note = بدون افت کیفیت: صدا دقیقاً همان‌طور که هست، به شکلی که دستگاه‌های Apple پخش می‌کنند
export-wav-note = بدون فشرده‌سازی: صدا دقیقاً همان‌طور که هست، با بزرگ‌ترین حجم پرونده
export-kilobits = { $rate } kb/s
export-bits = { $bits } بیتی
export-rate-lower = ذخیره با { $rate } kHz، کمتر از { $file } kHz پرونده
export-choose = صادر کردن…
export-name = { $name } صادرشده
export-name-edited = { $name } ویرایش‌شده
export-progress = در حال صادر کردن «{ $name }»
export-stop = توقف صادر کردن
notice-exported = «{ $name }» صادر شد
notice-export-stopped = صادر کردن متوقف شد
notice-export-failed = صادر کردن ممکن نشد: { $error }
close-title = بدون صادر کردن بسته شود؟
close-unexported = تغییرات «{ $name }» از دست می‌روند، مگر اینکه آن‌ها را صادر کنید.
close-exporting = صادر کردن «{ $name }» متوقف می‌شود و آنچه تاکنون نوشته شده از دست می‌رود.
close-anyway = بستن
close-quit-anyway = خروج

## Recording

record-title = ضبط صدا
record-file-name = ضبط صدا
record-start = ضبط
record-stop = توقف ضبط
record-no-input = بدون میکروفون
record-microphone = میکروفون
record-default-microphone = پیش‌فرض سیستم
record-playing = آنچه رایانه پخش می‌کند
record-mono = مونو
record-stereo = استریو
record-channels = { $count } کانال
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }، ذخیره با { $saved } kHz
action-record-sound = ضبط صدا
open-record-sound = ضبط صدای جدید
start-record-sound = ضبط صدا
menu-record-sound = ضبط صدای جدید
record-screen-title = ضبط صفحه
record-screen-file-name = ضبط صفحه
record-screen-whole = صفحه‌نمایش
record-screen-number = صفحه‌نمایش { $number }
record-screen-main = صفحه‌نمایش { $number } (اصلی)
record-screens = صفحه‌نمایش‌ها
record-windows = پنجره‌ها
record-screen-what = چه چیزی ضبط شود
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = همهٔ فریم‌ها
record-rate-tip = فریم در ثانیه
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count } فریم جاافتاده
record-screen-window = پنجره
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = دیگر به اشتراک گذاشته نمی‌شود
record-screen-again = انتخاب دوباره
record-screen-failed = ضبط صفحه ممکن نشد
record-sound = صدا
record-no-sound = بدون صدا
# How much louder the input is taken, for a quiet microphone.
record-gain = تقویت
record-gain-value = +{ $decibels } dB
record-level-automatically = تنظیم خودکار سطح صدا
record-hide-title = plaa هنگام ضبط پنهان می‌شود
record-hide-tray = برای توقف ضبط، روی نقطهٔ قرمزی که plaa در سینی سیستم می‌گذارد کلیک کنید، یا plaa را دوباره باز کنید.
record-hide-reopen = برای توقف ضبط، plaa را دوباره باز کنید.
record-hide-start = شروع ضبط
record-tray-title = plaa در حال ضبط صفحه است
record-tray-tip = برای توقف ضبط کلیک کنید
action-record-screen = ضبط صفحه
open-record-screen = ضبط صفحهٔ جدید
start-record-screen = ضبط صفحه
menu-record-screen = ضبط صفحهٔ جدید
record-camera-title = ضبط دوربین
record-camera-file-name = ضبط دوربین
record-camera = دوربین
record-camera-failed = باز کردن دوربین ممکن نشد
action-record-camera = ضبط دوربین
open-record-camera = ضبط دوربین جدید
start-record-camera = ضبط دوربین
menu-record-camera = ضبط دوربین جدید
notice-no-camera = دوربینی برای ضبط وجود ندارد
notice-recorded = «{ $name }» در { $folder } ذخیره شد
notice-record-failed = ضبط ممکن نشد: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = اجازهٔ کنترل از بیرون
settings-allow-outside-control-note = عامل‌های هوش مصنوعی مانند Claude Code می‌توانند از راه plaa --mcp ببینند در plaa چه پخش می‌شود و آن را کنترل کنند. plaa پیش از هر عامل تازه می‌پرسد.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = مجاز: { $agents }
settings-forget-agents = فراموش کردن
settings-ask-before-note = پرسیدن پیش از این کارهای عامل:
settings-ask-reading = دیدن آنچه پخش می‌شود، یا تصویر
settings-ask-playing = کنترل پخش، صدا یا حالت تمام‌صفحه
settings-ask-opening = باز کردن پرونده یا بستن پنجره
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = به { $agent } اجازهٔ کنترل plaa داده شود؟
agent-prompt-body = { $agent } می‌خواهد از کنترل از بیرون plaa استفاده کند تا ببیند چه پخش می‌شود و آن را کنترل کند. می‌توانید کنترل از بیرون را در تنظیمات خاموش کنید.
agent-prompt-allow = اجازه دادن
agent-prompt-deny = اجازه ندادن
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = به { $agent } اجازه داده شود ببیند اینجا چه پخش می‌شود؟
agent-ask-play = به { $agent } اجازهٔ کنترل پخش در اینجا داده شود؟
agent-ask-open = به { $agent } اجازهٔ باز کردن یا بستن پنجره داده شود؟
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } می‌خواهد از «{ $tool }» استفاده کند. در تنظیمات انتخاب می‌کنید plaa دربارهٔ چه چیزهایی بپرسد.
