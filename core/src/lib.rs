//! The core Z2L emulator runtime.
//!
//! This crate defines the core components required for the emulator: The core processor, clocks,
//! memory, etc.; plus the [`ExecutionEnvironment`] struct, which is a single interface linking all
//! of these emulated hardware components to provide a RISC-V bare-metal EEI.

pub mod clock;
pub mod error;
pub mod extension;
pub mod instruction;
pub mod mmu;
pub mod processor;
pub mod ram;
pub mod rom;

use crate::error::ProcessorException;
use bus::{Bus, BusReader};
use log::info;
use std::io::Read;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, RwLock};

/// A control message sent to the processor.
///
/// These control messages are used for inter-thread communication, sent via a [`Bus`](bus::Bus),
/// referred to as the control bus.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ControlMessage {
    /// Halt execution.
    ///
    /// Stops the processor execution with no possibility for resuming execution from the same state
    /// later on. This indicates resources consumed by the processor can be freed.
    Halt,

    /// Reset the processor.
    ///
    /// Reset all internal counters/registers/RAM, restart execution from the entrypoint.
    Reset,

    /// Manually advance the processor clock.
    ///
    /// This only functions if the [`ManualClock`](clock::ManualClock) is in use.
    ManualTick,
}

/// Configuration to instantiate an [`ExecutionEnvironment`].
pub struct Config<R, C> {
    /// Number of harts to run on the processor.
    ///
    /// Each hart runs in its own thread on the host hardware.
    ///
    /// Currently unused.
    pub harts: usize,

    /// Extensions to support.
    ///
    /// This includes the base integer instruction set to use, plus any extensions.
    pub extensions: Vec<Box<dyn extension::Extension>>,

    /// Rom from which execution should begin.
    ///
    /// The processor will start execution at address `0x00000000` of this ROM. This could be used
    /// to define a bootloader, or just a small RISC-V program which does not need to dynamically
    /// load any program code.
    pub rom: R,

    /// Size for the RAM, in bytes.
    ///
    /// This will all be allocated upfront.
    pub ram_size: usize,

    /// [`Clock`](clock::Clock) to use to run the processor.
    pub clock: C,

    /// Receiver for the control bus.
    pub control_rx: BusReader<ControlMessage>,
}

/// Message indicating the state of the processor following each cycle.
#[derive(Clone, Debug, Hash)]
pub enum InstructionLog {
    /// Cycle completed successfully.
    Ok {
        /// Instruction executed most recently.
        ///
        /// If None, no instruction was executed on the most recent cycle, only decoded.
        instr: Option<String>,

        /// Current values of all registers.
        registers: Vec<i32>,

        /// Current value of the program counter.
        pc: u32,
    },

    /// An exception was encountered.
    Exception {
        /// The exception which occurred.
        exception: ProcessorException,

        /// Current values of all registers.
        registers: Vec<i32>,

        /// Current value of the program counter.
        pc: u32,
    },
}

/// A RISC-V system.
pub struct ExecutionEnvironment<C> {
    /// The RISC-V processor.
    processor: processor::Processor,

    /// The processor clock.
    clock: C,

    /// Control message bus.
    ///
    /// This is used to control processor operation (advance the manual clock, reset, halt, etc).
    control_rx: BusReader<ControlMessage>,

    /// Log message bus.
    ///
    /// This is used to report what the processor is doing to the UI.
    log_bus: Bus<InstructionLog>,
}

impl<C> ExecutionEnvironment<C>
where
    C: clock::Clock,
{
    /// Create a new RISC-V system.
    pub fn new<R: Read>(config: Config<R, C>) -> Result<Self, std::io::Error> {
        let rom = rom::ROM::from(config.rom)?;
        let ram = ram::RAM::new(config.ram_size);
        let mmu = Arc::new(RwLock::new(mmu::MMU::new(rom, ram)));

        let processor_config = processor::ProcessorConfig {
            harts: config.harts,
            mmu,
            extensions: config.extensions,
        };
        let processor = processor::Processor::new(processor_config);

        Ok(Self {
            processor,
            clock: config.clock,
            control_rx: config.control_rx,
            log_bus: Bus::new(0xffff),
        })
    }

    /// Add a log message receiver.
    pub fn add_rx(&mut self) -> BusReader<InstructionLog> {
        self.log_bus.add_rx()
    }

    /// Get the current values of all registers.
    fn get_registers(&self) -> Vec<i32> {
        self.processor
            .hart
            .registers
            .iter()
            .map(|(_, reg)| reg.load().unwrap_or(0))
            .collect()
    }

    /// Run the processor.
    ///
    /// This will block indefinitely, until the processor halts or encounters an unhandled
    /// exception.
    pub fn run(&mut self) {
        loop {
            loop {
                match self.control_rx.try_recv() {
                    Err(TryRecvError::Empty) => break,
                    Ok(ControlMessage::Reset) => {
                        info!("Received reset");
                        self.processor.reset();
                    }
                    Ok(ControlMessage::Halt) | Err(TryRecvError::Disconnected) => {
                        info!("Received halt");
                        return;
                    }
                    _ => continue,
                }
            }

            self.clock.next_tick();

            match self.processor.cycle() {
                Ok(()) => self.log_bus.broadcast(InstructionLog::Ok {
                    instr: self.processor.hart.last_instr.clone(),
                    registers: self.get_registers(),
                    pc: self.processor.hart.prev_pc,
                }),

                Err((exception, pc)) => {
                    self.log_bus.broadcast(InstructionLog::Exception {
                        exception,
                        registers: self.get_registers(),
                        pc,
                    });
                    return;
                }
            }
        }
    }
}
