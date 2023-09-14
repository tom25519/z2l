//! Processor registers.
//!
//! Rather than hard-coding a set of registers for each hart, we instead define a generic
//! [`Register`] interface, and use a HashMap to store the available registers. This allows
//! extensions to add new registers, which can then be accessed by other extensions, without
//! affecting the base ISA.
//!
//! The [`GeneralPurposeRegister`] is, as the name implies, a general-purpose 32-bit register
//! capable of stores and loads. The RISC-V base ISA requires 31 such general-purpose registers.
//!
//! The [`ZeroRegister`] allows stores, but discards them, always returning zero on load. The RISC-V
//! ISA requires one such "register".

use crate::error::ProcessorException;
use std::collections::BTreeMap;

/// A RV32I register file.
///
/// By default, `RegisterFile[1]` through `RegisterFile[31]` are populated with general-purpose
/// registers, and `RegisterFile[0]` is a zero register.
pub type RegisterFile = BTreeMap<u8, Box<dyn Register>>;

/// A 32-bit register.
pub trait Register: std::fmt::Debug + Send + Sync + 'static {
    /// Get the current value stored in this register.
    fn load(&self) -> Result<i32, ProcessorException>;

    /// Store a value in this register.
    ///
    /// Returns the value of the register prior to this store.
    fn store(&mut self, val: i32) -> Result<i32, ProcessorException>;
}

/// A general-purpose register, which behaves "as you would expect".
///
/// Stores a single 32-bit value. When a value is stored, this change is reflected in later loads.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GeneralPurposeRegister {
    value: i32,
}

impl GeneralPurposeRegister {
    /// Create a new GeneralPurposeRegister.
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Register for GeneralPurposeRegister {
    fn load(&self) -> Result<i32, ProcessorException> {
        Ok(self.value)
    }

    fn store(&mut self, val: i32) -> Result<i32, ProcessorException> {
        let prev = self.value;
        self.value = val;
        Ok(prev)
    }
}

/// A zero register.
///
/// While stores do not raise an exception, they have no effect: This register always returns zero
/// on load. This is especially useful for loading immediates (via `addi dest, x0, imm`), and for
/// discarding instruction results (using the zero register as the destination).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZeroRegister;

impl Register for ZeroRegister {
    fn load(&self) -> Result<i32, ProcessorException> {
        Ok(0)
    }

    fn store(&mut self, _val: i32) -> Result<i32, ProcessorException> {
        Ok(0)
    }
}
