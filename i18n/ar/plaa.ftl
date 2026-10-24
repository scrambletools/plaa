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
language-name = العربية

## Common

app-name = plaa
common-cancel = إلغاء
common-close = إغلاق
common-more = المزيد

## Command line

usage-help =
    الاستخدام: plaa [FILE OR ADDRESS]...

    تشغيل ملفات الفيديو والصوت، والبث من عناوين مثل
    https://example.com/live.m3u8. يُفتح كلٌّ منها في نافذة من نوافذ plaa
    قيد التشغيل، والذي يبدأ عند الحاجة.

    الخيارات:
      -h, --help        عرض هذه المساعدة
      -V, --version     عرض الإصدار
          --interfaces  عرض الواجهات السلكية، وما إذا كان plaa يستطيع
                        إرسال تدفقات AVB عليها
          --mcp         تقديم MCP عبر stdin وstdout، ليرى وكلاء الذكاء الاصطناعي
                        ما يشغّله plaa قيد التشغيل ويتحكموا فيه

## Start window

start-hint = افتح ملف فيديو أو صوت، أو بثًّا، أو فيديو من الويب.
start-open = فتح
start-open-address = فتح عنوان
# Opens a list of the talkers on the AVB network to listen to.
start-connect = اتصال

## Actions, as buttons, tooltips and menus name them

action-open = فتح
action-open-address = فتح عنوان
open-file = فتح ملف…
open-address = فتح عنوان…
action-settings = الإعدادات
action-info = المراقب
action-loop = تكرار
action-screenshot = حفظ الإطار كصورة
action-play = تشغيل
action-pause = إيقاف مؤقت
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [zero] رجوع { $seconds } ثانية
    [one] رجوع ثانية واحدة
    [two] رجوع ثانيتين
    [few] رجوع { $seconds } ثوانٍ
    [many] رجوع { $seconds } ثانية
   *[other] رجوع { $seconds } ثانية
}
action-forward = { $seconds ->
    [zero] تقديم { $seconds } ثانية
    [one] تقديم ثانية واحدة
    [two] تقديم ثانيتين
    [few] تقديم { $seconds } ثوانٍ
    [many] تقديم { $seconds } ثانية
   *[other] تقديم { $seconds } ثانية
}
action-mute = كتم الصوت
action-unmute = إلغاء كتم الصوت
action-speed = السرعة
action-tracks = الصوت والترجمة
action-full-screen = ملء الشاشة
action-playlist = قائمة التشغيل
action-chapters = الفصول
action-picture-in-picture = صورة داخل صورة
action-previous = السابق
action-next = التالي
action-exit-full-screen = الخروج من ملء الشاشة
action-quality = الجودة
action-visualizer = المؤثرات المرئية
action-network = الصوت عبر الشبكة
action-cast = إرسال
action-connect = اتصال
action-assistant = المساعد

## What a sound file shows under its cover

visualizer-spectrum = الطيف
visualizer-waveform = الشكل الموجي
visualizer-prism = المنشور
visualizer-cover = الغلاف فقط

## The floating controls

overlay-keep-shown = إبقاء عناصر التحكم مثبّتة
overlay-auto-hide = تعويم عناصر التحكم فوق الصورة

## Audio and subtitle tracks

