//! The playback clock pictures follow: set from the audio device while a
//! file has audio, so pictures keep to the sound, and from the system's
//! clock otherwise. It waits while a stream reads ahead.

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
    /// Held while the timeline is dragged, without pausing.
    held: bool,
    /// Since when it has waited for the source to read far enough ahead
    /// to play on without stopping again.
    waiting: Option<Instant>,
}

impl State {
    fn running(&self) -> bool {
        !self.paused && !self.stalled && !self.held && self.waiting.is_none()
    }

    /// Notes where the clock is now, before something changes how it runs.
    fn settle(&mut self) {
        let now = Instant::now();
        self.position = self.position_at(now);
        self.at = now;
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
                held: false,
                waiting: None,
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
            state.settle();
            state.stalled = true;
        }
    }

    /// The sound ran out while playing: stops until it comes again, and
    /// waits for the source to read ahead, so it does not run out again at
    /// once.
    pub fn ran_out(&self) {
        let mut state = self.state();
        if state.running() {
            state.waiting = Some(Instant::now());
        }
        if !state.stalled {
            state.settle();
            state.stalled = true;
        }
    }

    /// Waits for the source to read ahead, as after a seek.
    pub fn wait(&self) {
        let mut state = self.state();
        if state.waiting.is_none() {
            state.settle();
            state.waiting = Some(Instant::now());
        }
    }

    /// The source has read far enough ahead: plays on.
    pub fn end_wait(&self) {
        let mut state = self.state();
        if state.waiting.is_some() {
            state.settle();
            state.waiting = None;
        }
    }

    /// How long the clock has waited for the source, if it is waiting.
    pub fn waited(&self) -> Option<Duration> {
        self.state().waiting.map(|since| since.elapsed())
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

    /// Holds the clock where it is while the timeline is dragged, or lets
    /// it go on, paused or playing as it was.
    pub fn hold(&self, held: bool) {
        let mut state = self.state();
        if state.held == held {
            return;
        }
        let now = Instant::now();
        state.position = state.position_at(now);
        state.at = now;
        state.held = held;
    }

    pub fn held(&self) -> bool {
        self.state().held
    }

    /// Whether the clock is kept still, paused, held or waiting for the
    /// source, so the sound waits where it is.
    pub fn still(&self) -> bool {
        let state = self.state();
        state.paused || state.held || state.waiting.is_some()
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
    fn held_clock_stands_still_and_goes_on_after() {
        let clock = Clock::new();
        clock.set_paused(false);
        clock.anchor(3.0, Instant::now());
        clock.hold(true);
        let held = clock.now();
        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(clock.now(), held);
        assert!(clock.still() && !clock.paused());
        clock.jump(8.0);
        clock.hold(false);
        assert!(clock.running());
        assert!(clock.now() >= 8.0);
    }

    #[test]
    fn running_out_waits_until_the_source_reads_ahead() {
        let clock = Clock::new();
        clock.set_paused(false);
        clock.anchor(4.0, Instant::now());
        clock.ran_out();
        assert!(clock.waited().is_some() && clock.still());
        // Sound coming back does not end the wait by itself.
        clock.anchor(4.0, Instant::now());
        assert!(!clock.running());
        clock.end_wait();
        assert!(clock.running() && clock.waited().is_none());
        // Running out again while stalled after a seek is no new wait.
        clock.stall();
        clock.ran_out();
        assert!(clock.waited().is_none());
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
