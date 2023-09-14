//! OP bitshifting operations.
//!
//! These instructions shift the positions of bits in rs1 by the value of rs2, storing the result in
//! rd. SLL performs a logical left-shift, SRL performs a logical right-shift (zero-extended), and
//! SRA performs an arithmetic right-shift (sign-extended).

use crate::rv32i::RightShiftBehaviour;
use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// SLL instruction.
pub struct SllInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
}

impl SllInstruction {
    /// Create a new SllInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SllInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = src1.wrapping_shl((src2 & 0b11111) as u32);

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("sll x{}, x{}, x{}", self.dest, self.src1, self.src2)
    }
}

/// SRL or SRA instruction.
pub struct SrInstruction {
    src1: u8,
    src2: u8,
    dest: u8,
    behaviour: RightShiftBehaviour,
}

impl SrInstruction {
    /// Create a new SrInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let behaviour = match instruction.funct7 {
            0b0000000 => RightShiftBehaviour::Logical,
            0b0100000 => RightShiftBehaviour::Arithmetic,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            src1: instruction.rs1,
            src2: instruction.rs2,
            dest: instruction.rd,
            behaviour,
        })
    }
}

impl Instruction for SrInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let result = match self.behaviour {
            RightShiftBehaviour::Logical => {
                (src1 as u32).wrapping_shr((src2 & 0b11111) as u32) as i32
            }
            RightShiftBehaviour::Arithmetic => src1.wrapping_shr((src2 & 0b11111) as u32),
        };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!(
            "sr{} x{}, x{}, x{}",
            self.behaviour, self.dest, self.src1, self.src2
        )
    }
}
