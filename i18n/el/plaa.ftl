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
language-name = Ελληνικά

## Common

app-name = plaa
common-cancel = Ακύρωση
common-close = Κλείσιμο
common-more = Περισσότερα

## Command line

usage-help =
    Χρήση: plaa [ΑΡΧΕΙΟ Ή ΔΙΕΥΘΥΝΣΗ]...

    Αναπαραγωγή αρχείων βίντεο και ήχου, καθώς και ροών από διευθύνσεις όπως
    https://example.com/live.m3u8. Καθένα ανοίγει σε παράθυρο του plaa που
    εκτελείται, το οποίο ξεκινά αν χρειάζεται.

    Επιλογές:
      -h, --help        Εμφάνιση αυτής της βοήθειας
      -V, --version     Εμφάνιση της έκδοσης
          --interfaces  Εμφάνιση των ενσύρματων διεπαφών και του αν μπορεί
                        το plaa να στέλνει ροές AVB μέσω αυτών
          --mcp         Εξυπηρέτηση MCP μέσω stdin και stdout, ώστε πράκτορες AI
                        να βλέπουν τι παίζει το εκτελούμενο plaa και να το ελέγχουν

## Start window

start-hint = Ανοίξτε ένα αρχείο βίντεο ή ήχου, μια ροή ή ένα βίντεο από τον ιστό.
start-open = Άνοιγμα
start-open-address = Άνοιγμα διεύθυνσης
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Σύνδεση

## Actions, as buttons, tooltips and menus name them

action-open = Άνοιγμα
action-open-address = Άνοιγμα διεύθυνσης
open-file = Άνοιγμα αρχείου…
open-address = Άνοιγμα διεύθυνσης…
action-settings = Ρυθμίσεις
action-info = Επιθεωρητής
action-loop = Βρόχος
action-screenshot = Αποθήκευση καρέ ως εικόνας
action-play = Αναπαραγωγή
action-pause = Παύση
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Πίσω { $seconds } δευτερόλεπτο
   *[other] Πίσω { $seconds } δευτερόλεπτα
}
action-forward = { $seconds ->
    [one] Μπροστά { $seconds } δευτερόλεπτο
   *[other] Μπροστά { $seconds } δευτερόλεπτα
}
action-mute = Σίγαση
action-unmute = Κατάργηση σίγασης
action-speed = Ταχύτητα
action-tracks = Ήχος και υπότιτλοι
action-full-screen = Πλήρης οθόνη
action-playlist = Λίστα αναπαραγωγής
action-chapters = Κεφάλαια
action-picture-in-picture = Εικόνα σε εικόνα
action-previous = Προηγούμενο
action-next = Επόμενο
action-exit-full-screen = Έξοδος από πλήρη οθόνη
action-quality = Ποιότητα
action-visualizer = Οπτικοποίηση
action-network = Ήχος δικτύου
action-cast = Μετάδοση
action-connect = Σύνδεση
action-assistant = Βοηθός

## What a sound file shows under its cover

visualizer-spectrum = Φάσμα
visualizer-waveform = Κυματομορφή
visualizer-prism = Πρίσμα
visualizer-cover = Μόνο εξώφυλλο

## The floating controls

overlay-keep-shown = Στοιχεία ελέγχου σε σταθερή θέση
overlay-auto-hide = Αιωρούμενα στοιχεία ελέγχου πάνω από την εικόνα

## Audio and subtitle tracks

