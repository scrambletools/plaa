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
language-name = Français

## Common

app-name = plaa
common-cancel = Annuler
common-close = Fermer
common-more = Plus

## Command line

usage-help =
    Utilisation : plaa [FICHIER OU ADRESSE]...

    Lit des fichiers vidéo et audio, et des flux à des adresses comme
    https://example.com/live.m3u8. Chacun s’ouvre dans une fenêtre de plaa
    s’il est déjà lancé ; sinon, plaa démarre.

    Options :
      -h, --help        Afficher cette aide
      -V, --version     Afficher la version
          --interfaces  Afficher les interfaces filaires, et si plaa peut y
                        envoyer des flux AVB
          --mcp         Servir MCP sur stdin et stdout, pour que des agents IA
                        voient ce que lit le plaa en cours et le pilotent

## Start window

start-hint = Ouvrez un fichier vidéo ou audio, un flux ou une vidéo web.
start-open = Ouvrir
start-open-address = Ouvrir une adresse
# Opens a list of the talkers on the AVB network to listen to.
start-connect = Se connecter

## Actions, as buttons, tooltips and menus name them

action-open = Ouvrir
action-open-address = Ouvrir une adresse
open-file = Ouvrir un fichier…
open-address = Ouvrir une adresse…
action-settings = Réglages
action-info = Inspecteur
action-loop = Boucle
action-screenshot = Enregistrer l’image affichée
action-play = Lire
action-pause = Pause
# $seconds is how far the button moves, such as 10.
action-back = { $seconds ->
    [one] Reculer de { $seconds } seconde
   *[other] Reculer de { $seconds } secondes
}
action-forward = { $seconds ->
    [one] Avancer de { $seconds } seconde
   *[other] Avancer de { $seconds } secondes
}
action-mute = Couper le son
action-unmute = Rétablir le son
action-speed = Vitesse
action-tracks = Audio et sous-titres
action-full-screen = Plein écran
action-playlist = Liste de lecture
action-chapters = Chapitres
action-picture-in-picture = Image dans l’image
action-previous = Précédent
action-next = Suivant
action-exit-full-screen = Quitter le plein écran
action-quality = Qualité
action-visualizer = Visualiseur
action-network = Audio en réseau
action-cast = Caster
action-connect = Se connecter
action-assistant = Assistant

## What a sound file shows under its cover

visualizer-spectrum = Spectre
visualizer-waveform = Forme d’onde
visualizer-prism = Prisme
visualizer-cover = Pochette seule

## The floating controls

overlay-keep-shown = Garder les commandes ancrées
overlay-auto-hide = Faire flotter les commandes sur l’image

## Audio and subtitle tracks

tracks-audio = Audio
tracks-subtitles = Sous-titres
tracks-off = Désactivés
# A track the file gives no name or language, by its number.
tracks-numbered = Piste { $number }

## The qualities a stream or a web video comes in

# A picture's height in lines, as qualities are named: 1080p.
quality-lines = { $lines }p
quality-audio-only = Audio seul

## Inspector

info-title = Inspecteur
playlist-title = Liste de lecture
chapters-numbered = Chapitre { $number }
info-file = Fichier
info-name = Nom
info-address = Adresse
info-folder = Dossier
info-size = Taille
info-format = Format
info-duration = Durée
info-bit-rate = Débit
info-video = Vidéo
info-audio = Audio
info-codec = Codec
info-dimensions = Dimensions
info-frame-rate = Fréquence d’images
info-frames-per-second = { $rate } images par seconde
info-pixel-format = Format de pixel
info-decoding = Décodage
# $api is the graphics card interface, such as vaapi or d3d11va.
info-decoding-hardware = Carte graphique ({ $api })
info-decoding-software = Processeur
info-sample-rate = Fréquence d’échantillonnage
info-hertz = { $rate } Hz
info-channels = Canaux
info-language = Langue
info-no-sound = Pas de son
info-tags = Métadonnées
# For a stream plaa listens to on the network: the talker sending it, and
# the stream as it comes.
info-talker = Talker
info-stream = Flux
# The talker's listeners other than plaa.
info-other-listeners = Autres listeners
# The stream's SR class, A or B, by how often its frames come.
info-sr-class = Classe SR
# Over the talker's formats to change to: its other listeners do not take
# the new one.
info-format-cuts-off = Un nouveau format coupe { $count ->
        [one] l’autre listener du talker.
       *[other] les { $count } autres listeners du talker.
    }

