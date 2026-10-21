//! The Network Audio sheet: the listeners on the network that can play
//! what plaa plays, by name, each with one button: play there, where it
//! takes plaa's stream as it is; stop, where it plays it; or, where it is
//! in another format, a menu of the formats both share, to connect in.
//! Under them, how the one chosen takes its stream.

use std::sync::Arc;

use iced::widget::{container, text};
use iced::{Center, Element, Fill, Length};
use plaa_avb::{
    EntityId, Found, Ptp, Snapshot, Stream, StreamFormat, StreamInfo, StreamInfoFlags, TalkerState,
};
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::{Icon, Type, button, column, component, row, style};

use crate::window::{Control, Menu};

/// An entity's stream: the entity, and its output's or input's index.
type End = (EntityId, u16);

fn note<'a>(content: String) -> Element<'a, Control> {
    scramble_ui::aligned(
        scramble_ui::styled(content, Type::BodyMedium).style(style::on_surface_variant),
    )
}

/// A listener that can play plaa's stream: its input, its name in the
/// list, and the formats both can use.
struct Listener<'s> {
    end: End,
    name: String,
    input: &'s Stream,
    shared: Vec<StreamFormat>,
    /// On this computer.
    here: bool,
}

/// The inputs that can take one of plaa's formats, named by their entity,
/// and by the input too where an entity has several.
fn listeners<'s>(snapshot: &'s Snapshot, ours: &Stream) -> Vec<Listener<'s>> {
    let mut found = Vec::new();
    for entity in snapshot.entities.iter().filter(|entity| !entity.own) {
        let able: Vec<&Stream> = entity
            .inputs
            .iter()
            .filter(|input| {
                input
                    .formats
                    .iter()
                    .any(|format| ours.formats.contains(format))
            })
            .collect();
        for input in &able {
            found.push(Listener {
                end: (entity.id, input.index),
                name: name(entity, input, able.len() > 1),
                input,
                shared: ours
                    .formats
                    .iter()
                    .copied()
                    .filter(|format| input.formats.contains(format))
                    .collect(),
                here: entity.here,
            });
        }
    }
    found
}

fn name(entity: &Found, stream: &Stream, several: bool) -> String {
    if several {
        crate::fl!(
            "network-stream",
            entity = entity.name.clone(),
            stream = stream.name.clone()
        )
    } else {
        entity.name.clone()
    }
}

/// What is wrong with ptp4l, where something is.
fn ptp_note(ptp: &Ptp) -> Option<String> {
    Some(match ptp {
        Ptp::Unknown | Ptp::Running => return None,
        Ptp::Starting => crate::fl!("network-ptp-starting"),
        Ptp::Elsewhere => crate::fl!("network-ptp-elsewhere"),
        Ptp::Missing(None) => crate::fl!("network-ptp-missing"),
        Ptp::Missing(Some(why)) => crate::fl!("network-ptp-failed", reason = why.clone()),
    })
}