tracks-audio = الصوت
tracks-subtitles = الترجمة
tracks-off = إيقاف
# A track the file gives no name or language, by its number.
tracks-numbered = المسار { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = الصوت فقط

## Inspector

info-title = المراقب
playlist-title = قائمة التشغيل
chapters-numbered = الفصل { $number }
info-file = الملف
info-name = الاسم
info-address = العنوان
info-folder = المجلد
info-size = الحجم
info-format = التنسيق
info-duration = المدة
info-bit-rate = معدل البت
info-video = الفيديو
info-audio = الصوت
info-codec = الترميز
info-dimensions = الأبعاد
info-frame-rate = معدل الإطارات
info-frames-per-second = { $rate } إطار/ثانية
info-pixel-format = تنسيق البكسل
info-decoding = فك الترميز
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = بطاقة الرسومات ({ $api })
info-decoding-software = المعالج
info-sample-rate = معدل العينات
info-hertz = { $rate } Hz
info-channels = القنوات
info-language = اللغة
info-no-sound = لا يوجد صوت
info-tags = الوسوم
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = المُرسِل
info-stream = التدفق
# The talker's listeners other than plaa.
info-other-listeners = المستقبِلات الأخرى
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = فئة SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = يفصل التنسيق الجديد { $count ->
        [zero] { $count } مستقبِل آخر للمُرسِل.
        [one] المستقبِل الآخر للمُرسِل.
        [two] المستقبِلَين الآخرَين للمُرسِل.
        [few] { $count } مستقبِلات أخرى للمُرسِل.
        [many] { $count } مستقبِلًا آخر للمُرسِل.
       *[other] { $count } مستقبِل آخر للمُرسِل.
    }

## Dialogs

dialog-open = فتح
dialog-filter-media = الفيديو والصوت
dialog-filter-all = كل الملفات

address-title = فتح عنوان
address-note = صفحة ويب تحتوي على فيديو، مثل فيديو على YouTube، أو بث أو ملف على الشبكة، مثل https://example.com/live.m3u8.
address-label = العنوان
address-open = فتح
address-recent = الأخيرة
address-forget = إزالة من العناوين الأخيرة

## Notices

notice-dialog-failed = تعذّر فتح مربع حوار الملفات: { $error }
notice-screenshot-saved = تم حفظ “{ $name }”.
notice-screenshot-failed = تعذّر حفظ الصورة: { $error }
notice-no-sound = جارٍ التشغيل بدون صوت: { $reason }
notice-loop-from = التكرار من { $time }. اضغط مرة أخرى عند نقطة النهاية.
notice-loop-between = التكرار من { $from } إلى { $to }.
notice-loop-off = تم إيقاف التكرار.

## Errors, shown in place of what could not play

error-needs-yt-dlp = يبدو أن هذه صفحة ويب. يشغّل plaa مقاطع الفيديو في صفحات الويب باستخدام yt-dlp، وهو غير مثبّت.
error-empty-list = لا تحتوي هذه القائمة على شيء لتشغيله.

## Settings

settings-title = الإعدادات
settings-tab-appearance = المظهر
settings-tab-playback = التشغيل
settings-tab-storage = التخزين
settings-tab-network = الشبكة
settings-tab-language = اللغة
settings-tab-agents = الوكلاء
settings-tab-assistant = المساعد
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (إصدار تطويري، { $build })
settings-choose = اختيار…

settings-appearance-system = اتباع النظام
settings-appearance-light = فاتح
settings-appearance-dark = داكن
settings-colors = الألوان
settings-system-accent = استخدام لون التمييز في النظام
settings-omarchy-note = الألوان مبنية على لون التمييز في سمة “{ $theme }”.
settings-system-accent-note = الألوان مبنية على لون التمييز في النظام.
settings-system-accent-none = لا يوجد لون تمييز في النظام، لذا يستخدم plaa اللون المختار أدناه.
settings-accent-chosen-note = الألوان مبنية على اللون المختار أدناه.
settings-controls = عناصر التحكم
settings-overlay = عناصر تحكم عائمة
settings-overlay-note = يطفو شريط الأدوات وعناصر التحكم في التشغيل فوق الصورة أو الغلاف، وتنزلق بعيدًا عندما يستقر المؤشر أو يكون خارج النافذة.
settings-overlay-delay = الإخفاء بعد
settings-overlay-delay-note = المدة التي يستقر فيها المؤشر فوق ما يُشغَّل قبل أن تنزلق عناصر التحكم بعيدًا.
settings-seconds = { $seconds } ث
settings-transparency = الشفافية
settings-transparency-note = مقدار ما يظهر من الصورة عبر عناصر التحكم العائمة.
settings-percent = { $percent }%
settings-corner-radius = نصف قطر الزوايا
settings-corner-radius-note = لمربعات الحوار وعناصر التحكم العائمة.
settings-pixels = { $pixels } بكسل
settings-animations = الحركات
settings-animations-note = أشرطة ولوحات منزلقة، ومربعات حوار تتسع، وأزرار نابضة.
settings-animations-reduced = متوقفة عندما يطلب النظام تقليل الحركة.

settings-hardware = استخدام بطاقة الرسومات
settings-hardware-note = يُفك عليها ترميز الفيديو وتُرمَّز الملفات المُصدَّرة: فتستهلك طاقة أقل، وتُشغَّل مقاطع الفيديو الكبيرة بسلاسة، ويصبح التصدير أسرع. يسري على الملفات التي تُفتح بعد ذلك.
settings-audio-device = إخراج الصوت
settings-audio-device-system = الافتراضي للنظام
settings-resume = الاستئناف من حيث توقفت
settings-resume-note = تبدأ الملفات من حيث كانت عند إغلاقها آخر مرة.
settings-folder-playlists = تشغيل الملفات الأخرى في المجلد
settings-folder-playlists-note = يؤدي فتح ملف واحد إلى إدراج الملفات الأخرى في مجلده، لتُشغَّل بعده.
settings-streams = البث وفيديوهات الويب
settings-stream-quality = الجودة
settings-stream-quality-note = للبث وفيديوهات الويب المتاحة بعدة مستويات من الجودة. يسري على ما يُفتح بعد ذلك.
settings-quality-auto = تلقائي، بارتفاع الشاشة
settings-quality-best = الأفضل
settings-web-pages = تشغيل فيديوهات الويب باستخدام yt-dlp
settings-web-pages-found = يعثر yt-dlp على الفيديو في صفحات مثل صفحات YouTube.
settings-web-pages-missing = yt-dlp غير مثبّت. ثبّته لتشغيل مقاطع الفيديو في صفحات مثل صفحات YouTube.
settings-default-app = التطبيق الافتراضي
settings-default-app-label = فتح الملفات باستخدام plaa
settings-default-app-note = اجعل plaa التطبيق الذي يفتح ملفات الفيديو والصوت.
settings-default-app-note-windows = يتيح Windows اختيار التطبيقات الافتراضية في إعداداته فقط. يفتح هذا صفحة plaa هناك.
settings-default-app-note-macos = يطلب macOS تأكيد كل نوع: MP4 وMOV وMKV وWebM وMP3 وM4A وFLAC وWAV.
settings-default-app-status = { $set } من { $total } من أنواع الملفات تُفتح باستخدام plaa.
settings-default-app-button = تعيين كافتراضي
settings-default-app-button-windows = فتح الإعدادات
settings-default-app-no-entry = إدخال سطح المكتب الخاص بـ plaa غير مثبّت، لذا لا يستطيع النظام فتح الملفات به. ثبّت plaa من حزمة.
settings-default-app-no-bundle = افتح plaa من plaa.app لجعله التطبيق الافتراضي.
settings-default-app-failed = تعذّر جعل plaa التطبيق الافتراضي: { $error }
settings-folders = المجلدات
settings-sound-recordings = تسجيلات الصوت
settings-video-recordings = تسجيلات الفيديو
settings-screenshots = الإطارات المحفوظة

settings-interface-language = لغة الواجهة
# The first entry of the language list.
settings-language-system = اتباع النظام
settings-track-languages = لغات المسارات
settings-track-languages-note = رموز لغات مثل en أو fra، الأكثر تفضيلًا أولًا، مفصولة بفواصل. يختار plaa الصوت والترجمة بهذه اللغات عندما تتوفر في الملف.
settings-audio-languages = الصوت
settings-subtitle-languages = الترجمة

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = واجهة AVB
settings-avb-interface-note = المنفذ السلكي الذي يبحث فيه plaa عن مُرسِلات AVB وMilan ومستقبِلاتها. يتطلب الإرسال منفذًا بساعة PTP في العتاد، مثل منفذ Intel I210.
settings-avb-off = إيقاف
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (للعرض فقط: لا توجد ساعة PTP)
settings-avb-off-mac = { $interface } (للعرض فقط: AVB متوقف)
settings-avb-interface-note-mac = المنفذ السلكي الذي يبحث فيه plaa عن مُرسِلات AVB وMilan ومستقبِلاتها. يمر الإرسال عبر كيان AVB الخاص بجهاز Mac هذا، والذي يتطلب تشغيل AVB لهذا المنفذ في متصفح أجهزة الشبكة ضمن إعداد Audio MIDI.
settings-avb-talker-note-mac = يرسل plaa صوت آخر نافذة جرى التشغيل فيها عبر كيان AVB الخاص بجهاز Mac هذا، إلى المستقبِلات المتصلة به.
settings-avb-talker = إرسال ما يُشغَّل
settings-avb-talker-note = يصبح plaa مُرسِل Milan، فيرسل صوت آخر نافذة جرى التشغيل فيها إلى المستقبِلات المتصلة به.
settings-avb-name = الاسم على الشبكة
settings-avb-name-note = تعرض وحدات التحكم plaa باسم “{ $name }”. اضغط Enter لإعادة تسميته.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa على { $computer }
avb-could-not-start = تعذّر استخدام { $interface }: { $error }
network-title = الصوت عبر الشبكة
network-off = شغّل “إرسال ما يُشغَّل” في الإعدادات للتشغيل على الشبكة.
network-blocked = لا يتم الإرسال: { $reason }
network-waiting = جاهز للإرسال، في انتظار اتصال مستقبِل.
network-sending = جارٍ الإرسال إلى المستقبِلات المتصلة.
network-looking = جارٍ البحث عن مستقبِلات…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = جارٍ بدء ptp4l، الذي يحافظ على وقت الشبكة الذي يُرسَل به تدفق plaa…
network-ptp-elsewhere = يعمل ptp4l على واجهة أخرى، لذا لا يحافظ تدفق plaa على وقت الشبكة هنا وقد لا تتمكن المستقبِلات من تشغيله. اختر تلك الواجهة في الإعدادات، أو شغّل ptp4l على هذه الواجهة.
network-ptp-missing = ptp4l لا يعمل، لذا لا يحافظ تدفق plaa على وقت الشبكة وقد لا تتمكن المستقبِلات من تشغيله. ثبّت linuxptp مع حزمة plaa، أو ثبّت triib الذي يشغّله.
network-ptp-failed = ptp4l لا يعمل، لذا لا يحافظ تدفق plaa على وقت الشبكة وقد لا تتمكن المستقبِلات من تشغيله. تعذّر على plaa تشغيله: { $reason }
network-no-listeners = لا يوجد على الشبكة مستقبِل يمكنه تشغيل صوت plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = تشغيل على { $listener }
network-stop-on = إيقاف التشغيل على { $listener }
network-choose-format = اختر تنسيقًا للتشغيل على { $listener }
network-failed = تعذّر الاتصال: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = حدّث triib للتشغيل هنا.
network-choose = اختر مستقبِلًا لترى كيف يستقبل تدفقه.
network-no-sources = لا يوجد على الشبكة مُرسِل يرسل بتنسيق يشغّله plaa، أو يمكنه التحوّل إلى تنسيق كهذا.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = إيقاف الاستماع إلى { $talker }
network-listen-update-triib = حدّث triib للاستماع هنا: لا يشغّل plaa إلا ما يصل إلى هذا الكمبيوتر على أي حال.
network-listen-lite = يعمل المنفذ بنظام AVB Lite الذي لا يحجز أي تدفقات: لا يشغّل plaa إلا ما يصل إلى هذا الكمبيوتر على أي حال.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = جارٍ البحث عن مُرسِلات…
# plaa listens through its entity, which exists only while it sends.
connect-off = شغّل “إرسال ما يُشغَّل” في الإعدادات للاستماع إلى المُرسِلات هنا.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = لا يمكن لـ plaa الاستماع هنا: { $reason }
connect-sending = ترسل بتنسيق يشغّله plaa
connect-changing = تتحوّل إلى تنسيق يشغّله plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }، بتنسيق { $format }
connect-change-cuts-off = { $talker }، بتنسيق { $format }، مع فصل { $count ->
        [zero] { $count } مستقبِل آخر له
        [one] مستقبِله الآخر
        [two] مستقبِلَيه الآخرَين
        [few] { $count } مستقبِلات أخرى له
        [many] { $count } مستقبِلًا آخر له
       *[other] { $count } مستقبِل آخر له
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = الحالة
network-not-connected = غير متصل
network-from = من { $stream }
network-from-receiving = من { $stream }، جارٍ الاستقبال
network-from-waiting = من { $stream }، في انتظار المُرسِل
network-from-failed = من { $stream }، فشل حجز المُرسِل: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = ساعة الوسائط
network-locked = متزامنة مع التدفق
network-not-locked = غير متزامنة
network-format = التنسيق
# The formats both the listener and plaa's stream can be in.
network-formats = التنسيقات التي يمكنه استقبالها من plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = معرّف التدفق
# The MAC address the stream is sent to.
network-destination = الوجهة
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = زمن الانتقال
network-microseconds = { $time } µs
network-frames = الإطارات المستلمة
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = إطارات خارج التسلسل
# How often the stream stopped coming while it was taken.
network-interrupted = الانقطاعات
# Frames that came after, or long before, their time to be played.
network-late = الإطارات المتأخرة
network-early = الإطارات المبكرة
network-entity-id = معرّف الكيان

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = عرض النطاق غير كافٍ
msrp-failure-2 = موارد الجسر غير كافية
msrp-failure-3 = عرض النطاق غير كافٍ لفئة حركة المرور
msrp-failure-4 = معرّف التدفق مستخدم من قِبل مُرسِل آخر
msrp-failure-5 = عنوان الوجهة مستخدم بالفعل
msrp-failure-6 = أُزيح لصالح تدفق ذي رتبة أعلى
msrp-failure-7 = تغيّر زمن الانتقال المُبلَّغ عنه
msrp-failure-8 = منفذ الخروج لا يدعم AVB
msrp-failure-9 = استخدم عنوان وجهة مختلفًا
msrp-failure-10 = نفدت موارد MSRP
msrp-failure-11 = نفدت موارد MMRP
msrp-failure-12 = لا يمكن تخزين عنوان الوجهة
msrp-failure-13 = الأولوية ليست أولوية فئة SR
msrp-failure-14 = الإطارات أكبر من أن يحملها الوسيط
msrp-failure-15 = تم بلوغ حد منافذ fan-in
msrp-failure-16 = تغيّرت القيمة الأولى لتدفق مسجَّل
msrp-failure-17 = شبكة VLAN محظورة على منفذ الخروج
msrp-failure-18 = وسم VLAN معطّل على منفذ الخروج
msrp-failure-19 = عدم تطابق أولوية فئة SR
msrp-failure-unknown = سبب غير معروف
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }، عند الجسر { $bridge }

