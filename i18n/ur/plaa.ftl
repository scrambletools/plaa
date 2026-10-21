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
language-name = اردو

## Common

app-name = plaa
common-cancel = منسوخ کریں
common-close = بند کریں
common-more = مزید

## Command line

usage-help =
    استعمال: plaa [FILE OR ADDRESS]...

    ویڈیو اور آڈیو فائلیں، اور https://example.com/live.m3u8 جیسے پتوں پر
    اسٹریمز چلائیں۔ ہر ایک چلتے ہوئے plaa کی ایک ونڈو میں کھلتی ہے، جو
    ضرورت ہو تو شروع ہو جاتا ہے۔

    اختیارات:
      -h, --help        یہ مدد دکھائیں
      -V, --version     ورژن دکھائیں
          --interfaces  وائرڈ انٹرفیسز دکھائیں، اور یہ کہ کیا plaa ان پر AVB
                        اسٹریمز بھیج سکتا ہے
          --mcp         stdin اور stdout پر MCP فراہم کریں، تاکہ AI ایجنٹ دیکھ سکیں
                        کہ چلتا ہوا plaa کیا چلا رہا ہے، اور اسے کنٹرول کر سکیں

## Start window

start-hint = کوئی ویڈیو یا آڈیو فائل، اسٹریم یا ویب ویڈیو کھولیں۔
start-open = کھولیں
start-open-address = پتہ کھولیں

## Actions, as buttons, tooltips and menus name them

action-open = کھولیں
action-open-address = پتہ کھولیں
open-file = فائل کھولیں…
open-address = پتہ کھولیں…
action-settings = ترتیبات
action-info = انسپکٹر
action-loop = لوپ
action-screenshot = فریم بطور تصویر محفوظ کریں
action-play = چلائیں
action-pause = موقوف کریں
# $seconds is how far the button moves, such as 10.
action-back = { $seconds } سیکنڈ پیچھے
action-forward = { $seconds } سیکنڈ آگے
action-mute = آواز بند کریں
action-unmute = آواز چالو کریں
action-speed = رفتار
action-tracks = آڈیو اور سب ٹائٹلز
action-full-screen = پوری اسکرین
action-playlist = پلے لسٹ
action-chapters = ابواب
action-picture-in-picture = تصویر میں تصویر
action-previous = پچھلا
action-next = اگلا
action-exit-full-screen = پوری اسکرین سے باہر نکلیں
action-quality = معیار
action-visualizer = ویژولائزر
action-network = نیٹ ورک آڈیو
action-cast = کاسٹ کریں

## What a sound file shows under its cover

visualizer-spectrum = اسپیکٹرم
visualizer-waveform = ویوفارم
visualizer-prism = پرزم
visualizer-cover = صرف کور

## The floating controls

overlay-keep-shown = کنٹرولز ایک جگہ جما کر رکھیں
overlay-auto-hide = کنٹرولز کو تصویر کے اوپر تیرنے دیں

## Audio and subtitle tracks

