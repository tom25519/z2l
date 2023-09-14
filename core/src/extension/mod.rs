//! Traits for implementing RISC-V ISA extensions.
//!
//! RISC-V is designed around extensibility: A RISC-V platform must implement a base integer
//! instruction set (RV32I or RV64I, which have been ratified; or RV32E or RV128I, which are
//! proposed), which defines a core set of basic instructions. Further capability is then defined
//! via optional extensions, for example: The "M" extension implements integer
//! multiplication/division, and the "F" extension implements support for single-precision
//! floating-point arithmetic.
//!
//! This module defines the [`Extension`] trait, representing such an extension. Extensions consist
//! of [`OpcodeHandler`]s, which define how a processor should decode instructions with a given
//! opcode. The key method of an [`Extension`] is [`register`](Extension::register), which is passed
//! a mutable reference to a [`Hart`], and updates the hart to implement the extension's opcode
//! handlers.
//!
//! Note that the [`Extension`] trait is also used to implement the base integer instruction sets,
//! since these are functionally equivalent to extensions for this application.

mod opcode_handler;

pub use opcode_handler::OpcodeHandler;

use crate::processor::hart::Hart;

/// A RISC-V ISA extension.
pub trait Extension: Send + Sync + 'static {
    /// Get the short "name" by which an extension is referred to in the RISC-V spec.
    ///
    /// For example, the Integer Multiplication/Division extension would return `"M"` here, and the
    /// Control and Status Register Instructions extension would return `"Zicsr"`.
    ///
    /// This is used by the end-user of Z2L to specify which extensions an emulated processor should
    /// implement.
    fn code(&self) -> &'static str;

    /// A human-readable name/description of the extension.
    ///
    /// There are no requirements on what this name should consist of, but it should inform the user
    /// of what this extension does, and its status (proposed/ratified/etc).
    ///
    /// Currently unused.
    fn name(&self) -> &'static str;

    /// Register this extension with the provided hart.
    ///
    /// This function should update the provided [`Hart`] to support this extension: The usual
    /// way to do this would be to add a set of [`OpcodeHandler`]s to the [`Hart::opcodes`]
    /// hashmap.
    ///
    /// This is deliberately given a lot of freedom in what it can do, and how it does it, so as to
    /// support extensions which may fundamentally change properties of the processor (e.g: The
    /// compressed instructions extension, which changes the instruction alignment requirements), or
    /// wish to modify existing opcode handlers (for example, the Zicsr extension implements new
    /// instructions in the same SYSTEM opcode already in use by the base RV32I instruction set).
    fn register(&self, hart: &mut Hart);
}
