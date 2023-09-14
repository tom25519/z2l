//! OP-IMM bitwise logical instructions.
//!
//! These instructions perform a bitwise logical operation on rs1 and an immediate, storing the
//! result in rd.

use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// ANDI instruction.
pub struct AndIInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl AndIInstruction {
    /// Create a new AndIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for AndIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = src & self.imm;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("andi x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}

/// ORI instruction.
pub struct OrIInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl OrIInstruction {
    /// Create a new OrIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for OrIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = src | self.imm;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("ori x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}

/// XORI instruction.
pub struct XorIInstruction {
    src: u8,
    imm: i32,
    dest: u8,
}

impl XorIInstruction {
    /// Create a new XorIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            imm: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for XorIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = src ^ self.imm;

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("xori x{}, x{}, 0x{:08x}", self.dest, self.src, self.imm)
    }
}
