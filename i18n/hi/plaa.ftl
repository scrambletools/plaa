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
language-name = हिन्दी

## Common

app-name = plaa
common-cancel = रद्द करें
common-close = बंद करें
common-more = अधिक

## Command line

usage-help =
    उपयोग: plaa [FILE OR ADDRESS]...

    वीडियो और ऑडियो फ़ाइलें, और https://example.com/live.m3u8 जैसे पतों पर
    मौजूद स्ट्रीम चलाएँ। हर एक चल रहे plaa की किसी विंडो में खुलती है, जो
    ज़रूरत पड़ने पर शुरू हो जाता है।

    विकल्प:
      -h, --help        यह सहायता दिखाएँ
      -V, --version     संस्करण दिखाएँ
          --interfaces  वायर्ड interfaces दिखाएँ, और यह कि plaa उन पर AVB
                        streams भेज सकता है या नहीं
          --mcp         stdin और stdout पर MCP चलाएँ, ताकि AI एजेंट देख सकें
                        कि चल रहा plaa क्या चला रहा है, और उसे नियंत्रित कर सकें

## Start window

start-hint = कोई वीडियो या ऑडियो फ़ाइल, स्ट्रीम या वेब वीडियो खोलें।
start-open = खोलें
start-open-address = पता खोलें
# Opens a list of the talkers on the AVB network to listen to.
start-connect = कनेक्ट करें

## Actions, as buttons, tooltips and menus name them

action-open = खोलें
action-open-address = पता खोलें
open-file = फ़ाइल खोलें…
open-address = पता खोलें…
action-settings = सेटिंग्स
action-info = इंस्पेक्टर
action-loop = लूप
action-screenshot = फ़्रेम को छवि के रूप में सहेजें
action-play = चलाएँ
action-pause = रोकें
# $seconds is how far the button moves, such as 10.
action-back = { $seconds } सेकंड पीछे
action-forward = { $seconds } सेकंड आगे
action-mute = म्यूट करें
action-unmute = अनम्यूट करें
action-speed = गति
action-tracks = ऑडियो और सबटाइटल
action-full-screen = पूर्ण स्क्रीन
action-playlist = प्लेलिस्ट
action-chapters = अध्याय
action-picture-in-picture = पिक्चर इन पिक्चर
action-previous = पिछला
action-next = अगला
action-exit-full-screen = पूर्ण स्क्रीन से बाहर निकलें
action-quality = गुणवत्ता
action-visualizer = विज़ुअलाइज़र
action-network = नेटवर्क ऑडियो
action-cast = कास्ट करें
action-connect = कनेक्ट करें
action-assistant = असिस्टेंट

## What a sound file shows under its cover

visualizer-spectrum = स्पेक्ट्रम
visualizer-waveform = वेवफ़ॉर्म
visualizer-prism = प्रिज़्म
visualizer-cover = केवल कवर

## The floating controls

overlay-keep-shown = नियंत्रण डॉक करके रखें
overlay-auto-hide = नियंत्रण चित्र के ऊपर तैरते रखें

## Audio and subtitle tracks

