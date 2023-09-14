//! Non-blocking clock.

use crate::clock::{Clock, ClockStatus};
use log::trace;

/// A "free" clock which will tick without blocking whenever requested.
///
/// The `next_tick` method of this clock will always return immediately, without blocking. This
/// means that a processor using this clock will attempt to run as fast as the host hardware will
/// allow, without trying to stick to some specific clock speed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FreeClock {}

impl FreeClock {
    /// Create a new `FreeClock`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Clock for FreeClock {
    #[inline(always)]
    fn next_tick(&mut self) -> ClockStatus {
        trace!("Tick");
        ClockStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use crate::clock::{Clock, ClockStatus, FreeClock};
    use std::time::{Duration, Instant};

    #[test]
    fn does_not_add_overhead() {
        let mut clock = FreeClock::new();
        let start = Instant::now();

        for _ in 0..500 {
            assert_eq!(clock.next_tick(), ClockStatus::Ok);
        }

        assert!(start.elapsed() < Duration::from_millis(1));
    }
}

