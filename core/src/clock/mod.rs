//! Clocks for synchronising processor execution with a signal or fixed time period.
//!
//! In hardware, a clock signal is vital to synchronise the state of all internal components
//! (registers, cache, etc). In emulation, the clock is less important, as we can use sequential
//! logic to synchronise register access (/other timing-sensitive operations), but it may still be
//! desirable to control how fast the processor runs, rather than just running it as fast as
//! possible. For example, we may wish to run the processor at a fixed speed, or manually step
//! through instructions one-by-one for debugging process. Therefore, it is useful to define an
//! interface which can produce a clock signal, and run each processor cycle in line with this
//! clock.
//!
//! The [`Clock`] trait defined in this module represents an abstract clock which can synchronise
//! processor execution to some signal. The key method for doing this is via the
//! [`Clock::next_tick`] function, which blocks the current thread until the next clock pulse.
//! Pseudocode making use of this trait could be as follows:
//!
//! ```text
//! // Processor main loop
//! loop {
//!     // Block until the next clock pulse
//!     clock.next_tick()
//!
//!     // Run a single processor cycle (i.e. fetch, decode, and execute a single instruction)
//!     run_single_cycle()
//! }
//! ```
//!
//! # Available Clocks
//! Three [`Clock`] structs are provided:
//! * The [`FreeClock`] clock returns from `next_tick` immediately without blocking. This causes the
//!   processor to effectively run as fast as the host hardware will allow.
//! * The [`FixedClock`] clock attempts to run at a specific frequency as accurately as possible.
//! * The [`ManualClock`] clock only advances when it receives a control signal to do so from the
//!   user.

mod fixed;
mod free;
mod manual;

pub use fixed::FixedClock;
pub use free::FreeClock;
pub use manual::ManualClock;

/// Result of calling [`Clock::next_tick`], indicating whether any ticks were missed.
///
/// A tick is "missed" if it would have occurred before `next_tick` was called (i.e: while the
/// processor was still running the previous instruction). Missing lots of ticks may indicate the
/// clock is running too fast for the host hardware to handle.
///
/// How clocks should behave on missed ticks is not specified: The [`FixedClock`] will always block
/// until the next tick, even if previous ticks have been missed, while the [`ManualClock`] will
/// immediately return if a tick has been missed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClockStatus {
    /// No ticks were missed.
    Ok,

    /// Some ticks were missed.
    MissedTicks(usize),
}

/// Trait for types which can produce a clock signal.
pub trait Clock: Send + Sync + 'static {
    /// Block until the next tick completes.
    ///
    /// What "next tick" means here is somewhat implementation-defined: If this is a fixed clock,
    /// for example, pulsing every 10ms, then you might wish to wait until the next 10ms boundary,
    /// even if ticks have been missed beforehand. Whereas, if this is a clock which is manually
    /// advanced by the user, and the user requested a tick which has been missed, then you may wish
    /// to immediately return.
    ///
    /// In the emulator, this is run in a loop, called before each processor cycle to synchronise
    /// the execution with the clock.
    ///
    /// Returns a [`ClockStatus`] indicating whether any ticks were missed.
    fn next_tick(&mut self) -> ClockStatus;

    /// Reset the clock.
    ///
    /// This is called when a processor reset is triggered, and the clock counter restarts. After a
    /// reset, the processor will immediately begin to try to make progress by calling
    /// [`Clock::next_tick`].
    ///
    /// By default, this does nothing, however clocks may wish to implement custom behaviour on
    /// reset.
    fn reset(&mut self) {}
}

impl Clock for Box<dyn Clock> {
    fn next_tick(&mut self) -> ClockStatus {
        self.as_mut().next_tick()
    }

    fn reset(&mut self) {
        self.as_mut().reset()
    }
}