tracks-audio = ऑडियो
tracks-subtitles = सबटाइटल
tracks-off = बंद
# A track the file gives no name or language, by its number.
tracks-numbered = ट्रैक { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = केवल ऑडियो

## Inspector

info-title = इंस्पेक्टर
playlist-title = प्लेलिस्ट
chapters-numbered = अध्याय { $number }
info-file = फ़ाइल
info-name = नाम
info-address = पता
info-folder = फ़ोल्डर
info-size = आकार
info-format = फ़ॉर्मेट
info-duration = अवधि
info-bit-rate = बिट रेट
info-video = वीडियो
info-audio = ऑडियो
info-codec = कोडेक
info-dimensions = आयाम
info-frame-rate = फ़्रेम रेट
info-frames-per-second = { $rate } फ़्रेम प्रति सेकंड
info-pixel-format = पिक्सेल फ़ॉर्मेट
info-decoding = डिकोडिंग
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = ग्राफ़िक्स कार्ड ({ $api })
info-decoding-software = प्रोसेसर
info-sample-rate = सैंपल रेट
info-hertz = { $rate } Hz
info-channels = चैनल
info-language = भाषा
info-no-sound = कोई ध्वनि नहीं
info-tags = टैग
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Stream
# The talker's listeners other than plaa.
info-other-listeners = दूसरे Listeners
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR class
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = नए फ़ॉर्मेट से { $count ->
        [one] Talker का दूसरा Listener डिस्कनेक्ट हो जाएगा।
       *[other] Talker के { $count } दूसरे Listeners डिस्कनेक्ट हो जाएँगे।
    }

## Dialogs

dialog-open = खोलें
dialog-filter-media = वीडियो और ऑडियो
dialog-filter-all = सभी फ़ाइलें

address-title = पता खोलें
address-note = वीडियो वाला कोई वेब पेज, जैसे किसी YouTube वीडियो का पेज, या नेटवर्क पर कोई स्ट्रीम या फ़ाइल, जैसे https://example.com/live.m3u8।
address-label = पता
address-open = खोलें
address-recent = हाल के पते
address-forget = हाल के पतों से हटाएँ

## Notices

notice-dialog-failed = फ़ाइल डायलॉग नहीं खुल सका: { $error }
notice-screenshot-saved = “{ $name }” सहेजा गया।
notice-screenshot-failed = छवि सहेजी नहीं जा सकी: { $error }
notice-no-sound = बिना ध्वनि के चल रहा है: { $reason }
notice-loop-from = { $time } से लूप। जहाँ इसे खत्म होना है, वहाँ फिर से दबाएँ।
notice-loop-between = { $from } से { $to } तक लूप हो रहा है।
notice-loop-off = लूप बंद।

## Errors, shown in place of what could not play

error-needs-yt-dlp = यह एक वेब पेज लगता है। plaa वेब पेजों के वीडियो yt-dlp से चलाता है, जो इंस्टॉल नहीं है।
error-empty-list = इस सूची में चलाने के लिए कुछ नहीं है।

## Settings

settings-title = सेटिंग्स
settings-tab-appearance = दिखावट
settings-tab-playback = प्लेबैक
settings-tab-storage = स्टोरेज
settings-tab-network = नेटवर्क
settings-tab-language = भाषा
settings-tab-agents = एजेंट
settings-tab-assistant = असिस्टेंट
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (डेवलपमेंट बिल्ड, { $build })
settings-choose = चुनें…

settings-appearance-system = सिस्टम का पालन करें
settings-appearance-light = लाइट
settings-appearance-dark = डार्क
settings-colors = रंग
settings-system-accent = सिस्टम के एक्सेंट रंग का उपयोग करें
settings-omarchy-note = रंग “{ $theme }” के एक्सेंट से बनाए जाते हैं।
settings-system-accent-note = रंग सिस्टम के एक्सेंट रंग से बनाए जाते हैं।
settings-system-accent-none = सिस्टम में कोई एक्सेंट रंग नहीं है, इसलिए plaa नीचे चुने गए रंग का उपयोग करता है।
settings-accent-chosen-note = रंग नीचे चुने गए रंग से बनाए जाते हैं।
settings-controls = नियंत्रण
settings-overlay = तैरते नियंत्रण
settings-overlay-note = टूलबार और प्लेबैक नियंत्रण चित्र या कवर के ऊपर तैरते हैं, और पॉइंटर के रुके रहने या विंडो से बाहर रहने पर खिसककर हट जाते हैं।
settings-overlay-delay = छिपाने में देरी
settings-overlay-delay-note = जो चल रहा है उस पर पॉइंटर कितनी देर रुका रहे, उसके बाद नियंत्रण खिसककर हट जाते हैं।
settings-seconds = { $seconds } सेकंड
settings-transparency = पारदर्शिता
settings-transparency-note = तैरते नियंत्रणों के आर-पार चित्र कितना दिखाई देता है।
settings-percent = { $percent }%
settings-corner-radius = कोने की त्रिज्या
settings-corner-radius-note = डायलॉग और तैरते नियंत्रणों के लिए।
settings-pixels = { $pixels } px
settings-animations = एनिमेशन
settings-animations-note = खिसकते बार और पैनल, बढ़ते डायलॉग और उछलते बटन।
settings-animations-reduced = जब सिस्टम कम गति माँगता है, तब बंद रहता है।

settings-hardware = ग्राफ़िक्स कार्ड का उपयोग करें
settings-hardware-note = वीडियो उस पर डिकोड होता है और एक्सपोर्ट उस पर एन्कोड होते हैं: बिजली कम लगती है, बड़े वीडियो सुचारु रूप से चलते हैं और एक्सपोर्ट तेज़ी से होता है। इसके बाद खोली गई फ़ाइलों पर लागू होता है।
settings-audio-device = ध्वनि आउटपुट
settings-audio-device-system = सिस्टम डिफ़ॉल्ट
settings-resume = जहाँ छोड़ा था, वहीं से जारी रखें
settings-resume-note = फ़ाइलें वहीं से शुरू होती हैं, जहाँ वे पिछली बार बंद करते समय थीं।
settings-folder-playlists = फ़ोल्डर की अन्य फ़ाइलें चलाएँ
settings-folder-playlists-note = एक फ़ाइल खोलने पर उसके फ़ोल्डर की अन्य फ़ाइलें सूची में आ जाती हैं, जो उसके बाद चलती हैं।
settings-streams = स्ट्रीम और वेब वीडियो
settings-stream-quality = गुणवत्ता
settings-stream-quality-note = उन स्ट्रीम और वेब वीडियो के लिए, जो कई गुणवत्ताओं में आते हैं। इसके बाद खोली गई चीज़ों पर लागू होता है।
settings-quality-auto = स्वचालित, स्क्रीन जितनी ऊँचाई
settings-quality-best = सर्वश्रेष्ठ
settings-web-pages = वेब वीडियो yt-dlp से चलाएँ
settings-web-pages-found = yt-dlp, YouTube जैसे पेजों पर वीडियो ढूँढ लेता है।
settings-web-pages-missing = yt-dlp इंस्टॉल नहीं है। YouTube जैसे पेजों के वीडियो चलाने के लिए इसे इंस्टॉल करें।
settings-default-app = डिफ़ॉल्ट ऐप
settings-default-app-label = फ़ाइलें plaa से खोलें
settings-default-app-note = plaa को वह ऐप बनाएँ जो वीडियो और ध्वनि फ़ाइलें खोलता है।
settings-default-app-note-windows = Windows डिफ़ॉल्ट ऐप केवल अपनी सेटिंग्स में चुनने देता है। यह वहाँ plaa का पृष्ठ खोलता है।
settings-default-app-note-macos = macOS हर प्रकार की पुष्टि माँगता है: MP4, MOV, MKV, WebM, MP3, M4A, FLAC और WAV।
settings-default-app-status = { $total } में से { $set } फ़ाइल प्रकार plaa से खुलते हैं।
settings-default-app-button = डिफ़ॉल्ट बनाएँ
settings-default-app-button-windows = सेटिंग्स खोलें
settings-default-app-no-entry = plaa की डेस्कटॉप एंट्री इंस्टॉल नहीं है, इसलिए सिस्टम इससे फ़ाइलें नहीं खोल सकता। plaa को किसी पैकेज से इंस्टॉल करें।
settings-default-app-no-bundle = plaa को डिफ़ॉल्ट बनाने के लिए इसे plaa.app से खोलें।
settings-default-app-failed = plaa को डिफ़ॉल्ट नहीं बनाया जा सका: { $error }
settings-folders = फ़ोल्डर
settings-sound-recordings = ध्वनि रिकॉर्डिंग
settings-video-recordings = वीडियो रिकॉर्डिंग
settings-screenshots = सहेजे गए फ़्रेम

settings-interface-language = इंटरफ़ेस की भाषा
# The first entry of the language list.
settings-language-system = सिस्टम का पालन करें
settings-track-languages = ट्रैक की भाषाएँ
settings-track-languages-note = भाषा कोड, जैसे en या fra, सबसे पसंदीदा पहले, अल्पविराम से अलग करके। जब किसी फ़ाइल में इन भाषाओं के ऑडियो और सबटाइटल होते हैं, तो plaa उन्हें चुनता है।
settings-audio-languages = ऑडियो
settings-subtitle-languages = सबटाइटल

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB interface
settings-avb-interface-note = वह वायर्ड पोर्ट, जिस पर plaa AVB और Milan Talkers और Listeners ढूँढता है। भेजने के लिए PTP hardware clock वाला पोर्ट चाहिए, जैसे Intel I210 का।
settings-avb-off = बंद
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (केवल सूची: PTP clock नहीं है)
settings-avb-off-mac = { $interface } (केवल सूची: AVB बंद है)
settings-avb-interface-note-mac = वह वायर्ड पोर्ट, जिस पर plaa AVB और Milan Talkers और Listeners ढूँढता है। भेजना इस Mac की अपनी AVB entity से होता है, जिसके लिए Audio MIDI Setup के Network Device Browser में इस पोर्ट के लिए AVB चालू होना चाहिए।
settings-avb-talker-note-mac = plaa आखिरी बार चलाई गई विंडो की ध्वनि, इस Mac की अपनी AVB entity के ज़रिए, उससे कनेक्ट Listeners को भेजता है।
settings-avb-talker = जो चल रहा है, उसे भेजें
settings-avb-talker-note = plaa एक Milan Talker बन जाता है, और आखिरी बार चलाई गई विंडो की ध्वनि उससे कनेक्ट Listeners को भेजता है।
settings-avb-name = नेटवर्क पर नाम
settings-avb-name-note = Controller plaa को “{ $name }” के रूप में दिखाते हैं। नाम बदलने के लिए Enter दबाएँ।
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer } पर plaa
avb-could-not-start = { $interface } इस्तेमाल नहीं किया जा सका: { $error }
network-title = नेटवर्क ऑडियो
network-off = नेटवर्क पर चलाने के लिए सेटिंग्स में “जो चल रहा है, उसे भेजें” चालू करें।
network-blocked = नहीं भेजा जा रहा: { $reason }
network-waiting = भेजने के लिए तैयार, किसी Listener के कनेक्ट होने की प्रतीक्षा है।
network-sending = कनेक्ट हुए Listeners को भेजा जा रहा है।
network-looking = Listeners ढूँढे जा रहे हैं…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l शुरू हो रहा है, जो वह नेटवर्क समय रखता है जिसके अनुसार plaa की stream भेजी जाती है…
network-ptp-elsewhere = ptp4l किसी दूसरे interface पर चल रहा है, इसलिए यहाँ plaa की stream को नेटवर्क समय नहीं मिलता और हो सकता है कि Listeners उसे न चला पाएँ। सेटिंग्स में वह interface चुनें, या इस interface पर ptp4l चलाएँ।
network-ptp-missing = ptp4l नहीं चल रहा, इसलिए plaa की stream को नेटवर्क समय नहीं मिलता और हो सकता है कि Listeners उसे न चला पाएँ। plaa के पैकेज के साथ linuxptp इंस्टॉल करें, या triib, जो इसे चलाता है।
network-ptp-failed = ptp4l नहीं चल रहा, इसलिए plaa की stream को नेटवर्क समय नहीं मिलता और हो सकता है कि Listeners उसे न चला पाएँ। plaa इसे शुरू नहीं कर सका: { $reason }
network-no-listeners = नेटवर्क पर कोई भी Listener plaa की ध्वनि नहीं चला सकता।
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener } पर चलाएँ
network-stop-on = { $listener } पर चलाना बंद करें
network-choose-format = { $listener } पर चलाने के लिए फ़ॉर्मेट चुनें
network-failed = कनेक्ट नहीं किया जा सका: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = यहाँ चलाने के लिए triib अपडेट करें।
network-choose = कोई Listener चुनें, यह देखने के लिए कि वह अपनी stream कैसे लेता है।
network-no-sources = नेटवर्क पर कोई भी Talker ऐसे फ़ॉर्मेट में नहीं भेज रहा जिसे plaa चलाता है, और न ही ऐसे फ़ॉर्मेट में बदल सकता है।
# $talker is a talker's name, or its name and its output's.
network-listen-stop = { $talker } को सुनना बंद करें
network-listen-update-triib = यहाँ सुनने के लिए triib अपडेट करें: plaa केवल वही चलाता है जो वैसे भी इस कंप्यूटर तक पहुँचता है।
network-listen-lite = पोर्ट AVB Lite पर चल रहा है, जो कोई stream reserve नहीं करता: plaa केवल वही चलाता है जो वैसे भी इस कंप्यूटर तक पहुँचता है।

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Talkers ढूँढे जा रहे हैं…
# plaa listens through its entity, which exists only while it sends.
connect-off = यहाँ Talkers को सुनने के लिए सेटिंग्स में “जो चल रहा है, उसे भेजें” चालू करें।
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa यहाँ नहीं सुन सकता: { $reason }
connect-sending = ऐसे फ़ॉर्मेट में भेज रहे हैं जिसे plaa चलाता है
connect-changing = ऐसे फ़ॉर्मेट में बदलेंगे जिसे plaa चलाता है
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, { $format } में
connect-change-cuts-off = { $talker }, { $format } में, जिससे { $count ->
        [one] उसका दूसरा Listener डिस्कनेक्ट हो जाएगा
       *[other] उसके { $count } दूसरे Listeners डिस्कनेक्ट हो जाएँगे
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = स्थिति
network-not-connected = कनेक्ट नहीं है
network-from = { $stream } से
network-from-receiving = { $stream } से, प्राप्त कर रहा है
network-from-waiting = { $stream } से, Talker की प्रतीक्षा में
network-from-failed = { $stream } से, Talker का reservation विफल: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Media clock
network-locked = Stream से Lock है
network-not-locked = Lock नहीं है
network-format = फ़ॉर्मेट
# The formats both the listener and plaa's stream can be in.
network-formats = plaa से लिए जा सकने वाले फ़ॉर्मेट
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream ID
# The MAC address the stream is sent to.
network-destination = गंतव्य
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latency
network-microseconds = { $time } µs
network-frames = प्राप्त frames
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = क्रम से बाहर frames
# How often the stream stopped coming while it was taken.
network-interrupted = रुकावटें
# Frames that came after, or long before, their time to be played.
network-late = देर से आए frames
network-early = समय से पहले आए frames
network-entity-id = Entity ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = पर्याप्त bandwidth नहीं
msrp-failure-2 = bridge के संसाधन पर्याप्त नहीं
msrp-failure-3 = traffic class के लिए पर्याप्त bandwidth नहीं
msrp-failure-4 = stream ID किसी दूसरे Talker के इस्तेमाल में है
msrp-failure-5 = destination address पहले से इस्तेमाल में है
msrp-failure-6 = ऊँची rank वाली stream ने इसे हटा दिया
msrp-failure-7 = रिपोर्ट की गई latency बदल गई है
msrp-failure-8 = egress port AVB सक्षम नहीं है
msrp-failure-9 = कोई दूसरा destination address इस्तेमाल करें
msrp-failure-10 = MSRP संसाधन खत्म हो गए
msrp-failure-11 = MMRP संसाधन खत्म हो गए
msrp-failure-12 = destination address सहेजा नहीं जा सकता
msrp-failure-13 = प्राथमिकता SR class की प्राथमिकता नहीं है
msrp-failure-14 = frames माध्यम के लिए बहुत बड़े हैं
msrp-failure-15 = fan-in port की सीमा पूरी हो गई
msrp-failure-16 = पंजीकृत stream का पहला मान बदल गया
msrp-failure-17 = egress port पर VLAN ब्लॉक है
msrp-failure-18 = egress port पर VLAN tagging बंद है
msrp-failure-19 = SR class की प्राथमिकता मेल नहीं खाती
msrp-failure-unknown = अज्ञात कारण
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, bridge { $bridge } पर

## The macOS menu bar

menu-about = plaa के बारे में
menu-settings = सेटिंग्स…
menu-services = सेवाएँ
menu-hide = plaa छिपाएँ
menu-hide-others = अन्य को छिपाएँ
menu-show-all = सभी दिखाएँ
menu-quit = plaa बंद करें
menu-file = फ़ाइल
menu-open = खोलें…
menu-open-address = पता खोलें…
menu-close = विंडो बंद करें
menu-edit = संपादित करें
menu-cut = कट करें
menu-copy = कॉपी करें
menu-paste = पेस्ट करें
menu-select-all = सभी चुनें
menu-undo = पूर्ववत करें
menu-redo = फिर से करें
menu-trim = ट्रिम करें…
menu-split = क्लिप विभाजित करें
menu-rotate-left = बाएँ घुमाएँ
menu-rotate-right = दाएँ घुमाएँ
menu-flip-horizontal = क्षैतिज रूप से पलटें
menu-flip-vertical = लंबवत रूप से पलटें
menu-revert = मूल पर लौटें
menu-export = एक्सपोर्ट करें…
menu-view = देखें
menu-playback = प्लेबैक
menu-play-pause = चलाएँ या रोकें
menu-to-start = शुरुआत पर जाएँ
menu-previous-chapter = पिछला अध्याय
menu-next-chapter = अगला अध्याय
menu-faster = तेज़
menu-slower = धीमा
menu-normal-speed = सामान्य गति
menu-ab-loop = दो बिंदुओं के बीच लूप करें
menu-volume-up = आवाज़ बढ़ाएँ
menu-volume-down = आवाज़ घटाएँ
menu-mute = म्यूट या अनम्यूट करें
menu-next-subtitles = अगले सबटाइटल
menu-next-audio = अगला ऑडियो ट्रैक
menu-window = विंडो
menu-minimize = छोटा करें
menu-zoom = ज़ूम करें
menu-bring-all-to-front = सभी को सामने लाएँ

## Editing

action-edit = संपादित करें
title-edited = { $name } (संपादित)
edit-trim = ट्रिम करें
edit-trim-apply = ट्रिम करें
edit-split = क्लिप विभाजित करें
edit-remove-clip = क्लिप हटाएँ
edit-done = हो गया
edit-rotate-left = बाएँ घुमाएँ
edit-rotate-right = दाएँ घुमाएँ
edit-flip-horizontal = क्षैतिज रूप से पलटें
edit-flip-vertical = लंबवत रूप से पलटें
edit-undo = पूर्ववत करें
edit-redo = फिर से करें
edit-revert = मूल पर लौटें
edit-export = एक्सपोर्ट करें…

## Exporting

export-title = एक्सपोर्ट करें
export-format = फ़ॉर्मेट
export-size = आकार
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = केवल ध्वनि
export-size-original = { $lines }p, जैसा है वैसा
export-on-graphics-card = ग्राफ़िक्स कार्ड पर एन्कोड होता है ({ $api })
export-on-processor = प्रोसेसर पर एन्कोड होता है, जिसमें ज़्यादा समय लगता है
export-no-encoder = इस फ़ॉर्मेट के लिए यहाँ कोई एन्कोडर नहीं खुलता
export-checking = एन्कोडर ढूँढा जा रहा है…
export-sound-format = ध्वनि फ़ॉर्मेट
export-quality = गुणवत्ता
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = कंप्रेस्ड; लगभग हर जगह चलता है, Apple के डिवाइस पर भी
export-mp3-note = कंप्रेस्ड; हर जगह चलता है
export-opus-note = कंप्रेस्ड; अपने आकार के हिसाब से सबसे अच्छी ध्वनि, हालाँकि हर प्लेयर इसे नहीं चलाता
export-flac-note = लॉसलेस: ध्वनि ठीक वैसी ही, जैसी वह है, एक खुले फ़ॉर्मेट में
export-alac-note = लॉसलेस: ध्वनि ठीक वैसी ही, जैसी वह है, उस रूप में जिसे Apple के डिवाइस चलाते हैं
export-wav-note = अनकंप्रेस्ड: ध्वनि ठीक वैसी ही, जैसी वह है, सबसे बड़ी फ़ाइलों में
export-kilobits = { $rate } kb/s
export-bits = { $bits }-बिट
export-rate-lower = { $rate } kHz पर सहेजा जाता है, जो फ़ाइल के { $file } kHz से कम है
export-choose = एक्सपोर्ट करें…
export-name = { $name } एक्सपोर्ट
export-name-edited = { $name } संपादित
export-progress = “{ $name }” एक्सपोर्ट किया जा रहा है
export-stop = एक्सपोर्ट करना रोकें
notice-exported = “{ $name }” एक्सपोर्ट किया गया
notice-export-stopped = एक्सपोर्ट रुक गया
notice-export-failed = एक्सपोर्ट नहीं किया जा सका: { $error }
close-title = एक्सपोर्ट किए बिना बंद करें?
close-unexported = “{ $name }” में किए गए बदलाव एक्सपोर्ट न करने पर खो जाएँगे।
close-exporting = “{ $name }” का एक्सपोर्ट रुक जाएगा, और अब तक लिखा गया हिस्सा खो जाएगा।
close-anyway = बंद करें
close-quit-anyway = बंद करें

## Recording

record-title = ध्वनि रिकॉर्डिंग
record-file-name = ध्वनि रिकॉर्डिंग
record-start = रिकॉर्ड करें
record-stop = रिकॉर्डिंग रोकें
record-no-input = कोई माइक्रोफ़ोन नहीं
record-microphone = माइक्रोफ़ोन
record-default-microphone = सिस्टम का डिफ़ॉल्ट
record-playing = कंप्यूटर में जो चल रहा है
record-mono = मोनो
record-stereo = स्टीरियो
record-channels = { $count } चैनल
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, { $saved } kHz पर सहेजा जाता है
action-record-sound = ध्वनि रिकॉर्ड करें
open-record-sound = नई ध्वनि रिकॉर्डिंग
start-record-sound = ध्वनि रिकॉर्ड करें
menu-record-sound = नई ध्वनि रिकॉर्डिंग
record-screen-title = स्क्रीन रिकॉर्डिंग
record-screen-file-name = स्क्रीन रिकॉर्डिंग
record-screen-whole = स्क्रीन
record-screen-number = स्क्रीन { $number }
record-screen-main = स्क्रीन { $number } (मुख्य)
record-screens = स्क्रीन
record-windows = विंडो
record-screen-what = क्या रिकॉर्ड करें
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = हर फ़्रेम
record-rate-tip = प्रति सेकंड फ़्रेम
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $count ->
    [one] { $size } · { $count } छूटा
   *[other] { $size } · { $count } छूटे
}
record-screen-window = विंडो
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = अब साझा नहीं किया जा रहा
record-screen-again = फिर से चुनें
record-screen-failed = स्क्रीन रिकॉर्ड नहीं की जा सकी
record-sound = ध्वनि
record-no-sound = कोई ध्वनि नहीं
# How much louder the input is taken, for a quiet microphone.
record-gain = आवाज़ बढ़ाएँ
record-gain-value = +{ $decibels } dB
record-level-automatically = स्तर अपने आप समायोजित करें
record-hide-title = रिकॉर्ड करते समय plaa छिप जाता है
record-hide-tray = रिकॉर्डिंग रोकने के लिए, ट्रे में plaa के लाल बिंदु पर क्लिक करें, या plaa फिर से खोलें।
record-hide-reopen = रिकॉर्डिंग रोकने के लिए plaa फिर से खोलें।
record-hide-start = रिकॉर्डिंग शुरू करें
record-tray-title = plaa स्क्रीन रिकॉर्ड कर रहा है
record-tray-tip = रिकॉर्डिंग रोकने के लिए क्लिक करें
action-record-screen = स्क्रीन रिकॉर्ड करें
open-record-screen = नई स्क्रीन रिकॉर्डिंग
start-record-screen = स्क्रीन रिकॉर्ड करें
menu-record-screen = नई स्क्रीन रिकॉर्डिंग
record-camera-title = कैमरा रिकॉर्डिंग
record-camera-file-name = कैमरा रिकॉर्डिंग
record-camera = कैमरा
record-camera-failed = कैमरा नहीं खोला जा सका
record-name-title = रिकॉर्डिंग को नाम दें
record-name-note = यह { $folder } में “{ $name }” नाम से सहेजी गई है।
record-name-label = नाम
record-name-keep = रखें
record-name-rename = नाम बदलें
record-name-taken = वहाँ “{ $name }” नाम की फ़ाइल पहले से है।
record-name-invalid = नाम में इनमें से कोई भी नहीं हो सकता: / \ : * ? " < > |
record-name-failed = नाम नहीं बदला जा सका: { $error }
action-record-camera = कैमरा रिकॉर्ड करें
open-record-camera = नई कैमरा रिकॉर्डिंग
start-record-camera = कैमरा रिकॉर्ड करें
menu-record-camera = नई कैमरा रिकॉर्डिंग
notice-no-camera = रिकॉर्ड करने के लिए कोई कैमरा नहीं है
notice-recorded = “{ $name }” { $folder } में सहेजा गया
notice-record-failed = रिकॉर्ड नहीं किया जा सका: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = बाहरी नियंत्रण की अनुमति दें
settings-allow-outside-control-note = Claude Code जैसे AI एजेंट plaa --mcp के ज़रिए देख सकते हैं कि plaa में क्या चल रहा है, और उसे नियंत्रित कर सकते हैं। हर नए एजेंट से पहले plaa पूछता है।
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = अनुमति प्राप्त: { $agents }
settings-forget-agents = भूल जाएँ
settings-ask-before-note = एजेंट के ये काम करने से पहले पूछें:
settings-ask-reading = जो चल रहा है उसे, या चित्र को देखना
settings-ask-playing = प्लेबैक, ध्वनि या पूर्ण स्क्रीन नियंत्रित करना
settings-ask-opening = फ़ाइल खोलना या विंडो बंद करना
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent } को plaa नियंत्रित करने की अनुमति दें?
agent-prompt-body = { $agent } plaa के बाहरी नियंत्रण का उपयोग करना चाहता है, ताकि वह देख सके कि क्या चल रहा है और उसे नियंत्रित कर सके। आप सेटिंग्स में बाहरी नियंत्रण बंद कर सकते हैं।
agent-prompt-allow = अनुमति दें
agent-prompt-deny = अनुमति न दें
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent } को देखने दें कि यहाँ क्या चल रहा है?
agent-ask-play = { $agent } को यहाँ प्लेबैक नियंत्रित करने दें?
agent-ask-open = { $agent } को विंडो खोलने या बंद करने दें?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } “{ $tool }” का उपयोग करना चाहता है। plaa किन बातों पर पूछे, यह सेटिंग्स में चुना जाता है।

