//! LOAD opcode instructions.
//!
//! LOAD instructions copy a value from memory into a register.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::mmu::{LoadSpec, MemoryAccessType};
use z2l_core::processor::register::RegisterFile;

/// LOAD opcode handler.
pub struct LoadHandler;

impl OpcodeHandler for LoadHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(LoadInstruction::new(&instruction)?))
    }
}

/// LOAD instruction.
pub struct LoadInstruction {
    base: u8,
    offset: i32,
    dest: u8,
    width: MemoryAccessType,
}

impl LoadInstruction {
    /// Create a new LOAD instruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let width = match instruction.funct3 {
            0b000 => MemoryAccessType::SignedByte,
            0b001 => MemoryAccessType::SignedHalfWord,
            0b010 => MemoryAccessType::Word,
            0b100 => MemoryAccessType::UnsignedByte,
            0b101 => MemoryAccessType::UnsignedHalfWord,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            base: instruction.rs1,
            offset: instruction.imm_i,
            dest: instruction.rd,
            width,
        })
    }
}

impl Instruction for LoadInstruction {
    fn load(&self, registers: &RegisterFile) -> Result<Option<LoadSpec>, ProcessorException> {
        let base = registers.get(&self.base).unwrap().load()?;
        let addr = base.wrapping_add(self.offset) as usize;

        Ok(Some(LoadSpec::new(self.width, addr)))
    }

    fn execute(
        &self,
        registers: &mut RegisterFile,
        mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let dest = registers.get_mut(&self.dest).unwrap();
        dest.store(mem)?;

        Ok(InstructionResult::default())
    }

    fn format(&self) -> String {
        format!(
            "l{} x{}, 0x{:08x}(x{})",
            self.width, self.dest, self.offset, self.base
        )
    }
}