tracks-audio = آڈیو
tracks-subtitles = سب ٹائٹلز
tracks-off = بند
# A track the file gives no name or language, by its number.
tracks-numbered = ٹریک { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = صرف آڈیو

## Inspector

info-title = انسپکٹر
playlist-title = پلے لسٹ
chapters-numbered = باب { $number }
info-file = فائل
info-name = نام
info-address = پتہ
info-folder = فولڈر
info-size = سائز
info-format = فارمیٹ
info-duration = دورانیہ
info-bit-rate = بٹ ریٹ
info-video = ویڈیو
info-audio = آڈیو
info-codec = کوڈیک
info-dimensions = ابعاد
info-frame-rate = فریم ریٹ
info-frames-per-second = { $rate } فریم فی سیکنڈ
info-pixel-format = پکسل فارمیٹ
info-decoding = ڈی کوڈنگ
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = گرافکس کارڈ ({ $api })
info-decoding-software = پروسیسر
info-sample-rate = سیمپل ریٹ
info-hertz = { $rate } Hz
info-channels = چینلز
info-language = زبان
info-no-sound = کوئی آواز نہیں
info-tags = ٹیگز

## Dialogs

dialog-open = کھولیں
dialog-filter-media = ویڈیو اور آڈیو
dialog-filter-all = تمام فائلیں

address-title = پتہ کھولیں
address-note = ویڈیو والا کوئی ویب صفحہ، جیسے YouTube ویڈیو کا صفحہ، یا نیٹ ورک پر کوئی اسٹریم یا فائل، جیسے https://example.com/live.m3u8۔
address-label = پتہ
address-open = کھولیں
address-recent = حالیہ
address-forget = حالیہ سے ہٹائیں

## Notices

notice-dialog-failed = فائل ڈائیلاگ نہیں کھل سکا: { $error }
notice-screenshot-saved = “{ $name }” محفوظ ہو گیا۔
notice-screenshot-failed = تصویر محفوظ نہیں کی جا سکی: { $error }
notice-no-sound = آواز کے بغیر چل رہا ہے: { $reason }
notice-loop-from = { $time } سے لوپ۔ جہاں ختم کرنا ہو وہاں دوبارہ دبائیں۔
notice-loop-between = { $from } سے { $to } تک لوپ ہو رہا ہے۔
notice-loop-off = لوپ بند۔

## Errors, shown in place of what could not play

error-needs-yt-dlp = یہ ایک ویب صفحہ لگتا ہے۔ plaa ویب صفحات پر موجود ویڈیوز yt-dlp سے چلاتا ہے، جو انسٹال نہیں ہے۔
error-empty-list = اس فہرست میں چلانے کے لیے کچھ نہیں ہے۔

## Settings

settings-title = ترتیبات
settings-tab-appearance = ظاہری شکل
settings-tab-playback = پلے بیک
settings-tab-network = نیٹ ورک
settings-tab-language = زبان
settings-tab-agents = ایجنٹ
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (ڈیولپمنٹ بلڈ، { $build })
settings-choose = منتخب کریں…

settings-appearance-system = سسٹم کے مطابق
settings-appearance-light = روشن
settings-appearance-dark = تاریک
settings-colors = رنگ
settings-system-accent = سسٹم کا ایکسنٹ رنگ استعمال کریں
settings-omarchy-note = رنگ “{ $theme }” کے ایکسنٹ رنگ سے بنائے جاتے ہیں۔
settings-system-accent-note = رنگ سسٹم کے ایکسنٹ رنگ سے بنائے جاتے ہیں۔
settings-system-accent-none = سسٹم میں کوئی ایکسنٹ رنگ نہیں ہے، اس لیے plaa نیچے منتخب کیا گیا رنگ استعمال کرتا ہے۔
settings-accent-chosen-note = رنگ نیچے منتخب کیے گئے رنگ سے بنائے جاتے ہیں۔
settings-controls = کنٹرولز
settings-overlay = تیرتے کنٹرولز
settings-overlay-note = ٹول بار اور پلے بیک کنٹرولز تصویر یا کور کے اوپر تیرتے ہیں، اور جب پوائنٹر رکا ہو یا ونڈو سے باہر ہو تو کھسک کر ہٹ جاتے ہیں۔
settings-overlay-delay = اتنی دیر بعد چھپائیں
settings-overlay-delay-note = کنٹرولز کے کھسک کر ہٹنے سے پہلے پوائنٹر چلتی ہوئی چیز پر کتنی دیر رکا رہے۔
settings-seconds = { $seconds } سیکنڈ
settings-transparency = شفافیت
settings-transparency-note = تیرتے کنٹرولز میں سے تصویر کتنی نظر آئے۔
settings-percent = { $percent }%
settings-corner-radius = کونوں کی گولائی
settings-corner-radius-note = ڈائیلاگز اور تیرتے کنٹرولز کے لیے۔
settings-pixels = { $pixels } پکسل
settings-animations = اینیمیشنز
settings-animations-note = کھسکتی بارز اور پینلز، پھیلتے ڈائیلاگز اور لچکدار بٹن۔
settings-animations-reduced = جب سسٹم کم حرکت کا تقاضا کرے تو بند رہتی ہیں۔

settings-hardware = گرافکس کارڈ استعمال کریں
settings-hardware-note = ویڈیو کی ڈی کوڈنگ اور برآمدات کی انکوڈنگ اسی پر ہوتی ہے: کم بجلی لگتی ہے، بڑی ویڈیوز روانی سے چلتی ہیں اور برآمد تیزی سے ہوتی ہے۔ اس کے بعد کھولی گئی فائلوں پر لاگو ہوتا ہے۔
settings-audio-device = آواز کا آؤٹ پٹ
settings-audio-device-system = سسٹم ڈیفالٹ
settings-resume = جہاں چھوڑا تھا وہیں سے جاری رکھیں
settings-resume-note = فائلیں وہیں سے شروع ہوتی ہیں جہاں آخری بار بند کرتے وقت تھیں۔
settings-folder-playlists = فولڈر کی دوسری فائلیں چلائیں
settings-folder-playlists-note = ایک فائل کھولنے سے اس کے فولڈر کی دوسری فائلیں فہرست میں آ جاتی ہیں، جو اس کے بعد چلتی ہیں۔
settings-streams = اسٹریمز اور ویب ویڈیوز
settings-stream-quality = معیار
settings-stream-quality-note = ان اسٹریمز اور ویب ویڈیوز کے لیے جو کئی معیاروں میں آتی ہیں۔ اس کے بعد کھلنے والی چیزوں پر لاگو ہوتا ہے۔
settings-quality-auto = خودکار، اسکرین جتنی اونچائی
settings-quality-best = بہترین
settings-web-pages = ویب ویڈیوز yt-dlp سے چلائیں
settings-web-pages-found = yt-dlp ایسے صفحات پر ویڈیو ڈھونڈ لیتا ہے جیسے YouTube کے۔
settings-web-pages-missing = yt-dlp انسٹال نہیں ہے۔ YouTube جیسے صفحات پر ویڈیوز چلانے کے لیے اسے انسٹال کریں۔
settings-folders = فولڈرز
settings-recordings = ریکارڈنگز
settings-screenshots = محفوظ کردہ فریمز

settings-interface-language = انٹرفیس کی زبان
# The first entry of the language list.
settings-language-system = سسٹم کے مطابق
settings-track-languages = ٹریکس کی زبانیں
settings-track-languages-note = زبانوں کے کوڈ جیسے en یا fra، سب سے زیادہ پسندیدہ پہلے، کوما سے الگ کیے ہوئے۔ جب فائل میں یہ زبانیں ہوں تو plaa آڈیو اور سب ٹائٹلز انہی زبانوں میں چنتا ہے۔
settings-audio-languages = آڈیو
settings-subtitle-languages = سب ٹائٹلز

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB انٹرفیس
settings-avb-interface-note = وہ وائرڈ پورٹ جس پر plaa، AVB اور Milan ٹاکرز اور لسنرز ڈھونڈتا ہے۔ بھیجنے کے لیے PTP ہارڈویئر کلاک والا پورٹ درکار ہے، جیسے Intel I210 کا۔
settings-avb-off = بند
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (صرف فہرست: PTP کلاک نہیں)
settings-avb-off-mac = { $interface } (صرف فہرست: AVB بند ہے)
settings-avb-interface-note-mac = وہ وائرڈ پورٹ جس پر plaa، AVB اور Milan ٹاکرز اور لسنرز ڈھونڈتا ہے۔ بھیجنا اس Mac کی اپنی AVB اینٹٹی کے ذریعے ہوتا ہے، جس کے لیے Audio MIDI Setup کے Network Device Browser میں اس پورٹ کے لیے AVB آن ہونا ضروری ہے۔
settings-avb-talker-note-mac = plaa آخری بار چلنے والی ونڈو کی آواز اس Mac کی اپنی AVB اینٹٹی کے ذریعے اس سے جڑے لسنرز کو بھیجتا ہے۔
settings-avb-talker = جو چل رہا ہے وہ بھیجیں
settings-avb-talker-note = plaa ایک Milan ٹاکر بن جاتا ہے، اور آخری بار چلنے والی ونڈو کی آواز اس سے جڑے لسنرز کو بھیجتا ہے۔
settings-avb-name = نیٹ ورک پر نام
settings-avb-name-note = کنٹرولرز plaa کو “{ $name }” کے نام سے دکھاتے ہیں۔ نام بدلنے کے لیے Enter دبائیں۔
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } پر plaa
avb-could-not-start = { $interface } استعمال نہیں کیا جا سکا: { $error }
network-title = نیٹ ورک آڈیو
network-off = نیٹ ورک پر چلانے کے لیے ترتیبات میں “جو چل رہا ہے وہ بھیجیں” آن کریں۔
network-blocked = نہیں بھیجا جا رہا: { $reason }
network-waiting = بھیجنے کے لیے تیار، کسی لسنر کے جڑنے کا انتظار ہے۔
network-sending = جڑے ہوئے لسنرز کو بھیجا جا رہا ہے۔
network-looking = لسنرز تلاش کیے جا رہے ہیں…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l شروع ہو رہا ہے، جو وہ نیٹ ورک ٹائم رکھتا ہے جس کے مطابق plaa کی اسٹریم بھیجی جاتی ہے…
network-ptp-elsewhere = ptp4l کسی دوسرے انٹرفیس پر چل رہا ہے، اس لیے یہاں plaa کی اسٹریم کا نیٹ ورک ٹائم سے کوئی تعلق نہیں اور ممکن ہے لسنرز اسے نہ چلا سکیں۔ ترتیبات میں وہ انٹرفیس منتخب کریں، یا اس انٹرفیس پر ptp4l چلائیں۔
network-ptp-missing = ptp4l نہیں چل رہا، اس لیے plaa کی اسٹریم کا نیٹ ورک ٹائم سے کوئی تعلق نہیں اور ممکن ہے لسنرز اسے نہ چلا سکیں۔ plaa کے پیکیج کے ساتھ linuxptp انسٹال کریں، یا triib، جو اسے چلاتا ہے۔
network-ptp-failed = ptp4l نہیں چل رہا، اس لیے plaa کی اسٹریم کا نیٹ ورک ٹائم سے کوئی تعلق نہیں اور ممکن ہے لسنرز اسے نہ چلا سکیں۔ plaa اسے شروع نہیں کر سکا: { $reason }
network-no-listeners = نیٹ ورک پر کوئی لسنر plaa کی آواز نہیں چلا سکتا۔
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener } پر چلائیں
network-stop-on = { $listener } پر چلانا بند کریں
network-choose-format = { $listener } پر چلانے کے لیے فارمیٹ منتخب کریں
network-failed = کنکشن نہیں ہو سکا: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = یہاں چلانے کے لیے triib اپ ڈیٹ کریں۔
network-choose = کوئی لسنر منتخب کریں تاکہ دیکھ سکیں کہ وہ اپنی اسٹریم کیسے لیتا ہے۔
# The section of talkers plaa can listen to, playing their stream on
# this computer's sound output.
network-listen-section = یہاں سنیں
network-no-talkers = نیٹ ورک پر کوئی ٹاکر ایسے فارمیٹ میں نہیں بھیج رہا جو plaa چلا سکے۔
# $talker is a talker's name, or its name and its output's.
network-listen-to = { $talker } کو سنیں
network-listen-stop = { $talker } کو سننا بند کریں
network-listen-frames = موصول ہونے والے فریمز: { $frames }
network-listen-update-triib = یہاں سننے کے لیے triib اپ ڈیٹ کریں: plaa صرف وہی چلاتا ہے جو ویسے بھی اس کمپیوٹر تک پہنچتا ہے۔
network-listen-lite = پورٹ AVB Lite پر چل رہا ہے، جو کوئی اسٹریم ریزرو نہیں کرتا: plaa صرف وہی چلاتا ہے جو ویسے بھی اس کمپیوٹر تک پہنچتا ہے۔

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = حالت
network-not-connected = جڑا ہوا نہیں
network-from = { $stream } سے
network-from-receiving = { $stream } سے، وصول ہو رہا ہے
network-from-waiting = { $stream } سے، ٹاکر کا انتظار ہے
network-from-failed = { $stream } سے، ٹاکر کی ریزرویشن ناکام ہو گئی: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = میڈیا کلاک
network-locked = اسٹریم سے لاک ہے
network-not-locked = لاک نہیں
network-format = فارمیٹ
# The formats both the listener and plaa's stream can be in.
network-formats = وہ فارمیٹس جو یہ plaa سے لے سکتا ہے
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = اسٹریم ID
# The MAC address the stream is sent to.
network-destination = منزل
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = لیٹنسی
network-microseconds = { $time } µs
network-frames = موصول ہونے والے فریمز
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = ترتیب سے ہٹے ہوئے فریمز
# How often the stream stopped coming while it was taken.
network-interrupted = رکاوٹیں
network-entity-id = اینٹٹی ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = ناکافی بینڈوڈتھ
msrp-failure-2 = برج کے ناکافی وسائل
msrp-failure-3 = ٹریفک کلاس کے لیے ناکافی بینڈوڈتھ
msrp-failure-4 = اسٹریم ID کسی دوسرے ٹاکر کے زیرِ استعمال ہے
msrp-failure-5 = منزل کا پتہ پہلے سے زیرِ استعمال ہے
msrp-failure-6 = زیادہ درجے والی اسٹریم نے جگہ لے لی
msrp-failure-7 = رپورٹ کردہ لیٹنسی بدل گئی ہے
msrp-failure-8 = باہر جانے والا پورٹ AVB کے قابل نہیں
msrp-failure-9 = کوئی دوسرا منزل کا پتہ استعمال کریں
msrp-failure-10 = MSRP وسائل ختم ہو گئے
msrp-failure-11 = MMRP وسائل ختم ہو گئے
msrp-failure-12 = منزل کا پتہ محفوظ نہیں کیا جا سکتا
msrp-failure-13 = ترجیح SR کلاس کی ترجیح نہیں ہے
msrp-failure-14 = فریمز میڈیم کے لیے بہت بڑے ہیں
msrp-failure-15 = فین اِن پورٹ کی حد پوری ہو گئی
msrp-failure-16 = رجسٹرڈ اسٹریم کی پہلی ویلیو بدل گئی
msrp-failure-17 = باہر جانے والے پورٹ پر VLAN بلاک ہے
msrp-failure-18 = باہر جانے والے پورٹ پر VLAN ٹیگنگ بند ہے
msrp-failure-19 = SR کلاس کی ترجیح مماثل نہیں
msrp-failure-unknown = نامعلوم وجہ
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }، برج { $bridge } پر

