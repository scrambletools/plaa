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
language-name = বাংলা

## Common

app-name = plaa
common-cancel = বাতিল করুন
common-close = বন্ধ করুন
common-more = আরও

## Command line

usage-help =
    ব্যবহার: plaa [FILE OR ADDRESS]...

    ভিডিও ও অডিও ফাইল চালান, এবং https://example.com/live.m3u8-এর মতো
    ঠিকানার স্ট্রিম চালান। প্রতিটি চালু থাকা plaa-এর একটি উইন্ডোতে খোলে,
    যা প্রয়োজনে নিজেই চালু হয়।

    বিকল্প:
      -h, --help        এই সহায়তা দেখান
      -V, --version     সংস্করণ দেখান
          --interfaces  তারযুক্ত interface-গুলো দেখান, এবং plaa সেগুলোতে
                        AVB স্ট্রিম পাঠাতে পারে কি না
          --mcp         stdin ও stdout-এ MCP চালান, যাতে AI এজেন্ট চালু থাকা
                        plaa কী চালাচ্ছে তা দেখতে ও সেটি নিয়ন্ত্রণ করতে পারে

## Start window

start-hint = একটি ভিডিও বা অডিও ফাইল, স্ট্রিম বা ওয়েব ভিডিও খুলুন।
start-open = খুলুন
start-open-address = ঠিকানা খুলুন
# Opens a list of the talkers on the AVB network to listen to.
start-connect = সংযোগ করুন

## Actions, as buttons, tooltips and menus name them

action-open = খুলুন
action-open-address = ঠিকানা খুলুন
open-file = ফাইল খুলুন…
open-address = ঠিকানা খুলুন…
action-settings = সেটিংস
action-info = পরিদর্শক
action-loop = লুপ
action-screenshot = ফ্রেমটি ছবি হিসেবে সংরক্ষণ করুন
action-play = চালান
action-pause = বিরতি দিন
# $seconds is how far the button moves, such as 10.
action-back = { $seconds } সেকেন্ড পিছিয়ে যান
action-forward = { $seconds } সেকেন্ড এগিয়ে যান
action-mute = মিউট করুন
action-unmute = আনমিউট করুন
action-speed = গতি
action-tracks = অডিও ও সাবটাইটেল
action-full-screen = পূর্ণ স্ক্রিন
action-playlist = প্লেলিস্ট
action-chapters = অধ্যায়
action-picture-in-picture = পিকচার-ইন-পিকচার
action-previous = আগেরটি
action-next = পরেরটি
action-exit-full-screen = পূর্ণ স্ক্রিন থেকে বেরোন
action-quality = গুণমান
action-visualizer = ভিজ্যুয়ালাইজার
action-network = নেটওয়ার্ক অডিও
action-cast = কাস্ট করুন
action-connect = সংযোগ করুন
action-assistant = সহকারী

## What a sound file shows under its cover

visualizer-spectrum = স্পেকট্রাম
visualizer-waveform = ওয়েভফর্ম
visualizer-prism = প্রিজম
visualizer-cover = শুধু কভার

## The floating controls

overlay-keep-shown = কন্ট্রোল স্থির রাখুন
overlay-auto-hide = কন্ট্রোল ছবির উপরে ভাসিয়ে রাখুন

## Audio and subtitle tracks

