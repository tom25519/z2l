//! Random Access Memory (RAM).
//!
//! Volatile, mutable storage region.

use crate::error::{MemoryAccessError, ProcessorException};
use crate::mmu::Addressable;
use std::ops::Range;

/// RAM device.
#[derive(Debug)]
pub struct RAM {
    /// The internal storage.
    ///
    /// We just use a Vec here: may wish to look into a different backend in the future.
    contents: Vec<u8>,
}

impl RAM {
    /// Create a new RAM device, of the provided size.
    ///
    /// The entire storage will be allocated upfront on the host device.
    pub fn new(size: usize) -> Self {
        Self {
            contents: vec![0u8; size],
        }
    }
}

impl Addressable for RAM {
    fn reserve(&self) -> usize {
        self.contents.len().next_power_of_two()
    }

    fn load_raw(&self, range: Range<usize>) -> Result<&[u8], ProcessorException> {
        if range.end > self.contents.len() {
            return Err(MemoryAccessError::OutOfBounds.into());
        }

        Ok(&self.contents[range])
    }

    fn store_raw(&mut self, range: Range<usize>, values: &[u8]) -> Result<(), ProcessorException> {
        if range.end > self.contents.len() {
            return Err(MemoryAccessError::OutOfBounds.into());
        } else if values.len() != range.len() {
            return Err(MemoryAccessError::LengthMismatch.into());
        }

        self.contents[range].copy_from_slice(values);

        Ok(())
    }
}