## The macOS menu bar

menu-about = plaa کے بارے میں
menu-settings = ترتیبات…
menu-services = خدمات
menu-hide = plaa چھپائیں
menu-hide-others = دیگر چھپائیں
menu-show-all = سب دکھائیں
menu-quit = plaa بند کریں
menu-file = فائل
menu-open = کھولیں…
menu-open-address = پتہ کھولیں…
menu-close = ونڈو بند کریں
menu-edit = ترمیم
menu-cut = کاٹیں
menu-copy = کاپی کریں
menu-paste = چسپاں کریں
menu-select-all = سب منتخب کریں
menu-undo = کالعدم کریں
menu-redo = دوبارہ کریں
menu-trim = تراشیں…
menu-split = کلپ تقسیم کریں
menu-rotate-left = بائیں گھمائیں
menu-rotate-right = دائیں گھمائیں
menu-flip-horizontal = افقی پلٹیں
menu-flip-vertical = عمودی پلٹیں
menu-revert = اصل پر بحال کریں
menu-export = برآمد کریں…
menu-view = منظر
menu-playback = پلے بیک
menu-play-pause = چلائیں یا موقوف کریں
menu-to-start = شروع پر جائیں
menu-previous-chapter = پچھلا باب
menu-next-chapter = اگلا باب
menu-faster = تیز
menu-slower = آہستہ
menu-normal-speed = عام رفتار
menu-ab-loop = دو نقطوں کے درمیان لوپ
menu-volume-up = آواز بلند کریں
menu-volume-down = آواز دھیمی کریں
menu-mute = آواز بند یا چالو کریں
menu-next-subtitles = اگلے سب ٹائٹلز
menu-next-audio = اگلا آڈیو ٹریک
menu-window = ونڈو
menu-minimize = چھوٹا کریں
menu-zoom = زوم
menu-bring-all-to-front = سب کو سامنے لائیں

