//! Filters an edit puts the picture or the sound through: FFmpeg filter
//! graphs, as `-vf` and `-af` take them, checked before they run. Only
//! filters on a list of those that change frames one by one may be used,
//! none given a file, folder, address or model, so a graph can neither
//! read nor write files, reach a device or the network, nor change when
//! things play; trimming and cutting are the edit's own.

use ffmpeg::format::Pixel;
use ffmpeg_next as ffmpeg;

use crate::edit::Transform;
use crate::ffi;

/// The filters a picture may go through.
const PICTURE: &[&str] = &[
    "alphaextract",
    "alphamerge",
    "amplify",
    "atadenoise",
    "avgblur",
    "backgroundkey",
    "bilateral",
    "blend",
    "boxblur",
    "bwdif",
    "cas",
    "chromahold",
    "chromakey",
    "chromanr",
    "chromashift",
    "colorbalance",
    "colorchannelmixer",
    "colorcontrast",
    "colorcorrect",
    "colorhold",
    "colorize",
    "colorkey",
    "colorlevels",
    "colormatrix",
    "colorspace",
    "colortemperature",
    "convolution",
    "crop",
    "curves",
    "dblur",
    "dctdnoiz",
    "deband",
    "deblock",
    "dedot",
    "deflate",
    "deflicker",
    "delogo",
    "deshake",
    "despill",
    "dilation",
    "drawbox",
    "drawgrid",
    "drawtext",
    "edgedetect",
    "elbg",
    "epx",
    "eq",
    "erosion",
    "estdif",
    "exposure",
    "fade",
    "fftdnoiz",
    "fftfilt",
    "fillborders",
    "floodfill",
    "format",
    "gblur",
    "geq",
    "gradfun",
    "grayworld",
    "greyedge",
    "hflip",
    "histeq",
    "hqdn3d",
    "hqx",
    "hstack",
    "hsvhold",
    "hsvkey",
    "hue",
    "huesaturation",
    "inflate",
    "kirsch",
    "lagfun",
    "lenscorrection",
    "limiter",
    "lumakey",
    "lut",
    "lutrgb",
    "lutyuv",
    "maskedmerge",
    "maskfun",
    "median",
    "monochrome",
    "negate",
    "nlmeans",
    "noise",
    "normalize",
    "null",
    "overlay",
    "owdenoise",
    "pad",
    "perspective",
    "photosensitivity",
    "pixelize",
    "prewitt",
    "pseudocolor",
    "removegrain",
    "rgbashift",
    "roberts",
    "rotate",
    "sab",
    "scale",
    "scharr",
    "scroll",
    "selectivecolor",
    "setdar",
    "setparams",
    "setrange",
    "setsar",
    "shear",
    "smartblur",
    "sobel",
    "split",
    "super2xsai",
    "swapuv",
    "tblend",
    "tmedian",
    "tmix",
    "tonemap",
    "transpose",
    "unsharp",
    "vaguedenoiser",
    "vflip",
    "vibrance",
    "vignette",
    "vstack",
    "xbr",
    "xstack",
    "yadif",
    "yaepblur",
    "zscale",
];

