//! gPTP as gptpd keeps it on the interface, for plaa and triib alike: its
//! status, from the socket where it sends a snapshot as plaa connects, on
//! each change and every 2 s, in either profile it runs, gPTP's or the AVB
//! Lite PTP profile it falls back to.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

use atdecc::ClockIdentity;
use atdecc::entity::Gptp;
use atdecc::lite::FallbackReason;
use gptp::status::{Block, Client, Fallback, Profile};

/// How long plaa waits before asking again where gptpd does not answer.
const RETRY: Duration = Duration::from_secs(2);
/// How long gptpd may say nothing before plaa takes it as gone: it sends
/// a snapshot every 2 s.
const SILENCE: Duration = Duration::from_secs(5);

/// What gptpd says of the interface, as an entity reports it, and this
/// computer's clock; `None` when gptpd does not answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub gptp: Gptp,
    pub own_clock: ClockIdentity,
    /// gptpd's priority1, which its configuration sets: 255 never makes
    /// this computer the grandmaster.
    pub priority1: u8,
    /// How far the last Sync fell from gptpd's mapping, in nanoseconds,
    /// while it follows a grandmaster.
    pub residual: Option<i64>,
    /// gptpd runs the AVB Lite PTP profile on the port, having fallen back
    /// for this reason.
    pub lite: Option<FallbackReason>,
    /// gptpd names a grandmaster: none yet as it starts, or once the one
    /// it had is gone. Without one `gptp` names none, 0.
    pub named: bool,
    /// This computer is the grandmaster.
    pub is_us: bool,
}

/// What an entity on `interface` reports of gptpd's `status`.
pub fn read(status: &gptp::status::Status, interface: &str) -> Status {
    let own_clock = ClockIdentity(status.clock.0);
    let port = status
        .ports
        .iter()
        .find(|port| port.name == interface)
        .or(status.ports.first());
    let mut path: Vec<ClockIdentity> = status
        .path
        .iter()
        .map(|clock| ClockIdentity(clock.0))
        .collect();
    if path.is_empty() {
        path.push(own_clock);
    }
    Status {
        gptp: Gptp {
            grandmaster: status.grandmaster.map_or(ClockIdentity(0), |grandmaster| {
                ClockIdentity(grandmaster.identity.0)
            }),
            domain: status.domain,
            propagation_delay: port
                .and_then(|port| port.link_delay)
                .map_or(0, |delay| delay.clamp(0, i64::from(u32::MAX)) as u32),
            as_capable: port.is_some_and(|port| port.as_capable),
            path,
        },
        own_clock,
        priority1: status.priority1,
        residual: status.residual,
        lite: (status.profile == Profile::AvbLite).then(|| reason(status.fallback)),
        named: status.grandmaster.is_some(),
        is_us: status
            .grandmaster
            .is_some_and(|grandmaster| grandmaster.is_us),
    }
}

/// gptpd's reason for AVB Lite, as AVB Lite's profile 2.2 names it.
fn reason(fallback: Fallback) -> FallbackReason {
    match fallback {
        Fallback::EndpointTlv => FallbackReason::ENDPOINT_TLV,
        Fallback::PdelayUnanswered => FallbackReason::PDELAY_UNANSWERED,
        Fallback::MultipleResponders => FallbackReason::MULTIPLE_RESPONDERS,
        Fallback::Configured | Fallback::None => FallbackReason::CONFIGURED,
    }
}

/// Listens to gptpd on `interface` and sends what changed, until `stop`.
pub fn watch<T: Send + 'static>(
    interface: String,
    stop: Arc<AtomicBool>,
    sender: Sender<T>,
    wrap: fn(Option<Status>) -> T,
) {
    let _ = std::thread::Builder::new()
        .name("gptp".into())
        .spawn(move || {
            let mut last: Option<Option<Status>> = None;
            let mut tell = |status: Option<Status>| {
                if last.as_ref() == Some(&status) {
                    return true;
                }
                last = Some(status.clone());
                sender.send(wrap(status)).is_ok()
            };
            while !stop.load(Ordering::Relaxed) {
                let Ok(mut client) = Client::connect(&interface) else {
                    if !tell(None) {
                        return;
                    }
                    std::thread::sleep(RETRY);
                    continue;
                };
                while !stop.load(Ordering::Relaxed) {
                    let status = match client.next(Some(SILENCE)) {
                        Ok(Some(Block::Status(status))) => Some(read(&status, &interface)),
                        Ok(Some(Block::Reply(_))) => continue,
                        // Silent too long, or gone.
                        Ok(None) | Err(_) => None,
                    };
                    let gone = status.is_none();
                    if !tell(status) {
                        return;
                    }
                    if gone {
                        break;
                    }
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A snapshot as gptpd sends it.
    const SNAPSHOT: &str = "\
version 1
clock 001b21.fffe.f40f14
domain 0
priority1 248
profile gptp
fallback none
grandmaster 0001f2.fffe.ff3b14
grandmaster.priority1 246
grandmaster.steps_removed 1
grandmaster.is_us false
path 0001f2.fffe.ff3b14 001b21.fffe.f40f14
generation 3
residual -12
port enp2s0
port.up enp2s0 true
port.as_capable enp2s0 true
port.role enp2s0 timereceiver
port.link_delay enp2s0 305
counters.grandmaster_changes 1

";

    fn parsed(text: &str) -> gptp::status::Status {
        match Block::parse(text) {
            Ok(Block::Status(status)) => status,
            other => panic!("not a status: {other:?}"),
        }
    }

    #[test]
    fn reads_what_the_entities_report() {
        let status = read(&parsed(SNAPSHOT), "enp2s0");
        assert_eq!(status.own_clock, ClockIdentity(0x001b_21ff_fef4_0f14));
        assert_eq!(
            status.gptp.grandmaster,
            ClockIdentity(0x0001_f2ff_feff_3b14)
        );
        assert_eq!(status.gptp.propagation_delay, 305);
        assert!(status.gptp.as_capable);
        assert_eq!(
            status.gptp.path,
            [
                ClockIdentity(0x0001_f2ff_feff_3b14),
                ClockIdentity(0x001b_21ff_fef4_0f14)
            ]
        );
        assert_eq!((status.priority1, status.residual), (248, Some(-12)));
        assert_eq!(status.lite, None);
        assert!(status.named && !status.is_us);
    }

    #[test]
    fn avb_lite_says_why_it_fell_back() {
        let lite = SNAPSHOT
            .replace("profile gptp", "profile avb-lite")
            .replace("fallback none", "fallback pdelay-unanswered");
        assert_eq!(
            read(&parsed(&lite), "enp2s0").lite,
            Some(FallbackReason::PDELAY_UNANSWERED)
        );
    }

    /// With no grandmaster yet, as gptpd starts, the entity names none
    /// rather than itself, which may never be grandmaster; it is alone on
    /// the path.
    #[test]
    fn without_a_grandmaster_it_names_none() {
        let mut status = parsed(SNAPSHOT);
        status.grandmaster = None;
        status.path.clear();
        let alone = read(&status, "enp2s0");
        assert_eq!(alone.gptp.grandmaster, ClockIdentity(0));
        assert_eq!(alone.gptp.path, [alone.own_clock]);
        assert!(!alone.named && !alone.is_us);
    }
}
