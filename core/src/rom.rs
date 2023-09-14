//! Read-Only Memory (ROM).
//!
//! Immutable storage region. This can be used to store a simple RISC-V program which does not need
//! to dynamically load program code, or a bootloader to initialise RAM with a program to run.
//!
//! The processor will start executing code at address `0x00000000` of the ROM.

use crate::error::{MemoryAccessError, ProcessorException};
use crate::mmu::Addressable;
use std::io::{self, Read};
use std::ops::Range;

/// ROM device.
#[derive(Debug)]
pub struct ROM {
    /// The internal storage.
    ///
    /// We just use a Vec here: may wish to look into a different backend in the future.
    contents: Vec<u8>,
}

impl ROM {
    /// Create a new ROM with the provided contents.
    pub fn new<C>(contents: C) -> Self
    where
        Vec<u8>: From<C>,
    {
        Self {
            contents: Vec::from(contents),
        }
    }

    /// Load a ROM from a file/other [`Read`] source.
    pub fn from<R: Read>(mut source: R) -> Result<Self, io::Error> {
        let mut contents = Vec::new();
        source.read_to_end(&mut contents)?;
        Ok(Self::new(contents))
    }
}

impl Addressable for ROM {
    fn reserve(&self) -> usize {
        self.contents.len().next_power_of_two()
    }

    fn load_raw(&self, range: Range<usize>) -> Result<&[u8], ProcessorException> {
        if range.end > self.contents.len() {
            return Err(MemoryAccessError::OutOfBounds.into());
        }

        Ok(&self.contents[range])
    }

    fn store_raw(
        &mut self,
        _range: Range<usize>,
        _values: &[u8],
    ) -> Result<(), ProcessorException> {
        Err(MemoryAccessError::ReadOnly.into())
    }
}