## Editing

action-edit = ترمیم کریں
title-edited = { $name } (ترمیم شدہ)
edit-trim = تراشیں
edit-trim-apply = تراشیں
edit-split = کلپ تقسیم کریں
edit-remove-clip = کلپ ہٹائیں
edit-done = ہو گیا
edit-rotate-left = بائیں گھمائیں
edit-rotate-right = دائیں گھمائیں
edit-flip-horizontal = افقی پلٹیں
edit-flip-vertical = عمودی پلٹیں
edit-undo = کالعدم کریں
edit-redo = دوبارہ کریں
edit-revert = اصل پر بحال کریں
edit-export = برآمد کریں…

## Exporting

export-title = برآمد کریں
export-format = فارمیٹ
export-size = سائز
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = صرف آواز
export-size-original = { $lines }p، جیسا ہے ویسا
export-on-graphics-card = گرافکس کارڈ پر انکوڈ ہوتا ہے ({ $api })
export-on-processor = پروسیسر پر انکوڈ ہوتا ہے، جس میں زیادہ وقت لگتا ہے
export-no-encoder = اس فارمیٹ کا کوئی انکوڈر یہاں نہیں کھلتا
export-checking = انکوڈر تلاش کیا جا رہا ہے…
export-sound-format = آواز کا فارمیٹ
export-quality = معیار
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = کمپریسڈ؛ تقریباً ہر جگہ چلتا ہے، Apple کے آلات سمیت
export-mp3-note = کمپریسڈ؛ ہر جگہ چلتا ہے
export-opus-note = کمپریسڈ؛ اپنے سائز کے لحاظ سے بہترین آواز، اگرچہ ہر پلیئر اسے نہیں چلاتا
export-flac-note = بغیر نقصان: آواز بالکل ویسی ہی جیسی ہے، ایک کھلے فارمیٹ میں
export-alac-note = بغیر نقصان: آواز بالکل ویسی ہی جیسی ہے، اس طرح جیسے Apple کے آلات اسے چلاتے ہیں
export-wav-note = غیر کمپریسڈ: آواز بالکل ویسی ہی جیسی ہے، سب سے بڑی فائلوں میں
export-kilobits = { $rate } kb/s
export-bits = { $bits } بٹ
export-rate-lower = { $rate } kHz پر محفوظ ہوتا ہے، فائل کے { $file } kHz سے کم
export-choose = برآمد کریں…
export-name = { $name } برآمد
export-name-edited = { $name } ترمیم شدہ
export-progress = “{ $name }” برآمد ہو رہا ہے
export-stop = برآمد کرنا روکیں
notice-exported = “{ $name }” برآمد ہو گیا
notice-export-stopped = برآمد کرنا رک گیا
notice-export-failed = برآمد نہیں کیا جا سکا: { $error }
close-title = برآمد کیے بغیر بند کریں؟
close-unexported = “{ $name }” میں کی گئی تبدیلیاں ضائع ہو جائیں گی، جب تک آپ انہیں برآمد نہ کریں۔
close-exporting = “{ $name }” کی برآمد رک جائے گی، اور اب تک جو لکھا گیا ہے وہ ضائع ہو جائے گا۔
close-anyway = بند کریں
close-quit-anyway = باہر نکلیں

