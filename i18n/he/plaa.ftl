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
language-name = עברית

## Common

app-name = plaa
common-cancel = ביטול
common-close = סגירה
common-more = עוד

## Command line

usage-help =
    שימוש: plaa [FILE OR ADDRESS]...

    ניגון קובצי וידאו ושמע, וזרמים בכתובות כמו
    https://example.com/live.m3u8. כל אחד נפתח בחלון של plaa שכבר פועל,
    והוא מופעל אם צריך.

    אפשרויות:
      -h, --help        הצגת העזרה הזו
      -V, --version     הצגת הגרסה
          --interfaces  הצגת הממשקים הקוויים, והאם plaa יכול לשלוח
                        בהם זרמי AVB
          --mcp         הגשת MCP דרך stdin ו-stdout, כדי שסוכני AI יראו מה
                        מתנגן ב-plaa שכבר פועל וישלטו בו

## Start window

start-hint = אפשר לפתוח קובץ וידאו או שמע, זרם או סרטון מהאינטרנט.
start-open = פתיחה
start-open-address = פתיחת כתובת
# Opens a list of the talkers on the AVB network to listen to.
start-connect = התחברות

## Actions, as buttons, tooltips and menus name them

action-open = פתיחה
action-open-address = פתיחת כתובת
open-file = פתיחת קובץ…
open-address = פתיחת כתובת…
action-settings = הגדרות
action-info = פרטים
action-loop = לולאה
action-screenshot = שמירת הפריים כתמונה
action-play = ניגון
action-pause = השהיה
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] שנייה אחת אחורה
    [two] שתי שניות אחורה
   *[other] { $seconds } שניות אחורה
}
action-forward = { $seconds ->
    [one] שנייה אחת קדימה
    [two] שתי שניות קדימה
   *[other] { $seconds } שניות קדימה
}
action-mute = השתקה
action-unmute = ביטול השתקה
action-speed = מהירות
action-tracks = שמע וכתוביות
action-full-screen = מסך מלא
action-playlist = רשימת השמעה
action-chapters = פרקים
action-picture-in-picture = תמונה בתוך תמונה
action-previous = הקודם
action-next = הבא
action-exit-full-screen = יציאה ממסך מלא
action-quality = איכות
action-visualizer = ויזואליזציה
action-network = שמע ברשת
action-cast = העברה
action-connect = התחברות
action-assistant = עוזר

## What a sound file shows under its cover

visualizer-spectrum = ספקטרום
visualizer-waveform = צורת גל
visualizer-prism = מנסרה
visualizer-cover = עטיפה בלבד

## The floating controls

overlay-keep-shown = עיגון הפקדים
overlay-auto-hide = פקדים צפים מעל התמונה

## Audio and subtitle tracks

