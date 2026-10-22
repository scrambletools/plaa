//! AVB Lite on plaa's own participants, where no program here serves the
//! port's (the AVB Lite profile), as triib's entity service runs it: plaa
//! weighs from the Pdelay exchange whether a bridge reserves its streams,
//! and where none does it falls back. Its declarations then go by CVU
//! SRP in its entity's name, its stream as a unicast copy to each
//! listener admitted, and ptp4l to the AVB Lite PTP profile, until the
//! link comes up again, when it weighs afresh.

use std::sync::Arc;
use std::time::Duration;

use atdecc::MacAddress;
use atdecc::lite::FallbackReason;
use avb_lite::Event;
use avb_lite::fallback::{self, Fallback, Ptp4l};
use avb_lite::participant::{FANOUT, MEDIA_VLAN, Participant};
use avb_mrp::sharing::Mode;
use avb_net::Socket;
use avb_net::stream::FrameSender;

use super::Own;
use crate::runtime::{Input, read_into};
use crate::{Ptp, gptp, ptp};

pub(super) struct OwnLite {
    fallback: Fallback,
    /// CVU SRP's participant, while the port runs AVB Lite.
    participant: Option<Participant>,
    /// Sends CVU SRP's commands, VLAN-tagged.
    tagged: FrameSender,
    /// The link, as last looked at.
    carrier: Option<bool>,
    /// Whether streams may escalate to multicast, as a controller set it.
    escalation: bool,
}

/// The socket PTP's messages come in on, Pdelay's and Announce's, read
/// into `sender`; the fallback's beacon goes out on it.
pub(super) fn open_ptp(
    interface: &str,
    sender: &std::sync::mpsc::Sender<Input>,
    stop: &Arc<std::sync::atomic::AtomicBool>,
) -> std::io::Result<Arc<Socket>> {
    let socket = Arc::new(Socket::open(interface, fallback::PTP_ETHERTYPE)?);
    socket.join_multicast(fallback::PDELAY_DESTINATION)?;
    socket.join_multicast(fallback::PTP_DESTINATION)?;
    read_into(
        socket.clone(),
        stop.clone(),
        sender.clone(),
        "plaa-ptp",
        |_, _, bytes| Input::Ptp(bytes),
    )?;
    Ok(socket)
}

/// What runs PTP on the interface, as the fallback weighs it: ptp4l in
/// gPTP, peer to peer, or the AVB Lite profile, end to end, or none. The
/// service plaa started runs gPTP while it has yet to answer: the
/// fallback asking the peer itself meanwhile would make it fault.
fn ptp4l(status: Option<&gptp::Status>, ptp: &Ptp) -> Ptp4l {
    match (status, ptp) {
        (Some(status), _) if status.end_to_end => Ptp4l::Lite,
        (Some(_), _) | (None, Ptp::Starting) => Ptp4l::Gptp,
        (None, _) => Ptp4l::Absent,
    }
}

/// The port's mode while plaa runs its own participants: AVB Lite for
/// `lite`, why it fell back, or AVB, with AVB Lite's settings.
fn mode(lite: Option<FallbackReason>, escalation: bool) -> Mode {
    Mode {
        lite: lite.map(|reason| reason.0),
        media_vlan: MEDIA_VLAN,
        unicast_fanout: FANOUT,
        escalation,
    }
}

/// Sends the participant's commands from `mac`, VLAN-tagged in the media
/// VLAN at AVB Lite's priority (profile 6, item 5).
fn send_all(participant: &mut Participant, tagged: &FrameSender, mac: MacAddress) {
    let tag = participant.tag();
    while let Some((destination, pdu)) = participant.poll_transmit() {
        let mut frame = Vec::with_capacity(18 + pdu.len());
        frame.extend_from_slice(&destination.0);
        frame.extend_from_slice(&mac.0);
        frame.extend_from_slice(&0x8100u16.to_be_bytes());
        frame.extend_from_slice(&tag.to_be_bytes());
        frame.extend_from_slice(&atdecc::ETHERTYPE_AVTP.to_be_bytes());
        frame.extend_from_slice(&pdu);
        frame.resize(frame.len().max(64), 0);
        let _ = tagged.send(&frame);
    }
}

