//! OP opcode instructions.
//!
//! OP instructions perform integer computations on two registers, writing the result to a register.

mod arithmetic;
mod compare;
mod logic;
mod shift;

pub use arithmetic::ArithmeticInstruction;
pub use compare::{SltInstruction, SltUInstruction};
pub use logic::{AndInstruction, OrInstruction, XorInstruction};
pub use shift::{SllInstruction, SrInstruction};

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{Instruction, InstructionParts};

/// OP opcode handler.
pub struct OpHandler;

impl OpcodeHandler for OpHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;

        Ok(match instruction.funct3 & 0b111 {
            0b000 => Box::new(ArithmeticInstruction::new(&instruction)?),
            0b001 => Box::new(SllInstruction::new(&instruction)),
            0b010 => Box::new(SltInstruction::new(&instruction)),
            0b011 => Box::new(SltUInstruction::new(&instruction)),
            0b100 => Box::new(XorInstruction::new(&instruction)),
            0b101 => Box::new(SrInstruction::new(&instruction)?),
            0b110 => Box::new(OrInstruction::new(&instruction)),
            0b111 => Box::new(AndInstruction::new(&instruction)),
            _ => unreachable!("Masked to lowest 3 bits"),
        })
    }
}