## Casting to a TV or speaker on the network

cast-looking = टीवी और स्पीकर ढूँढे जा रहे हैं…
cast-none = कोई टीवी, स्पीकर या Chromecast नहीं मिला।
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer } पर चलाना बंद करें
notice-casting = { $renderer } पर चल रहा है।
notice-cast-failed = { $renderer } पर नहीं चलाया जा सका: { $reason }
cast-web-page = वेब पेज अभी कास्ट नहीं किए जा सकते: इसके बजाय वीडियो का अपना पता खोलें।

## The assistant panel, and its models in Settings

assistant-add-another = मॉडल जोड़ें…
assistant-add-model = असिस्टेंट को एक मॉडल चाहिए: अपनी API कुंजी के साथ क्लाउड मॉडल, या स्थानीय मॉडल।
assistant-ask = जो चल रहा है, उसके बारे में पूछें
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = चैट इतनी लंबी हो रही थी कि { $model } उसे एक बार में नहीं पढ़ सकता, इसलिए उसके पहले हिस्से का सारांश बनाया गया। असिस्टेंट जो देख चुका है, उसे फिर से देख सकता है।
assistant-compacting = पिछली चैट का सारांश बनाया जा रहा है
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = इस कंप्यूटर पर { $provider }
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $host } पर { $provider }
assistant-new-chat = नई चैट
assistant-no-model = पहले सेटिंग्स में एक मॉडल जोड़ें।
assistant-open-settings = मॉडल जोड़ें
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = बातचीत अब उससे बड़ी हो गई है जितना { $model } एक बार में पढ़ सकता है। नई चैट शुरू करें, या ऐसा मॉडल चुनें जो ज़्यादा पढ़ सके।
assistant-problem-credit = { $provider } के अनुसार खाते में कोई क्रेडिट नहीं है। नए खाते की कुंजी काम करे, इससे पहले { $provider } की साइट पर कुछ क्रेडिट खरीदना ज़रूरी है; उसके बाद फिर से कोशिश करें।
assistant-problem-key = { $provider } ने API कुंजी अस्वीकार कर दी। इसे सेटिंग्स में जाँचें।
assistant-problem-model = { $provider } में { $model } नाम का कोई मॉडल नहीं है। सेटिंग्स में इसका नाम जाँचें।
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } ने जवाब नहीं दिया: { $message }
assistant-problem-rate = { $provider } धीमा चलने को कह रहा है। थोड़ी देर में फिर से कोशिश करें।
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } धीमा चलने को कह रहा है: { $message }
assistant-problem-refused = { $model } ने जवाब देने से मना कर दिया।
assistant-problem-unavailable = { $provider } व्यस्त है या उसमें कोई समस्या है। थोड़ी देर में फिर से कोशिश करें।
assistant-problem-unreachable = plaa { $provider } तक नहीं पहुँच सका। अपना कनेक्शन जाँचें, या देखें कि सर्वर चल रहा है।
assistant-running = चल रहा है…
assistant-send = भेजें
assistant-shortened = चैट इतनी लंबी थी कि { $model } उसे एक बार में नहीं पढ़ सकता, इसलिए पहले के टूल परिणाम हटा दिए गए। असिस्टेंट जो देख चुका है, उसे फिर से देख सकता है।
assistant-stop = रोकें
assistant-stopped = रोक दिया गया।
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = अब { $model } से बात हो रही है।
assistant-thinking = सोच रहा है…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = विचार
assistant-title = असिस्टेंट
settings-assistant-add = मॉडल जोड़ें
settings-assistant-add-button = जोड़ें
# $model is the model's name.
settings-assistant-added = { $model } ने जवाब दिया और जोड़ दिया गया।
settings-assistant-added-tag = जोड़ा गया
# $example is an address, such as http://localhost:11434.
settings-assistant-address = पता, जैसे { $example }
settings-assistant-change-key = कुंजी बदलें
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI के साथ संगत सर्वर
# The menu of how much a local model reads at once.
settings-assistant-context = संदर्भ
settings-assistant-context-note = ज़्यादा होने पर असिस्टेंट एक चैट का ज़्यादा हिस्सा याद रख सकता है, लेकिन मॉडल ज़्यादा मेमोरी लेता है और धीरे जवाब दे सकता है।
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K टोकन
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = पेज खोलें
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } { $address } पर चल रहा है।
settings-assistant-found-none = { $provider } में अभी कोई मॉडल नहीं है। उससे एक डाउनलोड करें, फिर से खोजें।
settings-assistant-get-key = API कुंजी पाएँ
settings-assistant-get-server = { $provider } पाएँ
settings-assistant-in-use = उपयोग में
settings-assistant-key = API कुंजी
# $error is what the keychain said.
settings-assistant-key-failed = कुंजी कीचेन में नहीं रखी जा सकी: { $error }
settings-assistant-key-kept = आपकी { $provider } कुंजी सिस्टम के कीचेन में रखी गई है।
settings-assistant-key-needed = इस मॉडल को API कुंजी चाहिए।
settings-assistant-key-refused = { $provider } ने यह कुंजी अस्वीकार कर दी। जाँचें कि यह पूरी और सही खाते से कॉपी की गई है।
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = { $provider } की साइट पर एक कुंजी बनाएँ और उसे यहाँ पेस्ट करें।
settings-assistant-look-again = फिर से खोजें
settings-assistant-looking = मॉडल खोजे जा रहे हैं…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = मॉडल, जैसे { $example }
settings-assistant-no-server = plaa को इस कंप्यूटर पर कोई चलता हुआ { $provider } नहीं मिला। इसे शुरू करें, या नीचे इसका पता दें।
settings-assistant-no-tools = टूल का उपयोग नहीं कर सकता, जिनकी असिस्टेंट को ज़रूरत है
settings-assistant-none = अभी कोई मॉडल नहीं है। नीचे एक जोड़ें: स्थानीय मॉडल, जैसे Ollama का, इसी कंप्यूटर पर रहता है; क्लाउड मॉडल को अपने प्रदाता से API कुंजी चाहिए।
settings-assistant-note = वे मॉडल जिनसे असिस्टेंट पैनल बात कर सकता है। कुंजियाँ सिस्टम के कीचेन में रखी जाती हैं।
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = दूसरा पता उपयोग करें
settings-assistant-recommended = सुझाया गया
settings-assistant-remove = हटाएँ
settings-assistant-sees = तस्वीरें देखता है
settings-assistant-trying = आज़माया जा रहा है…
settings-assistant-use = उपयोग करें
settings-assistant-use-key = जारी रखें
settings-assistant-uses-tools = टूल का उपयोग करता है
