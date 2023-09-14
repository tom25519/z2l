//! STORE opcode instructions.
//!
//! STORE instructions copy a value from a register into memory.

use z2l_core::error::ProcessorException;
use z2l_core::extension::OpcodeHandler;
use z2l_core::instruction::{
    Instruction, InstructionParts, InstructionResult, InstructionWordParts,
};
use z2l_core::mmu::{MemoryAccessType, StoreSpec};
use z2l_core::processor::register::RegisterFile;

/// STORE opcode handler.
pub struct StoreHandler;

impl OpcodeHandler for StoreHandler {
    fn decode(
        &self,
        instruction: InstructionParts,
        _pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException> {
        let instruction = instruction.into_word()?;
        Ok(Box::new(StoreInstruction::new(&instruction)?))
    }
}

/// STORE instruction.
pub struct StoreInstruction {
    src: u8,
    base: u8,
    offset: i32,
    width: MemoryAccessType,
}

impl StoreInstruction {
    /// Create a new StoreInstruction.
    pub fn new(instruction: &InstructionWordParts) -> Result<Self, ProcessorException> {
        let width = match instruction.funct3 {
            0b000 => MemoryAccessType::SignedByte,
            0b001 => MemoryAccessType::SignedHalfWord,
            0b010 => MemoryAccessType::Word,
            _ => return Err(ProcessorException::IllegalInstruction),
        };

        Ok(Self {
            src: instruction.rs2,
            base: instruction.rs1,
            offset: instruction.imm_s,
            width,
        })
    }
}

impl Instruction for StoreInstruction {
    fn execute(
        &self,
        registers: &mut RegisterFile,
        _mem: i32,
    ) -> Result<InstructionResult, ProcessorException> {
        let src = registers.get(&self.src).unwrap().load()?;
        let base = registers.get(&self.base).unwrap().load()?;

        let addr = base.wrapping_add(self.offset) as usize;

        Ok(InstructionResult::set_store(StoreSpec::new(
            self.width, addr, src,
        )))
    }

    fn format(&self) -> String {
        format!(
            "s{} x{}, 0x{:08x}(x{})",
            self.width, self.src, self.offset, self.base
        )
    }
}