fn why(reason: FallbackReason) -> &'static str {
    match reason {
        FallbackReason::ENDPOINT_TLV => "another endpoint answers peer delay",
        FallbackReason::PDELAY_UNANSWERED => "no gPTP peer answers",
        FallbackReason::MULTIPLE_RESPONDERS => "more than one peer answers",
        _ => "configured",
    }
}

impl Own {
    /// plaa runs its own participants: it weighs whether a bridge reserves
    /// streams on the port, and its entity says it can run AVB Lite.
    pub(super) fn arm_lite(&mut self, since: Duration) {
        if self.own_lite.is_some() {
            return;
        }
        let tagged = match FrameSender::open(&self.interface) {
            Ok(tagged) => tagged,
            Err(error) => {
                eprintln!(
                    "plaa: {} cannot fall back to AVB Lite: {error}",
                    self.interface
                );
                return;
            }
        };
        let mut fallback = Fallback::new(self.own_clock.0, false, since);
        fallback.set_ptp4l(since, ptp4l(self.gptp.as_ref(), &self.ptp));
        self.own_lite = Some(OwnLite {
            fallback,
            participant: None,
            tagged,
            carrier: None,
            escalation: false,
        });
        self.mode = Some(mode(None, false));
        self.copies = None;
        self.update_lite_status();
    }

    /// The port is shared, or plaa steps aside: AVB Lite on its own
    /// participants ends, its declarations withdrawn where it ran, and
    /// ptp4l goes back to gPTP.
    pub(super) fn disarm_lite(&mut self) {
        let Some(mut lite) = self.own_lite.take() else {
            return;
        };
        if let Some(mut participant) = lite.participant.take() {
            participant.leave();
            send_all(&mut participant, &lite.tagged, self.mac);
            self.hear_media_vlan(false);
            if let Err(why) = ptp::switch(&self.interface, false) {
                eprintln!("plaa: ptp4l stays in AVB Lite's profile: {why}");
            }
        }
    }

    /// CVU SRP's commands come tagged in the media VLAN, which an
    /// interface filtering VLANs in hardware, as the I210 does, drops
    /// unless it is promiscuous: held so while AVB Lite runs, `on`.
    fn hear_media_vlan(&self, on: bool) {
        let Some(socket) = &self.ptp_socket else {
            return;
        };
        if on && !avb_net::interfaces::filters_vlans(&self.interface).unwrap_or(false) {
            return;
        }
        if let Err(error) = socket.set_promiscuous(on) {
            eprintln!(
                "plaa: {} may not hear AVB Lite's media VLAN: {error}",
                self.interface
            );
        }
    }

    /// CVU SRP's participant, while the port runs AVB Lite on plaa's own.
    pub(super) fn cvu(&mut self) -> Option<&mut Participant> {
        self.own_lite.as_mut()?.participant.as_mut()
    }

    /// A PTP message heard: the fallback weighs it, and a grandmaster's
    /// link speed corrects the media clock in AVB Lite (profile 5).
    pub(crate) fn handle_ptp(&mut self, since: Duration, bytes: &[u8]) {
        if let Some(lite) = &mut self.own_lite {
            lite.fallback.handle_ptp(since, bytes);
        }
        if let Some((grandmaster, speed)) = fallback::grandmaster_link(bytes)
            && self.grandmaster_links.insert(grandmaster, speed) != Some(speed)
        {
            self.update_correction();
        }
    }

    /// ptp4l said how it runs: the fallback goes by it.
    pub(super) fn lite_ptp4l(&mut self, since: Duration) {
        let running = ptp4l(self.gptp.as_ref(), &self.ptp);
        if let Some(lite) = &mut self.own_lite {
            lite.fallback.set_ptp4l(since, running);
        }
        self.update_correction();
    }

    /// Corrects the media clock for the asymmetry of the grandmaster's and
    /// this computer's link speeds, where ptp4l runs the AVB Lite PTP
    /// profile and the grandmaster announces its speed (profile 5).
    fn update_correction(&mut self) {
        let wanted = match &self.gptp {
            Some(status) if status.end_to_end && self.link_speed > 0 => self
                .grandmaster_links
                .get(&status.gptp.grandmaster.0)
                .map_or(0, |speed| fallback::link_asymmetry(*speed, self.link_speed)),
            _ => 0,
        };
        if wanted != self.correction {
            self.correction = wanted;
            self.clock.set_correction(wanted);
        }
    }