## Dialogs

dialog-open = Ouvrir
dialog-filter-media = Vidéo et audio
dialog-filter-all = Tous les fichiers

address-title = Ouvrir une adresse
address-note = Une page web avec une vidéo, comme celle d’une vidéo YouTube, ou un flux ou un fichier sur le réseau, comme https://example.com/live.m3u8.
address-label = Adresse
address-open = Ouvrir
address-recent = Récentes
address-forget = Retirer des adresses récentes

## Notices

notice-dialog-failed = Impossible d’ouvrir la zone de dialogue de fichier : { $error }
notice-screenshot-saved = « { $name } » enregistré.
notice-screenshot-failed = Impossible d’enregistrer l’image : { $error }
notice-no-sound = Lecture sans son : { $reason }
notice-loop-from = Boucle à partir de { $time }. Appuyez de nouveau là où elle doit se terminer.
notice-loop-between = Boucle de { $from } à { $to }.
notice-loop-off = Boucle désactivée.

## Errors, shown in place of what could not play

error-needs-yt-dlp = Ceci ressemble à une page web. plaa lit les vidéos des pages web avec yt-dlp, qui n’est pas installé.
error-empty-list = Cette liste ne contient rien à lire.

## Settings

settings-title = Réglages
settings-tab-appearance = Apparence
settings-tab-playback = Lecture
settings-tab-storage = Stockage
settings-tab-network = Réseau
settings-tab-language = Langue
settings-tab-agents = Agents
settings-tab-assistant = Assistant
settings-version = plaa { $version } ({ $build })
settings-version-development = plaa { $version } (version de développement, { $build })
settings-choose = Choisir…

settings-appearance-system = Suivre le système
settings-appearance-light = Clair
settings-appearance-dark = Sombre
settings-colors = Couleurs
settings-system-accent = Utiliser la couleur d’accentuation du système
settings-omarchy-note = Les couleurs sont dérivées de la couleur d’accentuation de « { $theme } ».
settings-system-accent-note = Les couleurs sont dérivées de la couleur d’accentuation du système.
settings-system-accent-none = Le système n’a pas de couleur d’accentuation, plaa utilise donc celle choisie ci-dessous.
settings-accent-chosen-note = Les couleurs sont dérivées de la couleur choisie ci-dessous.
settings-controls = Commandes
settings-overlay = Commandes flottantes
settings-overlay-note = La barre d’outils et les commandes de lecture flottent au-dessus de l’image ou de la pochette et s’escamotent tant que le pointeur est immobile ou hors de la fenêtre.
settings-overlay-delay = Masquer après
settings-overlay-delay-note = Le temps pendant lequel le pointeur reste immobile sur ce qui est lu avant que les commandes s’escamotent.
settings-seconds = { $seconds } s
settings-transparency = Transparence
settings-transparency-note = La part de l’image visible à travers les commandes flottantes.
settings-percent = { $percent } %
settings-corner-radius = Arrondi des angles
settings-corner-radius-note = Pour les zones de dialogue et les commandes flottantes.
settings-pixels = { $pixels } px
settings-animations = Animations
settings-animations-note = Barres et panneaux qui glissent, zones de dialogue qui s’agrandissent et boutons élastiques.
settings-animations-reduced = Désactivées tant que le système demande de réduire les animations.

