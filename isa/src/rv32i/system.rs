//! SYSTEM opcode instructions.
//!
//! SYSTEM instructions are used to access system functionality which may require privileged access.
//! In the base instruction set, the "SYSTEM" opcode is only used for the ECALL/EBREAK instructions.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::processor::register::RegisterFile;

/// SYSTEM [`OpcodeHandler`].
pub struct SystemHandler;

impl OpcodeHandler for SystemHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        match instruction.imm_i {
            0b000000000000 => Ok(Box::new(ECallInstruction::new(&instruction)?)),
            0b000000000001 => Ok(Box::new(EBreakInstruction::new(&instruction)?)),
            _ => Err(ProcessorException::IllegalInstruction),
        }
    }
}

/// An environment call (ECALL) instruction.
///
/// This instruction makes a service request to the execution environment.
pub struct ECallInstruction;

impl ECallInstruction {
    /// Create a new ECallInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        if instruction.rs1 != 0 || instruction.funct3 != 0 || instruction.rd != 0 {
            return Err(ProcessorException::IllegalInstruction);
        }

        Ok(Self)
    }
}

impl Instruction for ECallInstruction {
    fn execute(
        &self,
        _registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        Err(ProcessorException::EnvironmentCall)
    }

    fn format(&self) -> String {
        String::from("ecall")
    }
}

/// An environment debugger breakpoint (EBREAK) instruction.
///
/// This instruction returns control to a debugging environment.
pub struct EBreakInstruction;

impl EBreakInstruction {
    /// Create a new EBreakInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        if instruction.rs1 != 0 || instruction.funct3 != 0 || instruction.rd != 0 {
            return Err(ProcessorException::IllegalInstruction);
        }

        Ok(Self)
    }
}

impl Instruction for EBreakInstruction {
    fn execute(
        &self,
        _registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        Err(ProcessorException::EnvironmentBreak)
    }

    fn format(&self) -> String {
        String::from("ebreak")
    }
}
