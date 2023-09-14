//! OP logic instructions (AND, OR, XOR).
//!
//! These instructions perform bitwise logical operations on rs1 and rs2, storing the result in rd.

use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// AND instruction.
pub struct AndInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl AndInstruction {
    /// Create a new AndInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for AndInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = src1 & src2;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("and x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}

/// OR instruction.
pub struct OrInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl OrInstruction {
    /// Create a new OrInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for OrInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = src1 | src2;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("or x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}

/// XOR instruction.
pub struct XorInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl XorInstruction {
    /// Create a new XorInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for XorInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = src1 ^ src2;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("xor x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}
