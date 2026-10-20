//! The Network Audio sheet: the listeners on the network that can play
//! what plaa plays, by name, each with one button: play there, where it
//! takes plaa's stream as it is; stop, where it plays it; or, where it is
//! in another format, a menu of the formats both share, to connect in.

use std::sync::Arc;

use iced::widget::{container, text};
use iced::{Center, Element, Fill, Length};
use plaa_avb::{EntityId, Found, Snapshot, Stream, StreamFormat, TalkerState};
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
            });
        }
    }
    found
}

fn name(entity: &Found, input: &Stream, several: bool) -> String {
    if several {
        crate::fl!(
            "network-stream",
            entity = entity.name.clone(),
            stream = input.name.clone()
        )
    } else {
        entity.name.clone()
    }
}

pub fn sheet<'a>(snapshot: Arc<Snapshot>, menu: Option<Menu>) -> Element<'a, Control> {
    let mut content = column![].spacing(4).width(Fill);
    let own = snapshot.entities.iter().find(|entity| entity.own);
    let ours = own.and_then(|own| Some(((own.id, 0), own.outputs.first()?)));
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
            for listener in found {
                content = content.push(row_for(&listener, plaa, output.format, menu));
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

/// A listener's row: its name, and stop, play, or the formats to play in.
fn row_for<'a>(
    listener: &Listener<'_>,
    plaa: End,
    format: StreamFormat,
    menu: Option<Menu>,
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
    row![
        container(scramble_ui::aligned(scramble_ui::styled(
            listener.name.clone(),
            Type::BodyLarge
        )))
        .width(Fill),
        action,
    ]
    .spacing(12)
    .align_y(Center)
    .into()
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
        }
    }

    fn entity(id: u64, name: &str, inputs: Vec<Stream>) -> Found {
        Found {
            id: EntityId(id),
            name: name.into(),
            own: false,
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
}