/// The filters sound may go through.
const SOUND: &[&str] = &[
    "acompressor",
    "acontrast",
    "acrusher",
    "adeclick",
    "adeclip",
    "adecorrelate",
    "adelay",
    "adenorm",
    "adrc",
    "adynamicequalizer",
    "adynamicsmooth",
    "aecho",
    "aemphasis",
    "aeval",
    "aexciter",
    "afade",
    "afftdn",
    "afftfilt",
    "afreqshift",
    "afwtdn",
    "agate",
    "aformat",
    "alimiter",
    "allpass",
    "amerge",
    "amix",
    "anlmdn",
    "anull",
    "aphaser",
    "aphaseshift",
    "apsyclip",
    "apulsator",
    "aresample",
    "asoftclip",
    "asplit",
    "asubboost",
    "asubcut",
    "asupercut",
    "asuperpass",
    "asuperstop",
    "atilt",
    "bandpass",
    "bandreject",
    "bass",
    "biquad",
    "bs2b",
    "channelmap",
    "chorus",
    "compand",
    "compensationdelay",
    "crossfeed",
    "crystalizer",
    "dcshift",
    "deesser",
    "dialoguenhance",
    "dynaudnorm",
    "earwax",
    "equalizer",
    "extrastereo",
    "firequalizer",
    "flanger",
    "haas",
    "highpass",
    "highshelf",
    "join",
    "loudnorm",
    "lowpass",
    "lowshelf",
    "mcompand",
    "pan",
    "speechnorm",
    "stereotools",
    "stereowiden",
    "superequalizer",
    "surround",
    "tiltshelf",
    "treble",
    "tremolo",
    "vibrato",
    "virtualbass",
    "volume",
];

/// Whether an option's name says it takes a file, a folder, an address
/// or a model, which no filter here may be given.
fn reaches_out(option: &str) -> bool {
    let option = option.to_ascii_lowercase();
    ["file", "path", "url", "model", "dir"]
        .iter()
        .any(|part| option.contains(part))
}

/// What a graph of pictures takes: their size, pixel format and shape,
/// and their frame rate, if known.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PictureInput {
    pub width: u32,
    pub height: u32,
    pub format: Pixel,
    pub sample: (i32, i32),
    pub rate: Option<(i32, i32)>,
}

impl PictureInput {
    /// Pictures as `track`'s are.
    pub fn of_track(track: &crate::info::VideoTrack) -> Self {
        let (width, height) = (track.width.max(1), track.height.max(1));
        // The pixels' shape, from the shape shown.
        let sample = if track.aspect.is_finite() && track.aspect > 0.0 {
            track.aspect * f64::from(height) / f64::from(width)
        } else {
            1.0
        };
        Self {
            width,
            height,
            format: ffi::pixel_named(&track.pixel_format).unwrap_or(Pixel::YUV420P),
            sample: ((sample * 1000.0).round().max(1.0) as i32, 1000),
            rate: track
                .frame_rate
                .filter(|rate| *rate > 0.0)
                .map(|rate| ((rate * 1000.0).round() as i32, 1000)),
        }
    }

    fn of(frame: &ffmpeg::frame::Video, rate: Option<(i32, i32)>) -> Self {
        let sample = frame.aspect_ratio();
        Self {
            width: frame.width(),
            height: frame.height(),
            format: frame.format(),
            sample: if sample.numerator() > 0 && sample.denominator() > 0 {
                (sample.numerator(), sample.denominator())
            } else {
                (1, 1)
            },
            rate,
        }
    }
}

/// Pictures' time base in their graphs: microseconds.
const PICTURE_BASE: i32 = 1_000_000;

/// Builds `chain` between a buffer for pictures as `input` says, stamped
/// in microseconds, and a sink, and checks it.
fn picture_graph(chain: &str, input: PictureInput) -> Result<ffmpeg::filter::Graph, String> {
    let failed = |error: ffmpeg::Error| error.to_string();
    let mut graph = ffmpeg::filter::Graph::new();
    let mut arguments = format!(
        "video_size={}x{}:pix_fmt={}:time_base=1/{PICTURE_BASE}:pixel_aspect={}/{}",
        input.width.max(1),
        input.height.max(1),
        input
            .format
            .descriptor()
            .map_or("yuv420p", |descriptor| descriptor.name()),
        input.sample.0,
        input.sample.1,
    );
    if let Some((numerator, denominator)) = input.rate {
        arguments.push_str(&format!(":frame_rate={numerator}/{denominator}"));
    }
    let buffer = ffmpeg::filter::find("buffer").ok_or("no buffer filter")?;
    let sink = ffmpeg::filter::find("buffersink").ok_or("no buffersink filter")?;
    graph.add(&buffer, "in", &arguments).map_err(failed)?;
    graph.add(&sink, "out", "").map_err(failed)?;
    ffi::parse_checked(&mut graph, chain, &|name, options| {
        allowed(name, options, PICTURE)
    })?;
    graph
        .validate()
        .map_err(|error| format!("the filters do not fit together: {error}"))?;
    Ok(graph)
}