settings-hardware = Utiliser la carte graphique
settings-hardware-note = Décode les vidéos et encode les exportations sur la carte graphique : moins d’énergie consommée, lecture fluide des grandes vidéos et exportations plus rapides. S’applique aux fichiers ouverts ensuite.
settings-audio-device = Sortie audio
settings-audio-device-system = Par défaut du système
settings-resume = Reprendre là où vous en étiez
settings-resume-note = Les fichiers reprennent là où ils en étaient à leur dernière fermeture.
settings-folder-playlists = Lire les autres fichiers du dossier
settings-folder-playlists-note = Ouvrir un fichier ajoute à la liste les autres fichiers de son dossier, qui sont lus ensuite.
settings-streams = Flux et vidéos web
settings-stream-quality = Qualité
settings-stream-quality-note = Pour les flux et vidéos web proposés en plusieurs qualités. S’applique à ce qui est ouvert ensuite.
settings-quality-auto = Automatique, à la hauteur de l’écran
settings-quality-best = Optimale
settings-web-pages = Lire les vidéos web avec yt-dlp
settings-web-pages-found = yt-dlp trouve la vidéo sur des pages comme celles de YouTube.
settings-web-pages-missing = yt-dlp n’est pas installé. Installez-le pour lire les vidéos de pages comme celles de YouTube.
settings-default-app = App par défaut
settings-default-app-label = Ouvrir les fichiers avec plaa
settings-default-app-note = Faire de plaa l’app qui ouvre les fichiers vidéo et audio.
settings-default-app-note-windows = Windows ne permet de choisir les apps par défaut que dans ses propres Paramètres. Ceci y ouvre la page de plaa.
settings-default-app-note-macos = macOS vous demande de confirmer chaque type : MP4, MOV, MKV, WebM, MP3, M4A, FLAC et WAV.
settings-default-app-status = { $set } types de fichiers sur { $total } s’ouvrent avec plaa.
settings-default-app-button = Définir par défaut
settings-default-app-button-windows = Ouvrir les Paramètres
settings-default-app-no-entry = L’entrée de bureau de plaa n’est pas installée, le système ne peut donc pas ouvrir de fichiers avec elle. Installez plaa depuis un paquet.
settings-default-app-no-bundle = Ouvrez plaa depuis plaa.app pour le définir par défaut.
settings-default-app-failed = Impossible de définir plaa par défaut : { $error }
settings-folders = Dossiers
settings-sound-recordings = Enregistrements audio
settings-video-recordings = Enregistrements vidéo
settings-screenshots = Images enregistrées

settings-interface-language = Langue de l’interface
# The first entry of the language list.
settings-language-system = Suivre le système
settings-track-languages = Langues des pistes
settings-track-languages-note = Codes de langue comme en ou fra, par ordre de préférence, séparés par des virgules. plaa choisit l’audio et les sous-titres dans ces langues quand un fichier les propose.
settings-audio-languages = Audio
settings-subtitle-languages = Sous-titres

## The AVB network, in Settings and the Network Audio sheet

