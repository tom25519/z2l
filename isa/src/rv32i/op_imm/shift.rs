//! OP-IMM bitshifting operations.
//!
//! These instructions shift the positions of bits in rs1 by an immediate value, storing the result
//! in rd. SLLI performs a logical left-shift, SRLI performs a logical right-shift (zero-extended),
//! and SRAI performs an arithmetic right-shift (sign-extended).

use crate::rv32i::RightShiftBehaviour;
use z2l_core::error::ProcessorException;
use z2l_core::instruction::{Instruction, InstructionResult, InstructionWordParts};
use z2l_core::processor::register::RegisterFile;

/// SLLI instruction.
pub struct SllIInstruction {
    src: u8,
    shift: u32,
    dest: u8,
}

impl SllIInstruction {
    /// Create a new SllIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Self {
        Self {
            src: instruction.rs1,
            shift: (instruction.imm_i & 0b11111) as u32,
            dest: instruction.rd,
        }
    }
}

impl Instruction for SllIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = src.wrapping_shl(self.shift);

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!("slli x{}, x{}, {}", self.dest, self.src, self.shift)
    }
}

/// SRLI or SRAI instruction.
pub struct SrIInstruction {
    src: u8,
    behaviour: RightShiftBehaviour,
    imm: u32,
    dest: u8,
}

impl SrIInstruction {
    /// Create a new SrIInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let behaviour = match instruction.imm_i & 0b111111100000 {
            0b0000000 => RightShiftBehaviour::Logical,
            0b0100000 => RightShiftBehaviour::Arithmetic,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            src: instruction.rs1,
            behaviour,
            imm: (instruction.imm_i & 0b11111) as u32,
            dest: instruction.rd,
        })
    }
}

impl Instruction for SrIInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;

        let result = match self.behaviour {
            RightShiftBehaviour::Logical => (src as u32).wrapping_shr(self.imm) as i32,
            RightShiftBehaviour::Arithmetic => src.wrapping_shr(self.imm),
        };

        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(result)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!(
            "sr{}i x{}, x{}, {}",
            self.behaviour, self.dest, self.src, self.imm
        )
    }
}
