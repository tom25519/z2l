//! Comparison instructions (SLT, SLTU).
//!
//! These instructions set rd to 1 if rs1 is less than rs2: SLT performs a signed comparison, SLTU
//! unsigned.

use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// SLT Instruction.
pub struct SltInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl SltInstruction {
    /// Create a new SltInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SltInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = if src1 < src2 { 1 } else { 0 };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("slt x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}

/// SLTU instruction.
pub struct SltUInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl SltUInstruction {
    /// Create a new SltUInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SltUInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()? as u32;
        let src2 = registers.get(&self.src2).unwrap().load()? as u32;

        let result = if src1 < src2 { 1 } else { 0 };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("sltu x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}