tracks-audio = שמע
tracks-subtitles = כתוביות
tracks-off = כבוי
# A track the file gives no name or language, by its number.
tracks-numbered = רצועה { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = שמע בלבד

## Inspector

info-title = פרטים
playlist-title = רשימת השמעה
chapters-numbered = פרק { $number }
info-file = קובץ
info-name = שם
info-address = כתובת
info-folder = תיקייה
info-size = גודל
info-format = פורמט
info-duration = משך
info-bit-rate = קצב סיביות
info-video = וידאו
info-audio = שמע
info-codec = קודק
info-dimensions = ממדים
info-frame-rate = קצב פריימים
info-frames-per-second = { $rate } פריימים לשנייה
info-pixel-format = פורמט פיקסלים
info-decoding = פענוח
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = כרטיס גרפי ({ $api })
info-decoding-software = מעבד
info-sample-rate = קצב דגימה
info-hertz = { $rate } Hz
info-channels = ערוצים
info-language = שפה
info-no-sound = ללא שמע
info-tags = תגיות
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = משדר
info-stream = זרם
# The talker's listeners other than plaa.
info-other-listeners = מאזינים אחרים
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = מחלקת SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = פורמט חדש מנתק את { $count ->
        [one] המאזין האחר של המשדר.
        [two] שני המאזינים האחרים של המשדר.
       *[other] { $count } המאזינים האחרים של המשדר.
    }

## Dialogs

dialog-open = פתיחה
dialog-filter-media = וידאו ושמע
dialog-filter-all = כל הקבצים

address-title = פתיחת כתובת
address-note = דף אינטרנט עם סרטון, כמו דף של סרטון ב-YouTube, או זרם או קובץ ברשת, כמו https://example.com/live.m3u8.
address-label = כתובת
address-open = פתיחה
address-recent = אחרונים
address-forget = הסרה מהאחרונים

## Notices

notice-dialog-failed = לא ניתן לפתוח את חלון בחירת הקבצים: { $error }
notice-screenshot-saved = הקובץ "{ $name }" נשמר.
notice-screenshot-failed = לא ניתן לשמור את התמונה: { $error }
notice-no-sound = מתנגן ללא שמע: { $reason }
notice-loop-from = לולאה מ-{ $time }. יש ללחוץ שוב בנקודה שבה היא צריכה להסתיים.
notice-loop-between = לולאה מ-{ $from } עד { $to }.
notice-loop-off = הלולאה כבויה.

## Errors, shown in place of what could not play

error-needs-yt-dlp = זה נראה כמו דף אינטרנט. plaa מנגן סרטונים מדפי אינטרנט באמצעות yt-dlp, שאינו מותקן.
error-empty-list = אין ברשימה הזו מה לנגן.

## Settings

settings-title = הגדרות
settings-tab-appearance = מראה
settings-tab-playback = ניגון
settings-tab-network = רשת
settings-tab-language = שפה
settings-tab-agents = סוכנים
settings-tab-assistant = עוזר
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (גרסת פיתוח, { $build })
settings-choose = בחירה…

settings-appearance-system = לפי המערכת
settings-appearance-light = בהיר
settings-appearance-dark = כהה
settings-colors = צבעים
settings-system-accent = שימוש בצבע ההדגשה של המערכת
settings-omarchy-note = הצבעים נבנים מצבע ההדגשה של "{ $theme }".
settings-system-accent-note = הצבעים נבנים מצבע ההדגשה של המערכת.
settings-system-accent-none = למערכת אין צבע הדגשה, ולכן plaa משתמש בצבע שנבחר למטה.
settings-accent-chosen-note = הצבעים נבנים מהצבע שנבחר למטה.
settings-controls = פקדים
settings-overlay = פקדים צפים
settings-overlay-note = סרגל הכלים ופקדי הניגון צפים מעל התמונה או העטיפה, ומחליקים הצידה כשהסמן נח או נמצא מחוץ לחלון.
settings-overlay-delay = הסתרה אחרי
settings-overlay-delay-note = כמה זמן הסמן נח מעל מה שמתנגן לפני שהפקדים מחליקים הצידה.
settings-seconds = { $seconds } ש׳
settings-transparency = שקיפות
settings-transparency-note = כמה מהתמונה נראה דרך הפקדים הצפים.
settings-percent = { $percent }%
settings-corner-radius = רדיוס פינות
settings-corner-radius-note = לחלונות דו-שיח ולפקדים הצפים.
settings-pixels = { $pixels } פיקסלים
settings-animations = אנימציות
settings-animations-note = סרגלים וחלוניות שמחליקים, חלונות דו-שיח שגדלים וכפתורים קפיציים.
settings-animations-reduced = כבויות כשהמערכת מבקשת להפחית תנועה.

settings-hardware = שימוש בכרטיס הגרפי
settings-hardware-note = פענוח הווידאו וקידוד הייצוא נעשים בו: צריכת חשמל נמוכה יותר, ניגון חלק של סרטונים גדולים וייצוא מהיר יותר. חל על קבצים שייפתחו מעכשיו.
settings-audio-device = פלט שמע
settings-audio-device-system = ברירת המחדל של המערכת
settings-resume = המשך מהמקום שבו הפסקת
settings-resume-note = קבצים מתחילים מהנקודה שבה היו כשנסגרו לאחרונה.
settings-folder-playlists = ניגון שאר הקבצים בתיקייה
settings-folder-playlists-note = פתיחת קובץ אחד מוסיפה לרשימה את שאר הקבצים בתיקייה שלו, והם מתנגנים אחריו.
settings-streams = זרמים וסרטונים מהאינטרנט
settings-stream-quality = איכות
settings-stream-quality-note = לזרמים ולסרטונים מהאינטרנט שמגיעים בכמה איכויות. חל על מה שייפתח מעכשיו.
settings-quality-auto = אוטומטית, בגובה המסך
settings-quality-best = הטובה ביותר
settings-web-pages = ניגון סרטונים מהאינטרנט עם yt-dlp
settings-web-pages-found = yt-dlp מוצא את הסרטון בדפים כמו אלה של YouTube.
settings-web-pages-missing = yt-dlp אינו מותקן. אפשר להתקין אותו כדי לנגן סרטונים מדפים כמו אלה של YouTube.
settings-folders = תיקיות
settings-recordings = הקלטות
settings-screenshots = פריימים שנשמרו

settings-interface-language = שפת הממשק
# The first entry of the language list.
settings-language-system = לפי המערכת
settings-track-languages = שפות הרצועות
settings-track-languages-note = קודי שפה כמו en או fra, מופרדים בפסיקים, המועדפת ראשונה. plaa בוחר שמע וכתוביות בשפות האלה כשהן קיימות בקובץ.
settings-audio-languages = שמע
settings-subtitle-languages = כתוביות

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = ממשק AVB
settings-avb-interface-note = היציאה הקווית שבה plaa מוצא משדרים ומאזינים של AVB ו-Milan. כדי לשלוח צריך יציאה עם שעון חומרה של PTP, כמו זו של Intel I210.
settings-avb-off = כבוי
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (הצגה בלבד: אין שעון PTP)
settings-avb-off-mac = { $interface } (הצגה בלבד: AVB כבוי)
settings-avb-interface-note-mac = היציאה הקווית שבה plaa מוצא משדרים ומאזינים של AVB ו-Milan. השליחה עוברת דרך ישות ה-AVB של ה-Mac הזה, ולשם כך צריך להפעיל AVB ליציאה הזו ב-Network Device Browser של Audio MIDI Setup.
settings-avb-talker-note-mac = plaa שולח את הצליל של החלון האחרון שהתנגן דרך ישות ה-AVB של ה-Mac הזה, אל המאזינים שמחוברים אליה.
settings-avb-talker = שליחת מה שמתנגן
settings-avb-talker-note = plaa הופך למשדר Milan ושולח את הצליל של החלון האחרון שהתנגן אל המאזינים שמחוברים אליו.
settings-avb-name = שם ברשת
settings-avb-name-note = בקרים מציגים את plaa בשם "{ $name }". כדי לשנות את השם יש להקיש Enter.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa ב-{ $computer }
avb-could-not-start = לא ניתן להשתמש ב-{ $interface }: { $error }
network-title = שמע ברשת
network-off = כדי לנגן ברשת, יש להפעיל את "שליחת מה שמתנגן" בהגדרות.
network-blocked = אין שליחה: { $reason }
network-waiting = מוכן לשליחה, ממתין שמאזין יתחבר.
network-sending = שולח אל המאזינים המחוברים.
network-looking = מחפש מאזינים…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = מפעיל את ptp4l, ששומר על זמן הרשת שלפיו נשלח הזרם של plaa…
network-ptp-elsewhere = ptp4l פועל בממשק אחר, ולכן הזרם של plaa לא שומר כאן על זמן הרשת וייתכן שמאזינים לא ינגנו אותו. אפשר לבחור בהגדרות את הממשק ההוא, או להפעיל את ptp4l בממשק הזה.
network-ptp-missing = ptp4l לא פועל, ולכן הזרם של plaa לא שומר על זמן הרשת וייתכן שמאזינים לא ינגנו אותו. אפשר להתקין את linuxptp עם החבילה של plaa, או את triib, שמפעיל אותו.
network-ptp-failed = ptp4l לא פועל, ולכן הזרם של plaa לא שומר על זמן הרשת וייתכן שמאזינים לא ינגנו אותו. plaa לא הצליח להפעיל אותו: { $reason }
network-no-listeners = אין ברשת מאזין שיכול לנגן את הצליל של plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = ניגון ב-{ $listener }
network-stop-on = הפסקת הניגון ב-{ $listener }
network-choose-format = בחירת פורמט לניגון ב-{ $listener }
network-failed = לא ניתן להתחבר: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = כדי לנגן כאן, יש לעדכן את triib.
network-choose = יש לבחור מאזין כדי לראות איך הוא מקבל את הזרם שלו.
network-no-sources = אין ברשת משדר ששולח בפורמט ש-plaa מנגן, או שיכול לעבור לפורמט כזה.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = הפסקת ההאזנה ל-{ $talker }
network-listen-update-triib = כדי להאזין כאן, יש לעדכן את triib: plaa מנגן רק את מה שמגיע ממילא למחשב הזה.
network-listen-lite = היציאה פועלת ב-AVB Lite, שאינו מקצה רוחב פס לזרמים: plaa מנגן רק את מה שמגיע ממילא למחשב הזה.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = מחפש משדרים…
# plaa listens through its entity, which exists only while it sends.
connect-off = כדי להאזין כאן למשדרים, יש להפעיל את "שליחת מה שמתנגן" בהגדרות.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa לא יכול להאזין כאן: { $reason }
connect-sending = שולחים בפורמט ש-plaa מנגן
connect-changing = עוברים לפורמט ש-plaa מנגן
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, בפורמט { $format }
connect-change-cuts-off = { $talker }, בפורמט { $format }, תוך ניתוק { $count ->
        [one] המאזין האחר שלו
        [two] שני המאזינים האחרים שלו
       *[other] { $count } המאזינים האחרים שלו
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = מצב
network-not-connected = לא מחובר
network-from = מ-{ $stream }
network-from-receiving = מ-{ $stream }, בקליטה
network-from-waiting = מ-{ $stream }, ממתין למשדר
network-from-failed = מ-{ $stream }, הקצאת רוחב הפס של המשדר נכשלה: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = שעון מדיה
network-locked = נעול לזרם
network-not-locked = לא נעול
network-format = פורמט
# The formats both the listener and plaa's stream can be in.
network-formats = פורמטים שהוא יכול לקבל מ-plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = מזהה זרם
# The MAC address the stream is sent to.
network-destination = יעד
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = השהיה
network-microseconds = { $time } µs
network-frames = מסגרות שהתקבלו
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = מסגרות שלא לפי הסדר
# How often the stream stopped coming while it was taken.
network-interrupted = הפסקות
# Frames that came after, or long before, their time to be played.
network-late = מסגרות שאיחרו
network-early = מסגרות שהקדימו
network-entity-id = מזהה ישות

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = אין מספיק רוחב פס
msrp-failure-2 = אין מספיק משאבים בגשר
msrp-failure-3 = אין מספיק רוחב פס למחלקת התעבורה
msrp-failure-4 = מזהה הזרם נמצא בשימוש של משדר אחר
msrp-failure-5 = כתובת היעד כבר בשימוש
msrp-failure-6 = נדחק על ידי זרם בדרגה גבוהה יותר
msrp-failure-7 = ההשהיה המדווחת השתנתה
msrp-failure-8 = יציאת השליחה אינה תומכת ב-AVB
msrp-failure-9 = יש להשתמש בכתובת יעד אחרת
msrp-failure-10 = נגמרו משאבי MSRP
msrp-failure-11 = נגמרו משאבי MMRP
msrp-failure-12 = לא ניתן לשמור את כתובת היעד
msrp-failure-13 = העדיפות אינה עדיפות של מחלקת SR
msrp-failure-14 = המסגרות גדולות מדי עבור התווך
msrp-failure-15 = הושגה מגבלת היציאות המתכנסות (fan-in)
msrp-failure-16 = הערך הראשון השתנה עבור זרם רשום
msrp-failure-17 = ה-VLAN חסום ביציאת השליחה
msrp-failure-18 = תיוג VLAN מושבת ביציאת השליחה
msrp-failure-19 = אי-התאמה בעדיפות מחלקת SR
msrp-failure-unknown = סיבה לא ידועה
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, בגשר { $bridge }

## The macOS menu bar

menu-about = אודות plaa
menu-settings = הגדרות…
menu-services = שירותים
menu-hide = הסתרת plaa
menu-hide-others = הסתרת האחרים
menu-show-all = הצגת הכול
menu-quit = יציאה מ-plaa
menu-file = קובץ
menu-open = פתיחה…
menu-open-address = פתיחת כתובת…
menu-close = סגירת החלון
menu-edit = עריכה
menu-cut = גזירה
menu-copy = העתקה
menu-paste = הדבקה
menu-select-all = בחירת הכול
menu-undo = ביטול
menu-redo = ביצוע חוזר
menu-trim = קיצוץ…
menu-split = פיצול הקליפ
menu-rotate-left = סיבוב שמאלה
menu-rotate-right = סיבוב ימינה
menu-flip-horizontal = היפוך אופקי
menu-flip-vertical = היפוך אנכי
menu-revert = שחזור למקור
menu-export = ייצוא…
menu-view = תצוגה
menu-playback = ניגון
menu-play-pause = ניגון או השהיה
menu-to-start = מעבר להתחלה
menu-previous-chapter = הפרק הקודם
menu-next-chapter = הפרק הבא
menu-faster = מהר יותר
menu-slower = לאט יותר
menu-normal-speed = מהירות רגילה
menu-ab-loop = לולאה בין שתי נקודות
menu-volume-up = חזק יותר
menu-volume-down = שקט יותר
menu-mute = השתקה או ביטול השתקה
menu-next-subtitles = הכתוביות הבאות
menu-next-audio = רצועת השמע הבאה
menu-window = חלון
menu-minimize = מזעור
menu-zoom = הגדלת החלון
menu-bring-all-to-front = הבאת הכול לחזית

## Editing

action-edit = עריכה
title-edited = { $name } (נערך)
edit-trim = קיצוץ
edit-trim-apply = קיצוץ
edit-split = פיצול הקליפ
edit-remove-clip = הסרת הקליפ
edit-done = סיום
edit-rotate-left = סיבוב שמאלה
edit-rotate-right = סיבוב ימינה
edit-flip-horizontal = היפוך אופקי
edit-flip-vertical = היפוך אנכי
edit-undo = ביטול פעולה
edit-redo = ביצוע חוזר
edit-revert = שחזור למקור
edit-export = ייצוא…

## Exporting

export-title = ייצוא
export-format = פורמט
export-size = גודל
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = שמע בלבד
export-size-original = { $lines }p, כמו שהוא
export-on-graphics-card = מקודד בכרטיס הגרפי ({ $api })
export-on-processor = מקודד במעבד, וזה לוקח יותר זמן
export-no-encoder = אין כאן מקודד זמין לפורמט הזה
export-checking = מחפש מקודד…
export-sound-format = פורמט שמע
export-quality = איכות
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = דחוס; מתנגן כמעט בכל מקום, כולל במכשירים של Apple
export-mp3-note = דחוס; מתנגן בכל מקום
export-opus-note = דחוס; הצליל הטוב ביותר ביחס לגודל, אבל לא כל נגן תומך בו
export-flac-note = ללא אובדן: הצליל בדיוק כמו שהוא, בפורמט פתוח
export-alac-note = ללא אובדן: הצליל בדיוק כמו שהוא, כפי שמכשירי Apple מנגנים אותו
export-wav-note = לא דחוס: הצליל בדיוק כמו שהוא, בקבצים הגדולים ביותר
export-kilobits = { $rate } kb/s
export-bits = { $bits } ביט
export-rate-lower = נשמר ב-{ $rate } kHz, פחות מ-{ $file } kHz של הקובץ
export-choose = ייצוא…
export-name = { $name } מיוצא
export-name-edited = { $name } ערוך
export-progress = מייצא את "{ $name }"
export-stop = הפסקת הייצוא
notice-exported = הקובץ "{ $name }" יוצא
notice-export-stopped = הייצוא הופסק
notice-export-failed = לא ניתן לייצא: { $error }
close-title = לסגור בלי לייצא?
close-unexported = השינויים ב-"{ $name }" יאבדו אם לא מייצאים אותם.
close-exporting = הייצוא של "{ $name }" ייפסק, ומה שנכתב עד עכשיו יאבד.
close-anyway = סגירה
close-quit-anyway = יציאה

## Recording

record-title = הקלטת שמע
record-file-name = הקלטת שמע
record-start = הקלטה
record-stop = הפסקת ההקלטה
record-no-input = אין מיקרופון
record-microphone = מיקרופון
record-default-microphone = ברירת המחדל של המערכת
record-playing = מה שהמחשב מנגן
record-mono = מונו
record-stereo = סטריאו
record-channels = { $count ->
    [one] ערוץ אחד
    [two] שני ערוצים
   *[other] { $count } ערוצים
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, נשמר ב-{ $saved } kHz
action-record-sound = הקלטת שמע
open-record-sound = הקלטת שמע חדשה
start-record-sound = הקלטת שמע
menu-record-sound = הקלטת שמע חדשה
record-screen-title = הקלטת מסך
record-screen-file-name = הקלטת מסך
record-screen-whole = מסך
record-screen-number = מסך { $number }
record-screen-main = מסך { $number } (ראשי)
record-screens = מסכים
record-windows = חלונות
record-screen-what = מה להקליט
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = כל פריים
record-rate-tip = פריימים לשנייה
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } הושמט
   *[other] { $size } · { $count } הושמטו
}
record-screen-window = חלון
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = כבר לא משותף
record-screen-again = בחירה מחדש
record-screen-failed = לא ניתן להקליט את המסך
record-sound = שמע
record-no-sound = ללא שמע
# How much louder the input is taken, for a quiet microphone.
record-gain = הגברה
record-gain-value = +{ $decibels } dB
record-level-automatically = כוונון עוצמה אוטומטי
record-hide-title = plaa מוסתר בזמן ההקלטה
record-hide-tray = כדי להפסיק את ההקלטה, יש ללחוץ על הנקודה האדומה ש-plaa מציג במגש המערכת, או לפתוח שוב את plaa.
record-hide-reopen = כדי להפסיק את ההקלטה, יש לפתוח שוב את plaa.
record-hide-start = התחלת ההקלטה
record-tray-title = plaa מקליט את המסך
record-tray-tip = יש ללחוץ כדי להפסיק את ההקלטה
action-record-screen = הקלטת המסך
open-record-screen = הקלטת מסך חדשה
start-record-screen = הקלטת מסך
menu-record-screen = הקלטת מסך חדשה
record-camera-title = הקלטת מצלמה
record-camera-file-name = הקלטת מצלמה
record-camera = מצלמה
record-camera-failed = לא ניתן לפתוח את המצלמה
action-record-camera = הקלטה מהמצלמה
open-record-camera = הקלטת מצלמה חדשה
start-record-camera = הקלטה מהמצלמה
menu-record-camera = הקלטת מצלמה חדשה
notice-no-camera = אין מצלמה להקלטה
notice-recorded = הקובץ "{ $name }" נשמר ב-{ $folder }
notice-record-failed = לא ניתן להקליט: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = התרת שליטה מבחוץ
settings-allow-outside-control-note = סוכני AI כמו Claude Code יכולים לראות מה מתנגן ב-plaa ולשלוט בו, דרך plaa --mcp. plaa שואל לפני כל סוכן חדש.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = מורשים: { $agents }
settings-forget-agents = הסרה
settings-ask-before-note = לשאול לפני פעולות אלה של סוכן:
settings-ask-reading = צפייה במה שמתנגן, או בתמונה
settings-ask-playing = שליטה בניגון, בשמע או במסך מלא
settings-ask-opening = פתיחת קובץ או סגירת חלון
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = לאפשר ל-{ $agent } לשלוט ב-plaa?
agent-prompt-body = { $agent } מבקש להשתמש בשליטה מבחוץ של plaa, כדי לראות מה מתנגן ולשלוט בו. אפשר לכבות את השליטה מבחוץ בהגדרות.
agent-prompt-allow = אישור
agent-prompt-deny = דחייה
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = לאפשר ל-{ $agent } לראות מה מתנגן כאן?
agent-ask-play = לאפשר ל-{ $agent } לשלוט בניגון כאן?
agent-ask-open = לאפשר ל-{ $agent } לפתוח או לסגור חלון?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } מבקש להשתמש ב-"{ $tool }". בהגדרות אפשר לבחור על מה plaa שואל.