/// Builds `chain` between a buffer for sound of `format`, `rate` Hz and
/// channel `layout`, stamped in samples, and a sink, and checks it.
fn sound_graph(
    chain: &str,
    (format, rate, layout): (ffmpeg::format::Sample, u32, &str),
) -> Result<ffmpeg::filter::Graph, String> {
    let failed = |error: ffmpeg::Error| error.to_string();
    let mut graph = ffmpeg::filter::Graph::new();
    let arguments = format!(
        "time_base=1/{rate}:sample_rate={rate}:sample_fmt={}:channel_layout={layout}",
        format.name(),
    );
    let buffer = ffmpeg::filter::find("abuffer").ok_or("no abuffer filter")?;
    let sink = ffmpeg::filter::find("abuffersink").ok_or("no abuffersink filter")?;
    graph.add(&buffer, "in", &arguments).map_err(failed)?;
    graph.add(&sink, "out", "").map_err(failed)?;
    ffi::parse_checked(&mut graph, chain, &|name, options| {
        allowed(name, options, SOUND)
    })?;
    graph
        .validate()
        .map_err(|error| format!("the filters do not fit together: {error}"))?;
    Ok(graph)
}

/// Whether filter `name`, given string `options`, may run: it is on
/// `list`, and given no file, folder, address or model.
fn allowed(name: &str, options: &[(String, String)], list: &[&str]) -> Result<(), String> {
    if !list.contains(&name) {
        return Err(format!(
            "plaa does not run the {name} filter: only filters that change each frame as it \
             comes, and reach no file, device or network, are allowed"
        ));
    }
    if let Some((option, _)) = options.iter().find(|(option, _)| reaches_out(option)) {
        return Err(format!(
            "plaa does not let the {name} filter use its {option} option, which reaches a file \
             or the network"
        ));
    }
    Ok(())
}

/// Checks `chain` for pictures as `input` says, which their own `turns`
/// put upright first. Says why it cannot run, if it cannot.
pub fn check_picture(chain: &str, turns: u8, input: PictureInput) -> Result<(), String> {
    crate::init();
    picture_graph(&with_turns(chain, turns), input).map(drop)
}

/// Checks `chain` for sound of `rate` Hz in `channels` channels.
pub fn check_sound(chain: &str, rate: u32, channels: u16) -> Result<(), String> {
    crate::init();
    let layout = ffi::default_layout(i32::from(channels.max(1)));
    sound_graph(
        chain,
        (
            ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Planar),
            rate.max(1),
            &layout,
        ),
    )
    .map(drop)
}

/// The size and pixel shape of pictures as `input` says once through
/// `chain`, turned first by their own `turns`.
pub fn picture_size(chain: &str, turns: u8, input: PictureInput) -> Option<(u32, u32, (i32, i32))> {
    crate::init();
    let chain = with_turns(chain, turns);
    let mut graph = picture_graph(&chain, input).ok()?;
    let mut sink = graph.get("out")?;
    Some(ffi::sink_picture(&mut sink))
}

/// The filters that mirror and turn a picture as `transform` says.
pub(crate) fn turn_filters(transform: Transform) -> Vec<&'static str> {
    let mut filters = Vec::new();
    if transform.mirrored {
        filters.push("hflip");
    }
    match transform.turns % 4 {
        1 => filters.push("transpose=clock"),
        2 => filters.push("hflip,vflip"),
        3 => filters.push("transpose=cclock"),
        _ => {}
    }
    filters
}

