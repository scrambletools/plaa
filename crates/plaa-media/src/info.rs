//! What a file holds: its streams, length, tags and chapters.

use std::time::Duration;

use ffmpeg::media::Type;
use ffmpeg_next as ffmpeg;

use crate::ffi;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaInfo {
    /// The container, such as "Matroska / WebM".
    pub format: String,
    pub duration: Option<Duration>,
    /// Bits per second over the whole file.
    pub bit_rate: Option<u64>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    /// Every tag, as the file names it.
    pub tags: Vec<(String, String)>,
    pub chapters: Vec<Chapter>,
    pub video: Vec<VideoTrack>,
    pub audio: Vec<AudioTrack>,
    pub subtitles: Vec<SubtitleTrack>,
    /// The versions to choose between, when the source offers several, as
    /// an HLS stream's qualities: the tallest picture first.
    pub variants: Vec<Variant>,
    /// Whether the source can seek, which live streams cannot.
    pub seekable: bool,
}

impl MediaInfo {
    /// Whether the file has a moving picture, rather than only cover art.
    pub fn has_video(&self) -> bool {
        self.video.iter().any(|track| !track.cover)
    }

    /// The cover art's stream, if the file has one.
    pub fn cover(&self) -> Option<&VideoTrack> {
        self.video.iter().find(|track| track.cover)
    }

    /// The picture's track: stream `index`, or the first that is not a
    /// cover.
    pub fn picture(&self, index: Option<usize>) -> Option<&VideoTrack> {
        let pictures = || self.video.iter().filter(|track| !track.cover);
        index
            .and_then(|index| pictures().find(|track| track.track.index == index))
            .or_else(|| pictures().next())
    }

    /// The variant picture stream `video` belongs to, if the source has
    /// variants.
    pub fn variant_of(&self, video: usize) -> Option<&Variant> {
        self.variants.iter().find(|variant| variant.video == video)
    }

    /// The sound tracks that go with picture stream `video`: its variant's
    /// own, else every one.
    pub fn audio_with(&self, video: Option<usize>) -> Vec<&AudioTrack> {
        let variant = video.and_then(|video| self.variant_of(video));
        let own: Vec<_> = self
            .audio
            .iter()
            .filter(|track| {
                variant.is_some_and(|variant| variant.streams.contains(&track.track.index))
            })
            .collect();
        if own.is_empty() {
            self.audio.iter().collect()
        } else {
            own
        }
    }

    /// The tag `key`, in any case.
    pub fn tag(&self, key: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value.as_str())
    }
}