## The macOS menu bar

menu-about = حول plaa
menu-settings = الإعدادات…
menu-services = الخدمات
menu-hide = إخفاء plaa
menu-hide-others = إخفاء الآخرين
menu-show-all = إظهار الكل
menu-quit = إنهاء plaa
menu-file = ملف
menu-open = فتح…
menu-open-address = فتح عنوان…
menu-close = إغلاق النافذة
menu-edit = تحرير
menu-cut = قص
menu-copy = نسخ
menu-paste = لصق
menu-select-all = تحديد الكل
menu-undo = تراجع
menu-redo = إعادة
menu-trim = تشذيب…
menu-split = تقسيم المقطع
menu-rotate-left = تدوير إلى اليسار
menu-rotate-right = تدوير إلى اليمين
menu-flip-horizontal = قلب أفقي
menu-flip-vertical = قلب عمودي
menu-revert = الرجوع إلى الأصل
menu-export = تصدير…
menu-view = عرض
menu-playback = التشغيل
menu-play-pause = تشغيل أو إيقاف مؤقت
menu-to-start = الانتقال إلى البداية
menu-previous-chapter = الفصل السابق
menu-next-chapter = الفصل التالي
menu-faster = أسرع
menu-slower = أبطأ
menu-normal-speed = السرعة العادية
menu-ab-loop = التكرار بين نقطتين
menu-volume-up = رفع الصوت
menu-volume-down = خفض الصوت
menu-mute = كتم الصوت أو إلغاء كتمه
menu-next-subtitles = الترجمة التالية
menu-next-audio = المسار الصوتي التالي
menu-window = نافذة
menu-minimize = تصغير
menu-zoom = تكبير/تصغير
menu-bring-all-to-front = إحضار الكل إلى الأمام

