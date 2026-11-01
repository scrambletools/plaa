//! AVB Lite on plaa's own participants, where no program here serves the
//! port's (the AVB Lite profile), as triib's entity service runs it:
//! gptpd weighs from the Pdelay exchange whether a bridge reserves
//! streams, and where none does it falls back to the AVB Lite PTP
//! profile, and plaa with it. Its declarations then go by CVU SRP in its
//! entity's name, and its stream as a unicast copy to each listener
//! admitted, until gptpd returns to gPTP, as when the link comes up again.

use std::time::Duration;

use atdecc::MacAddress;
use atdecc::lite::FallbackReason;
use avb_lite::Event;
use avb_lite::participant::{FANOUT, MEDIA_VLAN, Participant};
use avb_mrp::sharing::Mode;
use avb_net::stream::FrameSender;

use super::Own;
use crate::gptp;

pub(super) struct OwnLite {
    /// CVU SRP's participant, while the port runs AVB Lite.
    participant: Option<Participant>,
    /// Sends CVU SRP's commands, VLAN-tagged.
    tagged: FrameSender,
    /// Whether streams may escalate to multicast, as a controller set it.
    escalation: bool,
}

/// What plaa's own participants do as gptpd says how the port runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Follow {
    /// Fall back to AVB Lite, as gptpd did, for this reason.
    Enter(FallbackReason),
    /// Return to AVB, as gptpd returned to gPTP.
    Leave,
    Stay,
}

/// What follows from gptpd's `status`, the port running AVB Lite now or
/// not, `lite`. gptpd silent changes nothing: it starts again in gPTP and
/// says so.
fn follow(status: Option<&gptp::Status>, lite: bool) -> Follow {
    match (status.map(|status| status.lite), lite) {
        (Some(Some(reason)), false) => Follow::Enter(reason),
        (Some(None), true) => Follow::Leave,
        _ => Follow::Stay,
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
        self.own_lite = Some(OwnLite {
            participant: None,
            tagged,
            escalation: false,
        });
        self.mode = Some(mode(None, false));
        self.copies = None;
        self.lite_gptp(since);
        self.update_lite_status();
    }

    /// The port is shared, or plaa steps aside: AVB Lite on its own
    /// participants ends, its declarations withdrawn where it ran. gptpd
    /// keeps its profile, which the port's other program follows too.
    pub(super) fn disarm_lite(&mut self) {
        let Some(mut lite) = self.own_lite.take() else {
            return;
        };
        if let Some(mut participant) = lite.participant.take() {
            participant.leave();
            send_all(&mut participant, &lite.tagged, self.mac);
            self.hear_media_vlan(false);
        }
    }

    /// CVU SRP's commands come tagged in the media VLAN, which an
    /// interface filtering VLANs in hardware, as the I210 does, drops
    /// unless it is promiscuous: held so while AVB Lite runs, `on`.
    fn hear_media_vlan(&self, on: bool) {
        if on && !avb_net::interfaces::filters_vlans(&self.interface).unwrap_or(false) {
            return;
        }
        if let Err(error) = self.avtp.set_promiscuous(on) {
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

    /// gptpd said how the port runs: on plaa's own participants plaa
    /// falls back to AVB Lite as gptpd does, and returns to AVB as gptpd
    /// returns to gPTP.
    pub(super) fn lite_gptp(&mut self, since: Duration) {
        let Some(lite) = &self.own_lite else {
            return;
        };
        match follow(self.gptp.as_ref(), lite.participant.is_some()) {
            Follow::Enter(reason) => self.enter_lite(since, reason),
            Follow::Leave => self.leave_lite(since),
            Follow::Stay => {}
        }
    }

    /// Runs CVU SRP's timers, and acts on what it heard.
    pub(super) fn lite_turn(&mut self, since: Duration) {
        if let Some(participant) = self.cvu()
            && participant.poll_timeout() <= since
        {
            participant.handle_timeout(since);
        }
        self.lite_events(since);
    }

    /// When CVU SRP's timers next run.
    pub(super) fn lite_timeout(&self) -> Option<Duration> {
        let lite = self.own_lite.as_ref()?;
        lite.participant.as_ref().map(Participant::poll_timeout)
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

    /// Sends CVU SRP's commands.
    pub(crate) fn send_lite(&mut self) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
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
        self.update_lite_status();
    }

    /// gptpd returned to gPTP, as when the link comes up again: AVB Lite
    /// ends, its declarations withdrawn, and MSRP's declared again.
    fn leave_lite(&mut self, since: Duration) {
        let Some(lite) = &mut self.own_lite else {
            return;
        };
        let Some(mut participant) = lite.participant.take() else {
            return;
        };
        participant.leave();
        send_all(&mut participant, &lite.tagged, self.mac);
        let escalation = lite.escalation;
        self.hear_media_vlan(false);
        eprintln!(
            "plaa: gPTP on {} again: AVB, declaring with MSRP",
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

    fn status(lite: Option<FallbackReason>) -> gptp::Status {
        gptp::Status {
            gptp: Gptp::default(),
            own_clock: ClockIdentity(1),
            priority1: 248,
            residual: Some(3),
            lite,
            named: true,
            is_us: false,
        }
    }

    /// plaa falls back as gptpd does and returns as it does, and gptpd
    /// silent changes nothing.
    #[test]
    fn plaa_follows_gptpd_into_avb_lite_and_out() {
        let fell_back = status(Some(FallbackReason::PDELAY_UNANSWERED));
        assert_eq!(
            follow(Some(&fell_back), false),
            Follow::Enter(FallbackReason::PDELAY_UNANSWERED)
        );
        assert_eq!(follow(Some(&fell_back), true), Follow::Stay);
        assert_eq!(follow(Some(&status(None)), true), Follow::Leave);
        assert_eq!(follow(Some(&status(None)), false), Follow::Stay);
        assert_eq!(follow(None, true), Follow::Stay);
        assert_eq!(follow(None, false), Follow::Stay);
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