tracks-audio = Ήχος
tracks-subtitles = Υπότιτλοι
tracks-off = Απενεργοποίηση
# A track the file gives no name or language, by its number.
tracks-numbered = Κομμάτι { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Μόνο ήχος

## Inspector

info-title = Επιθεωρητής
playlist-title = Λίστα αναπαραγωγής
chapters-numbered = Κεφάλαιο { $number }
info-file = Αρχείο
info-name = Όνομα
info-address = Διεύθυνση
info-folder = Φάκελος
info-size = Μέγεθος
info-format = Μορφή
info-duration = Διάρκεια
info-bit-rate = Ρυθμός bit
info-video = Βίντεο
info-audio = Ήχος
info-codec = Κωδικοποιητής
info-dimensions = Διαστάσεις
info-frame-rate = Ρυθμός καρέ
info-frames-per-second = { $rate } καρέ ανά δευτερόλεπτο
info-pixel-format = Μορφή εικονοστοιχείων
info-decoding = Αποκωδικοποίηση
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Κάρτα γραφικών ({ $api })
info-decoding-software = Επεξεργαστής
info-sample-rate = Ρυθμός δειγματοληψίας
info-hertz = { $rate } Hz
info-channels = Κανάλια
info-language = Γλώσσα
info-no-sound = Χωρίς ήχο
info-tags = Ετικέτες
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Πομπός
info-stream = Ροή
# The talker's listeners other than plaa.
info-other-listeners = Άλλοι δέκτες
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Κλάση SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Μια νέα μορφή αποσυνδέει { $count ->
        [one] τον άλλο δέκτη του πομπού.
       *[other] τους άλλους { $count } δέκτες του πομπού.
    }

## Dialogs

dialog-open = Άνοιγμα
dialog-filter-media = Βίντεο και ήχος
dialog-filter-all = Όλα τα αρχεία

address-title = Άνοιγμα διεύθυνσης
address-note = Μια ιστοσελίδα με βίντεο, όπως η σελίδα ενός βίντεο στο YouTube, ή μια ροή ή ένα αρχείο στο δίκτυο, όπως https://example.com/live.m3u8.
address-label = Διεύθυνση
address-open = Άνοιγμα
address-recent = Πρόσφατα
address-forget = Αφαίρεση από τα πρόσφατα

## Notices

notice-dialog-failed = Δεν ήταν δυνατό το άνοιγμα του παραθύρου αρχείων: { $error }
notice-screenshot-saved = Αποθηκεύτηκε το «{ $name }».
notice-screenshot-failed = Δεν ήταν δυνατή η αποθήκευση της εικόνας: { $error }
notice-no-sound = Αναπαραγωγή χωρίς ήχο: { $reason }
notice-loop-from = Βρόχος από { $time }. Πατήστε ξανά στο σημείο όπου θα τελειώνει.
notice-loop-between = Βρόχος από { $from } έως { $to }.
notice-loop-off = Ο βρόχος απενεργοποιήθηκε.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Αυτό μοιάζει με ιστοσελίδα. Το plaa αναπαράγει τα βίντεο των ιστοσελίδων με το yt-dlp, το οποίο δεν είναι εγκατεστημένο.
error-empty-list = Αυτή η λίστα δεν έχει τίποτα για αναπαραγωγή.

## Settings

settings-title = Ρυθμίσεις
settings-tab-appearance = Εμφάνιση
settings-tab-playback = Αναπαραγωγή
settings-tab-storage = Αποθηκευτικός χώρος
settings-tab-network = Δίκτυο
settings-tab-language = Γλώσσα
settings-tab-agents = Πράκτορες
settings-tab-assistant = Βοηθός
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (έκδοση ανάπτυξης, { $build })
settings-choose = Επιλογή…

settings-appearance-system = Σύμφωνα με το σύστημα
settings-appearance-light = Φωτεινή
settings-appearance-dark = Σκούρα
settings-colors = Χρώματα
settings-system-accent = Χρήση του χρώματος έμφασης του συστήματος
settings-omarchy-note = Τα χρώματα προκύπτουν από το χρώμα έμφασης του θέματος «{ $theme }».
settings-system-accent-note = Τα χρώματα προκύπτουν από το χρώμα έμφασης του συστήματος.
settings-system-accent-none = Το σύστημα δεν έχει χρώμα έμφασης, οπότε το plaa χρησιμοποιεί αυτό που έχει επιλεγεί παρακάτω.
settings-accent-chosen-note = Τα χρώματα προκύπτουν από το χρώμα που έχει επιλεγεί παρακάτω.
settings-controls = Στοιχεία ελέγχου
settings-overlay = Αιωρούμενα στοιχεία ελέγχου
settings-overlay-note = Η γραμμή εργαλείων και τα στοιχεία ελέγχου αναπαραγωγής αιωρούνται πάνω από την εικόνα ή το εξώφυλλο και αποσύρονται όσο ο δείκτης μένει ακίνητος ή βρίσκεται έξω από το παράθυρο.
settings-overlay-delay = Απόκρυψη μετά από
settings-overlay-delay-note = Πόση ώρα μένει ακίνητος ο δείκτης πάνω από ό,τι αναπαράγεται πριν αποσυρθούν τα στοιχεία ελέγχου.
settings-seconds = { $seconds } δευτ.
settings-transparency = Διαφάνεια
settings-transparency-note = Πόσο διακρίνεται η εικόνα μέσα από τα αιωρούμενα στοιχεία ελέγχου.
settings-percent = { $percent }%
settings-corner-radius = Ακτίνα γωνιών
settings-corner-radius-note = Για τα παράθυρα διαλόγου και τα αιωρούμενα στοιχεία ελέγχου.
settings-pixels = { $pixels } px
settings-animations = Κινούμενα εφέ
settings-animations-note = Γραμμές και πλαίσια που γλιστρούν, παράθυρα διαλόγου που μεγαλώνουν και ελαστικά κουμπιά.
settings-animations-reduced = Ανενεργά όσο το σύστημα ζητά μειωμένη κίνηση.

settings-hardware = Χρήση της κάρτας γραφικών
settings-hardware-note = Η αποκωδικοποίηση των βίντεο και η κωδικοποίηση των εξαγωγών γίνονται στην κάρτα γραφικών: λιγότερη κατανάλωση ενέργειας, ομαλή αναπαραγωγή μεγάλων βίντεο και ταχύτερες εξαγωγές. Ισχύει για τα αρχεία που ανοίγουν στη συνέχεια.
settings-audio-device = Έξοδος ήχου
settings-audio-device-system = Προεπιλογή συστήματος
settings-resume = Συνέχιση από εκεί που σταματήσατε
settings-resume-note = Τα αρχεία ξεκινούν από το σημείο όπου βρίσκονταν όταν έκλεισαν την τελευταία φορά.
settings-folder-playlists = Αναπαραγωγή των υπόλοιπων αρχείων του φακέλου
settings-folder-playlists-note = Όταν ανοίγετε ένα αρχείο, τα υπόλοιπα αρχεία του φακέλου του μπαίνουν στη λίστα και αναπαράγονται μετά από αυτό.
settings-streams = Ροές και βίντεο ιστού
settings-stream-quality = Ποιότητα
settings-stream-quality-note = Για ροές και βίντεο ιστού που διατίθενται σε διάφορες ποιότητες. Ισχύει για ό,τι ανοίγει στη συνέχεια.
settings-quality-auto = Αυτόματη, όσο το ύψος της οθόνης
settings-quality-best = Βέλτιστη
settings-web-pages = Αναπαραγωγή βίντεο ιστού με το yt-dlp
settings-web-pages-found = Το yt-dlp βρίσκει το βίντεο σε σελίδες όπως του YouTube.
settings-web-pages-missing = Το yt-dlp δεν είναι εγκατεστημένο. Εγκαταστήστε το για να αναπαράγετε τα βίντεο σε σελίδες όπως του YouTube.
settings-default-app = Προεπιλεγμένη εφαρμογή
settings-default-app-label = Άνοιγμα αρχείων με το plaa
settings-default-app-note = Ορισμός του plaa ως της εφαρμογής που ανοίγει αρχεία βίντεο και ήχου.
settings-default-app-note-windows = Στα Windows οι προεπιλεγμένες εφαρμογές ορίζονται μόνο από τις Ρυθμίσεις του συστήματος. Με αυτό ανοίγει εκεί η σελίδα του plaa.
settings-default-app-note-macos = Το macOS ζητά επιβεβαίωση για κάθε τύπο: MP4, MOV, MKV, WebM, MP3, M4A, FLAC και WAV.
settings-default-app-status = Τύποι αρχείων που ανοίγουν με το plaa: { $set } από { $total }.
settings-default-app-button = Ορισμός ως προεπιλογής
settings-default-app-button-windows = Άνοιγμα Ρυθμίσεων
settings-default-app-no-entry = Η καταχώριση επιφάνειας εργασίας (desktop entry) του plaa δεν είναι εγκατεστημένη, οπότε το σύστημα δεν μπορεί να ανοίξει αρχεία με αυτό. Εγκαταστήστε το plaa από πακέτο.
settings-default-app-no-bundle = Ανοίξτε το plaa από το plaa.app για να το ορίσετε ως προεπιλογή.
settings-default-app-failed = Δεν ήταν δυνατός ο ορισμός του plaa ως προεπιλογής: { $error }
settings-folders = Φάκελοι
settings-sound-recordings = Εγγραφές ήχου
settings-video-recordings = Εγγραφές βίντεο
settings-screenshots = Αποθηκευμένα καρέ

settings-interface-language = Γλώσσα περιβάλλοντος
# The first entry of the language list.
settings-language-system = Σύμφωνα με το σύστημα
settings-track-languages = Γλώσσες κομματιών
settings-track-languages-note = Κωδικοί γλωσσών όπως en ή fra, με σειρά προτίμησης, χωρισμένοι με κόμματα. Το plaa επιλέγει ήχο και υπότιτλους σε αυτές τις γλώσσες όταν τους έχει το αρχείο.
settings-audio-languages = Ήχος
settings-subtitle-languages = Υπότιτλοι

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Διεπαφή AVB
settings-avb-interface-note = Η ενσύρματη θύρα στην οποία το plaa βρίσκει πομπούς και δέκτες AVB και Milan. Για αποστολή χρειάζεται θύρα με ρολόι υλικού PTP, όπως αυτή μιας Intel I210.
settings-avb-off = Ανενεργό
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (μόνο προβολή: χωρίς ρολόι PTP)
settings-avb-off-mac = { $interface } (μόνο προβολή: το AVB είναι ανενεργό)
settings-avb-interface-note-mac = Η ενσύρματη θύρα στην οποία το plaa βρίσκει πομπούς και δέκτες AVB και Milan. Η αποστολή γίνεται μέσω της οντότητας AVB αυτού του Mac, για την οποία πρέπει να ενεργοποιηθεί το AVB για τη θύρα στο Πρόγραμμα περιήγησης συσκευών δικτύου της εφαρμογής Ρύθμιση ήχου MIDI.
settings-avb-talker-note-mac = Το plaa στέλνει τον ήχο του παραθύρου που έπαιξε τελευταίο μέσω της οντότητας AVB αυτού του Mac, στους δέκτες που είναι συνδεδεμένοι σε αυτήν.
settings-avb-talker = Αποστολή του ήχου αναπαραγωγής
settings-avb-talker-note = Το plaa γίνεται πομπός Milan και στέλνει τον ήχο του παραθύρου που έπαιξε τελευταίο στους δέκτες που είναι συνδεδεμένοι σε αυτόν.
settings-avb-name = Όνομα στο δίκτυο
settings-avb-name-note = Οι ελεγκτές εμφανίζουν το plaa ως «{ $name }». Πατήστε Enter για μετονομασία.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa στο { $computer }
avb-could-not-start = Δεν ήταν δυνατή η χρήση της διεπαφής { $interface }: { $error }
network-title = Ήχος δικτύου
network-off = Ενεργοποιήστε την «Αποστολή του ήχου αναπαραγωγής» στις Ρυθμίσεις για αναπαραγωγή στο δίκτυο.
network-blocked = Δεν γίνεται αποστολή: { $reason }
network-waiting = Έτοιμο για αποστολή, αναμονή για σύνδεση δέκτη.
network-sending = Αποστολή στους συνδεδεμένους δέκτες.
network-looking = Αναζήτηση δεκτών…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Εκκίνηση του ptp4l, που διατηρεί τον χρόνο δικτύου με τον οποίο στέλνεται η ροή του plaa…
network-ptp-elsewhere = Το ptp4l εκτελείται σε άλλη διεπαφή, οπότε η ροή του plaa δεν έχει χρόνο δικτύου εδώ και οι δέκτες ίσως να μην μπορούν να την αναπαράγουν. Επιλέξτε εκείνη τη διεπαφή στις Ρυθμίσεις ή εκτελέστε το ptp4l σε αυτήν.
network-ptp-missing = Το ptp4l δεν εκτελείται, οπότε η ροή του plaa δεν έχει χρόνο δικτύου και οι δέκτες ίσως να μην μπορούν να την αναπαράγουν. Εγκαταστήστε το linuxptp μαζί με το πακέτο του plaa ή το triib, που το εκτελεί.
network-ptp-failed = Το ptp4l δεν εκτελείται, οπότε η ροή του plaa δεν έχει χρόνο δικτύου και οι δέκτες ίσως να μην μπορούν να την αναπαράγουν. Το plaa δεν μπόρεσε να το ξεκινήσει: { $reason }
network-no-listeners = Κανένας δέκτης στο δίκτυο δεν μπορεί να αναπαράγει τον ήχο του plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = Αναπαραγωγή στο { $listener }
network-stop-on = Διακοπή αναπαραγωγής στο { $listener }
network-choose-format = Επιλέξτε μορφή για αναπαραγωγή στο { $listener }
network-failed = Δεν ήταν δυνατή η σύνδεση: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Ενημερώστε το triib για αναπαραγωγή εδώ.
network-choose = Επιλέξτε έναν δέκτη για να δείτε πώς λαμβάνει τη ροή του.
network-no-sources = Κανένας πομπός στο δίκτυο δεν στέλνει σε μορφή που αναπαράγει το plaa, ούτε μπορεί να αλλάξει σε τέτοια μορφή.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Διακοπή ακρόασης του { $talker }
network-listen-update-triib = Ενημερώστε το triib για ακρόαση εδώ: το plaa αναπαράγει μόνο ό,τι φτάνει έτσι κι αλλιώς σε αυτόν τον υπολογιστή.
network-listen-lite = Η θύρα λειτουργεί με AVB Lite, που δεν κάνει κράτηση ροών: το plaa αναπαράγει μόνο ό,τι φτάνει έτσι κι αλλιώς σε αυτόν τον υπολογιστή.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Αναζήτηση πομπών…
# plaa listens through its entity, which exists only while it sends.
connect-off = Ενεργοποιήστε την «Αποστολή του ήχου αναπαραγωγής» στις Ρυθμίσεις για ακρόαση πομπών εδώ.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = Το plaa δεν μπορεί να κάνει ακρόαση εδώ: { $reason }
connect-sending = Στέλνουν σε μορφή που αναπαράγει το plaa
connect-changing = Αλλάζουν σε μορφή που αναπαράγει το plaa
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, σε { $format }
connect-change-cuts-off = { $talker }, σε { $format }, με αποσύνδεση { $count ->
        [one] του άλλου δέκτη του
       *[other] των άλλων { $count } δεκτών του
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = Κατάσταση
network-not-connected = Δεν είναι συνδεδεμένος
network-from = Από { $stream }
network-from-receiving = Από { $stream }, λήψη σε εξέλιξη
network-from-waiting = Από { $stream }, αναμονή για τον πομπό
network-from-failed = Από { $stream }, η κράτηση του πομπού απέτυχε: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Ρολόι μέσων
network-locked = Κλειδωμένο στη ροή
network-not-locked = Μη κλειδωμένο
network-format = Μορφή
# The formats both the listener and plaa's stream can be in.
network-formats = Μορφές που μπορεί να λάβει από το plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Αναγνωριστικό ροής
# The MAC address the stream is sent to.
network-destination = Προορισμός
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Καθυστέρηση
network-microseconds = { $time } µs
network-frames = Πλαίσια που ελήφθησαν
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Πλαίσια εκτός σειράς
# How often the stream stopped coming while it was taken.
network-interrupted = Διακοπές
# Frames that came after, or long before, their time to be played.
network-late = Καθυστερημένα πλαίσια
network-early = Πρόωρα πλαίσια
network-entity-id = Αναγνωριστικό οντότητας

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = ανεπαρκές εύρος ζώνης
msrp-failure-2 = ανεπαρκείς πόροι γέφυρας
msrp-failure-3 = ανεπαρκές εύρος ζώνης για την κλάση κυκλοφορίας
msrp-failure-4 = το αναγνωριστικό ροής χρησιμοποιείται από άλλον πομπό
msrp-failure-5 = η διεύθυνση προορισμού χρησιμοποιείται ήδη
msrp-failure-6 = παραμερίστηκε από ροή υψηλότερης κατάταξης
msrp-failure-7 = η αναφερόμενη καθυστέρηση έχει αλλάξει
msrp-failure-8 = η θύρα εξόδου δεν υποστηρίζει AVB
msrp-failure-9 = χρησιμοποιήστε διαφορετική διεύθυνση προορισμού
msrp-failure-10 = εξαντλήθηκαν οι πόροι MSRP
msrp-failure-11 = εξαντλήθηκαν οι πόροι MMRP
msrp-failure-12 = δεν είναι δυνατή η αποθήκευση της διεύθυνσης προορισμού
msrp-failure-13 = η προτεραιότητα δεν είναι προτεραιότητα κλάσης SR
msrp-failure-14 = πλαίσια πολύ μεγάλα για το μέσο μετάδοσης
msrp-failure-15 = συμπληρώθηκε το όριο θυρών συγκέντρωσης (fan-in)
msrp-failure-16 = άλλαξε η πρώτη τιμή για καταχωρισμένη ροή
msrp-failure-17 = το VLAN είναι αποκλεισμένο στη θύρα εξόδου
msrp-failure-18 = η προσθήκη ετικετών VLAN είναι απενεργοποιημένη στη θύρα εξόδου
msrp-failure-19 = αναντιστοιχία προτεραιότητας κλάσης SR
msrp-failure-unknown = άγνωστη αιτία
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, στη γέφυρα { $bridge }

## The macOS menu bar

menu-about = Πληροφορίες για το plaa
menu-settings = Ρυθμίσεις…
menu-services = Υπηρεσίες
menu-hide = Απόκρυψη plaa
menu-hide-others = Απόκρυψη άλλων
menu-show-all = Εμφάνιση όλων
menu-quit = Τερματισμός plaa
menu-file = Αρχείο
menu-open = Άνοιγμα…
menu-open-address = Άνοιγμα διεύθυνσης…
menu-close = Κλείσιμο παραθύρου
menu-edit = Επεξεργασία
menu-cut = Αποκοπή
menu-copy = Αντιγραφή
menu-paste = Επικόλληση
menu-select-all = Επιλογή όλων
menu-undo = Αναίρεση
menu-redo = Επανάληψη
menu-trim = Περικοπή…
menu-split = Διαχωρισμός κλιπ
menu-rotate-left = Περιστροφή αριστερά
menu-rotate-right = Περιστροφή δεξιά
menu-flip-horizontal = Οριζόντια αναστροφή
menu-flip-vertical = Κατακόρυφη αναστροφή
menu-revert = Επαναφορά στο πρωτότυπο
menu-export = Εξαγωγή…
menu-view = Προβολή
menu-playback = Αναπαραγωγή
menu-play-pause = Αναπαραγωγή ή παύση
menu-to-start = Μετάβαση στην αρχή
menu-previous-chapter = Προηγούμενο κεφάλαιο
menu-next-chapter = Επόμενο κεφάλαιο
menu-faster = Ταχύτερα
menu-slower = Πιο αργά
menu-normal-speed = Κανονική ταχύτητα
menu-ab-loop = Βρόχος μεταξύ δύο σημείων
menu-volume-up = Δυνατότερα
menu-volume-down = Πιο σιγά
menu-mute = Σίγαση ή κατάργηση σίγασης
menu-next-subtitles = Επόμενοι υπότιτλοι
menu-next-audio = Επόμενο κομμάτι ήχου
menu-window = Παράθυρο
menu-minimize = Ελαχιστοποίηση
menu-zoom = Ζουμ
menu-bring-all-to-front = Μεταφορά όλων μπροστά

## Editing

action-edit = Επεξεργασία
title-edited = { $name } (επεξεργασμένο)
edit-trim = Περικοπή
edit-trim-apply = Περικοπή
edit-split = Διαχωρισμός κλιπ
edit-remove-clip = Αφαίρεση κλιπ
edit-done = Τέλος
edit-rotate-left = Περιστροφή αριστερά
edit-rotate-right = Περιστροφή δεξιά
edit-flip-horizontal = Οριζόντια αναστροφή
edit-flip-vertical = Κατακόρυφη αναστροφή
edit-undo = Αναίρεση
edit-redo = Επανάληψη
edit-revert = Επαναφορά στο πρωτότυπο
edit-export = Εξαγωγή…

## Exporting

export-title = Εξαγωγή
export-format = Μορφή
export-size = Μέγεθος
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Μόνο ήχος
export-size-original = { $lines }p, ως έχει
export-on-graphics-card = Κωδικοποίηση στην κάρτα γραφικών ({ $api })
export-on-processor = Κωδικοποίηση στον επεξεργαστή, που διαρκεί περισσότερο
export-no-encoder = Δεν είναι διαθέσιμος εδώ κωδικοποιητής για αυτή τη μορφή
export-checking = Αναζήτηση κωδικοποιητή…
export-sound-format = Μορφή ήχου
export-quality = Ποιότητα
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Με συμπίεση· αναπαράγεται σχεδόν παντού, ακόμη και στις συσκευές της Apple
export-mp3-note = Με συμπίεση· αναπαράγεται παντού
export-opus-note = Με συμπίεση· ο καλύτερος ήχος για το μέγεθός του, αν και δεν τον δέχονται όλα τα προγράμματα αναπαραγωγής
export-flac-note = Χωρίς απώλειες: ο ήχος ακριβώς όπως είναι, σε ανοιχτή μορφή
export-alac-note = Χωρίς απώλειες: ο ήχος ακριβώς όπως είναι, όπως τον αναπαράγουν οι συσκευές της Apple
export-wav-note = Χωρίς συμπίεση: ο ήχος ακριβώς όπως είναι, στα μεγαλύτερα αρχεία
export-kilobits = { $rate } kb/s
export-bits = { $bits } bit
export-rate-lower = Αποθήκευση στα { $rate } kHz, κάτω από τα { $file } kHz του αρχείου
export-choose = Εξαγωγή…
export-name = { $name } (εξαγωγή)
export-name-edited = { $name } (επεξεργασμένο)
export-progress = Εξαγωγή του «{ $name }»
export-stop = Διακοπή εξαγωγής
notice-exported = Ολοκληρώθηκε η εξαγωγή του «{ $name }»
notice-export-stopped = Η εξαγωγή διακόπηκε
notice-export-failed = Δεν ήταν δυνατή η εξαγωγή: { $error }
close-title = Κλείσιμο χωρίς εξαγωγή;
close-unexported = Οι αλλαγές στο «{ $name }» χάνονται αν δεν τις εξαγάγετε.
close-exporting = Η εξαγωγή του «{ $name }» διακόπτεται και ό,τι έχει γραφτεί μέχρι στιγμής χάνεται.
close-anyway = Κλείσιμο
close-quit-anyway = Τερματισμός

## Recording

record-title = Ηχογράφηση
record-file-name = Ηχογράφηση
record-start = Εγγραφή
record-stop = Διακοπή εγγραφής
record-no-input = Χωρίς μικρόφωνο
record-microphone = Μικρόφωνο
record-default-microphone = Η προεπιλογή του συστήματος
record-playing = Ό,τι αναπαράγει ο υπολογιστής
record-mono = μονοφωνικό
record-stereo = στερεοφωνικό
record-channels = { $count ->
    [one] { $count } κανάλι
   *[other] { $count } κανάλια
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, αποθήκευση στα { $saved } kHz
action-record-sound = Ηχογράφηση
open-record-sound = Νέα ηχογράφηση
start-record-sound = Ηχογράφηση
menu-record-sound = Νέα ηχογράφηση
record-screen-title = Εγγραφή οθόνης
record-screen-file-name = Εγγραφή οθόνης
record-screen-whole = Οθόνη
record-screen-number = Οθόνη { $number }
record-screen-main = Οθόνη { $number } (κύρια)
record-screens = Οθόνες
record-windows = Παράθυρα
record-screen-what = Τι θα καταγραφεί
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Κάθε καρέ
record-rate-tip = Καρέ ανά δευτερόλεπτο
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } χαμένο
   *[other] { $size } · { $count } χαμένα
}
record-screen-window = Παράθυρο
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = Η κοινή χρήση τερματίστηκε
record-screen-again = Νέα επιλογή
record-screen-failed = Δεν ήταν δυνατή η εγγραφή της οθόνης
record-sound = Ήχος
record-no-sound = Χωρίς ήχο
# How much louder the input is taken, for a quiet microphone.
record-gain = Ενίσχυση
record-gain-value = +{ $decibels } dB
record-level-automatically = Αυτόματη ρύθμιση στάθμης
record-hide-title = Το plaa κρύβεται κατά την εγγραφή
record-hide-tray = Για να σταματήσετε την εγγραφή, κάντε κλικ στην κόκκινη κουκκίδα που βάζει το plaa στην περιοχή ειδοποιήσεων ή ανοίξτε ξανά το plaa.
record-hide-reopen = Για να σταματήσετε την εγγραφή, ανοίξτε ξανά το plaa.
record-hide-start = Έναρξη εγγραφής
record-tray-title = Το plaa κάνει εγγραφή της οθόνης
record-tray-tip = Κάντε κλικ για διακοπή της εγγραφής
action-record-screen = Εγγραφή οθόνης
open-record-screen = Νέα εγγραφή οθόνης
start-record-screen = Εγγραφή οθόνης
menu-record-screen = Νέα εγγραφή οθόνης
record-camera-title = Εγγραφή κάμερας
record-camera-file-name = Εγγραφή κάμερας
record-camera = Κάμερα
record-camera-failed = Δεν ήταν δυνατό το άνοιγμα της κάμερας
record-name-title = Ονομάστε την εγγραφή
record-name-note = Αποθηκεύτηκε ως «{ $name }» στο { $folder }.
record-name-label = Όνομα
record-name-keep = Διατήρηση
record-name-rename = Μετονομασία
record-name-taken = Υπάρχει ήδη αρχείο με όνομα «{ $name }» εκεί.
record-name-invalid = Ένα όνομα δεν μπορεί να περιέχει κανέναν από αυτούς τους χαρακτήρες: / \ : * ? " < > |
record-name-failed = Δεν ήταν δυνατή η μετονομασία: { $error }
action-record-camera = Εγγραφή κάμερας
open-record-camera = Νέα εγγραφή κάμερας
start-record-camera = Εγγραφή κάμερας
menu-record-camera = Νέα εγγραφή κάμερας
notice-no-camera = Δεν υπάρχει κάμερα για εγγραφή
notice-recorded = Το «{ $name }» αποθηκεύτηκε στο { $folder }
notice-record-failed = Δεν ήταν δυνατή η εγγραφή: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Να επιτρέπεται ο εξωτερικός έλεγχος
settings-allow-outside-control-note = Πράκτορες AI όπως το Claude Code μπορούν να βλέπουν τι αναπαράγεται στο plaa και να το ελέγχουν, μέσω του plaa --mcp. Το plaa ρωτά πριν από κάθε νέο πράκτορα.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Επιτρέπονται: { $agents }
settings-forget-agents = Διαγραφή
settings-ask-before-note = Ερώτηση πριν από τις εξής ενέργειες ενός πράκτορα:
settings-ask-reading = Προβολή αυτού που αναπαράγεται ή της εικόνας
settings-ask-playing = Έλεγχος της αναπαραγωγής, του ήχου ή της πλήρους οθόνης
settings-ask-opening = Άνοιγμα αρχείου ή κλείσιμο παραθύρου
settings-ask-editing = Επεξεργασία ή εξαγωγή αρχείου
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Να επιτραπεί στο { $agent } να ελέγχει το plaa;
agent-prompt-body = Το { $agent } ζητά να χρησιμοποιήσει τον εξωτερικό έλεγχο του plaa, για να βλέπει τι αναπαράγεται και να το ελέγχει. Μπορείτε να απενεργοποιήσετε τον εξωτερικό έλεγχο στις Ρυθμίσεις.
agent-prompt-allow = Να επιτραπεί
agent-prompt-deny = Να μην επιτραπεί
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Να επιτραπεί στο { $agent } να βλέπει τι αναπαράγεται εδώ;
agent-ask-play = Να επιτραπεί στο { $agent } να ελέγχει την αναπαραγωγή εδώ;
agent-ask-open = Να επιτραπεί στο { $agent } να ανοίξει ή να κλείσει ένα παράθυρο;
agent-ask-edit = Να επιτραπεί στο { $agent } να κάνει επεξεργασία ή εξαγωγή εδώ;
agent-working = Το { $agent } εργάζεται…
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = Το { $agent } ζητά να χρησιμοποιήσει το «{ $tool }». Στις Ρυθμίσεις επιλέγετε για τι ρωτά το plaa.

## Casting to a TV or speaker on the network

cast-looking = Αναζήτηση τηλεοράσεων και ηχείων…
cast-none = Δεν βρέθηκε τηλεόραση, ηχείο ή Chromecast.
# $renderer is the TV's or speaker's name.
cast-stop = Διακοπή αναπαραγωγής στο { $renderer }
notice-casting = Αναπαραγωγή στο { $renderer }.
notice-cast-failed = Δεν ήταν δυνατή η αναπαραγωγή στο { $renderer }: { $reason }
cast-web-page = Δεν είναι ακόμη δυνατή η μετάδοση ιστοσελίδων: ανοίξτε καλύτερα τη διεύθυνση του ίδιου του βίντεο.

## The assistant panel, and its models in Settings

assistant-add-another = Προσθήκη μοντέλου…
assistant-add-model = Ο βοηθός χρειάζεται ένα μοντέλο: ένα μοντέλο cloud με το κλειδί API του ή ένα τοπικό μοντέλο.
assistant-ask = Ρωτήστε για αυτό που αναπαράγεται
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = Η συζήτηση είχε γίνει πολύ μεγάλη για να τη διαβάσει το { $model } μονομιάς, γι’ αυτό το προηγούμενο μέρος της συνοψίστηκε. Ο βοηθός μπορεί να ξανακοιτάξει όσα είδε.
assistant-compacting = Γίνεται σύνοψη της προηγούμενης συζήτησης
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } σε αυτόν τον υπολογιστή
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } στη διεύθυνση { $host }
assistant-new-chat = Νέα συνομιλία
assistant-no-model = Προσθέστε πρώτα ένα μοντέλο στις Ρυθμίσεις.
assistant-open-settings = Προσθήκη μοντέλου
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Η συνομιλία δεν χωρά πλέον σε όσα μπορεί να διαβάσει μονομιάς το μοντέλο { $model }. Ξεκινήστε νέα συνομιλία ή επιλέξτε ένα μοντέλο που διαβάζει περισσότερα.
assistant-problem-credit = Ο πάροχος { $provider } αναφέρει ότι ο λογαριασμός δεν έχει υπόλοιπο πίστωσης. Σε νέο λογαριασμό πρέπει πρώτα να αγοράσετε πίστωση στον ιστότοπο του παρόχου { $provider } για να λειτουργήσει το κλειδί· έπειτα δοκιμάστε ξανά.
assistant-problem-key = Ο πάροχος { $provider } απέρριψε το κλειδί API. Ελέγξτε το στις Ρυθμίσεις.
assistant-problem-model = Ο πάροχος { $provider } δεν έχει μοντέλο με το όνομα { $model }. Ελέγξτε το όνομά του στις Ρυθμίσεις.
# $message is what the provider said, untranslated.
assistant-problem-other = Το μοντέλο { $model } δεν απάντησε: { $message }
assistant-problem-rate = Ο πάροχος { $provider } ζητά να μειωθεί ο ρυθμός των αιτημάτων. Δοκιμάστε ξανά σε λίγο.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = Ο πάροχος { $provider } ζητά να μειωθεί ο ρυθμός των αιτημάτων: { $message }
assistant-problem-refused = Το μοντέλο { $model } αρνήθηκε να απαντήσει.
assistant-problem-unavailable = Ο πάροχος { $provider } είναι απασχολημένος ή αντιμετωπίζει προβλήματα. Δοκιμάστε ξανά σε λίγο.
assistant-problem-unreachable = Το plaa δεν μπόρεσε να συνδεθεί με τον πάροχο { $provider }. Ελέγξτε τη σύνδεσή σας ή ότι ο διακομιστής λειτουργεί.
assistant-running = Εκτελείται…
assistant-send = Αποστολή
assistant-shortened = Η συζήτηση ήταν πολύ μεγάλη για να τη διαβάσει το { $model } μονομιάς, γι’ αυτό παραλείφθηκαν προηγούμενα αποτελέσματα εργαλείων. Ο βοηθός μπορεί να ξανακοιτάξει όσα είδε.
assistant-stop = Διακοπή
assistant-stopped = Διακόπηκε.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Τώρα μιλάτε με το μοντέλο { $model }.
assistant-thinking = Σκέφτεται…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Σκέψεις
assistant-title = Βοηθός
settings-assistant-add = Προσθήκη μοντέλου
settings-assistant-add-button = Προσθήκη
# $model is the model's name.
settings-assistant-added = Το μοντέλο { $model } απάντησε και προστέθηκε.
settings-assistant-added-tag = Προστέθηκε
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Διεύθυνση, π.χ. { $example }
settings-assistant-change-key = Αλλαγή κλειδιού
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Διακομιστής συμβατός με OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Παράθυρο περιβάλλοντος
settings-assistant-context-note = Μεγαλύτερο παράθυρο επιτρέπει στον βοηθό να έχει υπόψη μεγαλύτερο μέρος μιας συνομιλίας, αλλά το μοντέλο καταλαμβάνει περισσότερη μνήμη και μπορεί να απαντά πιο αργά.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } χιλ. tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Άνοιγμα σελίδας
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = Ο διακομιστής { $provider } εκτελείται στη διεύθυνση { $address }.
settings-assistant-found-none = Ο διακομιστής { $provider } δεν έχει ακόμη μοντέλα. Κατεβάστε ένα μέσω αυτού και αναζητήστε ξανά.
settings-assistant-get-key = Λήψη κλειδιού API
settings-assistant-get-server = Λήψη { $provider }
settings-assistant-in-use = Σε χρήση
settings-assistant-key = Κλειδί API
# $error is what the keychain said.
settings-assistant-key-failed = Δεν ήταν δυνατή η φύλαξη του κλειδιού στην κλειδοθήκη: { $error }
settings-assistant-key-kept = Το κλειδί σας για τον πάροχο { $provider } φυλάσσεται στην κλειδοθήκη του συστήματος.
settings-assistant-key-needed = Αυτό το μοντέλο χρειάζεται κλειδί API.
settings-assistant-key-refused = Ο πάροχος { $provider } απέρριψε αυτό το κλειδί. Ελέγξτε ότι αντιγράφηκε ολόκληρο, από τον σωστό λογαριασμό.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Δημιουργήστε ένα κλειδί στον ιστότοπο του παρόχου { $provider } και επικολλήστε το εδώ.
settings-assistant-look-again = Νέα αναζήτηση
settings-assistant-looking = Αναζήτηση μοντέλων…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Μοντέλο, π.χ. { $example }
settings-assistant-no-server = Το plaa δεν βρήκε διακομιστή { $provider } σε λειτουργία σε αυτόν τον υπολογιστή. Ξεκινήστε τον ή δώστε τη διεύθυνσή του παρακάτω.
settings-assistant-no-tools = Δεν μπορεί να χρησιμοποιήσει εργαλεία, που τα χρειάζεται ο βοηθός
settings-assistant-none = Δεν υπάρχουν ακόμη μοντέλα. Προσθέστε ένα παρακάτω: ένα τοπικό μοντέλο, όπως ένα του Ollama, μένει σε αυτόν τον υπολογιστή· ένα μοντέλο cloud χρειάζεται κλειδί API από τον πάροχό του.
settings-assistant-note = Τα μοντέλα με τα οποία μπορεί να μιλά ο πίνακας του βοηθού. Τα κλειδιά φυλάσσονται στην κλειδοθήκη του συστήματος.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Χρήση άλλης διεύθυνσης
settings-assistant-recommended = Προτείνεται
settings-assistant-remove = Αφαίρεση
settings-assistant-sees = Βλέπει εικόνες
settings-assistant-trying = Δοκιμή…
settings-assistant-use = Χρήση
settings-assistant-use-key = Συνέχεια
settings-assistant-uses-tools = Χρησιμοποιεί εργαλεία
