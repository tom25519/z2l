//! Processor implementation.
//!
//! This module defines the [`Processor`] struct, which is composed of a number of [`Hart`]s. These
//! implement a basic decode-execute pipeline. Each cycle, [`Processor::cycle`] is called, and the
//! following occurs:
//! * The processor retrieves the instructions at the memory addresses specified by each hart's
//!   [`Hart::pc`] value
//! * The processor retrieves the value at the memory locations specified by each hart in its
//!   [`hart::MemoryAccess`] return value last cycle
//! * The processor calls [`Hart::cycle`] with the fetched instruction & value
//!     * The hart decodes the fetched instruction, and determines if it requires a memory load
//!         * If a memory load is required, this is indicated in the function return value
//!         * This decoded instruction is stored in the struct, to be executed next time
//!           [`Hart::cycle`] is called
//!     * The hart executes the instruction decoded in the previous cycle, supplying the memory
//!       value retrieved by the processor
//!         * The instruction indicates whether if a memory store is required: If so, this is
//!           indicated in the function return value
//! * If the return value of `Hart::cycle` indicates a store is required, the processor stores the
//!   provided value to memory at the provided address.
//!
//! Actual instruction behaviour is specified separately, in [`Extension`]s.

pub mod hart;
pub mod register;

use crate::error::{ProcessorException, WithPC};
use crate::extension::Extension;
use crate::mmu::{LoadSpec, MMU};
use hart::Hart;
use std::fmt;
use std::sync::{Arc, RwLock};

/// Configuration to instantiate a processor.
pub struct ProcessorConfig {
    /// Number of hardware threads (harts) to run.
    ///
    /// Each hart runs in its own thread on the host hardware.
    ///
    /// Currently unused.
    pub harts: usize,

    /// MMU for the system.
    pub mmu: Arc<RwLock<MMU>>,

    /// Extensions to support.
    ///
    /// This includes the base integer instruction set to use, plus any extensions.
    pub extensions: Vec<Box<dyn Extension>>,
}

impl fmt::Debug for ProcessorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extensions: Vec<&str> = self.extensions.iter().map(|e| e.code()).collect();
        f.write_fmt(format_args!(
            "ProcessorConfig {{ harts: {:?}, mmu: {:?}, extensions: {} }}",
            self.harts,
            self.mmu,
            extensions.join("")
        ))
    }
}

/// A RISC-V processor.
pub struct Processor {
    /// The single hart powering this processor.
    ///
    /// In the future, this will be replaced with multiple `HartManager`s, each managing a hart
    /// running on a separate thread.
    pub hart: Hart,

    /// MMU for the system.
    pub mmu: Arc<RwLock<MMU>>,

    /// Memory load request from the previous cycle.
    ///
    /// If `None`, no memory load is required for the instruction the hart will execute next, and
    /// any value may be legally supplied. Otherwise, the processor should fetch a memory value
    /// according to the provided specification, and supply this to the hart.
    load: Option<LoadSpec>,

    /// Program counter value of the hart at the previous cycle.
    prev_pc: u32,
}

impl Processor {
    /// Create a new [`Processor`].
    pub fn new(config: ProcessorConfig) -> Self {
        let mut hart = Hart::new();

        for extension in config.extensions {
            extension.register(&mut hart);
        }

        Self {
            hart,
            mmu: config.mmu,
            load: None,
            prev_pc: 0,
        }
    }

    /// Reset the processor.
    ///
    /// Resets each hart of the processor, so at the next cycle, instruction will start executing
    /// instructions at address 0.
    pub fn reset(&mut self) {
        self.hart.reset();
        self.load = None;
        self.prev_pc = 0;
    }

    /// Execute a processor cycle.
    ///
    /// Currently this returns a [`ProcessorException`] with the program counter indicating the
    /// location of the instruction which caused the exception if there is any exception, since
    /// M-mode is not yet implemented, so software exception-handling is not possible. In the
    /// future, the processor will only return an error (or some other indicator value) if a reset
    /// or halt is requested.
    pub fn cycle(&mut self) -> Result<(), (ProcessorException, u32)> {
        let prev_pc = self.prev_pc;
        let cur_pc = self.hart.pc;

        // Fetch the next instruction
        let mmu = self.mmu.read().unwrap();
        let instr = mmu.load_word(cur_pc as usize).with_pc(cur_pc)? as u32;

        // Fetch the memory value requested by the current instruction
        let mem = if let Some(access) = self.load {
            mmu.load(access).with_pc(prev_pc)?
        } else {
            0
        };
        drop(mmu);

        // Execute the current instruction & decode the next instruction
        let result = self.hart.cycle(instr, mem)?;

        // Store to memory if required by the current instruction
        if let Some(store) = result.store {
            let mut mmu = self.mmu.write().unwrap();
            mmu.store(store).with_pc(prev_pc)?;
        }

        // Save PC & memory load requests for next instruction
        self.prev_pc = cur_pc;
        self.load = result.load;

        Ok(())
    }
}
