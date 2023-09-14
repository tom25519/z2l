//! JALR instruction handler.
//!
//! The Jump-And-Link-Register (JALR) instruction unconditionally jumps to a memory address
//! determined by an offset from a register value.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};

use z2l_core::processor::register::RegisterFile;

/// JALR opcode handler.
pub struct JalrHandler;

impl OpcodeHandler for JalrHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word().unwrap();
        Ok(Box::new(JalrInstruction::new(&instruction, pc)))
    }
}

/// JALR instruction.
pub struct JalrInstruction {
    pc: u32,
    base: u8,
    offset: i32,
    dest: u8,
}

impl JalrInstruction {
    /// Create a new JALR instruction.
    pub fn new(instruction: &InstructionWordParts, pc: u32) -> Self {
        Self {
            pc,
            base: instruction.rs1,
            offset: instruction.imm_i,
            dest: instruction.rd,
        }
    }
}

impl Instruction for JalrInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let base = registers.get(&self.base).unwrap().load()? as u32;

        let jump_addr = base.wrapping_add(self.offset as u32) & 0xfffffffe;
        if jump_addr % 4 != 0 {
            return Err(ProcessorException::InstructionAddressMisaligned);
        }

        let dest = registers.get_mut(&self.dest).unwrap();
        let ret_addr = self.pc + 4;
        dest.store(ret_addr as i32)?;

        Ok(InstructionResult::set_jump(jump_addr))
    }

    fn format(&self) -> String {
        format!("jalr x{}, 0x{:08x}(x{})", self.dest, self.offset, self.dest)
    }
}