## Editing

action-edit = تحرير
title-edited = { $name } (مُعدَّل)
edit-trim = تشذيب
edit-trim-apply = تشذيب
edit-split = تقسيم المقطع
edit-remove-clip = إزالة المقطع
edit-done = تم
edit-rotate-left = تدوير إلى اليسار
edit-rotate-right = تدوير إلى اليمين
edit-flip-horizontal = قلب أفقيًا
edit-flip-vertical = قلب عموديًا
edit-undo = تراجع
edit-redo = إعادة
edit-revert = الرجوع إلى الأصل
edit-export = تصدير…

## Exporting

export-title = تصدير
export-format = التنسيق
export-size = الحجم
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = الصوت فقط
export-size-original = { $lines }p، كما هو
export-on-graphics-card = يُرمَّز على بطاقة الرسومات ({ $api })
export-on-processor = يُرمَّز على المعالج، وهذا يستغرق وقتًا أطول
export-no-encoder = لا يتوفر هنا مُرمِّز لهذا التنسيق
export-checking = جارٍ البحث عن مُرمِّز…
export-sound-format = تنسيق الصوت
export-quality = الجودة
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = مضغوط؛ يُشغَّل في كل مكان تقريبًا، بما في ذلك أجهزة Apple
export-mp3-note = مضغوط؛ يُشغَّل في كل مكان
export-opus-note = مضغوط؛ أفضل صوت بالنسبة إلى حجمه، لكن ليست كل المشغّلات تدعمه
export-flac-note = بلا فقدان: الصوت كما هو تمامًا، بتنسيق مفتوح
export-alac-note = بلا فقدان: الصوت كما هو تمامًا، كما تشغّله أجهزة Apple
export-wav-note = غير مضغوط: الصوت كما هو تمامًا، في أكبر الملفات حجمًا
export-kilobits = { $rate } kb/s
export-bits = { $bits } بت
export-rate-lower = يُحفظ بمعدل { $rate } kHz، أقل من معدل الملف البالغ { $file } kHz
export-choose = تصدير…
export-name = { $name } مُصدَّر
export-name-edited = { $name } مُعدَّل
export-progress = جارٍ تصدير “{ $name }”
export-stop = إيقاف التصدير
notice-exported = تم تصدير “{ $name }”
notice-export-stopped = توقف التصدير
notice-export-failed = تعذّر التصدير: { $error }
close-title = هل تريد الإغلاق دون تصدير؟
close-unexported = ستضيع التغييرات على “{ $name }” ما لم تصدّرها.
close-exporting = سيتوقف تصدير “{ $name }”، وسيضيع ما كُتب حتى الآن.
close-anyway = إغلاق
close-quit-anyway = إنهاء

