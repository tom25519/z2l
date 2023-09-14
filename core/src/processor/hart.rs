//! Hart implementation.
//!
//! This module defines the [`Hart`] struct, which represents a single hardware thread, which runs
//! instructions in sequence. A processor can consist of multiple such harts, running in parallel.

use crate::error::{ProcessorException, WithPC};
use crate::extension::OpcodeHandler;
use crate::instruction::{Instruction, InstructionParts};
use crate::mmu::{LoadSpec, StoreSpec};
use crate::processor::register::{GeneralPurposeRegister, RegisterFile, ZeroRegister};
use std::collections::{BTreeMap, HashMap};

/// Memory accesses required by the hart after a cycle.
///
/// Each instruction may require a value to be loaded from memory before it can be executed, or may
/// require a value to be stored to memory following its execution. This struct, the return value of
/// [`Hart::cycle`], informs the processor of such accesses, so that they can be performed before
/// the next cycle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemoryAccess {
    /// Value which must be loaded from memory before the next instruction can execute.
    pub load: Option<LoadSpec>,

    /// Value which must be stored to memory, having executed an instruction.
    pub store: Option<StoreSpec>,
}

/// A hardware thread.
pub struct Hart {
    /// Registers of this hart.
    pub registers: RegisterFile,

    /// The program counter.
    ///
    /// This stores the memory address of the instruction to execute next.
    pub pc: u32,

    /// Stores the memory adress of the instruction executed most recently.
    pub prev_pc: u32,

    /// Opcode handlers used to decode instructions.
    ///
    /// Each extension adds a number of opcode handlers to this field. Each opcode handler will be
    /// called if the hart encounters an instruction with the relevant opcode. It is the
    /// responsibility of the opcode handler to decode the provided instruction to produce an
    /// [`Instruction`], which may be executed on the next cycle.
    pub opcodes: HashMap<u8, Box<dyn OpcodeHandler>>,

    /// The previous instruction executed by this hart.
    ///
    /// Used for UI/debugging purposes.
    pub last_instr: Option<String>,

    /// Instruction decoded on the previous cycle.
    ///
    /// If this is `None`, the execute portion of this cycle will not run: only the decode portion.
    /// This happens when the processor jumps/branches.
    next_instr: Option<Result<Box<dyn Instruction>, ProcessorException>>,
}

impl Hart {
    /// Create a new Hart.
    ///
    /// This Hart will start executing instructions at address `0x00000000`. Change the value of
    /// `Hart::pc` to change this.
    pub fn new() -> Self {
        let mut registers: RegisterFile = BTreeMap::new();
        registers.insert(0, Box::new(ZeroRegister));
        for i in 1..32 {
            registers.insert(i, Box::new(GeneralPurposeRegister::new()));
        }

        Self {
            registers,
            pc: 0,
            prev_pc: 0,
            opcodes: HashMap::with_capacity(256),
            last_instr: None,
            next_instr: None,
        }
    }

    /// Reset the hart.
    ///
    /// On the next cycle, the hart will resume execution at address 0, discarding any intermediate
    /// instruction decodings to execute.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.prev_pc = 0;
        self.last_instr = None;
        self.next_instr = None;
    }

    /// Perform a single decode-execute cycle.
    ///
    /// `raw_instr` should be the 32-bit memory value starting at address `self.pc`.
    ///
    /// If the previous cycle's [`MemoryAccess`] return value specified a [`LoadSpec`], then `mem`
    /// should be the result of loading from memory according ot this spec. Otherwise, the value of
    /// `mem` is unspecified.
    ///
    /// If the cycle was successful, returns a [`MemoryAccess`] value indicating whether data needs
    /// to be loaded from/stored to memory before the next cycle.
    ///
    /// If an exception occurs, currently returns a [`ProcessorException`], together with the
    /// address of the instruction which caused the exception. In the future exceptions will be
    /// handled by higher-privileged trap handlers.
    pub fn cycle(
        &mut self,
        raw_instr: u32,
        mem: i32,
    ) -> Result<MemoryAccess, (ProcessorException, u32)> {
        let cur_pc = self.pc;
        let mut next_pc = self.pc + 4;

        // Decode the next instruction
        let mut next_instr = Some(self.decode(raw_instr));

        // Execute the current instruction
        let store = match &self.next_instr {
            Some(Ok(instr)) => {
                self.last_instr = Some(instr.format());

                let result = instr.execute(&mut self.registers, mem).with_pc(cur_pc)?;

                // If the instruction specifies a jump, invalidate the next instruction decoding and
                // set the pc as required.
                if let Some(pc) = result.jump {
                    next_instr = None;
                    next_pc = pc;
                }

                result.store
            }
            Some(Err(e)) => return Err((*e, cur_pc)),
            None => {
                self.last_instr = None;
                None
            }
        };

        // Determine memory load spec for use by the next instruction
        let mut load = None;
        if let Some(Ok(instr)) = &next_instr {
            load = instr.load(&self.registers).with_pc(next_pc)?;
        }

        // Update state for next instruction.
        self.pc = next_pc;
        self.prev_pc = cur_pc;
        self.next_instr = next_instr;

        Ok(MemoryAccess { load, store })
    }

    /// Decode the provided raw instruction.
    fn decode(&self, raw_instr: u32) -> Result<Box<dyn Instruction>, ProcessorException> {
        let parts = InstructionParts::new(raw_instr)?;
        let handler = self
            .opcodes
            .get(&parts.opcode())
            .ok_or(ProcessorException::IllegalInstruction)?;
        handler.decode(parts, self.pc)
    }
}
