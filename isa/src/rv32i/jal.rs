//! JAL instruction handler.
//!
//! The Jump-And-Link (JAL) instruction unconditionally jumps to a memory address determined by an
//! offset from the current program counter.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};

use z2l_core::processor::register::RegisterFile;

/// JAL opcode handler.
pub struct JalHandler;

impl OpcodeHandler for JalHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(JalInstruction::new(&instruction, pc)))
    }
}

/// JAL instruction.
pub struct JalInstruction {
    pc: u32,
    offset: i32,
    dest: u8,
}

impl JalInstruction {
    /// Create a new JAL instruction.
    pub fn new(instruction: &InstructionWordParts, pc: u32) -> Self {
        Self {
            pc,
            offset: instruction.imm_j,
            dest: instruction.rd,
        }
    }
}

impl Instruction for JalInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let jump_addr = self.pc.wrapping_add(self.offset as u32);
        if jump_addr % 4 != 0 {
            return Err(ProcessorException::InstructionAddressMisaligned);
        }

        let dest = registers.get_mut(&self.dest).unwrap();
        let ret_addr = self.pc + 4;
        dest.store(ret_addr as i32)?;

        Ok(InstructionResult::set_jump(jump_addr))
    }

    fn format(&self) -> String {
        format!("jal x{}, 0x{:08x}", self.dest, self.offset)
    }
}