settings-avb-interface = Interface AVB
settings-avb-interface-note = Le port filaire sur lequel plaa trouve les talkers et listeners AVB et Milan. L’envoi nécessite un port doté d’une horloge matérielle PTP, comme celui d’une Intel I210.
settings-avb-off = Désactivée
# An interface plaa can list streams on but not send from.
settings-avb-no-clock = { $interface } (liste seulement : pas d’horloge PTP)
settings-avb-off-mac = { $interface } (liste seulement : AVB désactivé)
settings-avb-interface-note-mac = Le port filaire sur lequel plaa trouve les talkers et listeners AVB et Milan. L’envoi passe par l’entité AVB propre à ce Mac, ce qui nécessite d’activer AVB pour ce port dans le navigateur d’appareils réseau de Configuration audio et MIDI.
settings-avb-talker-note-mac = plaa envoie le son de la dernière fenêtre lue via l’entité AVB propre à ce Mac, aux listeners qui y sont connectés.
settings-avb-talker = Envoyer ce qui est lu
settings-avb-talker-note = plaa devient un talker Milan et envoie le son de la dernière fenêtre lue aux listeners qui y sont connectés.
settings-avb-name = Nom sur le réseau
settings-avb-name-note = Les contrôleurs affichent plaa sous le nom « { $name } ». Appuyez sur Enter pour le renommer.
# plaa's name as a Milan entity, by default.
avb-default-name = plaa sur { $computer }
avb-could-not-start = Impossible d’utiliser { $interface } : { $error }
network-title = Audio en réseau
network-off = Activez « Envoyer ce qui est lu » dans les Réglages pour lire sur le réseau.
network-blocked = Pas d’envoi : { $reason }
network-waiting = Prêt à envoyer, en attente de la connexion d’un listener.
network-sending = Envoi aux listeners connectés.
network-looking = Recherche de listeners…
# ptp4l (linuxptp's) keeps the network's time, which plaa's stream is
# sent by and listeners play it by.
network-ptp-starting = Démarrage de ptp4l, qui fournit l’heure réseau sur laquelle se cale l’envoi du flux de plaa…
network-ptp-elsewhere = ptp4l tourne sur une autre interface : le flux de plaa n’a donc pas d’heure réseau ici et les listeners risquent de ne pas le lire. Choisissez cette interface dans les Réglages, ou lancez ptp4l sur celle-ci.
network-ptp-missing = ptp4l ne tourne pas : le flux de plaa n’a donc pas d’heure réseau et les listeners risquent de ne pas le lire. Installez linuxptp avec le paquet de plaa, ou installez triib, qui le lance.
network-ptp-failed = ptp4l ne tourne pas : le flux de plaa n’a donc pas d’heure réseau et les listeners risquent de ne pas le lire. plaa n’a pas pu le lancer : { $reason }
network-no-listeners = Aucun listener du réseau ne peut lire le son de plaa.
# A listener with several inputs: its name and the input's.
network-stream = { $entity } : { $stream }
network-play-on = Lire sur { $listener }
network-stop-on = Arrêter la lecture sur { $listener }
network-choose-format = Choisir le format de lecture sur { $listener }
network-failed = Connexion impossible : { $reason }
# Under a listener on this computer, such as one of triib's entities, while
# triib's entity service is too old to pass plaa's stream to it.
network-update-triib = Mettez triib à jour pour lire ici.
network-choose = Choisissez un listener pour voir comment il reçoit son flux.
network-no-sources = Aucun talker du réseau n’envoie dans un format que plaa lit, ni ne peut passer à un tel format.
# $talker is a talker's name, or its name and its output's.
network-listen-stop = Arrêter d’écouter { $talker }
network-listen-update-triib = Mettez triib à jour pour écouter ici : plaa ne lit que ce qui parvient de toute façon à cet ordinateur.
network-listen-lite = Le port utilise AVB Lite, qui ne réserve aucun flux : plaa ne lit que ce qui parvient de toute façon à cet ordinateur.

# The Connect menu: the talkers on the AVB network plaa can listen to,
# those sending in a format it plays, then those that change format.
connect-looking = Recherche de talkers…
# plaa listens through its entity, which exists only while it sends.
connect-off = Activez « Envoyer ce qui est lu » dans les Réglages pour écouter des talkers ici.
# $reason is why plaa's entity cannot run, as network-blocked gives it.
connect-blocked = plaa ne peut pas écouter ici : { $reason }
connect-sending = Envoient dans un format que plaa lit
connect-changing = Passent à un format que plaa lit
# A talker that changes to $format for plaa, under connect-changing.
connect-change = { $talker }, en { $format }
connect-change-cuts-off = { $talker }, en { $format }, en coupant { $count ->
        [one] son autre listener
       *[other] ses { $count } autres listeners
    }

