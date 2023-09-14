//! LUI instruction handler.
//!
//! The LUI instruction places an immediate value into the high bits of a register, filling the low
//! bits with zeros.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::processor::register::RegisterFile;

/// LUI opcode handler.
pub struct LuiHandler;

impl OpcodeHandler for LuiHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(LuiInstruction::new(&instruction)))
    }
}

/// LUI instruction.
pub struct LuiInstruction {
    imm: i32,
    dest: u8,
}

impl LuiInstruction {
    /// Create a new LuiInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            imm: instruction.imm_u,
            dest: instruction.rd,
        }
    }
}

impl Instruction for LuiInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(self.imm)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("lui x{}, 0x{:08x}", self.dest, self.imm)
    }
}
