//! The Network Audio sheet: the listeners on the network that can play
//! what plaa plays, by name, each with one button: play there, where it
//! takes plaa's stream as it is; stop, where it plays it; or, where it is
//! in another format, a menu of the formats both share, to connect in.
//! Under them, how the one chosen takes its stream. The talkers plaa
//! listens to are the Connect menu's.

use std::sync::Arc;

use iced::widget::{container, text};
use iced::{Center, Element, Fill, Length};
use plaa_avb::{
    EntityId, Found, ListenBlocked, Ptp, Snapshot, Stream, StreamFormat, StreamInfo,
    StreamInfoFlags, TalkerState,
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

/// The inputs that can take one of plaa's formats, as a format taking up
/// to as many channels takes fewer too, named by their entity, and by the
/// input too where an entity has several.
fn listeners<'s>(snapshot: &'s Snapshot, ours: &Stream) -> Vec<Listener<'s>> {
    let mut found = Vec::new();
    for entity in snapshot.entities.iter().filter(|entity| !entity.own) {
        let able: Vec<&Stream> = entity
            .inputs
            .iter()
            .filter(|input| ours.formats.iter().any(|format| takes(input, *format)))
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
                    .filter(|format| takes(input, *format))
                    .collect(),
                here: entity.here,
            });
        }
    }
    found
}

