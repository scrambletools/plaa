//! The playback clock pictures follow: set from the audio device while a
//! file has audio, so pictures keep to the sound, and from the system's
//! clock otherwise.

use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Clock {
    state: Mutex<State>,
}

#[derive(Debug, Clone, Copy)]
struct State {
    /// Media seconds at `at`.
    position: f64,
    at: Instant,
    /// Media seconds per second: the playback speed.
    rate: f64,
    /// Held by the user.
    paused: bool,
    /// Waiting for sound, after a seek or while a stream buffers.
    stalled: bool,
}

impl State {
    fn running(&self) -> bool {
        !self.paused && !self.stalled
    }

    fn position_at(&self, instant: Instant) -> f64 {
        if !self.running() {
            return self.position;
        }
        let elapsed = match instant.checked_duration_since(self.at) {
            Some(elapsed) => elapsed.as_secs_f64(),
            None => -self.at.duration_since(instant).as_secs_f64(),
        };
        self.position + elapsed * self.rate
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State {
                position: 0.0,
                at: Instant::now(),
                rate: 1.0,
                paused: true,
                stalled: false,
            }),
        }
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The media time now, in seconds.
    pub fn now(&self) -> f64 {
        self.at(Instant::now())
    }

    /// The media time at `instant`.
    pub fn at(&self, instant: Instant) -> f64 {
        self.state().position_at(instant)
    }

    /// When the media time reaches `position`, if the clock runs.
    pub fn instant_of(&self, position: f64) -> Option<Instant> {
        let state = self.state();
        if !state.running() || state.rate <= 0.0 {
            return None;
        }
        let ahead = (position - state.position_at(Instant::now())) / state.rate;
        let now = Instant::now();
        Some(if ahead <= 0.0 {
            now
        } else {
            now + Duration::from_secs_f64(ahead.min(3600.0))
        })
    }

    /// Says the sound at media time `position` is heard at `at`, which
    /// also ends a stall.
    pub fn anchor(&self, position: f64, at: Instant) {
        let mut state = self.state();
        state.position = position;
        state.at = at;
        state.stalled = false;
    }

    /// Moves the clock to `position`, as after a seek.
    pub fn jump(&self, position: f64) {
        let mut state = self.state();
        state.position = position;
        state.at = Instant::now();
    }

    /// Stops the clock where it is until sound arrives again.
    pub fn stall(&self) {
        let mut state = self.state();
        if !state.stalled {
            let now = Instant::now();
            state.position = state.position_at(now);
            state.at = now;
            state.stalled = true;
        }
    }

    pub fn set_paused(&self, paused: bool) {
        let mut state = self.state();
        if state.paused == paused {
            return;
        }
        let now = Instant::now();
        state.position = state.position_at(now);
        state.at = now;
        state.paused = paused;
    }

    pub fn paused(&self) -> bool {
        self.state().paused
    }

    pub fn running(&self) -> bool {
        self.state().running()
    }

    pub fn set_rate(&self, rate: f64) {
        let mut state = self.state();
        let now = Instant::now();
        state.position = state.position_at(now);
        state.at = now;
        state.rate = rate;
    }

    pub fn rate(&self) -> f64 {
        self.state().rate
    }

    /// Media seconds a second the clock moves at now: its rate while it
    /// runs, else none.
    pub fn pace(&self) -> f64 {
        let state = self.state();
        if state.running() { state.rate } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paused_clock_stands_still() {
        let clock = Clock::new();
        clock.jump(5.0);
        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(clock.now(), 5.0);
        assert_eq!(clock.instant_of(6.0), None);
        assert_eq!(clock.pace(), 0.0);
    }

    #[test]
    fn running_clock_follows_its_anchor_and_rate() {
        let clock = Clock::new();
        clock.set_paused(false);
        let start = Instant::now();
        clock.anchor(10.0, start);
        clock.set_rate(2.0);
        let later = start + Duration::from_secs(1);
        let position = clock.at(later);
        assert!((position - 12.0).abs() < 0.01, "{position}");
        assert_eq!(clock.pace(), 2.0);
    }

    #[test]
    fn stall_holds_until_anchored() {
        let clock = Clock::new();
        clock.set_paused(false);
        clock.anchor(1.0, Instant::now());
        clock.stall();
        let held = clock.now();
        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(clock.now(), held);
        assert_eq!(clock.pace(), 0.0);
        clock.anchor(2.0, Instant::now());
        assert!(clock.running());
    }
}