/// `chain` after the picture's own `turns`, so the filters see it
/// upright.
pub(crate) fn with_turns(chain: &str, turns: u8) -> String {
    let turns = turn_filters(Transform {
        turns,
        mirrored: false,
    });
    if turns.is_empty() {
        chain.to_owned()
    } else {
        format!("{},{chain}", turns.join(","))
    }
}

/// Pictures through a chain, its graph built again when the pictures or
/// the chain change.
#[derive(Default)]
pub(crate) struct PictureFilter {
    graph: Option<((String, PictureInput), ffmpeg::filter::Graph)>,
}

impl PictureFilter {
    /// Puts `frame`, at `seconds`, through `chain`, upright first by its
    /// own `turns`; returns the pictures that come out, each with its
    /// time. None when the chain will not run on such pictures.
    pub fn run(
        &mut self,
        chain: &str,
        turns: u8,
        rate: Option<(i32, i32)>,
        frame: &ffmpeg::frame::Video,
        seconds: f64,
    ) -> Option<Vec<(ffmpeg::frame::Video, f64)>> {
        let chain = with_turns(chain, turns);
        let key = (chain, PictureInput::of(frame, rate));
        let mut out = Vec::new();
        if self.graph.as_ref().is_none_or(|(built, _)| *built != key) {
            out.extend(self.flush());
            match picture_graph(&key.0, key.1) {
                Ok(graph) => self.graph = Some((key, graph)),
                Err(error) => {
                    eprintln!("plaa: picture filters: {error}");
                    return None;
                }
            }
        }
        let (_, graph) = self.graph.as_mut()?;
        let mut stamped = frame.clone();
        stamped.set_pts(Some((seconds * f64::from(PICTURE_BASE)).round() as i64));
        graph.get("in")?.source().add(&stamped).ok()?;
        out.extend(pull_pictures(graph));
        Some(out)
    }

    /// What the graph still holds, its input ended.
    pub fn flush(&mut self) -> Vec<(ffmpeg::frame::Video, f64)> {
        let Some((_, mut graph)) = self.graph.take() else {
            return Vec::new();
        };
        if let Some(mut source) = graph.get("in") {
            let _ = source.source().flush();
        }
        pull_pictures(&mut graph)
    }

    /// Drops the graph and what it holds, as after a seek.
    pub fn reset(&mut self) {
        self.graph = None;
    }
}

fn pull_pictures(graph: &mut ffmpeg::filter::Graph) -> Vec<(ffmpeg::frame::Video, f64)> {
    let mut out = Vec::new();
    let Some(mut sink) = graph.get("out") else {
        return out;
    };
    let base = f64::from(sink.sink().time_base());
    loop {
        let mut frame = ffmpeg::frame::Video::empty();
        if sink.sink().frame(&mut frame).is_err() {
            break;
        }
        let seconds = frame.pts().map_or(0.0, |pts| pts as f64 * base);
        out.push((frame, seconds));
    }
    out
}

/// Sound through a chain, its graph built again when the sound or the
/// chain change.
#[derive(Default)]
pub(crate) struct SoundFilter {
    graph: Option<(
        (String, ffmpeg::format::Sample, u32, String),
        ffmpeg::filter::Graph,
    )>,
}

impl SoundFilter {
    /// Puts `frame`, its first sample at `seconds`, through `chain`;
    /// returns the sound that comes out, each frame with its time. None
    /// when the chain will not run on such sound.
    pub fn run(
        &mut self,
        chain: &str,
        frame: &ffmpeg::frame::Audio,
        seconds: f64,
    ) -> Option<Vec<(ffmpeg::frame::Audio, f64)>> {
        let key = (
            chain.to_owned(),
            frame.format(),
            frame.rate(),
            ffi::frame_layout(frame),
        );
        let mut out = Vec::new();
        if self.graph.as_ref().is_none_or(|(built, _)| *built != key) {
            out.extend(self.flush());
            match sound_graph(&key.0, (key.1, key.2, &key.3)) {
                Ok(graph) => self.graph = Some((key, graph)),
                Err(error) => {
                    eprintln!("plaa: sound filters: {error}");
                    return None;
                }
            }
        }
        let (_, graph) = self.graph.as_mut()?;
        let mut stamped = frame.clone();
        stamped.set_pts(Some((seconds * f64::from(frame.rate())).round() as i64));
        graph.get("in")?.source().add(&stamped).ok()?;
        out.extend(pull_sound(graph));
        Some(out)
    }