/// Whether an input can take a stream sent in `format`, in one of its
/// own.
fn takes(input: &Stream, format: StreamFormat) -> bool {
    input.formats.iter().any(|own| format.suits(*own))
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
    } else if format.suits(listener.input.format) {
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

/// A talker's output plaa's input can take: as it sends now, or once it
/// changes to `change`, which cuts off the `others` that take it now.
struct Source {
    end: End,
    name: String,
    change: Option<StreamFormat>,
    others: usize,
}

/// The outputs of other entities plaa's `input` can take, named as
/// listeners are: those sending in a format it takes, and those that can
/// change to one.
fn sources(snapshot: &Snapshot, input: &Stream) -> Vec<Source> {
    let mut found = Vec::new();
    for entity in snapshot.entities.iter().filter(|entity| !entity.own) {
        let able: Vec<(&Stream, Option<StreamFormat>)> = entity
            .outputs
            .iter()
            .filter_map(|output| {
                if takes(input, output.format) {
                    Some((output, None))
                } else {
                    change_to(output, input).map(|format| (output, Some(format)))
                }
            })
            .collect();
        for &(output, change) in &able {
            let end = (entity.id, output.index);
            found.push(Source {
                end,
                name: name(entity, output, able.len() > 1),
                change,
                others: listening(snapshot, end),
            });
        }
    }
    found
}

/// The format a talker's output changes to for plaa's input: of those it
/// offers that the input takes, one at the rate it sends at now, so its
/// entity's other streams keep theirs, and of as many channels, where
/// there is one, the first it lists of those nearest.
fn change_to(output: &Stream, input: &Stream) -> Option<StreamFormat> {
    let (rate, channels) = (output.format.sample_rate(), output.format.channels());
    output
        .formats
        .iter()
        .copied()
        .filter(|format| takes(input, *format))
        .min_by_key(|format| (format.sample_rate() != rate, format.channels() != channels))
}

/// How many inputs of entities other than plaa take a talker's output.
fn listening(snapshot: &Snapshot, talker: End) -> usize {
    snapshot
        .entities
        .iter()
        .filter(|entity| !entity.own)
        .flat_map(|entity| &entity.inputs)
        .filter(|input| input.bound == Some(talker))
        .count()
}

/// Listening to a talker that changes its format first, and what that
/// cuts off.
fn change_label(source: &Source, format: StreamFormat) -> String {
    let talker = source.name.clone();
    let format = format.to_string();
    match source.others {
        0 => crate::fl!("connect-change", talker = talker, format = format),
        count => crate::fl!(
            "connect-change-cuts-off",
            talker = talker,
            format = format,
            count = count
        ),
    }
}

/// The Connect menu: what plaa's input takes now, and stopping it; then
/// the talkers it can take, those sending in a format it plays before
/// those that change format for it.
pub fn connect_entries(snapshot: &Snapshot) -> Vec<Entry<Control>> {
    let mut entries = Vec::new();
    // Its input is its entity's, which sends too.
    match &snapshot.talker {
        TalkerState::Off => {
            entries.push(Entry::note(crate::fl!("connect-off")));
            return entries;
        }
        TalkerState::Blocked(why) => {
            entries.push(Entry::note(crate::fl!(
                "connect-blocked",
                reason = why.clone()
            )));
            return entries;
        }
        _ => {}
    }
    let own = snapshot.entities.iter().find(|entity| entity.own);
    let Some((own, input)) = own.and_then(|own| Some((own, own.inputs.first()?))) else {
        entries.push(Entry::note(crate::fl!("connect-looking")));
        return entries;
    };
    let ours = (own.id, input.index);
    match snapshot.listen_blocked {
        Some(ListenBlocked::OldService) => {
            entries.push(Entry::note(crate::fl!("network-listen-update-triib")));
        }
        Some(ListenBlocked::AvbLite) => {
            entries.push(Entry::note(crate::fl!("network-listen-lite")));
        }
        None => {}
    }
    if let Some(talker) = input.bound {
        entries.push(
            Entry::item(
                crate::fl!(
                    "network-listen-stop",
                    talker = talker_name(snapshot, talker)
                ),
                Control::AvbDisconnect(ours),
            )
            .icon(Icon::Stop),
        );
        entries.push(Entry::note(status(snapshot, input)));
        entries.push(Entry::divider());
    }
    let (sending, changing): (Vec<Source>, Vec<Source>) = sources(snapshot, input)
        .into_iter()
        .partition(|source| source.change.is_none());
    if sending.is_empty() && changing.is_empty() {
        entries.push(Entry::note(crate::fl!("network-no-sources")));
    }
    if !sending.is_empty() && !changing.is_empty() {
        entries.push(Entry::heading(crate::fl!("connect-sending")));
    }
    for source in sending {
        let picked = input.bound == Some(source.end);
        entries.push(
            Entry::item(source.name, Control::AvbConnect(source.end, ours, None)).checked(picked),
        );
    }
    if !changing.is_empty() {
        entries.push(Entry::heading(crate::fl!("connect-changing")));
    }
    for source in changing {
        let Some(format) = source.change else {
            continue;
        };
        entries.push(Entry::item(
            change_label(&source, format),
            Control::AvbConnect(source.end, ours, Some(format)),
        ));
    }
    if let Some(failure) = &snapshot.failure {
        entries.push(Entry::note(crate::fl!(
            "network-failed",
            reason = failure.clone()
        )));
    }
    entries
}

/// How a talker's stream is known as a live source.
pub fn live_id((entity, index): End) -> String {
    format!("avb:{:016x}:{index}", entity.0)
}

/// The talker of a live source's id.
pub fn talker_of(id: &str) -> Option<End> {
    let (entity, index) = id.strip_prefix("avb:")?.split_once(':')?;
    Some((
        EntityId(u64::from_str_radix(entity, 16).ok()?),
        index.parse().ok()?,
    ))
}

/// Whether plaa's input takes a talker's stream, for the Connect button
/// to show it in use.
pub fn listening_here(snapshot: &Snapshot) -> bool {
    snapshot
        .entities
        .iter()
        .find(|entity| entity.own)
        .and_then(|own| own.inputs.first())
        .is_some_and(|input| input.bound.is_some())
}

/// A labelled value, as the details and the inspector show them.
fn field<'a>(label: String, value: String) -> Element<'a, Control> {
    column![
        scramble_ui::styled(label, Type::LabelMedium).style(style::on_surface_variant),
        scramble_ui::aligned(scramble_ui::styled(value, Type::BodyMedium)),
    ]
    .spacing(2)
    .into()
}

/// How an input takes its stream: from which talker and how that goes,
/// and whether its media clock is locked to it.
fn taking<'a>(snapshot: &Snapshot, input: &Stream) -> Vec<Element<'a, Control>> {
    let mut fields = vec![field(crate::fl!("network-status"), status(snapshot, input))];
    if input.bound.is_some()
        && let Some(locked) = media_locked(input)
    {
        let clock = if locked {
            crate::fl!("network-locked")
        } else {
            crate::fl!("network-not-locked")
        };
        fields.push(field(crate::fl!("network-media-clock"), clock));
    }
    fields
}

/// The stream an input takes, as its stream info says, and what it
/// counted.
fn received<'a>(input: &Stream) -> Vec<Element<'a, Control>> {
    let mut fields = Vec::new();
    if let Some(info) = input.info.filter(|_| input.bound.is_some()) {
        // Some entities have it without marking it valid.
        if info.settled() || info.stream_id.0 != 0 {
            fields.push(field(
                crate::fl!("network-stream-id"),
                info.stream_id.to_string(),
            ));
        }
        if info.flags.contains(StreamInfoFlags::STREAM_DEST_MAC_VALID) {
            fields.push(field(
                crate::fl!("network-destination"),
                info.stream_dest_mac.to_string(),
            ));
        }
        if info.registering() {
            fields.push(field(
                crate::fl!("network-latency"),
                crate::fl!(
                    "network-microseconds",
                    time = (info.msrp_accumulated_latency / 1000).to_string()
                ),
            ));
        }
    }
    if let Some(counters) = input.counters {
        let counted = [
            (crate::fl!("network-frames"), counters.frames_rx),
            (
                crate::fl!("network-out-of-sequence"),
                counters.seq_num_mismatch,
            ),
            (
                crate::fl!("network-interrupted"),
                counters.stream_interrupted,
            ),
            (crate::fl!("network-late"), counters.late_timestamp),
            (crate::fl!("network-early"), counters.early_timestamp),
        ];
        for (label, count) in counted {
            if let Some(count) = count {
                fields.push(field(label, count.to_string()));
            }
        }
    }
    fields
}

/// How a listener's input takes its stream: from which talker and how
/// that goes, in what format and what others it shares with plaa's
/// output, and what it counted.
fn details<'a>(snapshot: &Snapshot, listener: &Listener<'_>) -> Element<'a, Control> {
    let input = listener.input;
    let mut content = column![component::section(listener.name.clone())].spacing(6);
    content = content.extend(taking(snapshot, input));
    content = content.push(field(
        crate::fl!("network-format"),
        input.format.to_string(),
    ));
    let shared: Vec<String> = listener.shared.iter().map(ToString::to_string).collect();
    content = content.push(field(crate::fl!("network-formats"), shared.join("\n")));
    content = content.extend(received(input));
    content = content.push(field(
        crate::fl!("network-entity-id"),
        listener.end.0.to_string(),
    ));
    content.into()
}

