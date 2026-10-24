//! Reading parts of a file, as edited, for agents: its pictures at given
//! times, its sound as samples, and its subtitles as text. Each read
//! opens the files on its own, apart from the player.

use std::path::{Path, PathBuf};

use ffmpeg::format::Pixel;
use ffmpeg::software::scaling;
use ffmpeg_next as ffmpeg;

use crate::decode::{self, Picture};
use crate::edit::{Edit, Feed, OWN, Track};
use crate::ffi;
use crate::info::{self, MediaInfo};
use crate::subtitle::Cue;

/// The file edited, and what of it plays.
#[derive(Debug, Clone)]
pub struct Edited<'a> {
    pub path: &'a Path,
    pub edit: Option<&'a Edit>,
    /// Its picture and sound streams that play.
    pub video: Option<usize>,
    pub audio: Option<usize>,
}

impl Edited<'_> {
    /// The file `source` of the edit is, and the stream of `track` it
    /// plays, when it is the file edited.
    fn file(&self, source: usize, track: Track) -> Option<(PathBuf, Option<usize>)> {
        if source == OWN {
            let stream = match track {
                Track::Picture => self.video,
                Track::Sound => self.audio,
            };
            return Some((self.path.to_path_buf(), stream));
        }
        let other = self.edit?.other(source)?;
        let has = match track {
            Track::Picture => other.picture,
            Track::Sound => other.sound,
        };
        has.then(|| (other.path.clone(), None))
    }

    /// The filters `track` goes through at `seconds` of the edited time.
    fn chain_at(&self, track: Track, seconds: f64) -> Option<String> {
        let edit = self.edit?;
        let (index, _) = edit.clip_at(seconds)?;
        let filters = edit.filters_of(&edit.clips()[index])?;
        match track {
            Track::Picture => filters.picture.clone(),
            Track::Sound => filters.sound.clone(),
        }
    }

    /// Where the picture or sound at `seconds` of the edited time comes
    /// from: a file and its time there, or none.
    fn shown_at(&self, track: Track, seconds: f64) -> Option<(usize, f64)> {
        match self.edit {
            Some(edit) => edit.fed_at(track, seconds),
            None => Some((OWN, seconds)),
        }
    }
}

/// The pictures at `times` of the edited time, turned and flipped as they
/// show, at most `side` pixels on their long side: none where it is black
/// or a file will not read. `progress` hears the share done as it goes.
pub fn pictures(
    edited: &Edited,
    times: &[f64],
    side: u32,
    progress: &dyn Fn(f64),
) -> Vec<Option<Picture>> {
    crate::init();
    let transform = edited.edit.map(Edit::transform).unwrap_or_default();
    let mut found = vec![None; times.len()];
    // Each file once, its times in order, with the filters there.
    let mut wanted: Vec<(usize, Wanted, usize)> = times
        .iter()
        .enumerate()
        .filter_map(|(slot, seconds)| {
            let (source, at) = edited.shown_at(Track::Picture, *seconds)?;
            let chain = edited.chain_at(Track::Picture, *seconds);
            Some((source, (at, chain.map(|chain| (chain, *seconds))), slot))
        })
        .collect();
    wanted.sort_by(|first, second| {
        first
            .0
            .cmp(&second.0)
            .then(first.1.0.total_cmp(&second.1.0))
    });
    for group in wanted.chunk_by(|first, second| first.0 == second.0) {
        let source = group[0].0;
        let Some((path, stream)) = edited.file(source, Track::Picture) else {
            continue;
        };
        let at: Vec<Wanted> = group.iter().map(|(_, wanted, _)| wanted.clone()).collect();
        let done = std::cell::Cell::new(0usize);
        let read = || {
            done.set(done.get() + 1);
            progress(done.get() as f64 / times.len().max(1) as f64);
        };
        match pictures_of(&path, stream, &at, side, &read) {
            Ok(pictures) => {
                for ((_, _, slot), picture) in group.iter().zip(pictures) {
                    found[*slot] = picture
                        .map(|(picture, turns)| picture.transformed(transform.after_turns(turns)));
                }
            }
            Err(error) => eprintln!("plaa: pictures of {}: {error}", path.display()),
        }
    }
    found
}

/// A picture wanted: its file's time, and the filters it goes through,
/// with its edited time, which they count in.
type Wanted = (f64, Option<(String, f64)>);

