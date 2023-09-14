//! FENCE opcode instructions.
//!
//! FENCE instructions coordinate memory accesses.
//!
//! In our implementation, FENCE instructions from the base instruction set are just no-ops, as we
//! never execute memory operations out-of-order.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::processor::register::RegisterFile;

/// FENCE opcode handler.
pub struct FenceHandler;

impl OpcodeHandler for FenceHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(FenceInstruction::new(&instruction)?))
    }
}

/// Mode of this fence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FenceMode {
    /// Normal fence.
    Normal,

    /// Total-Store-Ordering fence.
    TSO,
}

/// FENCE instruction.
pub struct FenceInstruction {
    mode: FenceMode,
    pi: bool,
    po: bool,
    pr: bool,
    pw: bool,
    si: bool,
    so: bool,
    sr: bool,
    sw: bool,
}

impl FenceInstruction {
    /// Create a new FenceInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let mode = match (instruction.raw >> 28) as u8 & 0b1111 {
            0b0000 => FenceMode::Normal,
            0b1000 => FenceMode::TSO,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            mode,
            pi: instruction.raw & 0x08000000 != 0,
            po: instruction.raw & 0x04000000 != 0,
            pr: instruction.raw & 0x02000000 != 0,
            pw: instruction.raw & 0x01000000 != 0,
            si: instruction.raw & 0x00800000 != 0,
            so: instruction.raw & 0x00400000 != 0,
            sr: instruction.raw & 0x00200000 != 0,
            sw: instruction.raw & 0x00100000 != 0,
        })
    }
}

impl Instruction for FenceInstruction {
    fn execute(
        &self,
        _registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        // NOP: We always order device I/O and memory accesses exactly in the order they occur in
        // the actual program flow
        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        let mut instruction = String::with_capacity(20);

        match self.mode {
            FenceMode::Normal => instruction.push_str("fence"),
            FenceMode::TSO => instruction.push_str("fence.tso"),
        }

        let mut spaced = false;

        if self.pi {
            if !spaced {
                instruction.push(' ');
                spaced = true;
            }
            instruction.push('I');
        }
        if self.po {
            if !spaced {
                instruction.push(' ');
                spaced = true;
            }
            instruction.push('O');
        }
        if self.pr {
            if !spaced {
                instruction.push(' ');
                spaced = true;
            }
            instruction.push('R');
        }
        if self.pw {
            if !spaced {
                instruction.push(' ');
                spaced = true;
            }
            instruction.push('W');
        }

        if spaced {
            instruction.push_str(", ");
        }

        if self.si {
            instruction.push('I');
        }
        if self.so {
            instruction.push('O');
        }
        if self.sr {
            instruction.push('R');
        }
        if self.sw {
            instruction.push('W');
        }

        instruction
    }
}