/// The inspector of a window playing the stream plaa listens to from
/// `talker`: the talker, and its other listeners, which a new format cuts
/// off; how the stream comes, in a format to change to another both take;
/// and what plaa's input counted.
pub fn stream_inspector<'a>(
    snapshot: &Snapshot,
    talker: End,
    menu: Option<Menu>,
) -> Element<'a, Control> {
    let mut content = column![component::section(crate::fl!("info-talker"))].spacing(6);
    content = content.push(field(
        crate::fl!("info-name"),
        talker_name(snapshot, talker),
    ));
    content = content.push(field(crate::fl!("network-entity-id"), talker.0.to_string()));
    let others = listening(snapshot, talker);
    if others > 0 {
        content = content.push(field(
            crate::fl!("info-other-listeners"),
            others.to_string(),
        ));
    }
    let own = snapshot.entities.iter().find(|entity| entity.own);
    let Some((own, input)) = own.and_then(|own| Some((own, own.inputs.first()?))) else {
        return content.into();
    };
    let ours = (own.id, input.index);
    let output = snapshot
        .entities
        .iter()
        .find(|entity| entity.id == talker.0)
        .and_then(|entity| {
            entity
                .outputs
                .iter()
                .find(|output| output.index == talker.1)
        });
    content = content.push(component::section(crate::fl!("info-stream")));
    content = content.extend(taking(snapshot, input));
    if let Some(output) = output {
        content = content.push(format_menu(output, input, talker, ours, others, menu));
        let class = if output.format.is_class_b() { "B" } else { "A" };
        content = content.push(field(crate::fl!("info-sr-class"), class.to_owned()));
    }
    content = content.extend(received(input));
    content.into()
}