/// The pictures of `path`'s stream `stream`, or its first, at `times` of
/// it, in order, through their filters; each with the quarter turns it
/// still needs, as filtered pictures come upright.
fn pictures_of(
    path: &Path,
    stream: Option<usize>,
    times: &[Wanted],
    side: u32,
    read: &dyn Fn(),
) -> Result<Vec<Option<(Picture, u8)>>, String> {
    let mut input = ffmpeg::format::input(path).map_err(|error| error.to_string())?;
    let info = MediaInfo::read(&input);
    let index = stream
        .or_else(|| info.picture(None).map(|track| track.track.index))
        .ok_or("it has no picture")?;
    let found = input.stream(index).ok_or("its picture is gone")?;
    let turns = ffi::quarter_turns(&found);
    let frame_duration = 1.0 / decode::frame_rate(&input, index);
    for other in 0..input.nb_streams() as usize {
        ffi::set_skipped(&mut input, other, other != index);
    }
    let start = decode::start_seconds(&input);
    let (decoder, time_base) =
        decode::open_decoder(&input, index).map_err(|error| error.to_string())?;
    let mut decoder = decoder.video().map_err(|error| error.to_string())?;
    let mut pictures = Vec::with_capacity(times.len());
    // The last picture decoded, and its time.
    let mut last: Option<(f64, ffmpeg::frame::Video)> = None;
    let mut ended = false;
    let mut packet = ffmpeg::Packet::empty();
    let rate = Some(((1000.0 / frame_duration).round() as i32, 1000));
    for (time, chain) in times {
        let time = *time;
        let far = last
            .as_ref()
            .is_none_or(|(at, _)| time < *at - frame_duration / 2.0 || time > *at + 4.0);
        if far {
            let timestamp = ((time + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
            let _ = input.seek(timestamp, ..timestamp);
            decoder.flush();
            last = None;
            ended = false;
        }
        // Decodes on until the picture after `time`'s shows, keeping the
        // one showing at `time`.
        let mut showing: Option<ffmpeg::frame::Video> = last
            .as_ref()
            .filter(|(at, _)| *at <= time + frame_duration / 2.0)
            .map(|(_, frame)| frame.clone());
        let mut done = last
            .as_ref()
            .is_some_and(|(at, _)| *at > time + frame_duration / 2.0);
        while !done {
            let mut decoded = ffmpeg::frame::Video::empty();
            if decoder.receive_frame(&mut decoded).is_ok() {
                let at = decoded
                    .timestamp()
                    .or(decoded.pts())
                    .and_then(|timestamp| info::seconds(timestamp, time_base))
                    .map_or(time, |seconds| seconds - start);
                if at <= time + frame_duration / 2.0 {
                    showing = Some(decoded.clone());
                } else {
                    done = true;
                }
                last = Some((at, decoded));
                continue;
            }
            if ended {
                break;
            }
            match packet.read(&mut input) {
                Ok(()) => {
                    if packet.stream() == index {
                        let _ = decoder.send_packet(&packet);
                    }
                }
                Err(ffmpeg::Error::InvalidData) => {}
                Err(_) => {
                    let _ = decoder.send_eof();
                    ended = true;
                }
            }
        }
        // Through the filters there, alone: they give it upright.
        let filtered = match (showing, chain) {
            (Some(frame), Some((chain, edited))) => {
                let mut filter = crate::filter::PictureFilter::default();
                match filter.run(chain, turns, rate, &frame, *edited) {
                    Some(mut out) => {
                        out.extend(filter.flush());
                        out.pop().map(|(frame, _)| (frame, 0))
                    }
                    None => Some((frame, turns)),
                }
            }
            (showing, _) => showing.map(|frame| (frame, turns)),
        };
        pictures.push(filtered.and_then(|(frame, turns)| Some((rgba(&frame, side)?, turns))));
        read();
    }
    Ok(pictures)
}

/// `frame` as RGBA, at most `side` pixels on its long side, its pixels
/// square.
fn rgba(frame: &ffmpeg::frame::Video, side: u32) -> Option<Picture> {
    let (width, height) = (frame.width(), frame.height());
    if width == 0 || height == 0 {
        return None;
    }
    let sample = frame.aspect_ratio();
    let across = if sample.numerator() > 0 && sample.denominator() > 0 {
        f64::from(width) * f64::from(sample)
    } else {
        f64::from(width)
    };
    let scale = (f64::from(side) / across.max(f64::from(height))).min(1.0);
    let (out_width, out_height) = (
        ((across * scale).round() as u32).max(1),
        ((f64::from(height) * scale).round() as u32).max(1),
    );
    let mut scaler = scaling::Context::get(
        frame.format(),
        width,
        height,
        Pixel::RGBA,
        out_width,
        out_height,
        scaling::Flags::BICUBIC,
    )
    .ok()?;
    let mut out = ffmpeg::frame::Video::empty();
    scaler.run(frame, &mut out).ok()?;
    let stride = out.stride(0);
    let row = out_width as usize * 4;
    let mut rgba = Vec::with_capacity(row * out_height as usize);
    for line in out.data(0).chunks(stride).take(out_height as usize) {
        rgba.extend_from_slice(&line[..row.min(line.len())]);
    }
    Some(Picture {
        width: out_width,
        height: out_height,
        rgba,
    })
}

/// `picture` as a JPEG of `quality`, from 2, the best, to 31.
pub fn jpeg(picture: &Picture, quality: u8) -> Result<Vec<u8>, String> {
    crate::init();
    let failed = |error: ffmpeg::Error| error.to_string();
    let codec = ffmpeg::encoder::find(ffmpeg::codec::Id::MJPEG).ok_or("no JPEG encoder")?;
    let (width, height) = (picture.width & !1, picture.height & !1);
    if width == 0 || height == 0 {
        return Err("the picture is empty".to_owned());
    }
    let mut encoder = ffmpeg::codec::Context::new_with_codec(codec)
        .encoder()
        .video()
        .map_err(failed)?;
    encoder.set_width(width);
    encoder.set_height(height);
    encoder.set_format(Pixel::YUVJ420P);
    encoder.set_time_base(ffmpeg::Rational::new(1, 25));
    let quality = quality.clamp(2, 31).to_string();
    let mut options = ffmpeg::Dictionary::new();
    options.set("qmin", &quality);
    options.set("qmax", &quality);
    let mut encoder = encoder.open_as_with(codec, options).map_err(failed)?;
    let mut source = ffmpeg::frame::Video::new(Pixel::RGBA, picture.width, picture.height);
    let stride = source.stride(0);
    let row = picture.width as usize * 4;
    for (line, pixels) in source
        .data_mut(0)
        .chunks_mut(stride)
        .zip(picture.rgba.chunks(row))
    {
        line[..row].copy_from_slice(pixels);
    }
    let mut scaler = scaling::Context::get(
        Pixel::RGBA,
        picture.width,
        picture.height,
        Pixel::YUVJ420P,
        width,
        height,
        scaling::Flags::BICUBIC,
    )
    .map_err(failed)?;
    let mut frame = ffmpeg::frame::Video::empty();
    scaler.run(&source, &mut frame).map_err(failed)?;
    frame.set_pts(Some(0));
    encoder.send_frame(&frame).map_err(failed)?;
    encoder.send_eof().map_err(failed)?;
    let mut packet = ffmpeg::Packet::empty();
    encoder.receive_packet(&mut packet).map_err(failed)?;
    Ok(packet.data().unwrap_or_default().to_vec())
}

/// The picture in the JPEG `bytes`, at most 1280 pixels on its long side.
pub fn jpeg_picture(bytes: &[u8]) -> Option<Picture> {
    crate::init();
    let codec = ffmpeg::decoder::find(ffmpeg::codec::Id::MJPEG)?;
    let mut decoder = ffmpeg::codec::Context::new_with_codec(codec)
        .decoder()
        .video()
        .ok()?;
    decoder.send_packet(&ffmpeg::Packet::copy(bytes)).ok()?;
    let _ = decoder.send_eof();
    let mut frame = ffmpeg::frame::Video::empty();
    decoder.receive_frame(&mut frame).ok()?;
    rgba(&frame, 1280)
}

/// Hands `each` the sound from `from` to `to` seconds of the edited time,
/// mixed to one channel at `rate`, in order: silence where there is none.
pub fn sound(
    edited: &Edited,
    from: f64,
    to: f64,
    rate: u32,
    each: &mut dyn FnMut(&[f32]),
) -> Result<(), String> {
    crate::init();
    // Sound the edit mixes in joins what is read, as it comes.
    let mut layers = crate::layer::Layers::new(edited.edit, Some(edited.path), rate, 1);
    let mut at = from;
    let step = 1.0 / f64::from(rate);
    let mut mixed = |chunk: &[f32]| {
        if layers.is_empty() {
            each(chunk);
        } else {
            let mut chunk = chunk.to_vec();
            layers.add(&mut chunk, at, step, 1.0);
            each(&chunk);
        }
        at += chunk.len() as f64 * step;
    };
    let each: &mut dyn FnMut(&[f32]) = &mut mixed;
    let parts = sound_parts(edited, from, to);
    let mut written = 0usize;
    let total = ((to - from).max(0.0) * f64::from(rate)).round() as usize;
    for (at, length, file, chain) in parts {
        // Where the part starts, in samples, past what came before.
        let first = (((at - from) * f64::from(rate)).round() as usize).min(total);
        let last = (((at + length - from) * f64::from(rate)).round() as usize).min(total);
        if first > written {
            silence(first - written, each);
            written = first;
        }
        let wanted = last.saturating_sub(written);
        if wanted == 0 {
            continue;
        }
        let got = match file {
            Some((path, stream, file_from)) => {
                let skip = (written - first) as f64 / f64::from(rate);
                // The filters count in the edited time.
                let filters = chain.as_deref().map(|chain| (chain, at - file_from));
                sound_of(
                    &path,
                    stream,
                    file_from + skip,
                    (wanted, rate),
                    filters,
                    each,
                )?
            }
            None => 0,
        };
        if got < wanted {
            silence(wanted - got, each);
        }
        written += wanted;
    }
    if total > written {
        silence(total - written, each);
    }
    Ok(())
}

/// The parts of the sound from `from` to `to`: where each starts in the
/// edited time, how long it lasts, the file, its stream and its time it
/// plays from, if any, and the filters it goes through.
type SoundPart = (
    f64,
    f64,
    Option<(PathBuf, Option<usize>, f64)>,
    Option<String>,
);

fn sound_parts(edited: &Edited, from: f64, to: f64) -> Vec<SoundPart> {
    let Some(edit) = edited.edit else {
        return vec![(
            from,
            to - from,
            edited
                .file(OWN, Track::Sound)
                .map(|(path, stream)| (path, stream, from)),
            None,
        )];
    };
    let mut parts = Vec::new();
    for (clip, start) in edit.clips().iter().zip(edit.starts()) {
        let end = start + clip.length();
        let (low, high) = (from.max(start), to.min(end));
        if high <= low {
            continue;
        }
        let file = match clip.sound_feed() {
            Feed::File { source, from: feed } => edited
                .file(source, Track::Sound)
                .map(|(path, stream)| (path, stream, feed + (low - start))),
            Feed::Blank => None,
        };
        let chain = edit
            .filters_of(clip)
            .and_then(|filters| filters.sound.clone());
        parts.push((low, high - low, file, chain));
    }
    parts
}

fn silence(count: usize, each: &mut dyn FnMut(&[f32])) {
    let zeros = vec![0.0; count.min(8192)];
    let mut left = count;
    while left > 0 {
        let take = left.min(zeros.len());
        each(&zeros[..take]);
        left -= take;
    }
}

/// Hands `each` up to `count` samples of `path`'s sound, stream `stream`
/// or its first, from `from` seconds of it, through `filters` when given
/// with what turns its time into the edited time, mixed to one channel at
/// `rate`. Returns how many it had.
fn sound_of(
    path: &Path,
    stream: Option<usize>,
    from: f64,
    (count, rate): (usize, u32),
    filters: Option<(&str, f64)>,
    each: &mut dyn FnMut(&[f32]),
) -> Result<usize, String> {
    let failed = |error: ffmpeg::Error| error.to_string();
    let mut input = ffmpeg::format::input(path).map_err(failed)?;
    let info = MediaInfo::read(&input);
    let index = stream
        .or_else(|| info.audio.first().map(|track| track.track.index))
        .ok_or("it has no sound")?;
    for other in 0..input.nb_streams() as usize {
        ffi::set_skipped(&mut input, other, other != index);
    }
    let start = decode::start_seconds(&input);
    let timestamp = ((from + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
    let _ = input.seek(timestamp, ..timestamp);
    let (decoder, time_base) = decode::open_decoder(&input, index).map_err(failed)?;
    let mut decoder = decoder.audio().map_err(failed)?;
    let mut graph: Option<ffmpeg::filter::Graph> = None;
    let mut handed = 0usize;
    let mut packet = ffmpeg::Packet::empty();
    let mut ended = false;
    // The file's time of the next sample out of the graph.
    let mut cursor: Option<f64> = None;
    let mut filter = crate::filter::SoundFilter::default();
    let mut flushed = false;
    // Hands sound starting at the file's time `at` to the graph.
    let feed = |frame: &ffmpeg::frame::Audio,
                at: Option<f64>,
                graph: &mut Option<ffmpeg::filter::Graph>,
                cursor: &mut Option<f64>|
     -> Result<(), String> {
        if graph.is_none() {
            *graph = Some(mono_graph(frame, rate).map_err(failed)?);
        }
        if cursor.is_none() {
            *cursor = at;
        }
        if let Some(graph) = graph
            && let Some(mut source) = graph.get("in")
        {
            let _ = source.source().add(frame);
        }
        Ok(())
    };
    while handed < count {
        let mut decoded = ffmpeg::frame::Audio::empty();
        if decoder.receive_frame(&mut decoded).is_ok() {
            let at = decoded
                .timestamp()
                .or(decoded.pts())
                .and_then(|timestamp| info::seconds(timestamp, time_base))
                .map(|seconds| seconds - start);
            let filtered = filters
                .zip(at)
                .and_then(|((chain, shift), at)| filter.run(chain, &decoded, at + shift));
            match (filtered, filters) {
                (Some(filtered), Some((_, shift))) => {
                    for (frame, edited) in filtered {
                        feed(&frame, Some(edited - shift), &mut graph, &mut cursor)?;
                    }
                }
                _ => feed(&decoded, at, &mut graph, &mut cursor)?,
            }
        } else if ended {
            // What the filters hold first, then what the graph holds.
            if !flushed {
                flushed = true;
                if let Some((_, shift)) = filters {
                    for (frame, edited) in filter.flush() {
                        feed(&frame, Some(edited - shift), &mut graph, &mut cursor)?;
                    }
                }
            }
            if let Some(graph) = &mut graph
                && let Some(mut source) = graph.get("in")
            {
                let _ = source.source().flush();
            }
        } else {
            match packet.read(&mut input) {
                Ok(()) => {
                    if packet.stream() == index {
                        let _ = decoder.send_packet(&packet);
                    }
                }
                Err(ffmpeg::Error::InvalidData) => {}
                Err(_) => {
                    let _ = decoder.send_eof();
                    ended = true;
                }
            }
            continue;
        }
        let Some(graph) = &mut graph else {
            if ended {
                break;
            }
            continue;
        };
        let mut out = ffmpeg::frame::Audio::empty();
        let mut any = false;
        while let Some(mut sink) = graph.get("out")
            && sink.sink().frame(&mut out).is_ok()
        {
            any = true;
            let samples = out.samples();
            let data = &out.data(0)[..(samples * 4).min(out.data(0).len())];
            let mut values: Vec<f32> = data
                .as_chunks::<4>()
                .0
                .iter()
                .map(|bytes| f32::from_ne_bytes(*bytes))
                .collect();
            // Sound before `from`, as a seek lands on a packet before it.
            let at = cursor.unwrap_or(from);
            cursor = Some(at + values.len() as f64 / f64::from(rate));
            if at < from {
                let skip = (((from - at) * f64::from(rate)).round() as usize).min(values.len());
                values.drain(..skip);
            }
            let take = values.len().min(count - handed);
            if take > 0 {
                each(&values[..take]);
                handed += take;
            }
            if handed >= count {
                break;
            }
        }
        if ended && !any {
            break;
        }
    }
    Ok(handed)
}

/// abuffer, then one channel of floats at `rate`, mixed down without
/// going past full scale.
fn mono_graph(
    frame: &ffmpeg::frame::Audio,
    rate: u32,
) -> Result<ffmpeg::filter::Graph, ffmpeg::Error> {
    let mut graph = ffmpeg::filter::Graph::new();
    let arguments = format!(
        "time_base=1/{rate_in}:sample_rate={rate_in}:sample_fmt={}:channel_layout={}",
        frame.format().name(),
        ffi::frame_layout(frame),
        rate_in = frame.rate(),
    );
    let abuffer = ffmpeg::filter::find("abuffer").ok_or(ffmpeg::Error::FilterNotFound)?;
    let abuffersink = ffmpeg::filter::find("abuffersink").ok_or(ffmpeg::Error::FilterNotFound)?;
    graph.add(&abuffer, "in", &arguments)?;
    graph.add(&abuffersink, "out", "")?;
    graph.output("in", 0)?.input("out", 0)?.parse(&format!(
        "aresample={rate}:rematrix_maxval=1,aformat=sample_fmts=flt:sample_rates={rate}:channel_layouts=mono"
    ))?;
    graph.validate()?;
    Ok(graph)
}

/// The text of subtitle stream `stream` of the file at `path`, or of the
/// subtitle file at `path` when `stream` is none, in the file's time.
/// `progress` hears how far through the file it has read.
pub fn subtitle_text(
    path: &Path,
    stream: Option<usize>,
    progress: &dyn Fn(f64),
) -> Result<Vec<Cue>, String> {
    crate::init();
    let Some(index) = stream else {
        return decode::read_subtitle_file(path)
            .map(|(_, cues)| cues)
            .ok_or_else(|| "the file has no subtitles plaa reads".to_owned());
    };
    let mut input = ffmpeg::format::input(path).map_err(|error| error.to_string())?;
    for other in 0..input.nb_streams() as usize {
        ffi::set_skipped(&mut input, other, other != index);
    }
    let start = decode::start_seconds(&input);
    let mut decoder = decode::SubtitleDecoder::new(&input, index, start, (0, 0))
        .map_err(|error| error.to_string())?;
    let mut cues = crate::subtitle::Cues::default();
    let mut pictures = false;
    let size = std::fs::metadata(path).map_or(0, |metadata| metadata.len()) as f64;
    for (found, packet) in input.packets() {
        if found.index() != index {
            continue;
        }
        if size > 0.0 && packet.position() > 0 {
            progress(packet.position() as f64 / size);
        }
        for decoded in decoder.decode(&packet) {
            match decoded {
                decode::Decoded::Text(cue) => cues.add(cue),
                decode::Decoded::Picture(_) => pictures = true,
                decode::Decoded::Clear(_) => {}
            }
        }
    }
    let cues = cues.into_vec();
    if cues.is_empty() && pictures {
        return Err("these subtitles are pictures, not text".to_owned());
    }
    Ok(cues)
}

/// `cues`, in the file edited's time, as they show in the edit: those in
/// parts cut out left out, and each cut to the clips it shows in.
pub fn edited_cues(edit: Option<&Edit>, cues: &[Cue]) -> Vec<Cue> {
    let Some(edit) = edit else {
        return cues.to_vec();
    };
    let mut shown = Vec::new();
    for (clip, start) in edit.clips().iter().zip(edit.starts()) {
        if clip.source != OWN {
            continue;
        }
        for cue in cues {
            let (low, high) = (cue.start.max(clip.from), cue.end.min(clip.to));
            if high > low {
                shown.push(Cue {
                    start: start + (low - clip.from),
                    end: start + (high - clip.from),
                    text: cue.text.clone(),
                });
            }
        }
    }
    shown.sort_by(|first, second| first.start.total_cmp(&second.start));
    // A line going on over a cut between clips that carry on one another
    // is one line.
    let mut merged: Vec<Cue> = Vec::with_capacity(shown.len());
    for cue in shown {
        match merged.last_mut() {
            Some(last) if last.text == cue.text && (cue.start - last.end).abs() < 1e-6 => {
                last.end = cue.end;
            }
            _ => merged.push(cue),
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edit::Clip;

    #[test]
    fn cues_follow_the_edit() {
        let cue = |start: f64, end: f64, text: &str| Cue {
            start,
            end,
            text: text.to_owned(),
        };
        let cues = [
            cue(1.0, 2.0, "one"),
            cue(5.0, 7.0, "two"),
            cue(12.0, 13.0, "three"),
        ];
        let mut edit = Edit::whole(20.0);
        edit.cut(4.0, 6.0);
        let shown = edited_cues(Some(&edit), &cues);
        let starts: Vec<_> = shown
            .iter()
            .map(|cue| (cue.text.as_str(), cue.start, cue.end))
            .collect();
        assert_eq!(
            starts,
            [("one", 1.0, 2.0), ("two", 4.0, 5.0), ("three", 10.0, 11.0)]
        );
        // Split, a line is still one.
        let mut edit = Edit::whole(20.0);
        edit.split(5.5);
        assert_eq!(edited_cues(Some(&edit), &cues).len(), 3);
        // A clip from elsewhere has none of the file's.
        let mut edit = Edit::whole(20.0);
        edit.insert(
            0.0,
            Clip {
                source: 1,
                ..Clip::new(0.0, 3.0)
            },
        );
        assert_eq!(edited_cues(Some(&edit), &cues)[0].start, 4.0);
    }
}