## Recording

record-title = تسجيل صوتي
record-file-name = تسجيل صوتي
record-start = تسجيل
record-stop = إيقاف التسجيل
record-no-input = لا يوجد ميكروفون
record-microphone = الميكروفون
record-default-microphone = الافتراضي للنظام
record-playing = ما يشغّله الكمبيوتر
record-mono = أحادي
record-stereo = ستيريو
record-channels = { $count ->
    [zero] { $count } قناة
    [one] قناة واحدة
    [two] قناتان
    [few] { $count } قنوات
    [many] { $count } قناة
   *[other] { $count } قناة
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }، يُحفظ بمعدل { $saved } kHz
action-record-sound = تسجيل الصوت
open-record-sound = تسجيل صوتي جديد
start-record-sound = تسجيل الصوت
menu-record-sound = تسجيل صوتي جديد
record-screen-title = تسجيل الشاشة
record-screen-file-name = تسجيل الشاشة
record-screen-whole = الشاشة
record-screen-number = الشاشة { $number }
record-screen-main = الشاشة { $number } (الرئيسية)
record-screens = الشاشات
record-windows = النوافذ
record-screen-what = ما الذي سيُسجَّل
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = كل الإطارات
record-rate-tip = الإطارات في الثانية
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [zero] { $size } · { $count } إطار مُسقَط
    [one] { $size } · إطار واحد مُسقَط
    [two] { $size } · إطاران مُسقَطان
    [few] { $size } · { $count } إطارات مُسقَطة
    [many] { $size } · { $count } إطارًا مُسقَطًا
   *[other] { $size } · { $count } إطار مُسقَط
}
record-screen-window = النافذة
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = انتهت المشاركة
record-screen-again = الاختيار مجددًا
record-screen-failed = تعذّر تسجيل الشاشة
record-sound = الصوت
record-no-sound = بدون صوت
# How much louder the input is taken, for a quiet microphone.
record-gain = التضخيم
record-gain-value = +{ $decibels } dB
record-level-automatically = ضبط المستوى تلقائيًا
record-hide-title = يختفي plaa أثناء التسجيل
record-hide-tray = لإيقاف التسجيل، انقر النقطة الحمراء التي يضعها plaa في علبة النظام، أو افتح plaa مرة أخرى.
record-hide-reopen = لإيقاف التسجيل، افتح plaa مرة أخرى.
record-hide-start = بدء التسجيل
record-tray-title = يسجّل plaa الشاشة
record-tray-tip = انقر لإيقاف التسجيل
action-record-screen = تسجيل الشاشة
open-record-screen = تسجيل شاشة جديد
start-record-screen = تسجيل الشاشة
menu-record-screen = تسجيل شاشة جديد
record-camera-title = تسجيل الكاميرا
record-camera-file-name = تسجيل الكاميرا
record-camera = الكاميرا
record-camera-failed = تعذّر فتح الكاميرا
record-name-title = سمِّ التسجيل
record-name-note = تم حفظه باسم “{ $name }” في { $folder }.
record-name-label = الاسم
record-name-keep = إبقاء
record-name-rename = إعادة التسمية
record-name-taken = يوجد بالفعل ملف باسم “{ $name }” هناك.
record-name-invalid = لا يمكن أن يحتوي الاسم على أيٍّ من هذه: / \ : * ? " < > |
record-name-failed = تعذّرت إعادة التسمية: { $error }
action-record-camera = تسجيل الكاميرا
open-record-camera = تسجيل كاميرا جديد
start-record-camera = تسجيل الكاميرا
menu-record-camera = تسجيل كاميرا جديد
notice-no-camera = لا توجد كاميرا للتسجيل
notice-recorded = تم حفظ “{ $name }” في { $folder }
notice-record-failed = تعذّر التسجيل: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = السماح بالتحكم الخارجي
settings-allow-outside-control-note = يمكن لوكلاء الذكاء الاصطناعي مثل Claude Code رؤية ما يُشغَّل في plaa والتحكم فيه، عبر plaa --mcp. يسأل plaa قبل كل وكيل جديد.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = المسموح لهم: { $agents }
settings-forget-agents = نسيان
settings-ask-before-note = السؤال قبل أن يقوم وكيل بما يلي:
settings-ask-reading = رؤية ما يُشغَّل، أو الصورة
settings-ask-playing = التحكم في التشغيل أو الصوت أو ملء الشاشة
settings-ask-opening = فتح ملف أو إغلاق نافذة
settings-ask-editing = تحرير ملف أو تصديره
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = هل تسمح لـ { $agent } بالتحكم في plaa؟
agent-prompt-body = يطلب { $agent } استخدام التحكم الخارجي في plaa، لرؤية ما يُشغَّل والتحكم فيه. يمكنك إيقاف التحكم الخارجي من الإعدادات.
agent-prompt-allow = سماح
agent-prompt-deny = عدم السماح
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = هل تسمح لـ { $agent } برؤية ما يُشغَّل هنا؟
agent-ask-play = هل تسمح لـ { $agent } بالتحكم في التشغيل هنا؟
agent-ask-open = هل تسمح لـ { $agent } بفتح نافذة أو إغلاقها؟
agent-ask-edit = هل تسمح لـ { $agent } بالتحرير أو التصدير هنا؟
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = يطلب { $agent } استخدام “{ $tool }”. تحدد الإعدادات ما يسأل عنه plaa.

