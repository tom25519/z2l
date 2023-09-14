//! Memory Management Unit (MMU).
//!
//! Computers consist of many memory-mapped devices. Our simple RISC-V system, for example, has both
//! a RAM and ROM, and instructions or values may be loaded from either. More complicated systems
//! will have many more memory-mapped peripherals. All of these are accessible from the processor
//! via addresses in the same 32-bit address space (`0x00000000` to `0xffffffff`). The MMU maps
//! processor memory accesses to specific devices, by dividing up the 32-bit address space into
//! portions, each of which corresponds to a specific device, then translating the processor's
//! addresses into addresses relative to each device.

use crate::error::ProcessorException;
use crate::ram::RAM;
use crate::rom::ROM;
use std::fmt;
use std::ops::Range;

/// Type of value to retrieve from memory.
///
/// Memory loads via [`MMU::load`] always result in an `i32` to store in a register, however this
/// may be the result of loading a smaller value and extending it to fill a 32-bit space, either
/// through sign-extension, in the case of signed values, or zero-extension, in the case of unsigned
/// values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryAccessType {
    /// Load a word from memory.
    Word,

    /// Load a half-word from memory, then sign-extend it to 32 bits.
    SignedHalfWord,

    /// Load a half-word from memory, then zero-extend it to 32 bits.
    UnsignedHalfWord,

    /// Load a byte from memory, then sign-extend it to 32 bits.
    SignedByte,

    /// Load a byte from memory, then zero-extend it to 32 bits.
    UnsignedByte,
}

impl fmt::Display for MemoryAccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryAccessType::Word => f.write_str("w"),
            MemoryAccessType::SignedHalfWord => f.write_str("h"),
            MemoryAccessType::UnsignedHalfWord => f.write_str("hu"),
            MemoryAccessType::SignedByte => f.write_str("b"),
            MemoryAccessType::UnsignedByte => f.write_str("bu"),
        }
    }
}

/// Specification for loading a value from memory.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LoadSpec {
    /// Width of the value to load.
    pub access_type: MemoryAccessType,

    /// Address of the value to load.
    pub addr: usize,
}

impl LoadSpec {
    /// Create a new LoadSpec.
    pub fn new(width: MemoryAccessType, addr: usize) -> Self {
        Self {
            access_type: width,
            addr,
        }
    }
}

/// Specification for storing a value to memory.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreSpec {
    /// Width of the value to store.
    ///
    /// If this is less than 32 bits, the most significant bits are discarded.
    pub access_type: MemoryAccessType,

    /// Address at which the value should be stored.
    pub addr: usize,

    /// Value to store.
    pub value: i32,
}

impl StoreSpec {
    /// Create a new StoreSpec.
    pub fn new(width: MemoryAccessType, addr: usize, value: i32) -> Self {
        Self {
            access_type: width,
            addr,
            value,
        }
    }
}

/// Trait for devices which can be mapped to memory.
pub trait Addressable {
    /// Portion of the 32-bit address space to reserve for this device.
    ///
    /// This should be a power of 2.
    fn reserve(&self) -> usize;

    /// Load a value from this device.
    ///
    /// `range` should be the range of addresses to load.
    ///
    /// If `range` is invalid for this device, or loads are not supported for this range, this
    /// should return an exception.
    fn load_raw(&self, range: Range<usize>) -> Result<&[u8], ProcessorException>;

    /// Store a value to this device.
    ///
    /// `range` should be the range of addresses where the provided value should be stored.
    ///
    /// If `values` is not the same size as `range`, `range` is invalid for this device, or stores
    /// are not supported for this range, this should return an exception.
    fn store_raw(&mut self, range: Range<usize>, values: &[u8]) -> Result<(), ProcessorException>;
}

/// Memory-management unit.
#[derive(Debug)]
pub struct MMU {
    rom: ROM,
    ram: RAM,
}

impl MMU {
    /// Create a new MMU.
    pub fn new(rom: ROM, ram: RAM) -> Self {
        Self { rom, ram }
    }