## Recording

record-title = آواز کی ریکارڈنگ
record-file-name = آواز کی ریکارڈنگ
record-start = ریکارڈ کریں
record-stop = ریکارڈنگ روکیں
record-no-input = کوئی مائیکروفون نہیں
record-microphone = مائیکروفون
record-default-microphone = سسٹم کا ڈیفالٹ
record-playing = جو کمپیوٹر چلا رہا ہے
record-mono = مونو
record-stereo = اسٹیریو
record-channels = { $count } چینلز
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }، { $saved } kHz پر محفوظ
action-record-sound = آواز ریکارڈ کریں
open-record-sound = نئی آواز کی ریکارڈنگ
start-record-sound = آواز ریکارڈ کریں
menu-record-sound = نئی آواز کی ریکارڈنگ
record-screen-title = اسکرین ریکارڈنگ
record-screen-file-name = اسکرین ریکارڈنگ
record-screen-whole = اسکرین
record-screen-number = اسکرین { $number }
record-screen-main = اسکرین { $number } (مرکزی)
record-screens = اسکرینز
record-windows = ونڈوز
record-screen-what = کیا ریکارڈ کریں
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = ہر تصویر
record-rate-tip = فی سیکنڈ تصاویر
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } چھوٹ گئی
   *[other] { $size } · { $count } چھوٹ گئیں
}
record-screen-window = ونڈو
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = اب شیئر نہیں ہو رہا
record-screen-again = دوبارہ منتخب کریں
record-screen-failed = اسکرین ریکارڈ نہیں کی جا سکی
record-sound = آواز
record-no-sound = کوئی آواز نہیں
# How much louder the input is taken, for a quiet microphone.
record-gain = زیادہ بلند
record-gain-value = +{ $decibels } dB
record-level-automatically = سطح خود بخود برابر کریں
record-hide-title = ریکارڈنگ کے دوران plaa چھپ جاتا ہے
record-hide-tray = ریکارڈنگ روکنے کے لیے اس سرخ نقطے پر کلک کریں جو plaa ٹرے میں دکھاتا ہے، یا plaa دوبارہ کھولیں۔
record-hide-reopen = ریکارڈنگ روکنے کے لیے plaa دوبارہ کھولیں۔
record-hide-start = ریکارڈنگ شروع کریں
record-tray-title = plaa اسکرین ریکارڈ کر رہا ہے
record-tray-tip = ریکارڈنگ روکنے کے لیے کلک کریں
action-record-screen = اسکرین ریکارڈ کریں
open-record-screen = نئی اسکرین ریکارڈنگ
start-record-screen = اسکرین ریکارڈ کریں
menu-record-screen = نئی اسکرین ریکارڈنگ
record-camera-title = کیمرا ریکارڈنگ
record-camera-file-name = کیمرا ریکارڈنگ
record-camera = کیمرا
record-camera-failed = کیمرا نہیں کھولا جا سکا
action-record-camera = کیمرا ریکارڈ کریں
open-record-camera = نئی کیمرا ریکارڈنگ
start-record-camera = کیمرا ریکارڈ کریں
menu-record-camera = نئی کیمرا ریکارڈنگ
notice-no-camera = ریکارڈ کرنے کے لیے کوئی کیمرا نہیں ہے
notice-recorded = “{ $name }” { $folder } میں محفوظ ہو گیا
notice-record-failed = ریکارڈ نہیں کیا جا سکا: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = بیرونی کنٹرول کی اجازت دیں
settings-allow-outside-control-note = Claude Code جیسے AI ایجنٹ plaa --mcp کے ذریعے دیکھ سکتے ہیں کہ plaa میں کیا چل رہا ہے، اور اسے کنٹرول کر سکتے ہیں۔ ہر نئے ایجنٹ سے پہلے plaa پوچھتا ہے۔
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = اجازت یافتہ: { $agents }
settings-forget-agents = بھول جائیں
settings-ask-before-note = ایجنٹ کے یہ کام کرنے سے پہلے پوچھیں:
settings-ask-reading = جو چل رہا ہے یا تصویر دیکھنا
settings-ask-playing = پلے بیک، آواز یا پوری اسکرین کو کنٹرول کرنا
settings-ask-opening = فائل کھولنا یا ونڈو بند کرنا
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = کیا { $agent } کو plaa کنٹرول کرنے کی اجازت دیں؟
agent-prompt-body = { $agent } plaa کا بیرونی کنٹرول استعمال کرنا چاہتا ہے، تاکہ دیکھ سکے کہ کیا چل رہا ہے اور اسے کنٹرول کر سکے۔ آپ بیرونی کنٹرول کو ترتیبات میں بند کر سکتے ہیں۔
agent-prompt-allow = اجازت دیں
agent-prompt-deny = اجازت نہ دیں
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = کیا { $agent } کو یہ دیکھنے دیں کہ یہاں کیا چل رہا ہے؟
agent-ask-play = کیا { $agent } کو یہاں پلے بیک کنٹرول کرنے دیں؟
agent-ask-open = کیا { $agent } کو ونڈو کھولنے یا بند کرنے دیں؟
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } “{ $tool }” استعمال کرنا چاہتا ہے۔ plaa کن باتوں پر پوچھے، یہ ترتیبات میں منتخب کیا جاتا ہے۔

## Casting to a TV or speaker on the network

cast-looking = ٹی وی اور اسپیکرز تلاش کیے جا رہے ہیں…
cast-none = DLNA چلانے والا کوئی ٹی وی یا اسپیکر نہیں ملا۔
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer } پر چلانا بند کریں
notice-casting = { $renderer } پر چل رہا ہے۔
notice-cast-failed = { $renderer } پر نہیں چلایا جا سکا: { $reason }
cast-web-page = ویب صفحات ابھی کاسٹ نہیں کیے جا سکتے: اس کے بجائے ویڈیو کا اپنا پتہ کھولیں۔