## Casting to a TV or speaker on the network

cast-looking = جارٍ البحث عن أجهزة تلفزيون ومكبّرات صوت…
cast-none = لم يُعثر على تلفزيون أو مكبّر صوت أو Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = إيقاف التشغيل على { $renderer }
notice-casting = جارٍ التشغيل على { $renderer }.
notice-cast-failed = تعذّر التشغيل على { $renderer }: { $reason }
cast-web-page = لا يمكن إرسال صفحات الويب بعد: افتح عنوان الفيديو نفسه بدلًا من ذلك.

## The assistant panel, and its models in Settings

assistant-add-another = إضافة نموذج…
assistant-add-model = يحتاج المساعد إلى نموذج: نموذج سحابي مع مفتاح API الخاص به، أو نموذج محلي.
assistant-ask = اسأل عمّا يُشغَّل
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = أصبحت المحادثة أطول من أن يقرأها { $model } دفعة واحدة، لذا لُخّص جزؤها الأول. قد يلقي المساعد نظرة أخرى على ما رآه.
assistant-compacting = جارٍ تلخيص بداية المحادثة
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } على هذا الكمبيوتر
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } على { $host }
assistant-new-chat = محادثة جديدة
assistant-no-model = أضف نموذجًا في الإعدادات أولًا.
assistant-open-settings = إضافة نموذج
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = أصبحت المحادثة أكبر مما يستطيع { $model } قراءته دفعة واحدة. ابدأ محادثة جديدة، أو اختر نموذجًا يستطيع قراءة المزيد.
assistant-problem-credit = يقول { $provider } إن الحساب لا يحتوي على رصيد. يحتاج الحساب الجديد إلى شراء بعض الرصيد على موقع { $provider } قبل أن يعمل مفتاحه؛ ثم حاول مرة أخرى.
assistant-problem-key = رفض { $provider } مفتاح API. تحقّق منه في الإعدادات.
assistant-problem-model = لا يوجد لدى { $provider } نموذج باسم { $model }. تحقّق من اسمه في الإعدادات.
# $message is what the provider said, untranslated.
assistant-problem-other = لم يُجب { $model }: { $message }
assistant-problem-rate = يطلب { $provider } الإبطاء. حاول مرة أخرى بعد لحظة.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = يطلب { $provider } الإبطاء: { $message }
assistant-problem-refused = رفض { $model } الإجابة.
assistant-problem-unavailable = { $provider } مشغول أو يواجه مشكلة. حاول مرة أخرى بعد لحظة.
assistant-problem-unreachable = تعذّر على plaa الوصول إلى { $provider }. تحقّق من اتصالك، أو من أن الخادم يعمل.
assistant-running = جارٍ التشغيل…
assistant-send = إرسال
assistant-shortened = كانت المحادثة أطول من أن يقرأها { $model } دفعة واحدة، لذا حُذفت نتائج الأدوات السابقة. قد يلقي المساعد نظرة أخرى على ما رآه.
assistant-stop = إيقاف
assistant-stopped = تم الإيقاف.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = المحادثة الآن مع { $model }.
assistant-thinking = جارٍ التفكير…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = الأفكار
assistant-title = المساعد
settings-assistant-add = إضافة نموذج
settings-assistant-add-button = إضافة
# $model is the model's name.
settings-assistant-added = أجاب { $model } وتمت إضافته.
settings-assistant-added-tag = مُضاف
# $example is an address, such as http://localhost:11434.
settings-assistant-address = العنوان، مثل { $example }
settings-assistant-change-key = تغيير المفتاح
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = خادم متوافق مع OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = السياق
settings-assistant-context-note = يتيح الحجم الأكبر للمساعد أن يبقي في ذهنه جزءًا أكبر من المحادثة، لكن النموذج يستهلك ذاكرة أكثر وقد يجيب بشكل أبطأ.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } ألف رمز مميز
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = فتح الصفحة
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } يعمل على { $address }.
settings-assistant-found-none = لا توجد نماذج في { $provider } بعد. نزّل نموذجًا باستخدامه، ثم ابحث مرة أخرى.
settings-assistant-get-key = الحصول على مفتاح API
settings-assistant-get-server = الحصول على { $provider }
settings-assistant-in-use = قيد الاستخدام
settings-assistant-key = مفتاح API
# $error is what the keychain said.
settings-assistant-key-failed = تعذّر حفظ المفتاح في سلسلة المفاتيح: { $error }
settings-assistant-key-kept = مفتاح { $provider } الخاص بك محفوظ في سلسلة مفاتيح النظام.
settings-assistant-key-needed = يحتاج هذا النموذج إلى مفتاح API.
settings-assistant-key-refused = رفض { $provider } هذا المفتاح. تحقّق من أنه نُسخ كاملًا ومن الحساب الصحيح.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = أنشئ مفتاحًا على موقع { $provider } والصقه هنا.
settings-assistant-look-again = البحث مرة أخرى
settings-assistant-looking = جارٍ البحث عن النماذج…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = النموذج، مثل { $example }
settings-assistant-no-server = لم يجد plaa أي { $provider } يعمل على هذا الكمبيوتر. شغّله، أو أدخل عنوانه أدناه.
settings-assistant-no-tools = لا يمكنه استخدام الأدوات، وهي ما يحتاج إليه المساعد
settings-assistant-none = لا توجد نماذج بعد. أضف نموذجًا أدناه: النموذج المحلي، مثل نماذج Ollama، يبقى على هذا الكمبيوتر؛ والنموذج السحابي يحتاج إلى مفتاح API من مزوّده.
settings-assistant-note = النماذج التي يمكن للوحة المساعد التحدث إليها. تُحفظ المفاتيح في سلسلة مفاتيح النظام.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = استخدام عنوان آخر
settings-assistant-recommended = موصى به
settings-assistant-remove = إزالة
settings-assistant-sees = يرى الصور
settings-assistant-trying = جارٍ التجربة…
settings-assistant-use = استخدام
settings-assistant-use-key = متابعة
settings-assistant-uses-tools = يستخدم الأدوات