# The listener chosen: how its input takes its stream. A reservation is
# the bandwidth the network sets aside for a stream.
network-status = État
network-not-connected = Non connecté
network-from = Depuis { $stream }
network-from-receiving = Depuis { $stream }, réception en cours
network-from-waiting = Depuis { $stream }, en attente du talker
network-from-failed = Depuis { $stream }, la réservation du talker a échoué : { $reason }
# Whether the listener's media clock follows the stream's.
network-media-clock = Horloge média
network-locked = Verrouillée sur le flux
network-not-locked = Non verrouillée
network-format = Format
# The formats both the listener and plaa's stream can be in.
network-formats = Formats qu’il peut recevoir de plaa
# A stream's ID, as in 0xd111e597f5440000.
network-stream-id = ID du flux
# The MAC address the stream is sent to.
network-destination = Destination
# How long the stream's frames take to reach the listener, as the
# switches on the way add it up.
network-latency = Latence
network-microseconds = { $time } µs
network-frames = Trames reçues
# Frames that came with a sequence number other than the next, as when
# some were lost.
network-out-of-sequence = Trames hors séquence
# How often the stream stopped coming while it was taken.
network-interrupted = Interruptions
# Frames that came after, or long before, their time to be played.
network-late = Trames en retard
network-early = Trames en avance
network-entity-id = ID d’entité

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = bande passante insuffisante
msrp-failure-2 = ressources du pont insuffisantes
msrp-failure-3 = bande passante insuffisante pour la classe de trafic
msrp-failure-4 = ID de flux déjà utilisé par un autre talker
msrp-failure-5 = adresse de destination déjà utilisée
msrp-failure-6 = préempté par un flux de rang supérieur
msrp-failure-7 = la latence signalée a changé
msrp-failure-8 = le port de sortie n’est pas compatible AVB
msrp-failure-9 = utiliser une autre adresse de destination
msrp-failure-10 = ressources MSRP épuisées
msrp-failure-11 = ressources MMRP épuisées
msrp-failure-12 = impossible de stocker l’adresse de destination
msrp-failure-13 = la priorité n’est pas une priorité de classe SR
msrp-failure-14 = trames trop grandes pour le support
msrp-failure-15 = limite de ports fan-in atteinte
msrp-failure-16 = première valeur modifiée pour un flux enregistré
msrp-failure-17 = VLAN bloqué sur le port de sortie
msrp-failure-18 = étiquetage VLAN désactivé sur le port de sortie
msrp-failure-19 = priorité de classe SR non concordante
msrp-failure-unknown = raison inconnue
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, au pont { $bridge }

## The macOS menu bar

menu-about = À propos de plaa
menu-settings = Réglages…
menu-services = Services
menu-hide = Masquer plaa
menu-hide-others = Masquer les autres
menu-show-all = Tout afficher
menu-quit = Quitter plaa
menu-file = Fichier
menu-open = Ouvrir…
menu-open-address = Ouvrir une adresse…
menu-close = Fermer la fenêtre
menu-edit = Édition
menu-cut = Couper
menu-copy = Copier
menu-paste = Coller
menu-select-all = Tout sélectionner
menu-undo = Annuler
menu-redo = Rétablir
menu-trim = Raccourcir…
menu-split = Scinder le clip
menu-rotate-left = Faire pivoter vers la gauche
menu-rotate-right = Faire pivoter vers la droite
menu-flip-horizontal = Retourner horizontalement
menu-flip-vertical = Retourner verticalement
menu-revert = Revenir à l’original
menu-export = Exporter…
menu-view = Présentation
menu-playback = Lecture
menu-play-pause = Lire ou mettre en pause
menu-to-start = Aller au début
menu-previous-chapter = Chapitre précédent
menu-next-chapter = Chapitre suivant
menu-faster = Plus vite
menu-slower = Moins vite
menu-normal-speed = Vitesse normale
menu-ab-loop = Boucle entre deux points
menu-volume-up = Plus fort
menu-volume-down = Moins fort
menu-mute = Couper ou rétablir le son
menu-next-subtitles = Sous-titres suivants
menu-next-audio = Piste audio suivante
menu-window = Fenêtre
menu-minimize = Placer dans le Dock
menu-zoom = Réduire/agrandir
menu-bring-all-to-front = Tout ramener au premier plan

## Editing

action-edit = Modifier
title-edited = { $name } (modifié)
edit-trim = Raccourcir
edit-trim-apply = Raccourcir
edit-split = Scinder le clip
edit-remove-clip = Supprimer le clip
edit-done = Terminé
edit-rotate-left = Faire pivoter vers la gauche
edit-rotate-right = Faire pivoter vers la droite
edit-flip-horizontal = Retourner horizontalement
edit-flip-vertical = Retourner verticalement
edit-undo = Annuler
edit-redo = Rétablir
edit-revert = Revenir à l’original
edit-export = Exporter…

## Exporting