## Casting to a TV or speaker on the network

cast-looking = מחפש טלוויזיות ורמקולים…
cast-none = לא נמצאו טלוויזיות, רמקולים או מכשירי Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = הפסקת הניגון ב-{ $renderer }
notice-casting = מתנגן ב-{ $renderer }.
notice-cast-failed = לא ניתן לנגן ב-{ $renderer }: { $reason }
cast-web-page = עדיין אי אפשר להעביר דפי אינטרנט: במקום זאת יש לפתוח את הכתובת של הסרטון עצמו.

## The assistant panel, and its models in Settings

assistant-add-another = הוספת מודל…
assistant-add-model = העוזר צריך מודל: מודל בענן עם מפתח ה-API שלו, או מודל מקומי.
assistant-ask = שאלה על מה שמתנגן
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = השיחה התארכה מעבר למה ש-{ $model } יכול לקרוא בבת אחת, ולכן החלק המוקדם שלה סוכם. ייתכן שהעוזר יסתכל שוב על מה שראה.
assistant-compacting = מסכם את תחילת השיחה
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } במחשב הזה
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } בכתובת { $host }
assistant-new-chat = שיחה חדשה
assistant-no-model = קודם צריך להוסיף מודל בהגדרות.
assistant-open-settings = הוספת מודל
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = השיחה כבר גדולה ממה ש-{ $model } יכול לקרוא בבת אחת. אפשר להתחיל שיחה חדשה, או לבחור מודל שיכול לקרוא יותר.
assistant-problem-credit = לפי { $provider }, אין בחשבון קרדיט. בחשבון חדש צריך לקנות קרדיט באתר של { $provider } לפני שהמפתח יעבוד; אחר כך אפשר לנסות שוב.
assistant-problem-key = { $provider } דחה את מפתח ה-API. אפשר לבדוק אותו בהגדרות.
assistant-problem-model = ל-{ $provider } אין מודל בשם { $model }. אפשר לבדוק את השם שלו בהגדרות.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } לא ענה: { $message }
assistant-problem-rate = { $provider } מבקש להאט. אפשר לנסות שוב בעוד רגע.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } מבקש להאט: { $message }
assistant-problem-refused = { $model } סירב לענות.
assistant-problem-unavailable = { $provider } עמוס או שיש בו תקלה. אפשר לנסות שוב בעוד רגע.
assistant-problem-unreachable = plaa לא הצליח להתחבר ל-{ $provider }. כדאי לבדוק את החיבור, או שהשרת פועל.
assistant-running = פועל…
assistant-send = שליחה
assistant-shortened = השיחה הייתה ארוכה ממה ש-{ $model } יכול לקרוא בבת אחת, ולכן תוצאות קודמות של כלים הושמטו. ייתכן שהעוזר יסתכל שוב על מה שראה.
assistant-stop = עצירה
assistant-stopped = נעצר.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = השיחה עכשיו עם { $model }.
assistant-thinking = חושב…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = מחשבות
assistant-title = עוזר
settings-assistant-add = הוספת מודל
settings-assistant-add-button = הוספה
# $model is the model's name.
settings-assistant-added = { $model } ענה ונוסף.
settings-assistant-added-tag = נוסף
# $example is an address, such as http://localhost:11434.
settings-assistant-address = כתובת, למשל { $example }
settings-assistant-change-key = החלפת המפתח
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = שרת תואם OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = הקשר
settings-assistant-context-note = ערך גבוה יותר מאפשר לעוזר לזכור חלק גדול יותר מהשיחה, אבל המודל צורך יותר זיכרון ועשוי לענות לאט יותר.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K טוקנים
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = פתיחת הדף
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } פועל בכתובת { $address }.
settings-assistant-found-none = אין עדיין מודלים ב-{ $provider }. אפשר להוריד מודל דרכו ואז לחפש שוב.
settings-assistant-get-key = קבלת מפתח API
settings-assistant-get-server = הורדת { $provider }
settings-assistant-in-use = בשימוש
settings-assistant-key = מפתח API
# $error is what the keychain said.
settings-assistant-key-failed = לא ניתן לשמור את המפתח בצרור המפתחות: { $error }
settings-assistant-key-kept = המפתח שלך ל-{ $provider } נשמר בצרור המפתחות של המערכת.
settings-assistant-key-needed = המודל הזה צריך מפתח API.
settings-assistant-key-refused = { $provider } דחה את המפתח הזה. כדאי לבדוק שהוא הועתק במלואו ומהחשבון הנכון.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = יש ליצור מפתח באתר של { $provider } ולהדביק אותו כאן.
settings-assistant-look-again = חיפוש חוזר
settings-assistant-looking = מחפש מודלים…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = מודל, למשל { $example }
settings-assistant-no-server = plaa לא מצא { $provider } שפועל במחשב הזה. אפשר להפעיל אותו, או לתת את הכתובת שלו למטה.
settings-assistant-no-tools = לא יכול להשתמש בכלים, והעוזר זקוק להם
settings-assistant-none = אין עדיין מודלים. אפשר להוסיף אחד למטה: מודל מקומי, כמו של Ollama, נשאר במחשב הזה; מודל בענן צריך מפתח API מהספק שלו.
settings-assistant-note = המודלים שחלונית העוזר יכולה לדבר איתם. המפתחות נשמרים בצרור המפתחות של המערכת.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = שימוש בכתובת אחרת
settings-assistant-recommended = מומלץ
settings-assistant-remove = הסרה
settings-assistant-sees = רואה תמונות
settings-assistant-trying = מנסה…
settings-assistant-use = שימוש
settings-assistant-use-key = המשך
settings-assistant-uses-tools = משתמש בכלים
