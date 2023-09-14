//! AUIPC instruction handler.
//!
//! The AUIPC instruction adds an immediate offset to the current program counter, storing the
//! result in a register.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::processor::register::RegisterFile;

/// AUIPC opcode handler.
pub struct AUIPCHandler;

impl OpcodeHandler for AUIPCHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(AUIPCInstruction::new(&instruction, pc)))
    }
}

/// AUIPC instruction.
pub struct AUIPCInstruction {
    pc: u32,
    imm: i32,
    dest: u8,
}

impl AUIPCInstruction {
    /// Create a new AUIPCInstruction.
    pub fn new(instruction: &InstructionWordParts, pc: u32) -> Self {
        Self {
            pc,
            imm: instruction.imm_u,
            dest: instruction.rd,
        }
    }
}

impl Instruction for AUIPCInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let result = (self.pc as i32).wrapping_add(self.imm);

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("auipc x{}, 0x{:08x}", self.dest, self.imm)
    }
}
