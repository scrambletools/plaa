//! The Network sheet: how plaa's own talker stands and the listeners it
//! can send to, then every listener's input with the talker it is bound
//! to, to bind it to another or to none.

use std::sync::Arc;

use iced::widget::{text, toggler};
use iced::{Center, Element, Fill, Length};
use plaa_avb::{EntityId, Found, Snapshot, TalkerState};

use scramble_ui::dropdown;
use scramble_ui::{Type, column, component, row, style};

use crate::window::Control;

/// An entity's stream: the entity, and its output's or input's index.
type End = (EntityId, u16);

fn note<'a>(content: String) -> Element<'a, Control> {
    scramble_ui::aligned(
        scramble_ui::styled(content, Type::BodyMedium).style(style::on_surface_variant),
    )
}

/// What a talker's output is called in a list: its entity's name and its
/// own.
fn output_label(entity: &Found, name: &str) -> String {
    crate::fl!(
        "network-stream",
        entity = entity.name.clone(),
        stream = name.to_owned()
    )
}

pub fn sheet<'a>(snapshot: Arc<Snapshot>) -> Element<'a, Control> {
    let mut content = column![].spacing(10).width(Fill);
    let own = snapshot.entities.iter().find(|entity| entity.own);
    content = content.push(component::section(crate::fl!("network-plaa")));
    content = content.push(note(match &snapshot.talker {
        TalkerState::Off => crate::fl!("network-off"),
        TalkerState::Blocked(why) => crate::fl!("network-blocked", reason = why.clone()),
        TalkerState::Waiting => crate::fl!("network-waiting"),
        TalkerState::Streaming => crate::fl!("network-sending"),
    }));
    // plaa's output, to each listener: a switch apiece.
    if let Some(own) = own {
        let ours: End = (own.id, 0);
        for entity in snapshot.entities.iter().filter(|entity| !entity.own) {
            for input in &entity.inputs {
                let listener: End = (entity.id, input.index);
                let on = input.bound == Some(ours);
                let label = output_label(entity, &input.name);
                content = content.push(
                    row![
                        iced::widget::container(scramble_ui::aligned(scramble_ui::styled(
                            label,
                            Type::BodyLarge
                        )))
                        .width(Fill),
                        toggler(on)
                            .on_toggle(move |on| {
                                if on {
                                    Control::AvbConnect(ours, listener)
                                } else {
                                    Control::AvbDisconnect(listener)
                                }
                            })
                            .size(24)
                            .style(style::switch),
                    ]
                    .spacing(12)
                    .align_y(Center),
                );
            }
        }
    }
    content = content.push(component::section(crate::fl!("network-connections")));
    if snapshot.entities.is_empty() {
        content = content.push(note(crate::fl!("network-looking")));
    }
    // Every talker's output, to choose from for each input.
    let outputs: Vec<(End, String)> = snapshot
        .entities
        .iter()
        .flat_map(|entity| {
            entity.outputs.iter().map(move |output| {
                (
                    (entity.id, output.index),
                    output_label(entity, &output.name),
                )
            })
        })
        .collect();
    let none = crate::fl!("network-not-connected");
    for entity in &snapshot.entities {
        if entity.inputs.is_empty() {
            continue;
        }
        content = content.push(scramble_ui::aligned(scramble_ui::styled(
            entity.name.clone(),
            Type::TitleSmall,
        )));
        for input in &entity.inputs {
            let listener: End = (entity.id, input.index);
            let mut options = vec![none.clone()];
            options.extend(outputs.iter().map(|(_, label)| label.clone()));
            let selected = match input.bound {
                None => none.clone(),
                Some(bound) => outputs.iter().find(|(end, _)| *end == bound).map_or_else(
                    || crate::fl!("network-unknown-talker", talker = format!("{}", bound.0)),
                    |(_, label)| label.clone(),
                ),
            };
            let choices = outputs.clone();
            let none = none.clone();
            let pick = dropdown::pick(options, Some(selected), move |chosen| {
                match choices.iter().find(|(_, label)| *label == chosen) {
                    Some((talker, _)) if chosen != none => Control::AvbConnect(*talker, listener),
                    _ => Control::AvbDisconnect(listener),
                }
            })
            .width(Fill);
            content = content.push(
                column![
                    scramble_ui::aligned(
                        scramble_ui::styled(input.name.clone(), Type::LabelLarge)
                            .style(style::on_surface_variant)
                    ),
                    pick,
                    note(format!("{}", input.format)),
                ]
                .spacing(4),
            );
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