/// The sheet, `chosen` being the listener whose stream it details.
pub fn sheet<'a>(
    snapshot: Arc<Snapshot>,
    menu: Option<Menu>,
    chosen: Option<End>,
) -> Element<'a, Control> {
    let mut content = column![].spacing(4).width(Fill);
    let own = snapshot.entities.iter().find(|entity| entity.own);
    let ours = own.and_then(|own| Some(((own.id, 0), own.outputs.first()?)));
    // Without ptp4l the stream keeps no network time, which listeners
    // need to play it.
    let sending = !matches!(snapshot.talker, TalkerState::Off | TalkerState::Blocked(_));
    if let (true, Some(problem)) = (sending, ptp_note(&snapshot.ptp)) {
        content = content.push(scramble_ui::aligned(text(problem).style(style::error_text)));
    }
    match (&snapshot.talker, ours) {
        (TalkerState::Off, _) => content = content.push(note(crate::fl!("network-off"))),
        (TalkerState::Blocked(why), _) => {
            content = content.push(note(crate::fl!("network-blocked", reason = why.clone())));
        }
        (_, None) => content = content.push(note(crate::fl!("network-looking"))),
        (_, Some((plaa, output))) => {
            let found = listeners(&snapshot, output);
            if found.is_empty() {
                content = content.push(note(crate::fl!("network-no-listeners")));
            }
            for listener in &found {
                let picked = chosen == Some(listener.end);
                let unserved = listener.here && snapshot.local_unserved;
                content = content.push(row_for(
                    listener,
                    plaa,
                    output.format,
                    menu,
                    picked,
                    unserved,
                ));
            }
            match found.iter().find(|listener| chosen == Some(listener.end)) {
                Some(listener) => content = content.push(details(&snapshot, listener)),
                None if !found.is_empty() => {
                    content = content.push(note(crate::fl!("network-choose")));
                }
                None => {}
            }
        }
    }
    if let Some(failure) = &snapshot.failure {
        content = content.push(scramble_ui::aligned(
            text(crate::fl!("network-failed", reason = failure.clone())).style(style::error_text),
        ));
    }
    component::side_sheet_sized(
        crate::fl!("network-title"),
        Control::ToggleNetwork,
        content,
        Length::Fixed(component::SIDE_SHEET_WIDTH),
    )
}

/// A listener's row: its name, and stop, play, or the formats to play in,
/// chosen to see its details when pressed; and, where it is on this
/// computer and `unserved` by the participant plaa shares, what to do.
fn row_for<'a>(
    listener: &Listener<'_>,
    plaa: End,
    format: StreamFormat,
    menu: Option<Menu>,
    chosen: bool,
    unserved: bool,
) -> Element<'a, Control> {
    let end = listener.end;
    let action: Element<'a, Control> = if listener.input.bound == Some(plaa) {
        component::tool(
            Icon::Stop,
            crate::fl!("network-stop-on", listener = listener.name.clone()),
            Some(Control::AvbDisconnect(end)),
        )
    } else if listener.input.format == format {
        component::tool(
            Icon::PlayArrow,
            crate::fl!("network-play-on", listener = listener.name.clone()),
            Some(Control::AvbConnect(plaa, end, None)),
        )
    } else {
        let entries = listener
            .shared
            .iter()
            .map(|format| {
                Entry::item(
                    format.to_string(),
                    Control::AvbConnect(plaa, end, Some(*format)),
                )
            })
            .collect();
        dropdown::icon_menu(Icon::ChangeCircle, entries)
            .size(button::Size::Small)
            .tip(crate::fl!(
                "network-choose-format",
                listener = listener.name.clone()
            ))
            .open(menu == Some(Menu::AvbFormats(end)), move |open| {
                Control::Menu(Menu::AvbFormats(end), open)
            })
            .into()
    };
    let mut label = column![scramble_ui::aligned(scramble_ui::styled(
        listener.name.clone(),
        Type::BodyLarge
    ))]
    .spacing(2);
    if unserved {
        label = label.push(scramble_ui::aligned(scramble_ui::styled(
            crate::fl!("network-update-triib"),
            Type::BodyMedium,
        )));
    }
    let content = row![container(label).width(Fill), action]
        .spacing(12)
        .align_y(Center);
    iced::widget::button(content)
        .padding(scramble_ui::dir::padding(4.0, 4.0, 4.0, 16.0))
        .width(Fill)
        .style(move |theme, status| crate::window::list_row_style(theme, status, chosen))
        .on_press(Control::AvbChoose(end))
        .into()
}