export-title = Exporter
export-format = Format
export-size = Taille
export-h264 = H.264
export-hevc = HEVC
export-av1 = AV1
export-audio-only = Son seul
export-size-original = { $lines }p, tel quel
export-on-graphics-card = Encodé sur la carte graphique ({ $api })
export-on-processor = Encodé sur le processeur, ce qui prend plus de temps
export-no-encoder = Aucun encodeur pour ce format ne s’ouvre ici
export-checking = Recherche d’un encodeur…
export-sound-format = Format du son
export-quality = Qualité
export-aac = AAC
export-mp3 = MP3
export-opus = Opus
export-flac = FLAC
export-alac = ALAC
export-wav = WAV
export-aac-note = Compressé ; se lit presque partout, y compris sur les appareils Apple
export-mp3-note = Compressé ; se lit partout
export-opus-note = Compressé ; le meilleur son pour sa taille, mais tous les lecteurs ne le prennent pas en charge
export-flac-note = Sans perte : le son exactement tel qu’il est, dans un format ouvert
export-alac-note = Sans perte : le son exactement tel qu’il est, comme le lisent les appareils Apple
export-wav-note = Non compressé : le son exactement tel qu’il est, dans les fichiers les plus volumineux
export-kilobits = { $rate } kb/s
export-bits = { $bits } bits
export-rate-lower = Enregistré à { $rate } kHz, en dessous des { $file } kHz du fichier
export-choose = Exporter…
export-name = { $name } exporté
export-name-edited = { $name } modifié
export-progress = Exportation de « { $name } »
export-stop = Arrêter l’exportation
notice-exported = « { $name } » exporté
notice-export-stopped = Exportation arrêtée
notice-export-failed = Impossible d’exporter : { $error }
close-title = Fermer sans exporter ?
close-unexported = Les modifications de « { $name } » seront perdues si vous ne les exportez pas.
close-exporting = L’exportation de « { $name } » s’arrête, et ce qu’elle a déjà écrit est perdu.
close-anyway = Fermer
close-quit-anyway = Quitter

## Recording

record-title = Enregistrement sonore
record-file-name = Enregistrement sonore
record-start = Enregistrer
record-stop = Arrêter l’enregistrement
record-no-input = Aucun micro
record-microphone = Micro
record-default-microphone = Celui par défaut du système
record-playing = Ce que lit l’ordinateur
record-mono = mono
record-stereo = stéréo
record-channels = { $count ->
    [one] { $count } canal
   *[other] { $count } canaux
}
# The input's sample rate and channels, as "48 kHz · stereo".
record-input-format = { $rate } kHz · { $channels }
record-input-format-lowered = { $rate } kHz · { $channels }, enregistré à { $saved } kHz
action-record-sound = Enregistrer le son
open-record-sound = Nouvel enregistrement sonore
start-record-sound = Enregistrer le son
menu-record-sound = Nouvel enregistrement sonore
record-screen-title = Enregistrement de l’écran
record-screen-file-name = Enregistrement de l’écran
record-screen-whole = Écran
record-screen-number = Écran { $number }
record-screen-main = Écran { $number } (principal)
record-screens = Écrans
record-windows = Fenêtres
record-screen-what = Source de l’enregistrement
# The most pictures a second a screen recording keeps, as "60 fps".
record-rate = { $rate } fps
record-rate-every = Chaque image
record-rate-tip = Images par seconde
# The recording's size, and the pictures it dropped as the encoder fell
# behind, as "12.3 MB · 4 dropped".
record-dropped = { $size } · { $count ->
    [one] { $count } image perdue
   *[other] { $count } images perdues
}
record-screen-window = Fenêtre
# What is shared and its size, as "Whole screen · 3840 × 2160".
record-screen-size = { $what } · { $width } × { $height }
record-screen-ended = N’est plus partagé
record-screen-again = Choisir à nouveau
record-screen-failed = Impossible d’enregistrer l’écran
record-sound = Son
record-no-sound = Pas de son
# How much louder the input is taken, for a quiet microphone.
record-gain = Plus fort
record-gain-value = +{ $decibels } dB
record-level-automatically = Ajuster le niveau automatiquement
record-hide-title = plaa se masque pendant l’enregistrement
record-hide-tray = Pour arrêter l’enregistrement, cliquez sur le point rouge que plaa place dans la zone de notification, ou rouvrez plaa.
record-hide-reopen = Pour arrêter l’enregistrement, rouvrez plaa.
record-hide-start = Commencer l’enregistrement
record-tray-title = plaa enregistre l’écran
record-tray-tip = Cliquez pour arrêter l’enregistrement
action-record-screen = Enregistrer l’écran
open-record-screen = Nouvel enregistrement de l’écran
start-record-screen = Enregistrer l’écran
menu-record-screen = Nouvel enregistrement de l’écran
record-camera-title = Enregistrement de la caméra
record-camera-file-name = Enregistrement de la caméra
record-camera = Caméra
record-camera-failed = Impossible d’ouvrir la caméra
record-name-title = Nommer l’enregistrement
record-name-note = Il est enregistré sous « { $name } » dans { $folder }.
record-name-label = Nom
record-name-keep = Garder
record-name-rename = Renommer
record-name-taken = Il y a déjà un fichier nommé « { $name } » à cet endroit.
record-name-invalid = Un nom ne peut contenir aucun de ces caractères : / \ : * ? " < > |
record-name-failed = Impossible de renommer : { $error }
action-record-camera = Enregistrer la caméra
open-record-camera = Nouvel enregistrement de la caméra
start-record-camera = Enregistrer la caméra
menu-record-camera = Nouvel enregistrement de la caméra
notice-no-camera = Aucune caméra à enregistrer
notice-recorded = « { $name } » enregistré dans { $folder }
notice-record-failed = Impossible d’enregistrer : { $error }

