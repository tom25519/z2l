//! Fixed-frequency clock.

use crate::clock::{Clock, ClockStatus};
use log::{debug, trace};
use std::time::{Duration, Instant};

/// A clock which runs at a fixed frequency.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FixedClock {
    period: Duration,
    prev_tick: Instant,
}

impl FixedClock {
    /// Create a new [`FixedClock`] which generates a clock pulse every `period`.
    ///
    /// # Examples
    /// ```rust
    /// # use std::time::Duration;
    /// # use z2l_core::clock::FixedClock;
    /// // This clock will run at 20Hz
    /// let clock_a = FixedClock::new(Duration::from_millis(50));
    ///
    /// // This clock will run at 1MHz
    /// let clock_b = FixedClock::new(Duration::from_micros(1));
    /// ```
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            prev_tick: Instant::now(),
        }
    }
}

impl Clock for FixedClock {
    fn next_tick(&mut self) -> ClockStatus {
        trace!("Blocking on tick");

        let elapsed = self.prev_tick.elapsed();
        let mut wait_period = self.period;

        // Determine if any ticks have been missed
        let mut missed = 0;
        if elapsed > wait_period {
            // We have missed at least one tick: Work out how many and determine the next tick
            // boundary
            missed = (elapsed.as_nanos() / self.period.as_nanos()) as usize;
            wait_period += (missed as u32) * self.period;
        }

        // Wait until the next tick boundary is reached
        let mut elapsed = self.prev_tick.elapsed();
        while elapsed < wait_period {
            elapsed = self.prev_tick.elapsed();
            std::hint::spin_loop();
        }

        trace!("Ticking {:?} after last tick", elapsed);

        self.prev_tick = self.prev_tick + wait_period;

        if missed == 0 {
            trace!("Tick");
            ClockStatus::Ok
        } else {
            debug!("Tick: Missed {} ticks", missed);
            ClockStatus::MissedTicks(missed)
        }
    }

    fn reset(&mut self) {
        trace!("Reset");
        self.prev_tick = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use crate::clock::{Clock, ClockStatus, FixedClock};
    use std::time::{Duration, Instant};

    /// Interval to pass to the [`FixedClock`] constructor, determining the speed at which the clock
    /// will run.
    ///
    /// We use a period of 20ms (=> 50Hz clock) here, as at this speed, it is unlikely a tick will
    /// ever be missed on modern processors. If tests are failing, tweak this duration & the slack
    /// duration.
    const PERIOD: Duration = Duration::from_millis(20);

    /// [`PERIOD`] with some slack (5ms default).
    ///
    /// Ticks should finish between [`PERIOD`] and this value.
    // n.b: We can't just add the two Durations here, as adding Durations is non-const.
    const PERIOD_END: Duration = Duration::from_millis((PERIOD.as_millis() + 5) as u64);

    #[test]
    fn run_at_set_frequency() {
        let mut clock = FixedClock::new(PERIOD);
        let start = Instant::now();

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= PERIOD);
        assert!(start.elapsed() < PERIOD_END);

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= 2 * PERIOD);
        assert!(start.elapsed() < 2 * PERIOD_END);

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= 4 * PERIOD);
        assert!(start.elapsed() <= 4 * PERIOD_END);

        for _ in 0..50 {
            clock.next_tick();
        }

        assert!(start.elapsed() >= 54 * PERIOD);
        assert!(start.elapsed() <= 54 * PERIOD_END);
    }

    #[test]
    fn detect_missed_ticks() {
        let mut clock = FixedClock::new(PERIOD);
        let start = Instant::now();

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= PERIOD);
        assert!(start.elapsed() < PERIOD_END);

        // Sleep for 50ms: Should miss 2 ticks
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(clock.next_tick(), ClockStatus::MissedTicks(2));

        // Sleep for 19ms: Should not miss the tick
        std::thread::sleep(Duration::from_millis(19));
        assert_eq!(clock.next_tick(), ClockStatus::Ok);
    }

    #[test]
    fn stay_on_clock_after_missed_ticks() {
        let mut clock = FixedClock::new(PERIOD);
        let start = Instant::now();

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= PERIOD);
        assert!(start.elapsed() < PERIOD_END);

        // Sleep for 70ms: Should miss 3 ticks
        std::thread::sleep(Duration::from_millis(70));

        // Ensure ticking gets us back on track with the cycle: We ticked once, then missed 3 ticks,
        // so when next_tick is called, 4 ticks of real time have elapsed. Therefore, we should
        // block until the next tick boundary (5 ticks).
        clock.next_tick();
        assert!(start.elapsed() >= 5 * PERIOD);
        assert!(start.elapsed() < 5 * PERIOD_END);
    }

    #[test]
    fn reset_restarts_timer() {
        let mut clock = FixedClock::new(PERIOD);
        let start = Instant::now();

        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(start.elapsed() >= PERIOD);
        assert!(start.elapsed() < PERIOD_END);

        // We've just called `next_tick`, so it should be ~20ms until the next tick. Sleep 5ms, then
        // call `reset`, then `next_tick` again, and ensure we wait 20ms from the time of *reset*.
        std::thread::sleep(Duration::from_millis(5));
        let reset = Instant::now();
        clock.reset();
        assert_eq!(clock.next_tick(), ClockStatus::Ok);
        assert!(reset.elapsed() >= PERIOD);
        assert!(reset.elapsed() < PERIOD_END);
    }
}

