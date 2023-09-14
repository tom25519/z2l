//! The OpcodeHandler trait.

use crate::error::ProcessorException;
use crate::instruction::{Instruction, InstructionParts};

/// A decoder for instructions with a given opcode.
pub trait OpcodeHandler: Send + Sync + 'static {
    /// Decode the provided instruction.
    ///
    /// This function will be called by the hart when it encounters an instruction with the opcode
    /// associated with this handler. The OpcodeHandler should decode the instruction to return an
    /// [`Instruction`], which may later be executed by the processor, or return an error if the
    /// instruction is invalid for the associated opcode.
    fn decode(
        &self,
        instruction: InstructionParts,
        pc: u32,
    ) -> Result<Box<dyn Instruction>, ProcessorException>;
}