## Outside control: AI agents, through plaa --mcp

settings-allow-outside-control = Autoriser le contrôle externe
settings-allow-outside-control-note = Les agents IA comme Claude Code peuvent voir ce qui est lu dans plaa et le contrôler, via plaa --mcp. plaa demande avant chaque nouvel agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Autorisés : { $agents }
settings-forget-agents = Oublier
settings-ask-before-note = Demander avant qu’un agent :
settings-ask-reading = Voit ce qui est lu, ou l’image
settings-ask-playing = Contrôle la lecture, le son ou le plein écran
settings-ask-opening = Ouvre un fichier ou ferme une fenêtre
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Autoriser { $agent } à contrôler plaa ?
agent-prompt-body = { $agent } demande à utiliser le contrôle externe de plaa, pour voir ce qui est lu et le contrôler. Vous pouvez désactiver le contrôle externe dans les Réglages.
agent-prompt-allow = Autoriser
agent-prompt-deny = Ne pas autoriser
# The prompt before an agent's tool runs; $agent is the agent's name.
agent-ask-read = Autoriser { $agent } à voir ce qui est lu ici ?
agent-ask-play = Autoriser { $agent } à contrôler la lecture ici ?
agent-ask-open = Autoriser { $agent } à ouvrir ou fermer une fenêtre ?
# $tool is the name of what the agent asks to do, such as Seek.
agent-ask-body = { $agent } demande à utiliser « { $tool } ». Dans les Réglages, vous choisissez ce que plaa demande.

## Casting to a TV or speaker on the network

cast-looking = Recherche de téléviseurs et d’enceintes…
cast-none = Aucun téléviseur, aucune enceinte ni aucun Chromecast n’a été trouvé.
# $renderer is the TV's or speaker's name.
cast-stop = Arrêter la lecture sur { $renderer }
notice-casting = Lecture sur { $renderer }.
notice-cast-failed = Impossible de lire sur { $renderer } : { $reason }
cast-web-page = Les pages web ne peuvent pas encore être castées : ouvrez plutôt l’adresse de la vidéo elle-même.

## The assistant panel, and its models in Settings