/// What every kind of stream has.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Track {
    /// The stream's index in the file.
    pub index: usize,
    /// The codec's short name, such as "h264" or "opus".
    pub codec: String,
    /// The codec's full name, such as "H.264 / AVC / MPEG-4 AVC".
    pub codec_name: String,
    /// The ISO 639 language code the file gives.
    pub language: Option<String>,
    pub title: Option<String>,
    pub default: bool,
    pub forced: bool,
    pub bit_rate: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VideoTrack {
    pub track: Track,
    pub width: u32,
    pub height: u32,
    /// Frames per second, when the file says.
    pub frame_rate: Option<f64>,
    /// Width over height as shown, with the pixels' own shape.
    pub aspect: f64,
    /// Clockwise quarter turns to show it upright.
    pub quarter_turns: u8,
    /// A still picture, such as an album cover, not a video.
    pub cover: bool,
    pub pixel_format: String,
    /// High dynamic range: PQ or HLG light.
    pub hdr: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioTrack {
    pub track: Track,
    pub sample_rate: u32,
    pub channels: u16,
    /// Such as "stereo" or "5.1(side)".
    pub layout: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubtitleTrack {
    pub track: Track,
    /// Text subtitles, rather than pictures such as DVD and Blu-ray ones.
    pub text: bool,
    /// The file beside the video the subtitles come from, rather than the
    /// video's own.
    pub file: Option<std::path::PathBuf>,
}

/// One version of the source to choose, as one of an HLS stream's
/// qualities.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// Its picture's stream.
    pub video: usize,
    pub width: u32,
    pub height: u32,
    /// The bits per second it needs, when the source says.
    pub bit_rate: Option<u64>,
    /// Every stream it plays, its sound among them.
    pub streams: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chapter {
    pub title: Option<String>,
    pub start: Duration,
    pub end: Duration,
}

/// Seconds in `time_base` units, when given.
pub(crate) fn seconds(value: i64, time_base: ffmpeg::Rational) -> Option<f64> {
    (value != ffmpeg::ffi::AV_NOPTS_VALUE && time_base.denominator() != 0)
        .then(|| value as f64 * f64::from(time_base))
}

fn duration_from_seconds(seconds: f64) -> Option<Duration> {
    (seconds.is_finite() && seconds >= 0.0).then(|| Duration::from_secs_f64(seconds))
}

fn non_empty(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

impl MediaInfo {
    pub(crate) fn read(input: &ffmpeg::format::context::Input) -> Self {
        let tags: Vec<(String, String)> = input
            .metadata()
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect();
        let tag = |key: &str| {
            non_empty(
                tags.iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(key))
                    .map(|(_, value)| value.as_str()),
            )
        };
        let duration = (input.duration() > 0)
            .then(|| input.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE))
            .and_then(duration_from_seconds);
        let chapters = input
            .chapters()
            .filter_map(|chapter| {
                let time_base = chapter.time_base();
                Some(Chapter {
                    title: non_empty(chapter.metadata().get("title")),
                    start: duration_from_seconds(seconds(chapter.start(), time_base)?)?,
                    end: duration_from_seconds(seconds(chapter.end(), time_base)?)?,
                })
            })
            .collect();
        let mut info = MediaInfo {
            format: input.format().description().to_owned(),
            duration,
            bit_rate: (input.bit_rate() > 0).then_some(input.bit_rate() as u64),
            title: tag("title"),
            artist: tag("artist").or_else(|| tag("album_artist")),
            album: tag("album"),
            tags: tags.clone(),
            chapters,
            seekable: seekable(input),
            ..MediaInfo::default()
        };
        for stream in input.streams() {
            let parameters = stream.parameters();
            let codec = ffmpeg::codec::decoder::find(parameters.id());
            let metadata = stream.metadata();
            let disposition = stream.disposition();
            let rate = parameters_bit_rate(&stream);
            let track = Track {
                index: stream.index(),
                codec: parameters.id().name().to_owned(),
                codec_name: codec
                    .map(|codec| codec.description().to_owned())
                    .unwrap_or_default(),
                language: non_empty(metadata.get("language")).filter(|code| code != "und"),
                title: non_empty(metadata.get("title")).or_else(|| {
                    non_empty(metadata.get("handler_name")).filter(|name| !is_generic_handler(name))
                }),
                default: disposition.contains(ffmpeg::format::stream::Disposition::DEFAULT),
                forced: disposition.contains(ffmpeg::format::stream::Disposition::FORCED),
                bit_rate: rate,
            };
            match parameters.medium() {
                Type::Video => {
                    let Ok(context) = ffmpeg::codec::Context::from_parameters(parameters) else {
                        continue;
                    };
                    let Ok(video) = context.decoder().video() else {
                        continue;
                    };
                    let (width, height) = (video.width(), video.height());
                    let sample_aspect = stream_sample_aspect(&stream, video.aspect_ratio());
                    let aspect = if height > 0 {
                        f64::from(width) * sample_aspect / f64::from(height)
                    } else {
                        1.0
                    };
                    let frame_rate = Some(stream.avg_frame_rate())
                        .filter(|rate| rate.numerator() > 0 && rate.denominator() > 0)
                        .map(f64::from);
                    let cover =
                        disposition.contains(ffmpeg::format::stream::Disposition::ATTACHED_PIC);
                    info.video.push(VideoTrack {
                        track,
                        width,
                        height,
                        frame_rate: if cover { None } else { frame_rate },
                        aspect,
                        quarter_turns: ffi::quarter_turns(&stream),
                        cover,
                        pixel_format: video
                            .format()
                            .descriptor()
                            .map(|descriptor| descriptor.name().to_owned())
                            .unwrap_or_default(),
                        hdr: ffi::is_hdr(&stream),
                    });
                }
                Type::Audio => {
                    let Ok(context) = ffmpeg::codec::Context::from_parameters(parameters) else {
                        continue;
                    };
                    let Ok(audio) = context.decoder().audio() else {
                        continue;
                    };
                    info.audio.push(AudioTrack {
                        track,
                        sample_rate: audio.rate(),
                        channels: audio.channels(),
                        layout: layout_name(ffi::decoder_layout(&audio), audio.channels()),
                    });
                }
                Type::Subtitle => {
                    let text = codec.is_some_and(|codec| is_text_subtitle(codec.id()));
                    info.subtitles.push(SubtitleTrack {
                        track,
                        text,
                        file: None,
                    });
                }
                _ => {}
            }
        }
        info.variants = variants(&info, ffi::programs(input));
        info
    }
}

/// The programs that each have a picture of their own, as the variants to
/// choose between, the tallest first; none when there is no choice.
fn variants(info: &MediaInfo, programs: Vec<(Vec<usize>, Option<u64>)>) -> Vec<Variant> {
    let mut variants: Vec<Variant> = programs
        .into_iter()
        .filter_map(|(streams, bit_rate)| {
            let picture = info
                .video
                .iter()
                .find(|track| !track.cover && streams.contains(&track.track.index))?;
            Some(Variant {
                video: picture.track.index,
                width: picture.width,
                height: picture.height,
                bit_rate: bit_rate.or(picture.track.bit_rate),
                streams,
            })
        })
        .collect();
    variants.sort_by(|first, second| {
        (second.height, second.bit_rate).cmp(&(first.height, first.bit_rate))
    });
    variants.dedup_by_key(|variant| variant.video);
    if variants.len() < 2 {
        variants.clear();
    }
    variants
}

/// Container handler names that say nothing about the track, such as
/// MP4's "SoundHandler".
fn is_generic_handler(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    [
        "handler",
        "media",
        "core media",
        "isomedia",
        "gpac",
        "mainconcept",
        "apple",
    ]
    .iter()
    .any(|generic| lower.contains(generic))
}

fn parameters_bit_rate(stream: &ffmpeg::format::stream::Stream) -> Option<u64> {
    #[allow(unsafe_code)]
    // SAFETY: the parameters are valid while the stream is borrowed.
    let rate = unsafe { (*stream.parameters().as_ptr()).bit_rate };
    (rate > 0).then_some(rate as u64)
}

/// The pixels' width over their height: the stream's, else the codec's.
fn stream_sample_aspect(stream: &ffmpeg::format::stream::Stream, codec: ffmpeg::Rational) -> f64 {
    #[allow(unsafe_code)]
    // SAFETY: a valid stream while borrowed.
    let from_stream = unsafe { (*stream.as_ptr()).sample_aspect_ratio };
    let usable = |numerator: i32, denominator: i32| numerator > 0 && denominator > 0;
    if usable(from_stream.num, from_stream.den) {
        f64::from(from_stream.num) / f64::from(from_stream.den)
    } else if usable(codec.numerator(), codec.denominator()) {
        f64::from(codec)
    } else {
        1.0
    }
}

pub(crate) fn layout_name(described: String, channels: u16) -> String {
    if described.is_empty() || described.contains("channels") {
        match channels {
            1 => "mono".to_owned(),
            2 => "stereo".to_owned(),
            channels => format!("{channels} channels"),
        }
    } else {
        described
    }
}

/// Whether the source can seek: files can, live streams and pipes not.
fn seekable(input: &ffmpeg::format::context::Input) -> bool {
    #[allow(unsafe_code)]
    // SAFETY: a valid format context; its `pb` is null for formats that
    // do their own I/O.
    unsafe {
        let context = input.as_ptr();
        let io = (*context).pb;
        io.is_null() || (*io).seekable != 0
    }
}

fn is_text_subtitle(id: ffmpeg::codec::Id) -> bool {
    use ffmpeg::codec::Id;
    matches!(
        id,
        Id::SSA
            | Id::ASS
            | Id::SUBRIP
            | Id::SRT
            | Id::TEXT
            | Id::WEBVTT
            | Id::MOV_TEXT
            | Id::MICRODVD
            | Id::SAMI
            | Id::REALTEXT
            | Id::SUBVIEWER
            | Id::SUBVIEWER1
            | Id::JACOSUB
            | Id::MPL2
            | Id::PJS
            | Id::VPLAYER
            | Id::STL
            | Id::TTML
            | Id::EIA_608
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_with_their_own_pictures_are_variants() {
        let video = |index, height| VideoTrack {
            track: Track {
                index,
                ..Track::default()
            },
            width: height * 16 / 9,
            height,
            ..VideoTrack::default()
        };
        let info = MediaInfo {
            video: vec![video(1, 720), video(3, 1080)],
            ..MediaInfo::default()
        };
        let found = variants(
            &info,
            vec![(vec![0, 1], Some(2_000_000)), (vec![2, 3], Some(6_000_000))],
        );
        assert_eq!(found.len(), 2);
        assert_eq!((found[0].video, found[0].height), (3, 1080));
        assert_eq!(found[1].streams, vec![0, 1]);
        // One program is no choice.
        assert!(variants(&info, vec![(vec![0, 1, 2, 3], None)]).is_empty());
    }

    #[test]
    fn generic_handler_names_are_not_titles() {
        assert!(is_generic_handler("SoundHandler"));
        assert!(is_generic_handler("Core Media Video"));
        assert!(!is_generic_handler("Director's commentary"));
    }
}