tracks-audio = অডিও
tracks-subtitles = সাবটাইটেল
tracks-off = বন্ধ
# A track the file gives no name or language, by its number.
tracks-numbered = ট্র্যাক { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = শুধু অডিও

## Inspector

info-title = পরিদর্শক
playlist-title = প্লেলিস্ট
chapters-numbered = অধ্যায় { $number }
info-file = ফাইল
info-name = নাম
info-address = ঠিকানা
info-folder = ফোল্ডার
info-size = আকার
info-format = ফরম্যাট
info-duration = সময়কাল
info-bit-rate = বিট রেট
info-video = ভিডিও
info-audio = অডিও
info-codec = কোডেক
info-dimensions = মাত্রা
info-frame-rate = ফ্রেম রেট
info-frames-per-second = প্রতি সেকেন্ডে { $rate } ফ্রেম
info-pixel-format = পিক্সেল ফরম্যাট
info-decoding = ডিকোডিং
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = গ্রাফিক্স কার্ড ({ $api })
info-decoding-software = প্রসেসর
info-sample-rate = স্যাম্পল রেট
info-hertz = { $rate } Hz
info-channels = চ্যানেল
info-language = ভাষা
info-no-sound = কোনো শব্দ নেই
info-tags = ট্যাগ
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = স্ট্রিম
# The talker's listeners other than plaa.
info-other-listeners = অন্যান্য Listener
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = SR class
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = নতুন ফরম্যাটে { $count ->
        [one] Talker-এর অন্য Listener-টির সংযোগ কেটে যাবে।
       *[other] Talker-এর অন্য { $count }টি Listener-এর সংযোগ কেটে যাবে।
    }

## Dialogs

dialog-open = খুলুন
dialog-filter-media = ভিডিও ও অডিও
dialog-filter-all = সব ফাইল

address-title = ঠিকানা খুলুন
address-note = ভিডিও আছে এমন একটি ওয়েব পেজ, যেমন কোনো YouTube ভিডিওর পেজ, অথবা নেটওয়ার্কে থাকা কোনো স্ট্রিম বা ফাইল, যেমন https://example.com/live.m3u8।
address-label = ঠিকানা
address-open = খুলুন
address-recent = সাম্প্রতিক
address-forget = সাম্প্রতিক থেকে সরান

## Notices

notice-dialog-failed = ফাইল ডায়ালগ খোলা যায়নি: { $error }
notice-screenshot-saved = “{ $name }” সংরক্ষণ করা হয়েছে।
notice-screenshot-failed = ছবিটি সংরক্ষণ করা যায়নি: { $error }
notice-no-sound = শব্দ ছাড়াই চলছে: { $reason }
notice-loop-from = { $time } থেকে লুপ। যেখানে শেষ হবে সেখানে আবার চাপুন।
notice-loop-between = { $from } থেকে { $to } পর্যন্ত লুপ চলছে।
notice-loop-off = লুপ বন্ধ।

## Errors, shown in place of what could not play

error-needs-yt-dlp = এটি একটি ওয়েব পেজ বলে মনে হচ্ছে। plaa ওয়েব পেজের ভিডিও yt-dlp দিয়ে চালায়, যা ইনস্টল করা নেই।
error-empty-list = এই তালিকায় চালানোর মতো কিছু নেই।

## Settings

settings-title = সেটিংস
settings-tab-appearance = চেহারা
settings-tab-playback = প্লেব্যাক
settings-tab-network = নেটওয়ার্ক
settings-tab-language = ভাষা
settings-tab-agents = এজেন্ট
settings-tab-assistant = সহকারী
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (ডেভেলপমেন্ট বিল্ড, { $build })
settings-choose = বেছে নিন…

settings-appearance-system = সিস্টেম অনুযায়ী
settings-appearance-light = হালকা
settings-appearance-dark = গাঢ়
settings-colors = রং
settings-system-accent = সিস্টেমের অ্যাকসেন্ট রং ব্যবহার করুন
settings-omarchy-note = রংগুলো “{ $theme }”-এর অ্যাকসেন্ট থেকে তৈরি।
settings-system-accent-note = রংগুলো সিস্টেমের অ্যাকসেন্ট রং থেকে তৈরি।
settings-system-accent-none = সিস্টেমে কোনো অ্যাকসেন্ট রং নেই, তাই plaa নিচে বেছে নেওয়া রং ব্যবহার করে।
settings-accent-chosen-note = রংগুলো নিচে বেছে নেওয়া রং থেকে তৈরি।
settings-controls = কন্ট্রোল
settings-overlay = ভাসমান কন্ট্রোল
settings-overlay-note = টুলবার ও প্লেব্যাক কন্ট্রোল ছবি বা কভারের উপরে ভেসে থাকে, এবং পয়েন্টার স্থির থাকলে বা উইন্ডোর বাইরে থাকলে সরে যায়।
settings-overlay-delay = লুকানোর বিলম্ব
settings-overlay-delay-note = যা চলছে তার উপরে পয়েন্টার কতক্ষণ স্থির থাকলে কন্ট্রোল সরে যাবে।
settings-seconds = { $seconds } সেকেন্ড
settings-transparency = স্বচ্ছতা
settings-transparency-note = ভাসমান কন্ট্রোলের ভেতর দিয়ে ছবির কতটা দেখা যায়।
settings-percent = { $percent }%
settings-corner-radius = কোণের ব্যাসার্ধ
settings-corner-radius-note = ডায়ালগ ও ভাসমান কন্ট্রোলের জন্য।
settings-pixels = { $pixels } px
settings-animations = অ্যানিমেশন
settings-animations-note = সরে যাওয়া বার ও প্যানেল, বড় হয়ে ওঠা ডায়ালগ এবং লাফিয়ে ওঠা বোতাম।
settings-animations-reduced = সিস্টেম কম গতি চাইলে বন্ধ থাকে।

settings-hardware = গ্রাফিক্স কার্ড ব্যবহার করুন
settings-hardware-note = ভিডিও ডিকোড ও রপ্তানির এনকোড এতেই হয়: বিদ্যুৎ কম লাগে, বড় ভিডিও মসৃণভাবে চলে এবং রপ্তানি দ্রুত হয়। এরপর খোলা ফাইলে কার্যকর হয়।
settings-audio-device = শব্দের আউটপুট
settings-audio-device-system = সিস্টেম ডিফল্ট
settings-resume = যেখানে থেমেছিলেন সেখান থেকে চালান
settings-resume-note = ফাইল শেষবার বন্ধ করার সময় যেখানে ছিল, সেখান থেকে শুরু হয়।
settings-folder-playlists = ফোল্ডারের অন্য ফাইলগুলোও চালান
settings-folder-playlists-note = একটি ফাইল খুললে তার ফোল্ডারের অন্য ফাইলগুলোও তালিকায় আসে, যেগুলো এর পরে চলে।
settings-streams = স্ট্রিম ও ওয়েব ভিডিও
settings-stream-quality = গুণমান
settings-stream-quality-note = যেসব স্ট্রিম ও ওয়েব ভিডিও একাধিক গুণমানে আসে, সেগুলোর জন্য। এরপর যা খোলা হয় তাতে কার্যকর হয়।
settings-quality-auto = স্বয়ংক্রিয়, স্ক্রিনের উচ্চতা অনুযায়ী
settings-quality-best = সর্বোত্তম
settings-web-pages = yt-dlp দিয়ে ওয়েব ভিডিও চালান
settings-web-pages-found = yt-dlp YouTube-এর মতো পেজে ভিডিও খুঁজে নেয়।
settings-web-pages-missing = yt-dlp ইনস্টল করা নেই। YouTube-এর মতো পেজের ভিডিও চালাতে এটি ইনস্টল করুন।
settings-folders = ফোল্ডার
settings-recordings = রেকর্ডিং
settings-screenshots = সংরক্ষিত ফ্রেম

settings-interface-language = ইন্টারফেসের ভাষা
# The first entry of the language list.
settings-language-system = সিস্টেম অনুযায়ী
settings-track-languages = ট্র্যাকের ভাষা
settings-track-languages-note = en বা fra-এর মতো ভাষার কোড, সবচেয়ে পছন্দেরটি আগে, কমা দিয়ে আলাদা করা। কোনো ফাইলে এসব ভাষার অডিও ও সাবটাইটেল থাকলে plaa সেগুলো বেছে নেয়।
settings-audio-languages = অডিও
settings-subtitle-languages = সাবটাইটেল

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = AVB interface
settings-avb-interface-note = যে তারযুক্ত পোর্টে plaa AVB ও Milan-এর Talker ও Listener খুঁজে পায়। পাঠানোর জন্য PTP hardware clock আছে এমন পোর্ট লাগে, যেমন Intel I210-এর।
settings-avb-off = বন্ধ
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (শুধু তালিকা: PTP clock নেই)
settings-avb-off-mac = { $interface } (শুধু তালিকা: AVB বন্ধ)
settings-avb-interface-note-mac = যে তারযুক্ত পোর্টে plaa AVB ও Milan-এর Talker ও Listener খুঁজে পায়। পাঠানো হয় এই Mac-এর নিজস্ব AVB entity দিয়ে, যার জন্য Audio MIDI Setup-এর Network Device Browser-এ পোর্টটির AVB চালু থাকতে হবে।
settings-avb-talker-note-mac = সবশেষে চালানো উইন্ডোর শব্দ plaa এই Mac-এর নিজস্ব AVB entity দিয়ে, এর সঙ্গে সংযুক্ত Listener-দের কাছে পাঠায়।
settings-avb-talker = যা চলছে তা পাঠান
settings-avb-talker-note = plaa একটি Milan Talker হয়ে সবশেষে চালানো উইন্ডোর শব্দ এর সঙ্গে সংযুক্ত Listener-দের কাছে পাঠায়।
settings-avb-name = নেটওয়ার্কে নাম
settings-avb-name-note = Controller-গুলো plaa-কে “{ $name }” নামে দেখায়। নাম বদলাতে Enter চাপুন।
# plaa's name as a Milan entity, by default.
avb-default-name = { $computer }-এ plaa
avb-could-not-start = { $interface } ব্যবহার করা যায়নি: { $error }
network-title = নেটওয়ার্ক অডিও
network-off = নেটওয়ার্কে চালাতে সেটিংসে “যা চলছে তা পাঠান” চালু করুন।
network-blocked = পাঠানো হচ্ছে না: { $reason }
network-waiting = পাঠানোর জন্য প্রস্তুত, কোনো Listener সংযোগ করার অপেক্ষায়।
network-sending = সংযুক্ত Listener-দের কাছে পাঠানো হচ্ছে।
network-looking = Listener খোঁজা হচ্ছে…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = ptp4l চালু করা হচ্ছে, যা নেটওয়ার্কের সেই সময় রাখে যা ধরে plaa-এর স্ট্রিম পাঠানো হয়…
network-ptp-elsewhere = ptp4l অন্য একটি interface-এ চলছে, তাই এখানে plaa-এর স্ট্রিম কোনো নেটওয়ার্ক সময় মেনে চলে না এবং Listener-রা হয়তো এটি চালাতে পারবে না। সেটিংসে সেই interface বেছে নিন, অথবা এটিতে ptp4l চালান।
network-ptp-missing = ptp4l চলছে না, তাই plaa-এর স্ট্রিম কোনো নেটওয়ার্ক সময় মেনে চলে না এবং Listener-রা হয়তো এটি চালাতে পারবে না। plaa-এর প্যাকেজের সঙ্গে linuxptp ইনস্টল করুন, অথবা triib ইনস্টল করুন, যা এটি চালায়।
network-ptp-failed = ptp4l চলছে না, তাই plaa-এর স্ট্রিম কোনো নেটওয়ার্ক সময় মেনে চলে না এবং Listener-রা হয়তো এটি চালাতে পারবে না। plaa এটি চালু করতে পারেনি: { $reason }
network-no-listeners = নেটওয়ার্কের কোনো Listener plaa-এর শব্দ চালাতে পারে না।
# A listener with several inputs: its name and the input's.
network-stream = { $entity }: { $stream }
network-play-on = { $listener }-এ চালান
network-stop-on = { $listener }-এ চালানো বন্ধ করুন
network-choose-format = { $listener }-এ কোন ফরম্যাটে চালাবেন তা বেছে নিন
network-failed = সংযোগ করা যায়নি: { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = এখানে চালাতে triib আপডেট করুন।
network-choose = কোনো Listener কীভাবে তার স্ট্রিম নেয় তা দেখতে সেটি বেছে নিন।
network-no-sources = নেটওয়ার্কের কোনো Talker এমন ফরম্যাটে পাঠায় না যা plaa চালায়, আর এমন ফরম্যাটে বদলাতেও পারে না।
# $talker is a talker's name, or its name and its output's.
network-listen-stop = { $talker } শোনা বন্ধ করুন
network-listen-update-triib = এখানে শুনতে triib আপডেট করুন: plaa শুধু সেটুকুই চালায় যা এমনিতেই এই কম্পিউটারে পৌঁছায়।
network-listen-lite = পোর্টটি AVB Lite-এ চলছে, যা কোনো স্ট্রিম reserve করে না: plaa শুধু সেটুকুই চালায় যা এমনিতেই এই কম্পিউটারে পৌঁছায়।

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Talker খোঁজা হচ্ছে…
# plaa listens through its entity, which exists only while it sends.
connect-off = এখানে Talker শুনতে সেটিংসে “যা চলছে তা পাঠান” চালু করুন।
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa এখানে শুনতে পারছে না: { $reason }
connect-sending = plaa চালায় এমন ফরম্যাটে পাঠাচ্ছে
connect-changing = plaa চালায় এমন ফরম্যাটে বদলাবে
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, { $format }-এ
connect-change-cuts-off = { $talker }, { $format }-এ, এতে { $count ->
        [one] এর অন্য Listener-টির সংযোগ কেটে যাবে
       *[other] এর অন্য { $count }টি Listener-এর সংযোগ কেটে যাবে
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = অবস্থা
network-not-connected = সংযুক্ত নয়
network-from = { $stream } থেকে
network-from-receiving = { $stream } থেকে, গ্রহণ করছে
network-from-waiting = { $stream } থেকে, Talker-এর অপেক্ষায়
network-from-failed = { $stream } থেকে, Talker-এর reservation ব্যর্থ: { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Media clock
network-locked = স্ট্রিমের সঙ্গে lock হয়েছে
network-not-locked = Lock হয়নি
network-format = ফরম্যাট
# The formats both the listener and plaa's stream can be in.
network-formats = plaa থেকে যেসব ফরম্যাট নিতে পারে
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = Stream ID
# The MAC address the stream is sent to.
network-destination = গন্তব্য
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latency
network-microseconds = { $time } µs
network-frames = প্রাপ্ত frames
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = ক্রমের বাইরে আসা frames
# How often the stream stopped coming while it was taken.
network-interrupted = বিঘ্ন
# Frames that came after, or long before, their time to be played.
network-late = দেরিতে আসা frames
network-early = আগেভাগে আসা frames
network-entity-id = Entity ID

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = যথেষ্ট bandwidth নেই
msrp-failure-2 = bridge-এর যথেষ্ট রিসোর্স নেই
msrp-failure-3 = traffic class-এর জন্য যথেষ্ট bandwidth নেই
msrp-failure-4 = stream ID অন্য একটি Talker ব্যবহার করছে
msrp-failure-5 = destination address আগে থেকেই ব্যবহৃত হচ্ছে
msrp-failure-6 = উচ্চতর rank-এর একটি stream এটিকে সরিয়ে দিয়েছে
msrp-failure-7 = জানানো latency বদলে গেছে
msrp-failure-8 = egress port AVB সক্ষম নয়
msrp-failure-9 = অন্য একটি destination address ব্যবহার করুন
msrp-failure-10 = MSRP রিসোর্স শেষ
msrp-failure-11 = MMRP রিসোর্স শেষ
msrp-failure-12 = destination address সংরক্ষণ করা যাচ্ছে না
msrp-failure-13 = অগ্রাধিকারটি SR class-এর অগ্রাধিকার নয়
msrp-failure-14 = frames মাধ্যমের জন্য খুব বড়
msrp-failure-15 = fan-in port-এর সীমা পূর্ণ
msrp-failure-16 = নিবন্ধিত stream-এর প্রথম মান বদলেছে
msrp-failure-17 = egress port-এ VLAN ব্লক করা
msrp-failure-18 = egress port-এ VLAN tagging বন্ধ
msrp-failure-19 = SR class-এর অগ্রাধিকার মেলে না
msrp-failure-unknown = অজানা কারণ
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, bridge { $bridge }-এ

## The macOS menu bar

menu-about = plaa সম্পর্কে
menu-settings = সেটিংস…
menu-services = পরিষেবা
menu-hide = plaa লুকান
menu-hide-others = অন্যগুলো লুকান
menu-show-all = সব দেখান
menu-quit = plaa থেকে প্রস্থান করুন
menu-file = ফাইল
menu-open = খুলুন…
menu-open-address = ঠিকানা খুলুন…
menu-close = উইন্ডো বন্ধ করুন
menu-edit = সম্পাদনা
menu-cut = কাট করুন
menu-copy = কপি করুন
menu-paste = পেস্ট করুন
menu-select-all = সব নির্বাচন করুন
menu-undo = পূর্বাবস্থায় ফেরান
menu-redo = পুনরায় করুন
menu-trim = ট্রিম করুন…
menu-split = ক্লিপ ভাগ করুন
menu-rotate-left = বাঁয়ে ঘোরান
menu-rotate-right = ডানে ঘোরান
menu-flip-horizontal = অনুভূমিকভাবে উল্টান
menu-flip-vertical = উল্লম্বভাবে উল্টান
menu-revert = মূল সংস্করণে ফেরান
menu-export = রপ্তানি করুন…
menu-view = দেখুন
menu-playback = প্লেব্যাক
menu-play-pause = চালান বা বিরতি দিন
menu-to-start = শুরুতে যান
menu-previous-chapter = আগের অধ্যায়
menu-next-chapter = পরের অধ্যায়
menu-faster = আরও দ্রুত
menu-slower = আরও ধীরে
menu-normal-speed = স্বাভাবিক গতি
menu-ab-loop = দুটি বিন্দুর মধ্যে লুপ
menu-volume-up = আরও জোরে
menu-volume-down = আরও আস্তে
menu-mute = মিউট বা আনমিউট করুন
menu-next-subtitles = পরের সাবটাইটেল
menu-next-audio = পরের অডিও ট্র্যাক
menu-window = উইন্ডো
menu-minimize = ছোট করুন
menu-zoom = জুম
menu-bring-all-to-front = সব সামনে আনুন

## Editing

action-edit = সম্পাদনা
title-edited = { $name } (সম্পাদিত)
edit-trim = ট্রিম
edit-trim-apply = ট্রিম করুন
edit-split = ক্লিপ ভাগ করুন
edit-remove-clip = ক্লিপ সরান
edit-done = সম্পন্ন
edit-rotate-left = বাঁয়ে ঘোরান
edit-rotate-right = ডানে ঘোরান
edit-flip-horizontal = অনুভূমিকভাবে উল্টান
edit-flip-vertical = উল্লম্বভাবে উল্টান
edit-undo = পূর্বাবস্থায় ফেরান
edit-redo = পুনরায় করুন
edit-revert = মূল সংস্করণে ফেরান
edit-export = রপ্তানি করুন…

## Exporting

export-title = রপ্তানি করুন
export-format = ফরম্যাট
export-size = আকার
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = শুধু শব্দ
export-size-original = { $lines }p, যেমন আছে
export-on-graphics-card = গ্রাফিক্স কার্ডে এনকোড করা হয় ({ $api })
export-on-processor = প্রসেসরে এনকোড করা হয়, এতে বেশি সময় লাগে
export-no-encoder = এই ফরম্যাটের কোনো এনকোডার এখানে খোলে না
export-checking = এনকোডার খোঁজা হচ্ছে…
export-sound-format = শব্দের ফরম্যাট
export-quality = গুণমান
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = সংকুচিত; প্রায় সব জায়গায় চলে, Apple-এর ডিভাইসেও
export-mp3-note = সংকুচিত; সব জায়গায় চলে
export-opus-note = সংকুচিত; আকারের তুলনায় সেরা শব্দ, তবে সব প্লেয়ার এটি চালাতে পারে না
export-flac-note = ক্ষতিহীন: শব্দ হুবহু যেমন আছে, একটি মুক্ত ফরম্যাটে
export-alac-note = ক্ষতিহীন: শব্দ হুবহু যেমন আছে, Apple-এর ডিভাইস যেভাবে চালায়
export-wav-note = অসংকুচিত: শব্দ হুবহু যেমন আছে, সবচেয়ে বড় ফাইলে
export-kilobits = { $rate } kb/s
export-bits = { $bits }-বিট
export-rate-lower = { $rate } kHz-এ সংরক্ষিত, ফাইলের { $file } kHz-এর চেয়ে কম
export-choose = রপ্তানি করুন…
export-name = { $name } রপ্তানি
export-name-edited = { $name } সম্পাদিত
export-progress = “{ $name }” রপ্তানি করা হচ্ছে
export-stop = রপ্তানি থামান
notice-exported = “{ $name }” রপ্তানি করা হয়েছে
notice-export-stopped = রপ্তানি থেমে গেছে
notice-export-failed = রপ্তানি করা যায়নি: { $error }
close-title = রপ্তানি না করেই বন্ধ করবেন?
close-unexported = রপ্তানি না করলে “{ $name }”-এর পরিবর্তনগুলো হারিয়ে যাবে।
close-exporting = “{ $name }”-এর রপ্তানি থেমে যাবে, এবং এ পর্যন্ত যা লেখা হয়েছে তা হারিয়ে যাবে।
close-anyway = বন্ধ করুন
close-quit-anyway = প্রস্থান করুন

## Recording

record-title = শব্দ রেকর্ডিং
record-file-name = শব্দ রেকর্ডিং
record-start = রেকর্ড করুন
record-stop = রেকর্ডিং থামান
record-no-input = কোনো মাইক্রোফোন নেই
record-microphone = মাইক্রোফোন
record-default-microphone = সিস্টেমের ডিফল্ট
record-playing = কম্পিউটারে যা চলছে
record-mono = মোনো
record-stereo = স্টেরিও
record-channels = { $count } চ্যানেল
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, { $saved } kHz-এ সংরক্ষিত
action-record-sound = শব্দ রেকর্ড করুন
open-record-sound = নতুন শব্দ রেকর্ডিং
start-record-sound = শব্দ রেকর্ড করুন
menu-record-sound = নতুন শব্দ রেকর্ডিং
record-screen-title = স্ক্রিন রেকর্ডিং
record-screen-file-name = স্ক্রিন রেকর্ডিং
record-screen-whole = স্ক্রিন
record-screen-number = স্ক্রিন { $number }
record-screen-main = স্ক্রিন { $number } (প্রধান)
record-screens = স্ক্রিন
record-windows = উইন্ডো
record-screen-what = কী রেকর্ড করবেন
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = প্রতিটি ছবি
record-rate-tip = প্রতি সেকেন্ডে ছবি
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count }টি বাদ পড়েছে
record-screen-window = উইন্ডো
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = আর শেয়ার করা হচ্ছে না
record-screen-again = আবার বেছে নিন
record-screen-failed = স্ক্রিন রেকর্ড করা যায়নি
record-sound = শব্দ
record-no-sound = কোনো শব্দ নেই
# How much louder the input is taken, for a quiet microphone.
record-gain = আরও জোরে
record-gain-value = +{ $decibels } dB
record-level-automatically = স্বয়ংক্রিয়ভাবে লেভেল ঠিক করুন
record-hide-title = রেকর্ড করার সময় plaa লুকিয়ে থাকে
record-hide-tray = রেকর্ডিং থামাতে ট্রেতে plaa-এর দেখানো লাল বিন্দুতে ক্লিক করুন, অথবা আবার plaa খুলুন।
record-hide-reopen = রেকর্ডিং থামাতে আবার plaa খুলুন।
record-hide-start = রেকর্ডিং শুরু করুন
record-tray-title = plaa স্ক্রিন রেকর্ড করছে
record-tray-tip = রেকর্ডিং থামাতে ক্লিক করুন
action-record-screen = স্ক্রিন রেকর্ড করুন
open-record-screen = নতুন স্ক্রিন রেকর্ডিং
start-record-screen = স্ক্রিন রেকর্ড করুন
menu-record-screen = নতুন স্ক্রিন রেকর্ডিং
record-camera-title = ক্যামেরা রেকর্ডিং
record-camera-file-name = ক্যামেরা রেকর্ডিং
record-camera = ক্যামেরা
record-camera-failed = ক্যামেরা খোলা যায়নি
action-record-camera = ক্যামেরা রেকর্ড করুন
open-record-camera = নতুন ক্যামেরা রেকর্ডিং
start-record-camera = ক্যামেরা রেকর্ড করুন
menu-record-camera = নতুন ক্যামেরা রেকর্ডিং
notice-no-camera = রেকর্ড করার মতো কোনো ক্যামেরা নেই
notice-recorded = “{ $name }” { $folder }-এ সংরক্ষণ করা হয়েছে
notice-record-failed = রেকর্ড করা যায়নি: { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = বাইরের নিয়ন্ত্রণের অনুমতি দিন
settings-allow-outside-control-note = Claude Code-এর মতো AI এজেন্ট plaa --mcp ব্যবহার করে plaa-এ কী চলছে তা দেখতে ও সেটি নিয়ন্ত্রণ করতে পারে। প্রতিটি নতুন এজেন্টের আগে plaa জিজ্ঞাসা করে।
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = অনুমোদিত: { $agents }
settings-forget-agents = ভুলে যান
settings-ask-before-note = কোনো এজেন্ট এগুলো করার আগে জিজ্ঞাসা করুন:
settings-ask-reading = যা চলছে বা ছবিটি দেখা
settings-ask-playing = প্লেব্যাক, শব্দ বা পূর্ণ স্ক্রিন নিয়ন্ত্রণ করা
settings-ask-opening = ফাইল খোলা বা উইন্ডো বন্ধ করা
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent }-কে plaa নিয়ন্ত্রণ করার অনুমতি দেবেন?
agent-prompt-body = { $agent } plaa-এর বাইরের নিয়ন্ত্রণ ব্যবহার করে কী চলছে তা দেখতে ও সেটি নিয়ন্ত্রণ করতে চায়। সেটিংসে গিয়ে বাইরের নিয়ন্ত্রণ বন্ধ করতে পারেন।
agent-prompt-allow = অনুমতি দিন
agent-prompt-deny = অনুমতি দেবেন না
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = { $agent }-কে এখানে কী চলছে তা দেখতে দেবেন?
agent-ask-play = { $agent }-কে এখানে প্লেব্যাক নিয়ন্ত্রণ করতে দেবেন?
agent-ask-open = { $agent }-কে কোনো উইন্ডো খুলতে বা বন্ধ করতে দেবেন?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } “{ $tool }” ব্যবহার করতে চায়। plaa কোন বিষয়ে জিজ্ঞাসা করবে, তা সেটিংসে বেছে নেওয়া যায়।

## Casting to a TV or speaker on the network

cast-looking = টিভি ও স্পিকার খোঁজা হচ্ছে…
cast-none = কোনো টিভি, স্পিকার বা Chromecast পাওয়া যায়নি।
# $renderer is the TV's or speaker's name.
cast-stop = { $renderer }-এ চালানো বন্ধ করুন
notice-casting = { $renderer }-এ চলছে।
notice-cast-failed = { $renderer }-এ চালানো যায়নি: { $reason }
cast-web-page = ওয়েব পেজ এখনও কাস্ট করা যায় না: এর বদলে ভিডিওর নিজস্ব ঠিকানা খুলুন।

## The assistant panel, and its models in Settings

assistant-add-another = মডেল যোগ করুন…
assistant-add-model = সহকারীর একটি মডেল দরকার: API কী-সহ একটি ক্লাউড মডেল, অথবা একটি স্থানীয় মডেল।
assistant-ask = যা চলছে সে সম্পর্কে জিজ্ঞাসা করুন
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = চ্যাটটি { $model }-এর একবারে পড়ার জন্য বেশি লম্বা হয়ে যাচ্ছিল, তাই এর আগের অংশের সারাংশ করা হয়েছে। সহকারী যা দেখেছে তা আবার দেখতে পারে।
assistant-compacting = আগের চ্যাটের সারাংশ তৈরি হচ্ছে
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = এই কম্পিউটারে { $provider }
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $host }-এ { $provider }
assistant-new-chat = নতুন চ্যাট
assistant-no-model = আগে সেটিংসে একটি মডেল যোগ করুন।
assistant-open-settings = মডেল যোগ করুন
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = কথোপকথনটি এখন { $model } একবারে যতটা পড়তে পারে তার চেয়ে বড় হয়ে গেছে। নতুন চ্যাট শুরু করুন, অথবা এমন মডেল বেছে নিন যা বেশি পড়তে পারে।
assistant-problem-credit = { $provider } বলছে অ্যাকাউন্টে কোনো ক্রেডিট নেই। নতুন অ্যাকাউন্টের কী কাজ করার আগে { $provider }-এর সাইটে কিছু ক্রেডিট কিনতে হয়; তারপর আবার চেষ্টা করুন।
assistant-problem-key = { $provider } API কী গ্রহণ করেনি। সেটিংসে এটি দেখুন।
assistant-problem-model = { $provider }-এ { $model } নামে কোনো মডেল নেই। সেটিংসে এর নাম দেখুন।
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } উত্তর দেয়নি: { $message }
assistant-problem-rate = { $provider } ধীরে চলতে বলছে। একটু পরে আবার চেষ্টা করুন।
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } ধীরে চলতে বলছে: { $message }
assistant-problem-refused = { $model } উত্তর দিতে অস্বীকার করেছে।
assistant-problem-unavailable = { $provider } ব্যস্ত বা সমস্যায় আছে। একটু পরে আবার চেষ্টা করুন।
assistant-problem-unreachable = plaa { $provider }-এ পৌঁছাতে পারেনি। আপনার সংযোগ দেখুন, অথবা সার্ভারটি চলছে কি না।
assistant-running = চলছে…
assistant-send = পাঠান
assistant-shortened = চ্যাটটি { $model }-এর একবারে পড়ার জন্য বেশি লম্বা ছিল, তাই আগের টুলের ফলাফল বাদ দেওয়া হয়েছে। সহকারী যা দেখেছে তা আবার দেখতে পারে।
assistant-stop = থামান
assistant-stopped = থামানো হয়েছে।
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = এখন { $model }-এর সঙ্গে কথা হচ্ছে।
assistant-thinking = ভাবছে…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = চিন্তা
assistant-title = সহকারী
settings-assistant-add = মডেল যোগ করুন
settings-assistant-add-button = যোগ করুন
# $model is the model's name.
settings-assistant-added = { $model } উত্তর দিয়েছে এবং যোগ করা হয়েছে।
settings-assistant-added-tag = যোগ করা হয়েছে
# $example is an address, such as http://localhost:11434.
settings-assistant-address = ঠিকানা, যেমন { $example }
settings-assistant-change-key = কী পরিবর্তন করুন
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = OpenAI-এর সঙ্গে সামঞ্জস্যপূর্ণ সার্ভার
# The menu of how much a local model reads at once.
settings-assistant-context = কনটেক্সট
settings-assistant-context-note = বেশি হলে সহকারী একটি চ্যাটের বেশি অংশ মনে রাখতে পারে, কিন্তু মডেল বেশি মেমরি নেয় এবং ধীরে উত্তর দিতে পারে।
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K টোকেন
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = পৃষ্ঠাটি খুলুন
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } { $address }-এ চলছে।
settings-assistant-found-none = { $provider }-এ এখনো কোনো মডেল নেই। এটি দিয়ে একটি ডাউনলোড করুন, তারপর আবার খুঁজুন।
settings-assistant-get-key = API কী নিন
settings-assistant-get-server = { $provider } নিন
settings-assistant-in-use = ব্যবহৃত হচ্ছে
settings-assistant-key = API কী
# $error is what the keychain said.
settings-assistant-key-failed = কী কীচেইনে রাখা যায়নি: { $error }
settings-assistant-key-kept = আপনার { $provider } কী সিস্টেমের কীচেইনে রাখা আছে।
settings-assistant-key-needed = এই মডেলের জন্য API কী লাগে।
settings-assistant-key-refused = { $provider } এই কী গ্রহণ করেনি। দেখুন এটি পুরোটা এবং সঠিক অ্যাকাউন্ট থেকে কপি করা হয়েছে কি না।
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = { $provider }-এর সাইটে একটি কী তৈরি করুন এবং এখানে পেস্ট করুন।
settings-assistant-look-again = আবার খুঁজুন
settings-assistant-looking = মডেল খোঁজা হচ্ছে…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = মডেল, যেমন { $example }
settings-assistant-no-server = plaa এই কম্পিউটারে কোনো চলমান { $provider } খুঁজে পায়নি। এটি চালু করুন, অথবা নিচে এর ঠিকানা দিন।
settings-assistant-no-tools = টুল ব্যবহার করতে পারে না, যা সহকারীর দরকার
settings-assistant-none = এখনো কোনো মডেল নেই। নিচে একটি যোগ করুন: স্থানীয় মডেল, যেমন Ollama-এর, এই কম্পিউটারেই থাকে; ক্লাউড মডেলের জন্য তার প্রদানকারীর কাছ থেকে API কী লাগে।
settings-assistant-note = যেসব মডেলের সঙ্গে সহকারী প্যানেল কথা বলতে পারে। কী সিস্টেমের কীচেইনে রাখা হয়।
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = অন্য ঠিকানা ব্যবহার করুন
settings-assistant-recommended = প্রস্তাবিত
settings-assistant-remove = সরান
settings-assistant-sees = ছবি দেখে
settings-assistant-trying = চেষ্টা করা হচ্ছে…
settings-assistant-use = ব্যবহার করুন
settings-assistant-use-key = চালিয়ে যান
settings-assistant-uses-tools = টুল ব্যবহার করে
