//! Manually-advanced clock.

use crate::clock::{Clock, ClockStatus};
use crate::ControlMessage;
use bus::BusReader;
use log::{debug, trace};
use std::sync::mpsc::TryRecvError;

/// Manually-advanced, single-step clock.
///
/// This clock advances whenever it receives a [`ControlMessage::ManualTick`] on the emulator
/// control bus. This is intended to be used to allow a user to manually step through a RISC-V
/// program, for debugging purposes.
#[derive(Debug)]
pub struct ManualClock {
    control_rx: BusReader<ControlMessage>,
}

impl ManualClock {
    /// Create a new [`ManualClock`] which will listen for control messages on `control_rx`.
    pub fn new(control_rx: BusReader<ControlMessage>) -> Self {
        Self { control_rx }
    }

    /// Determine if we should return early from `next_tick` due to a message in the control bus
    /// receive buffer.
    ///
    /// This function loops through any messages which were sent before `next_tick` was called. If
    /// we have missed a tick, or received a control message indicating the processor should
    /// halt/reset, then we need to return early from `next_tick`.
    ///
    /// Returns `Some(status)` if we should return early, with `status` being the value to return.
    /// Otherwise returns `None`.
    fn consume_recv_buffer(&mut self) -> Option<ClockStatus> {
        let mut missed = 0;

        loop {
            #[allow(unreachable_patterns)]
            match self.control_rx.try_recv() {
                // Missed tick
                Ok(ControlMessage::ManualTick) => missed += 1,

                // Exhausted the receive buffer: No more missed messages
                Err(TryRecvError::Empty) => break,

                // The processor will halt or reset on the next clock cycle: Return ASAP
                Err(TryRecvError::Disconnected)
                | Ok(ControlMessage::Reset)
                | Ok(ControlMessage::Halt) => {
                    debug!("Missed disconnect/reset/halt");

                    if missed == 0 {
                        return Some(ClockStatus::Ok);
                    } else {
                        return Some(ClockStatus::MissedTicks(missed));
                    }
                }

                // Unrelated control message
                Ok(_) => continue,
            }
        }

        if missed == 0 {
            None
        } else {
            trace!("Missed ticks: {}", missed);
            Some(ClockStatus::MissedTicks(missed))
        }
    }
}

impl Clock for ManualClock {
    fn next_tick(&mut self) -> ClockStatus {
        if let Some(status) = self.consume_recv_buffer() {
            trace!("Early tick");
            return status;
        }

        loop {
            #[allow(unreachable_patterns)]
            match self.control_rx.recv() {
                Ok(ControlMessage::ManualTick) => {
                    trace!("Manual tick");
                    return ClockStatus::Ok;
                }

                Ok(ControlMessage::Reset) | Ok(ControlMessage::Halt) | Err(_) => {
                    debug!("Received disconnect/reset/halt");
                    return ClockStatus::Ok;
                }

                _ => continue,
            }
        }
    }
}
