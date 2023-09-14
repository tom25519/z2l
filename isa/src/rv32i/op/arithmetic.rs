//! OP arithmetic instructions (ADD, SUB).
//!
//! These instructions add or subtract rs2 from rs1, storing the result in rd.

use std::fmt;
use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// Operation to perform.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Add,
    Sub,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => f.write_str("add"),
            Operation::Sub => f.write_str("sub"),
        }
    }
}

/// An ADD or SUB instruction.
pub struct ArithmeticInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
    op: Operation,
}

impl ArithmeticInstruction {
    /// Create a new ArithmeticInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let op = match instruction.funct7 {
            0b0000000 => Operation::Add,
            0b0100000 => Operation::Sub,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
            op,
        })
    }
}

impl Instruction for ArithmeticInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = match self.op {
            Operation::Add => src1.wrapping_add(src2),
            Operation::Sub => src1.wrapping_sub(src2),
        };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("{} x{}, x{}, x{}", self.op, self.dest, self.src1, self.src2)
    }
}
