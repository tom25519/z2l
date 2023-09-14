//! BRANCH opcode instructions.
//!
//! BRANCH instructions conditionally jump to another location, depending on a comparison.

use std::fmt;
use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::processor::register::RegisterFile;

/// BRANCH opcode handler.
pub struct BranchHandler;

impl OpcodeHandler for BranchHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(BranchInstruction::new(&instruction, pc)?))
    }
}

/// Condition determining whether a branch should occur.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BranchCondition {
    /// Branch if the values are equal.
    Equal,

    /// Branch if the values are not equal.
    NotEqual,

    /// Branch if the first value is less than the second, when treated as signed values.
    Less,

    /// Branch if the first value is greater than/equal to the second, when treated as signed
    /// values.
    GreaterOrEqual,

    /// Branch if the first value is less than the second, when treated as unsigned values.
    LessUnsigned,

    /// Branch if the first value is greater than/equal to the second, when treated as unsigned
    /// values.
    GreaterOrEqualUnsigned,
}

impl fmt::Display for BranchCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchCondition::Equal => f.write_str("beq"),
            BranchCondition::NotEqual => f.write_str("bne"),
            BranchCondition::Less => f.write_str("blt"),
            BranchCondition::GreaterOrEqual => f.write_str("bge"),
            BranchCondition::LessUnsigned => f.write_str("bltu"),
            BranchCondition::GreaterOrEqualUnsigned => f.write_str("bgeu"),
        }
    }
}

/// BRANCH instruction.
pub struct BranchInstruction {
    pc: u32,
    src1: u8,
    src2: u8,
    offset: i32,
    condition: BranchCondition,
}

impl BranchInstruction {
    /// Create a new BranchInstruction.
    pub fn new(instruction: &InstructionWordParts, pc: u32) -> Result<Self, ProcessorException> {
        let condition = match instruction.funct3 & 0b111 {
            0b000 => BranchCondition::Equal,
            0b001 => BranchCondition::NotEqual,
            0b100 => BranchCondition::Less,
            0b101 => BranchCondition::GreaterOrEqual,
            0b110 => BranchCondition::LessUnsigned,
            0b111 => BranchCondition::GreaterOrEqualUnsigned,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            pc,
            src1: instruction.rs1,
            src2: instruction.rs2,
            offset: instruction.imm_b,
            condition,
        })
    }
}

impl Instruction for BranchInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src1 = registers.get(&self.src1).unwrap().load()?;
        let src2 = registers.get(&self.src2).unwrap().load()?;

        let jump_addr = self.pc.wrapping_add(self.offset as u32);
        if jump_addr % 4 != 0 {
            return Err(ProcessorException::InstructionAddressMisaligned);
        }

        let jump_cond = match self.condition {
            BranchCondition::Equal => src1 == src2,
            BranchCondition::NotEqual => src1 != src2,
            BranchCondition::Less => src1 < src2,
            BranchCondition::GreaterOrEqual => src1 >= src2,
            BranchCondition::LessUnsigned => (src1 as u32) < (src2 as u32),
            BranchCondition::GreaterOrEqualUnsigned => (src1 as u32) >= (src2 as u32),
        };

        if jump_cond {
            Ok(InstructionResult::set_jump(jump_addr))
        } else {
            Ok(InstructionResult::default())
        }
    }

    fn format(&self) -> String {
        format!(
            "{} x{}, x{}, 0x{:08x}",
            self.condition, self.src1, self.src2, self.offset
        )
    }
}