/// How a listener's input takes its stream: from which talker and how
/// that goes, in what format and what others it shares with plaa's
/// output, and what it counted.
fn details<'a>(snapshot: &Snapshot, listener: &Listener<'_>) -> Element<'a, Control> {
    let field = |label: String, value: String| -> Element<'a, Control> {
        column![
            scramble_ui::styled(label, Type::LabelMedium).style(style::on_surface_variant),
            scramble_ui::aligned(scramble_ui::styled(value, Type::BodyMedium)),
        ]
        .spacing(2)
        .into()
    };
    let input = listener.input;
    let mut content = column![component::section(listener.name.clone())].spacing(6);
    content = content.push(field(crate::fl!("network-status"), status(snapshot, input)));
    if input.bound.is_some()
        && let Some(locked) = media_locked(input)
    {
        let clock = if locked {
            crate::fl!("network-locked")
        } else {
            crate::fl!("network-not-locked")
        };
        content = content.push(field(crate::fl!("network-media-clock"), clock));
    }
    content = content.push(field(
        crate::fl!("network-format"),
        input.format.to_string(),
    ));
    let shared: Vec<String> = listener.shared.iter().map(ToString::to_string).collect();
    content = content.push(field(crate::fl!("network-formats"), shared.join("\n")));
    if let Some(info) = input.info.filter(|_| input.bound.is_some()) {
        // Some entities have it without marking it valid.
        if info.settled() || info.stream_id.0 != 0 {
            content = content.push(field(
                crate::fl!("network-stream-id"),
                info.stream_id.to_string(),
            ));
        }
        if info.flags.contains(StreamInfoFlags::STREAM_DEST_MAC_VALID) {
            content = content.push(field(
                crate::fl!("network-destination"),
                info.stream_dest_mac.to_string(),
            ));
        }
        if info.registering() {
            content = content.push(field(
                crate::fl!("network-latency"),
                crate::fl!(
                    "network-microseconds",
                    time = (info.msrp_accumulated_latency / 1000).to_string()
                ),
            ));
        }
    }
    if let Some(counters) = input.counters {
        if let Some(frames) = counters.frames_rx {
            content = content.push(field(crate::fl!("network-frames"), frames.to_string()));
        }
        if let Some(mismatches) = counters.seq_num_mismatch {
            content = content.push(field(
                crate::fl!("network-out-of-sequence"),
                mismatches.to_string(),
            ));
        }
        if let Some(interrupted) = counters.stream_interrupted {
            content = content.push(field(
                crate::fl!("network-interrupted"),
                interrupted.to_string(),
            ));
        }
    }
    content = content.push(field(
        crate::fl!("network-entity-id"),
        listener.end.0.to_string(),
    ));
    content.into()
}

/// What an input takes, in words: nothing, or a talker's stream, and
/// whether it has the stream, by its stream info, or by its media clock
/// locked to it while it hears the talker's reservation, as some
/// entities never mark the stream's ID valid and some count no lost
/// lock when the talker goes; or whether the talker's reservation failed.
fn status(snapshot: &Snapshot, input: &Stream) -> String {
    let Some(talker) = input.bound else {
        return crate::fl!("network-not-connected");
    };
    let stream = talker_name(snapshot, talker);
    match input.info {
        Some(info) if info.talker_failed() => crate::fl!(
            "network-from-failed",
            stream = stream,
            reason = reservation_failure(&info)
        ),
        Some(info)
            if info.settled() || (info.registering() && media_locked(input) == Some(true)) =>
        {
            crate::fl!("network-from-receiving", stream = stream)
        }
        Some(_) => crate::fl!("network-from-waiting", stream = stream),
        None => crate::fl!("network-from", stream = stream),
    }
}

/// A talker's output by name, as listeners are named, or by its entity's
/// ID where the entity is not found.
fn talker_name(snapshot: &Snapshot, (id, index): End) -> String {
    let Some(entity) = snapshot.entities.iter().find(|entity| entity.id == id) else {
        return id.to_string();
    };
    match entity.outputs.iter().find(|output| output.index == index) {
        Some(output) => name(entity, output, entity.outputs.len() > 1),
        None => entity.name.clone(),
    }
}

/// Whether an input's media clock is locked to its stream, by its
/// counters: when it has locked more often than it lost the lock.
fn media_locked(input: &Stream) -> Option<bool> {
    let counters = input.counters?;
    Some(counters.media_locked? > counters.media_unlocked.unwrap_or(0))
}

