//! OP-IMM opcode instructions.
//!
//! OP-IMM instructions perform integer computations on a register and an immediate value, writing
//! the result to a register.

mod add;
mod compare;
mod logic;
mod shift;

pub use add::AddIInstruction;
pub use compare::{SltIInstruction, SltIUInstruction};
pub use logic::{AndIInstruction, OrIInstruction, XorIInstruction};
pub use shift::{SllIInstruction, SrIInstruction};
use z2l_core::error::ProcessorException;

use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{Instruction, InstructionParts};

/// OP-IMM opcode handler.
pub struct OpImmHandler;

impl OpcodeHandler for OpImmHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;

        Ok(match instruction.funct3 & 0b111 {
            0b000 => Box::new(AddIInstruction::new(&instruction)),
            0b001 => Box::new(SllIInstruction::new(&instruction)),
            0b010 => Box::new(SltIInstruction::new(&instruction)),
            0b011 => Box::new(SltIUInstruction::new(&instruction)),
            0b100 => Box::new(XorIInstruction::new(&instruction)),
            0b101 => Box::new(SrIInstruction::new(&instruction)?),
            0b110 => Box::new(OrIInstruction::new(&instruction)),
            0b111 => Box::new(AndIInstruction::new(&instruction)),
            _ => unreachable!("Masked to lowest 3 bits"),
        })
    }
}
