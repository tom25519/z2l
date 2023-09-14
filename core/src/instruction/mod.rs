//! Traits and utilities for defining RISC-V instructions.
//!
//! This module defines the [`Instruction`] trait, which represents an instruction which can be
//! executed. Such instructions are returned as the result of decoding an instruction with an
//! [`OpcodeHandler`](crate::extension::OpcodeHandler), and executed by a hart.
//!
//! This module also defines the [`InstructionParts`] struct, which represents the component parts
//! of an instruction, split according to the "base instruction formats" listed in the RISC-V spec,
//! and the logic for performing this splitting.

mod parts;

use crate::error::ProcessorException;
pub use parts::{InstructionParts, InstructionWordParts};

use crate::mmu::{LoadSpec, StoreSpec};
use crate::processor::register::RegisterFile;

/// Length of a RISC-V instruction.
///
/// Only the [`Word`](Self::Word) (standard) format is supported by this implementation. n.b. This
/// does not inherently imply lack of support for the RV64I/RV128I instruction sets, as these still
/// use 32-bit instructions, although for the time being these are not supported by this
/// implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InstructionLength {
    /// The compressed 16-bit instruction length.
    HalfWord,

    /// The standard 32-bit instruction length.
    Word,

    /// The proposed 48-bit instruction length.
    WordAndHalf,

    /// The proposed 64-bit instruction length.
    DoubleWord,

    /// The proposed custom instruction length.
    ///
    /// This may be 80, 96, 112, 128, 144, 160, or 176 bits.
    Custom(u8),

    /// Reserved for instruction lengths >= 192 bits.
    Reserved,
}

/// Result of executing an instruction.
///
/// This is used to communicate to the hart whether it needs to jump or store a value in memory.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct InstructionResult {
    /// If set to `Some(addr)`, the hart will jump to `addr` following the instruction execution.
    pub jump: Option<u32>,

    /// If set to `Some(store_spec)`, the hart will write a value to memory according to the
    /// provided [`StoreSpec`].
    pub store: Option<StoreSpec>,
}

impl InstructionResult {
    /// Create an InstructionResult which will instruct the hart to jump to the provided address.
    pub fn set_jump(addr: u32) -> Self {
        let mut result = Self::default();
        result.jump = Some(addr);
        result
    }

    /// Create an InstructionResult which will instruct the hart to store a value to memory
    /// according to the provided [`StoreSpec`].
    pub fn set_store(store: StoreSpec) -> Self {
        let mut result = Self::default();
        result.store = Some(store);
        result
    }
}

/// A decoded instruction which can be executed.
pub trait Instruction: Send + Sync + 'static {
    /// Returns a [`LoadSpec`] indicating a memory value the instruction requires.
    ///
    /// If this instruction needs to load a value from memory, this should return `Some(spec)`,
    /// where `spec` is a [`LoadSpec`] describing the address & type of the value to load. The
    /// required value will be loaded from memory immediately before the instruction is executed,
    /// then will be provided to the [`execute`](Self::execute) function as the `mem` argument.
    fn load(&self, _registers: &RegisterFile) -> Result<Option<LoadSpec>, ProcessorException> {
        Ok(None)
    }

    /// Execute this instruction.
    ///
    /// `registers` is a reference to the [`RegisterFile`] of the hart on which this instruction is
    /// executing: This can be used to load values from registers, or store values to registers.
    ///
    /// If the [`load`](Self::load) function of this instruction returns a [`LoadSpec`], then a
    /// value will be retrieved from memory according to this spec, and supplied as the `mem`
    /// argument for this function. If the load function returned `None`, the value of `mem` is
    /// unspecified.
    ///
    /// Returns an [`InstructionResult`], which can be used to perform a jump or store a value to
    /// memory, if required.
    fn execute(
        &self,
        registers: &mut RegisterFile,
        mem: i32,
    ) -> Result<InstructionResult, ProcessorException>;

    /// Provide a human-readable decoding of this instruction.
    ///
    /// This should correspond loosely to the assembly a human would type for this instruction. For
    /// example, an "ADDI" instruction could return `"addi x1, x2, 0xdeadbeef"`.
    fn format(&self) -> String;
}
