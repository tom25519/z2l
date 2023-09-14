//! ADDI instruction.
//!
//! The ADDI instruction adds the value of rs1 to an immediate value, storing the result in rd.

use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// ADDI instruction.
pub struct AddIInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl AddIInstruction {
    /// Create a new AddIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for AddIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        // TODO: Handle errors
        let src = registers.get(&self.src).unwrap().load()?;

        let result = src.wrapping_add(self.imm);

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("addi x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}