    /// What the graph still holds, its input ended.
    pub fn flush(&mut self) -> Vec<(ffmpeg::frame::Audio, f64)> {
        let Some((_, mut graph)) = self.graph.take() else {
            return Vec::new();
        };
        if let Some(mut source) = graph.get("in") {
            let _ = source.source().flush();
        }
        pull_sound(&mut graph)
    }

    /// Drops the graph and what it holds, as after a seek.
    pub fn reset(&mut self) {
        self.graph = None;
    }
}

fn pull_sound(graph: &mut ffmpeg::filter::Graph) -> Vec<(ffmpeg::frame::Audio, f64)> {
    let mut out = Vec::new();
    let Some(mut sink) = graph.get("out") else {
        return out;
    };
    let base = f64::from(sink.sink().time_base());
    loop {
        let mut frame = ffmpeg::frame::Audio::empty();
        if sink.sink().frame(&mut frame).is_err() {
            break;
        }
        let seconds = frame.pts().map_or(0.0, |pts| pts as f64 * base);
        out.push((frame, seconds));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> PictureInput {
        PictureInput {
            width: 320,
            height: 240,
            format: Pixel::YUV420P,
            sample: (1, 1),
            rate: Some((25, 1)),
        }
    }

    #[test]
    fn filters_that_change_frames_run() {
        assert_eq!(check_picture("hue=s=0", 0, input()), Ok(()));
        assert_eq!(
            check_picture("eq=contrast=1.2:brightness=0.05,unsharp", 1, input()),
            Ok(())
        );
        assert_eq!(
            check_picture("split[a][b];[b]hue=s=0[c];[a][c]hstack", 0, input()),
            Ok(())
        );
        assert_eq!(check_sound("highpass=f=80,loudnorm", 48000, 2), Ok(()));
        assert_eq!(
            picture_size("crop=ih:ih", 0, input()).map(|(width, height, _)| (width, height)),
            Some((240, 240))
        );
        // Turned upright first: a portrait picture's crop is of it upright.
        assert_eq!(
            picture_size("null", 1, input()).map(|(width, height, _)| (width, height)),
            Some((240, 320))
        );
    }

    #[test]
    fn filters_reaching_out_or_changing_time_are_refused() {
        for refused in [
            "movie=/etc/passwd",
            "setpts=2*PTS",
            "fps=10",
            "trim=0:5",
            "reverse",
            "sendcmd=c='0 hue s 0'",
            "subtitles=/etc/passwd",
            "drawtext=textfile=/etc/passwd",
            "curves=psfile=/tmp/a.acv",
        ] {
            assert!(check_picture(refused, 0, input()).is_err(), "{refused}");
        }
        // Positional arguments are checked as the options they set.
        assert!(check_picture("deshake=0:0:-1:-1:16:16:0:8:20:0:/tmp/log", 0, input()).is_err());
        for refused in ["atempo=2", "silenceremove=1", "amovie=/etc/passwd", "afir"] {
            assert!(check_sound(refused, 48000, 2).is_err(), "{refused}");
        }
        assert!(
            check_picture("hue=s=", 0, input()).is_err(),
            "a broken filter"
        );
        assert!(
            check_picture("volume=2", 0, input()).is_err(),
            "sound filters on a picture"
        );
    }
}