    /// The link is up or down, as looked at each second: while it is down
    /// nothing is weighed, and as it comes up again AVB Lite ends and the
    /// weighing starts afresh (profile 2.2).
    pub(super) fn lite_link(&mut self, since: Duration, up: bool) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        let came_up = up && lite.carrier == Some(false);
        lite.carrier = Some(up);
        lite.fallback.set_link(since, up);
        if came_up && lite.participant.is_some() {
            self.leave_lite(since);
        }
    }

    /// Runs the fallback's and CVU SRP's timers, falls back where the
    /// fallback says to, and acts on what CVU SRP heard.
    pub(super) fn lite_turn(&mut self, since: Duration) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        if lite.fallback.poll_timeout() <= since {
            lite.fallback.handle_timeout(since);
        }
        if let (Some(reason), None) = (lite.fallback.reason(), &lite.participant) {
            self.enter_lite(since, reason);
        }
        if let Some(participant) = self.cvu()
            && participant.poll_timeout() <= since
        {
            participant.handle_timeout(since);
        }
        self.lite_events(since);
    }

    /// When the fallback's or CVU SRP's timers next run.
    pub(super) fn lite_timeout(&self) -> Option<Duration> {
        let lite = self.own_lite.as_ref()?;
        let participant = lite.participant.as_ref().map(Participant::poll_timeout);
        Some(participant.map_or(lite.fallback.poll_timeout(), |at| {
            at.min(lite.fallback.poll_timeout())
        }))
    }

    /// An ATDECC frame from another computer, which CVU SRP takes where it
    /// is one of its commands.
    pub(super) fn cvu_heard(
        &mut self,
        since: Duration,
        source: MacAddress,
        group: bool,
        bytes: &[u8],
    ) {
        if let Some(participant) = self.cvu() {
            participant.handle_pdu(since, source, group, bytes);
        }
    }

    /// A controller allowed or disallowed escalation, which plaa decides
    /// for the port on its own participants.
    pub(super) fn lite_escalation(&mut self, since: Duration, allowed: bool) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        lite.escalation = allowed;
        if let Some(participant) = &mut lite.participant {
            participant.set_escalation(since, allowed);
        }
        if let Some(mode) = &mut self.mode {
            mode.escalation = allowed;
        }
        self.update_lite_status();
    }

    /// Sends the fallback's beacon and CVU SRP's commands.
    pub(crate) fn send_lite(&mut self) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        while let Some(message) = lite.fallback.poll_transmit() {
            if let Some(socket) = &self.ptp_socket {
                let _ = socket.send(fallback::PDELAY_DESTINATION, &message);
            }
        }
        if let Some(participant) = &mut lite.participant {
            send_all(participant, &lite.tagged, self.mac);
        }
    }

    /// Falls back to AVB Lite: MSRP's declarations go, and CVU SRP's
    /// declare the stream and the listener in their place.
    fn enter_lite(&mut self, since: Duration, reason: FallbackReason) {
        eprintln!(
            "plaa: AVB Lite on {}: {}, declaring with CVU SRP",
            self.interface,
            why(reason)
        );
        if let (Some(reservation), Some((stream_id, _))) =
            (&mut self.reservation, self.entity.output_stream(0))
            && self.declared
        {
            reservation.withdraw(since, stream_id.0);
        }
        let listened = self.listened();
        if let (Some(reservation), Some(stream_id)) = (&mut self.reservation, listened)
            && self.listening.declared.is_some()
        {
            reservation.declare_listener(since, stream_id, None);
        }
        self.listening.declared = None;
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        let mut participant = Participant::new(self.mac, since);
        participant.set_link_speed(since, self.link_speed);
        participant.set_escalation(since, lite.escalation);
        let escalation = lite.escalation;
        lite.participant = Some(participant);
        self.hear_media_vlan(true);
        self.mode = Some(mode(Some(reason), escalation));
        self.copies = None;
        self.ready = false;
        if self.declared
            && let Some(declaration) = self.declaration()
        {
            let entity = self.entity.entity_id();
            if let Some(participant) = self.cvu() {
                participant.declare_talker(since, entity, declaration);
            }
        }
        self.update_talker();
        self.listening.talker = None;
        self.listening.asked = false;
        self.ask_to_listen(since);
        self.update_listener(since);
        if let Err(why) = ptp::switch(&self.interface, true) {
            eprintln!("plaa: ptp4l stays in gPTP: {why}");
        }
        self.update_lite_status();
    }

    /// The link came up again: AVB Lite ends, its declarations withdrawn,
    /// MSRP's declared again, and the fallback weighs afresh.
    fn leave_lite(&mut self, since: Duration) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        let Some(mut participant) = lite.participant.take() else {
            return;
        };
        participant.leave();
        send_all(&mut participant, &lite.tagged, self.mac);
        let mut fallback = Fallback::new(self.own_clock.0, false, since);
        fallback.set_link(since, true);
        fallback.set_ptp4l(since, ptp4l(self.gptp.as_ref(), &self.ptp));
        lite.fallback = fallback;
        let escalation = lite.escalation;
        self.hear_media_vlan(false);
        eprintln!(
            "plaa: {}'s link came up again: AVB, declaring with MSRP",
            self.interface
        );
        self.mode = Some(mode(None, escalation));
        self.copies = None;
        self.ready = false;
        if std::mem::take(&mut self.declared) {
            self.declare(since);
        }
        self.update_talker();
        self.listening.talker = None;
        self.listening.asked = false;
        self.ask_to_listen(since);
        self.update_listener(since);
        if let Err(why) = ptp::switch(&self.interface, false) {
            eprintln!("plaa: ptp4l stays in AVB Lite's profile: {why}");
        }
        self.update_lite_status();
    }

    /// Acts on what CVU SRP says: where plaa's stream goes, and the
    /// stream its listener takes as its talker moves.
    fn lite_events(&mut self, since: Duration) {
        let ours = self.entity.output_stream(0).map(|(id, _)| id.0);
        let listened = self.listened();
        while let Some(event) = self.cvu().and_then(Participant::poll_event) {
            match event {
                Event::Copies {
                    stream_id,
                    copies,
                    ready,
                    ..
                } if Some(stream_id) == ours => {
                    let tag = self.cvu().map_or(0, |participant| participant.tag());
                    self.copies = Some((tag, copies));
                    self.ready = ready > 0;
                    self.follow_copies();
                    self.update_talker();
                    self.update_lite_status();
                }
                Event::Talker(stream_id) if Some(stream_id) == listened && self.listening.asked => {
                    let entity = self.entity.entity_id();
                    self.listening.talker = self
                        .cvu()
                        .and_then(|participant| participant.listen(since, stream_id, Some(entity)));
                    self.update_listener(since);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atdecc::ClockIdentity;
    use atdecc::entity::Gptp;

    fn status(end_to_end: bool) -> gptp::Status {
        gptp::Status {
            gptp: Gptp::default(),
            own_clock: ClockIdentity(1),
            offset: Some(3),
            end_to_end,
        }
    }

    /// ptp4l end to end is the AVB Lite profile; peer to peer, or plaa's
    /// service yet to answer, gPTP; nothing answering, none.
    #[test]
    fn the_fallback_knows_what_runs_ptp() {
        assert_eq!(ptp4l(Some(&status(true)), &Ptp::Running), Ptp4l::Lite);
        assert_eq!(ptp4l(Some(&status(false)), &Ptp::Running), Ptp4l::Gptp);
        assert_eq!(ptp4l(None, &Ptp::Starting), Ptp4l::Gptp);
        assert_eq!(ptp4l(None, &Ptp::Missing(None)), Ptp4l::Absent);
        assert_eq!(ptp4l(None, &Ptp::Elsewhere), Ptp4l::Absent);
    }

    /// Its mode says why it fell back, with AVB Lite's settings.
    #[test]
    fn its_mode_says_why_it_fell_back() {
        let lite = mode(Some(FallbackReason::ENDPOINT_TLV), true);
        assert_eq!(lite.lite, Some(1));
        assert!(lite.escalation);
        assert_eq!((lite.media_vlan, lite.unicast_fanout), (2, 5));
        assert_eq!(mode(None, false).lite, None);
    }
}