    /// Load a raw value from memory.
    ///
    /// Returns an error if the provided range is not mapped to a single device.
    pub fn load_raw(&self, range: Range<usize>) -> Result<&[u8], ProcessorException> {
        if range.start & 0x80000000 == 0 {
            self.rom.load_raw(range).into()
        } else {
            self.ram
                .load_raw(range.start & 0x7fffffff..range.end & 0x7fffffff)
                .into()
        }
    }

    /// Load a word from memory.
    pub fn load_word(&self, addr: usize) -> Result<i32, ProcessorException> {
        Ok(i32::from_le_bytes(
            self.load_raw(addr..addr + 4)?.try_into().unwrap(),
        ))
    }

    /// Load a half-word from memory, then sign-extend to a full word.
    pub fn load_signed_halfword(&self, addr: usize) -> Result<i32, ProcessorException> {
        Ok(((self.load_unsigned_halfword(addr)? as i32) << 16) >> 16)
    }

    /// Load a half-word from memory, then zero-extend to a full word.
    pub fn load_unsigned_halfword(&self, addr: usize) -> Result<u32, ProcessorException> {
        let value = u16::from_le_bytes(self.load_raw(addr..addr + 2)?.try_into().unwrap());
        Ok(value as u32)
    }

    /// Load a byte from memory, then sign-extend to a full word.
    pub fn load_signed_byte(&self, addr: usize) -> Result<i32, ProcessorException> {
        Ok(((self.load_unsigned_byte(addr)? as i32) << 24) >> 24)
    }

    /// Load a byte from memory, then zero-extend to a full word.
    pub fn load_unsigned_byte(&self, addr: usize) -> Result<u32, ProcessorException> {
        let value = u8::from_le_bytes(self.load_raw(addr..addr + 1)?.try_into().unwrap());
        Ok(value as u32)
    }

    /// Load a value from memory, according to the provided [`LoadSpec`].
    pub fn load(&self, load: LoadSpec) -> Result<i32, ProcessorException> {
        Ok(match load.access_type {
            MemoryAccessType::Word => self.load_word(load.addr)?,
            MemoryAccessType::SignedHalfWord => self.load_signed_halfword(load.addr)?,
            MemoryAccessType::UnsignedHalfWord => self.load_unsigned_halfword(load.addr)? as i32,
            MemoryAccessType::SignedByte => self.load_signed_byte(load.addr)?,
            MemoryAccessType::UnsignedByte => self.load_unsigned_byte(load.addr)? as i32,
        })
    }

    /// Store a raw value to memory.
    ///
    /// Returns an error if the provided range is not mapped to a single device.
    pub fn store_raw(
        &mut self,
        range: Range<usize>,
        values: &[u8],
    ) -> Result<(), ProcessorException> {
        if range.start & 0x80000000 == 0 {
            self.rom.store_raw(range, values).into()
        } else {
            self.ram
                .store_raw(range.start & 0x7fffffff..range.end & 0x7fffffff, values)
                .into()
        }
    }

    /// Store a word to memory.
    pub fn store_word(&mut self, addr: usize, value: i32) -> Result<(), ProcessorException> {
        self.store_raw(addr..addr + 4, &value.to_le_bytes())
    }

    /// Store the low 16 bits of the provided value to memory.
    pub fn store_halfword(&mut self, addr: usize, value: i32) -> Result<(), ProcessorException> {
        self.store_raw(addr..addr + 2, &(value as u16).to_le_bytes())
    }

    /// Store the low 8 bits of the provided value to memory.
    pub fn store_byte(&mut self, addr: usize, value: i32) -> Result<(), ProcessorException> {
        self.store_raw(addr..addr + 1, &(value as u8).to_le_bytes())
    }

    /// Store a value to memory, according to the provided [`StoreSpec`].
    pub fn store(&mut self, store: StoreSpec) -> Result<(), ProcessorException> {
        Ok(match store.access_type {
            MemoryAccessType::Word => self.store_word(store.addr, store.value)?,
            MemoryAccessType::SignedHalfWord | MemoryAccessType::UnsignedHalfWord => {
                self.store_halfword(store.addr, store.value)?
            }
            MemoryAccessType::SignedByte | MemoryAccessType::UnsignedByte => {
                self.store_halfword(store.addr, store.value)?
            }
        })
    }
}