/// Why a talker's reservation failed, and at which bridge where it says.
fn reservation_failure(info: &StreamInfo) -> String {
    let reason = msrp_failure(info.msrp_failure_code);
    let [_, _, mac @ ..] = info.msrp_failure_bridge_id.to_be_bytes();
    if mac == [0; 6] {
        reason
    } else {
        crate::fl!(
            "msrp-failure-at",
            reason = reason,
            bridge = plaa_avb::MacAddress(mac).to_string()
        )
    }
}

/// What an MSRP failure code means (IEEE 802.1Q, 35.2.2.8.7).
fn msrp_failure(code: u8) -> String {
    match code {
        1 => crate::fl!("msrp-failure-1"),
        2 => crate::fl!("msrp-failure-2"),
        3 => crate::fl!("msrp-failure-3"),
        4 => crate::fl!("msrp-failure-4"),
        5 => crate::fl!("msrp-failure-5"),
        6 => crate::fl!("msrp-failure-6"),
        7 => crate::fl!("msrp-failure-7"),
        8 => crate::fl!("msrp-failure-8"),
        9 => crate::fl!("msrp-failure-9"),
        10 => crate::fl!("msrp-failure-10"),
        11 => crate::fl!("msrp-failure-11"),
        12 => crate::fl!("msrp-failure-12"),
        13 => crate::fl!("msrp-failure-13"),
        14 => crate::fl!("msrp-failure-14"),
        15 => crate::fl!("msrp-failure-15"),
        16 => crate::fl!("msrp-failure-16"),
        17 => crate::fl!("msrp-failure-17"),
        18 => crate::fl!("msrp-failure-18"),
        19 => crate::fl!("msrp-failure-19"),
        _ => crate::fl!("msrp-failure-unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEREO: StreamFormat = StreamFormat(0x0205_0220_0080_6000);
    const STEREO_24: StreamFormat = StreamFormat(0x0205_0218_0080_6000);
    const EIGHT: StreamFormat = StreamFormat(0x0205_0220_0200_6000);
    const AM824: StreamFormat = StreamFormat(0x00a0_0208_4000_0800);

    fn stream(index: u16, name: &str, format: StreamFormat, formats: &[StreamFormat]) -> Stream {
        Stream {
            index,
            name: name.into(),
            format,
            formats: formats.to_vec(),
            bound: None,
            info: None,
            counters: None,
        }
    }

    fn entity(id: u64, name: &str, inputs: Vec<Stream>) -> Found {
        Found {
            id: EntityId(id),
            name: name.into(),
            own: false,
            here: false,
            outputs: Vec::new(),
            inputs,
        }
    }

    #[test]
    fn only_listeners_that_share_a_format_are_offered() {
        let ours = stream(0, "Playback", STEREO, &[STEREO, STEREO_24, EIGHT]);
        let snapshot = Snapshot {
            entities: vec![
                entity(1, "Amp", vec![stream(0, "In", STEREO, &[STEREO, EIGHT])]),
                entity(2, "Old", vec![stream(0, "In", AM824, &[AM824])]),
                entity(
                    3,
                    "Desk",
                    vec![
                        stream(0, "Main", EIGHT, &[EIGHT]),
                        stream(1, "Aux", STEREO_24, &[STEREO_24]),
                    ],
                ),
            ],
            ..Snapshot::default()
        };
        let found = listeners(&snapshot, &ours);
        let names: Vec<&str> = found
            .iter()
            .map(|listener| listener.name.as_str())
            .collect();
        // The AM824-only one is left out; an entity of several inputs
        // names each.
        assert_eq!(names, ["Amp", "Desk: Main", "Desk: Aux"]);
        assert_eq!(found[0].shared, [STEREO, EIGHT]);
        assert_eq!(found[1].shared, [EIGHT]);
        assert_eq!(found[2].shared, [STEREO_24]);
    }

    /// A listener's stream info, as GET_STREAM_INFO would carry it.
    fn info(flags: StreamInfoFlags, failure: u8, bridge: u64) -> StreamInfo {
        let mut payload = [0u8; 46];
        payload[4..8].copy_from_slice(&flags.bits().to_be_bytes());
        payload[34] = failure;
        payload[36..44].copy_from_slice(&bridge.to_be_bytes());
        StreamInfo::decode(&payload).unwrap()
    }

    #[test]
    fn status_names_the_talker_and_how_its_stream_comes() {
        let mut desk = entity(3, "Desk", Vec::new());
        desk.outputs = vec![
            stream(0, "Main", STEREO, &[STEREO]),
            stream(1, "Aux", STEREO, &[STEREO]),
        ];
        let mut plaa = entity(9, "plaa", Vec::new());
        plaa.outputs = vec![stream(0, "Playback", STEREO, &[STEREO])];
        let snapshot = Snapshot {
            entities: vec![desk, plaa],
            ..Snapshot::default()
        };
        let mut input = stream(0, "In", STEREO, &[STEREO]);
        assert_eq!(status(&snapshot, &input), "Not connected");
        input.bound = Some((EntityId(9), 0));
        assert_eq!(status(&snapshot, &input), "From plaa");
        input.info = Some(info(StreamInfoFlags::CONNECTED, 0, 0));
        assert_eq!(
            status(&snapshot, &input),
            "From plaa, waiting for the talker"
        );
        input.info = Some(info(
            StreamInfoFlags::CONNECTED.union(StreamInfoFlags::STREAM_ID_VALID),
            0,
            0,
        ));
        assert_eq!(status(&snapshot, &input), "From plaa, receiving");
        // Locked to the stream and hearing its reservation, as an entity
        // that leaves the stream's ID unmarked.
        input.counters = Some(plaa_avb::StreamInputCounters {
            media_locked: Some(1),
            ..plaa_avb::StreamInputCounters::default()
        });
        input.info = Some(info(
            StreamInfoFlags::CONNECTED.union(StreamInfoFlags::MSRP_ACC_LAT_VALID),
            0,
            0,
        ));
        assert_eq!(status(&snapshot, &input), "From plaa, receiving");
        // A lock counted before the talker went, its reservation gone.
        input.info = Some(info(StreamInfoFlags::CONNECTED, 0, 0));
        assert_eq!(
            status(&snapshot, &input),
            "From plaa, waiting for the talker"
        );
        input.counters = None;
        // An entity of several outputs names the one, as listeners are.
        input.bound = Some((EntityId(3), 1));
        input.info = Some(info(
            StreamInfoFlags::CONNECTED.union(StreamInfoFlags::MSRP_FAILURE_VALID),
            1,
            0x8000_001b_2100_0001,
        ));
        assert_eq!(
            status(&snapshot, &input),
            "From Desk: Aux, the talker's reservation failed: insufficient bandwidth, \
             at the bridge 00:1b:21:00:00:01"
        );
    }

    #[test]
    fn media_clock_is_locked_while_locks_outnumber_losses() {
        let mut input = stream(0, "In", STEREO, &[STEREO]);
        assert_eq!(media_locked(&input), None);
        let counters = |locked, unlocked| plaa_avb::StreamInputCounters {
            media_locked: locked,
            media_unlocked: unlocked,
            ..plaa_avb::StreamInputCounters::default()
        };
        input.counters = Some(counters(None, Some(1)));
        assert_eq!(media_locked(&input), None);
        input.counters = Some(counters(Some(2), Some(1)));
        assert_eq!(media_locked(&input), Some(true));
        input.counters = Some(counters(Some(2), Some(2)));
        assert_eq!(media_locked(&input), Some(false));
        input.counters = Some(counters(Some(1), None));
        assert_eq!(media_locked(&input), Some(true));
    }

    #[test]
    fn ptp4l_is_mentioned_only_when_it_keeps_no_time() {
        assert_eq!(ptp_note(&Ptp::Unknown), None);
        assert_eq!(ptp_note(&Ptp::Running), None);
        assert!(ptp_note(&Ptp::Missing(None)).unwrap().contains("linuxptp"));
        let failed = ptp_note(&Ptp::Missing(Some("access denied".into()))).unwrap();
        assert!(failed.ends_with("access denied"));
        assert!(
            ptp_note(&Ptp::Elsewhere)
                .unwrap()
                .contains("another interface")
        );
    }
}
