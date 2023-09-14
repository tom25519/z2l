//! OP-IMM comparison instructions.
//!
//! These instructions set rd to 1 if rs1 is less than an immediate, and 0 otherwise. SLTI performs
//! a signed comparison, SLTIU performs an unsigned comparison.

use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};

use z2l_core::processor::register::RegisterFile;

/// SLTI instruction.
pub struct SltIInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl SltIInstruction {
    /// Create a new SltIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SltIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = if src < self.imm { 1 } else { 0 };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("slti x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}

/// SLTIU instruction.
pub struct SltIUInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl SltIUInstruction {
    /// Create a new SltIUInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SltIUInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()? as u32;

        let result = if src < self.imm as u32 { 1 } else { 0 };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("sltiu x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}