assistant-add-another = Ajouter un modèle…
assistant-add-model = L’assistant a besoin d’un modèle : un modèle dans le cloud avec sa clé API, ou un modèle local.
assistant-ask = Posez une question sur ce qui est lu
# $model is the model's name, as in assistant-problem-context.
assistant-compacted = La discussion devenait trop longue pour que { $model } la lise d’un coup, son début a donc été résumé. L’assistant regardera peut-être à nouveau ce qu’il a vu.
assistant-compacting = Résumé du début de la discussion
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } sur cet ordinateur
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } sur { $host }
assistant-new-chat = Nouvelle discussion
assistant-no-model = Ajoutez d’abord un modèle dans les Réglages.
assistant-open-settings = Ajouter un modèle
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = La conversation dépasse ce que { $model } peut lire en une fois. Commencez une nouvelle discussion, ou choisissez un modèle capable d’en lire davantage.
assistant-problem-credit = { $provider } indique que le compte n’a pas de crédit. Un nouveau compte doit en acheter sur le site { $provider } avant que sa clé fonctionne ; réessayez ensuite.
assistant-problem-key = { $provider } a refusé la clé API. Vérifiez-la dans les Réglages.
assistant-problem-model = { $provider } n’a aucun modèle nommé { $model }. Vérifiez son nom dans les Réglages.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } n’a pas répondu : { $message }
assistant-problem-rate = { $provider } demande de ralentir. Réessayez dans un instant.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } demande de ralentir : { $message }
assistant-problem-refused = { $model } a refusé de répondre.
assistant-problem-unavailable = { $provider } est occupé ou rencontre des problèmes. Réessayez dans un instant.
assistant-problem-unreachable = plaa n’a pas pu joindre { $provider }. Vérifiez votre connexion, ou que le serveur est bien lancé.
assistant-running = Exécution…
assistant-send = Envoyer
assistant-shortened = La discussion était trop longue pour que { $model } la lise d’un coup, les résultats précédents des outils ont donc été retirés. L’assistant regardera peut-être à nouveau ce qu’il a vu.
assistant-stop = Arrêter
assistant-stopped = Arrêté.
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Vous discutez maintenant avec { $model }.
assistant-thinking = Réflexion…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Réflexion
assistant-title = Assistant
settings-assistant-add = Ajouter un modèle
settings-assistant-add-button = Ajouter
# $model is the model's name.
settings-assistant-added = { $model } a répondu et a été ajouté.
settings-assistant-added-tag = Ajouté
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adresse, par exemple { $example }
settings-assistant-change-key = Changer de clé
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Serveur compatible avec OpenAI
# The menu of how much a local model reads at once.
settings-assistant-context = Contexte
settings-assistant-context-note = Un contexte plus grand permet à l’assistant de garder en tête une plus grande partie d’une discussion, mais le modèle prend plus de mémoire et peut répondre plus lentement.
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } k tokens
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Ouvrir la page
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } fonctionne à l’adresse { $address }.
settings-assistant-found-none = { $provider } n’a encore aucun modèle. Téléchargez-en un avec { $provider }, puis cherchez à nouveau.
settings-assistant-get-key = Obtenir une clé API
settings-assistant-get-server = Obtenir { $provider }
settings-assistant-in-use = Utilisé
settings-assistant-key = Clé API
# $error is what the keychain said.
settings-assistant-key-failed = La clé n’a pas pu être conservée dans le trousseau : { $error }
settings-assistant-key-kept = Votre clé { $provider } est conservée dans le trousseau du système.
settings-assistant-key-needed = Ce modèle nécessite une clé API.
settings-assistant-key-refused = { $provider } a refusé cette clé. Vérifiez qu’elle a été copiée en entier, depuis le bon compte.
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Créez une clé sur le site { $provider } et collez-la ici.
settings-assistant-look-again = Chercher à nouveau
settings-assistant-looking = Recherche des modèles…
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modèle, par exemple { $example }
settings-assistant-no-server = plaa n’a trouvé aucun { $provider } en cours d’exécution sur cet ordinateur. Lancez-le, ou indiquez son adresse ci-dessous.
settings-assistant-no-tools = Ne peut pas utiliser d’outils, ce dont l’assistant a besoin
settings-assistant-none = Aucun modèle pour l’instant. Ajoutez-en un ci-dessous : un modèle local, comme ceux d’Ollama, reste sur cet ordinateur ; un modèle dans le cloud nécessite une clé API de son fournisseur.
settings-assistant-note = Les modèles avec lesquels le panneau de l’assistant peut discuter. Les clés sont conservées dans le trousseau du système.
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Utiliser une autre adresse
settings-assistant-recommended = Recommandé
settings-assistant-remove = Supprimer
settings-assistant-sees = Voit les images
settings-assistant-trying = Essai…
settings-assistant-use = Utiliser
settings-assistant-use-key = Continuer
settings-assistant-uses-tools = Utilise des outils
