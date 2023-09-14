//! Error types.

/// An exception encountered by a hart during execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProcessorException {
    /// Tried to decode an unrecognised instruction.
    ///
    /// The associated value is the memory location at which the illegal instruction is located
    /// (i.e: the value of the program counter when the illegal instruction was encountered).
    IllegalInstruction,

    /// Tried to execute an instruction at an address w/ invalid alignment for this processor.
    ///
    /// For the standard RV32I instruction set, without compressed instruction extension,
    /// instructions must be placed on 4-byte boundaries.
    InstructionAddressMisaligned,

    /// Attempted an invalid memory load/store.
    InvalidMemoryAccess(MemoryAccessError),

    /// Encountered an unhandled `ECALL` instruction.
    EnvironmentCall,

    /// Encountered an unhandled `EBREAK` instruction.
    EnvironmentBreak,
}

/// An exception relating to a load/store from the MMU.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryAccessError {
    /// Tried to load/store from a range which is (at least partially) out of bounds.
    ///
    /// This indicates the load/store overlapped with a portion of the address space which is not
    /// mapped to a memory device.
    OutOfBounds,

    /// Tried to store to a range which is read-only.
    ///
    /// This could happen if you try to overwrite a portion of the ROM, for example.
    ReadOnly,

    /// Mismatch in the length of the value to store and the range to which the value should be
    /// stored.
    ///
    /// This indicates an error in the implementation of the processor/MMU, *not* an error on the
    /// part of the emulated program. In theory, this should never occur.
    LengthMismatch,
}

impl From<MemoryAccessError> for ProcessorException {
    fn from(value: MemoryAccessError) -> Self {
        ProcessorException::InvalidMemoryAccess(value)
    }
}

/// Helper trait for adding a program counter value to an error.
pub trait WithPC<T, E> {
    /// Add the program counter to the error value of this Result.
    fn with_pc(self, pc: u32) -> Result<T, (E, u32)>;
}

impl<T, E> WithPC<T, E> for Result<T, E> {
    fn with_pc(self, pc: u32) -> Result<T, (E, u32)> {
        self.map_err(|e| (e, pc))
    }
}