/// The formats a talker offers that plaa's input takes, its own checked,
/// each connecting in it, after a note of the listeners a new one cuts
/// off.
fn format_entries(
    output: &Stream,
    input: &Stream,
    talker: End,
    ours: End,
    others: usize,
) -> Vec<Entry<Control>> {
    let mut entries = Vec::new();
    if others > 0 {
        entries.push(Entry::note(crate::fl!(
            "info-format-cuts-off",
            count = others
        )));
    }
    for format in output.formats.iter().copied() {
        if takes(input, format) {
            entries.push(
                Entry::item(
                    format.to_string(),
                    Control::AvbConnect(talker, ours, Some(format)),
                )
                .checked(format == output.format),
            );
        }
    }
    entries
}

/// The talker's format, and the others it offers that plaa's input takes,
/// to change to, saying which listeners the change cuts off.
fn format_menu<'a>(
    output: &Stream,
    input: &Stream,
    talker: End,
    ours: End,
    others: usize,
    menu: Option<Menu>,
) -> Element<'a, Control> {
    let entries = format_entries(output, input, talker, ours, others);
    let picker = dropdown::menu(output.format.to_string(), entries)
        .width(Fill)
        .open(menu == Some(Menu::AvbFormats(ours)), move |open| {
            Control::Menu(Menu::AvbFormats(ours), open)
        });
    column![
        scramble_ui::styled(crate::fl!("network-format"), Type::LabelMedium)
            .style(style::on_surface_variant),
        picker,
    ]
    .spacing(2)
    .into()
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
pub fn talker_name(snapshot: &Snapshot, (id, index): End) -> String {
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

    const STEREO_12: StreamFormat = StreamFormat(0x0205_0220_0080_c000);
    const EIGHT_12: StreamFormat = StreamFormat(0x0205_0220_0200_c000);

    /// plaa, listening on an input that takes `formats`, and the talkers
    /// around it: one sending in a format plaa takes, two that can change
    /// to one, the second of eight channels, one that cannot, and a
    /// listener on the first that changes.
    fn network(formats: &[StreamFormat]) -> Snapshot {
        let mut plaa = entity(9, "plaa", vec![stream(0, "Listen", STEREO, formats)]);
        plaa.own = true;
        plaa.outputs = vec![stream(0, "Playback", STEREO, &[STEREO])];
        let mut mic = entity(1, "Mic", Vec::new());
        mic.outputs = vec![stream(0, "Out", STEREO, &[STEREO])];
        let mut synthia = entity(2, "Synthia", Vec::new());
        synthia.outputs = vec![stream(0, "Out", STEREO_12, &[STEREO_12, STEREO, EIGHT])];
        let mut desk = entity(3, "Desk", Vec::new());
        desk.outputs = vec![stream(0, "Main", EIGHT_12, &[EIGHT_12, STEREO, EIGHT])];
        let mut old = entity(4, "Old", Vec::new());
        old.outputs = vec![stream(0, "Out", AM824, &[AM824])];
        let mut amp = entity(5, "Amp", vec![stream(0, "In", STEREO_12, &[STEREO_12])]);
        amp.inputs[0].bound = Some((EntityId(2), 0));
        Snapshot {
            entities: vec![plaa, mic, synthia, desk, old, amp],
            talker: TalkerState::Waiting,
            ..Snapshot::default()
        }
    }

    #[test]
    fn talkers_that_can_change_to_a_format_plaa_takes_are_offered() {
        let snapshot = network(&[STEREO, STEREO_24, EIGHT]);
        let input = &snapshot.entities[0].inputs[0];
        let found = sources(&snapshot, input);
        let names: Vec<&str> = found.iter().map(|source| source.name.as_str()).collect();
        // plaa's own and the AM824-only one are left out.
        assert_eq!(names, ["Mic", "Synthia", "Desk"]);
        assert_eq!(found[0].change, None);
        // Each changes to one of as many channels as it sends.
        assert_eq!(found[1].change, Some(STEREO));
        assert_eq!(found[2].change, Some(EIGHT));
        // The amp listens to Synthia, and would be cut off.
        assert_eq!(
            found.iter().map(|source| source.others).collect::<Vec<_>>(),
            [0, 1, 0]
        );
        // Without eight channels, the first both take.
        let snapshot = network(&[STEREO_24, STEREO]);
        let input = &snapshot.entities[0].inputs[0];
        assert_eq!(sources(&snapshot, input)[2].change, Some(STEREO));
    }

    #[test]
    fn the_inspector_offers_the_talkers_formats_plaa_takes() {
        let snapshot = network(&[STEREO, STEREO_24, EIGHT]);
        let input = &snapshot.entities[0].inputs[0];
        // Synthia, whose output the amp listens to, offers 12 samples a
        // frame, which this input does not take.
        let synthia = &snapshot.entities[2].outputs[0];
        let entries = format_entries(synthia, input, (EntityId(2), 0), (EntityId(9), 0), 1);
        assert_eq!(
            shown(&entries),
            [
                "(A new format cuts off the talker's other listener.)",
                "AAF 48 kHz, 2 ch, 32-bit",
                "AAF 48 kHz, 8 ch, 32-bit",
            ]
        );
        // The mic's own format is checked.
        let mic = &snapshot.entities[1].outputs[0];
        let entries = format_entries(mic, input, (EntityId(1), 0), (EntityId(9), 0), 0);
        assert_eq!(shown(&entries), ["* AAF 48 kHz, 2 ch, 32-bit"]);
    }

    #[test]
    fn a_talker_comes_back_from_its_live_source() {
        let talker = (EntityId(0xe8f6_0ae8_16f5_0000), 0);
        let id = live_id(talker);
        assert_eq!(id, "avb:e8f60ae816f50000:0");
        assert_eq!(talker_of(&id), Some(talker));
        assert_eq!(talker_of("https://example.com/live"), None);
        assert_eq!(talker_of("avb:nothex:0"), None);
    }

    #[test]
    fn a_format_of_up_to_as_many_channels_takes_fewer() {
        // AM824 at 48 kHz: the MOTU 8D's, of 8 channels or fewer, and one
        // of 2 channels, as a Mac might send.
        const MOTU: StreamFormat = StreamFormat(0x00a0_0208_6000_0800);
        const AM824_2: StreamFormat = StreamFormat(0x00a0_0202_4000_0200);
        let mut motu = entity(1, "8D", vec![stream(0, "In", MOTU, &[MOTU])]);
        motu.outputs = vec![stream(0, "Out", MOTU, &[MOTU])];
        let mut mac = entity(2, "Mac", Vec::new());
        mac.outputs = vec![stream(0, "Out", AM824_2, &[AM824_2])];
        let mut plaa = entity(
            9,
            "plaa",
            vec![stream(0, "Listen", STEREO, &[STEREO, MOTU])],
        );
        plaa.own = true;
        let snapshot = Snapshot {
            entities: vec![plaa, motu, mac],
            ..Snapshot::default()
        };
        // plaa's input of up to 8 takes both as they send.
        let found = sources(&snapshot, &snapshot.entities[0].inputs[0]);
        let names: Vec<&str> = found.iter().map(|source| source.name.as_str()).collect();
        assert_eq!(names, ["8D", "Mac"]);
        assert!(found.iter().all(|source| source.change.is_none()));
        // The 8D's input takes plaa's 2 channels of AM824.
        let ours = stream(0, "Playback", STEREO, &[STEREO, AM824_2]);
        let found = listeners(&snapshot, &ours);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].shared, [AM824_2]);
    }

    #[test]
    fn a_talker_changes_format_at_its_own_rate_first() {
        // AAF of 32 bits at 96 kHz, 12 samples a frame, 8 and 2 channels.
        const AAF_96_8: StreamFormat = StreamFormat(0x0207_0220_0200_c000);
        const AAF_96_2: StreamFormat = StreamFormat(0x0207_0220_0080_c000);
        let input = stream(0, "Listen", STEREO, &[STEREO, EIGHT, AAF_96_2]);
        let output = stream(0, "Out", AAF_96_8, &[AAF_96_8, STEREO, EIGHT, AAF_96_2]);
        // Its rate kept rather than its channels, as its entity's other
        // streams keep theirs.
        assert_eq!(change_to(&output, &input), Some(AAF_96_2));
        let output = stream(0, "Out", AAF_96_8, &[AAF_96_8, STEREO, EIGHT]);
        assert_eq!(change_to(&output, &input), Some(EIGHT));
    }

    /// The menu's entries as text: items with a mark where checked, and
    /// headings and notes in brackets.
    fn shown(entries: &[Entry<Control>]) -> Vec<String> {
        entries
            .iter()
            .map(|entry| match entry {
                Entry::Item { label, checked, .. } if *checked => format!("* {label}"),
                Entry::Item { label, .. } => label.clone(),
                Entry::Heading(label) => format!("[{label}]"),
                Entry::Note(note) => format!("({note})"),
                Entry::Divider => "-".into(),
                _ => "?".into(),
            })
            .collect()
    }

    #[test]
    fn connect_menu_offers_talkers_and_stopping_the_one_heard() {
        let mut snapshot = network(&[STEREO, STEREO_24, EIGHT]);
        assert_eq!(
            shown(&connect_entries(&snapshot)),
            [
                "[Sending in a format plaa plays]",
                "Mic",
                "[Changing to a format plaa plays]",
                "Synthia, in AAF 48 kHz, 2 ch, 32-bit, cutting off its other listener",
                "Desk, in AAF 48 kHz, 8 ch, 32-bit",
            ]
        );
        snapshot.entities[0].inputs[0].bound = Some((EntityId(1), 0));
        let entries = connect_entries(&snapshot);
        assert_eq!(
            shown(&entries)[..4],
            [
                "Stop listening to Mic",
                "(From Mic)",
                "-",
                "[Sending in a format plaa plays]"
            ]
        );
        assert_eq!(shown(&entries)[4], "* Mic");
        assert!(listening_here(&snapshot));
        // Choosing one that changes sets its format as it connects.
        let Some(Entry::Item { message, .. }) = entries.last() else {
            panic!("no item last");
        };
        assert!(matches!(
            message,
            Control::AvbConnect((EntityId(3), 0), (EntityId(9), 0), Some(EIGHT))
        ));
        // Without an entity, as when it sends nothing.
        assert_eq!(
            shown(&connect_entries(&Snapshot::default())),
            ["(Turn on Send what plays in Settings to listen to talkers here.)"]
        );
        // Before plaa's own entity is found.
        let looking = Snapshot {
            talker: TalkerState::Waiting,
            ..Snapshot::default()
        };
        assert_eq!(
            shown(&connect_entries(&looking)),
            ["(Looking for talkers…)"]
        );
        assert!(!listening_here(&looking));
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
