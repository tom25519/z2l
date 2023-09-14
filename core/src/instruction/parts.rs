//! Basic initial processing for the RISC-V instruction formats.
//!
//! This module contains the logic for extracting opcodes, register numbers, and immediate values
//! from raw binary instructions.

use crate::error::ProcessorException;
use crate::instruction::InstructionLength;

/// Represents the component parts of an instruction of any length.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InstructionParts {
    /// This instruction is of the standard 32-bit length.
    Word(InstructionWordParts),
}

impl InstructionParts {
    /// Determine the length of the provided raw instruction & extract its component parts.
    ///
    /// Uses [`Self::identify_instruction_length`] to determine the length of the instruction.
    pub fn new(raw: u32) -> Result<Self, ProcessorException> {
        match Self::identify_instruction_length(raw) {
            InstructionLength::Word => Ok(Self::Word(InstructionWordParts::new(raw))),
            _ => Err(ProcessorException::IllegalInstruction),
        }
    }

    /// Determine the length of the provided raw instruction.
    ///
    /// This determination is done by looking at low bits of the instruction, as specified in the
    /// RISC-V spec, Section 1.5 (Base Instruction-Length Encoding). If extensions violate this
    /// spec, then their instructions will be parsed incorrectly, or treated as invalid.
    pub fn identify_instruction_length(raw: u32) -> InstructionLength {
        if raw & 0b11 != 0b11 {
            InstructionLength::HalfWord
        } else if raw & 0b11100 != 0b11100 {
            InstructionLength::Word
        } else if raw & 0b100000 == 0 {
            InstructionLength::WordAndHalf
        } else if raw & 0b1000000 == 0 {
            InstructionLength::DoubleWord
        } else {
            let len = (raw & 0b0111000000000000) >> 12;
            if len == 0b111 {
                InstructionLength::Reserved
            } else {
                InstructionLength::Custom((80 + (16 * len)) as u8)
            }
        }
    }

    /// Return the opcode of this instruction.
    pub fn opcode(&self) -> u8 {
        match self {
            Self::Word(parts) => parts.opcode,
        }
    }

    /// Get a reference to the underlying [`InstructionWordParts`].
    ///
    /// If this instruction is of the standard 32-bit length, returns a reference to the underlying
    /// parts of the instruction. Otherwise returns an error.
    pub fn word(&self) -> Result<&InstructionWordParts, ProcessorException> {
        #[allow(unreachable_patterns)]
        match self {
            Self::Word(parts) => Ok(parts),
            _ => Err(ProcessorException::IllegalInstruction),
        }
    }

    /// Convert to [`InstructionWordParts`].
    ///
    /// If this instruction is of the standard 32-bit length, returns the underlying parts of the
    /// instruction. Otherwise returns an error.
    pub fn into_word(self) -> Result<InstructionWordParts, ProcessorException> {
        #[allow(unreachable_patterns)]
        match self {
            Self::Word(parts) => Ok(parts),
            _ => Err(ProcessorException::IllegalInstruction),
        }
    }
}

/// Represents the component parts of an instruction of the standard 32-bit length.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InstructionWordParts {
    /// The raw instruction represented by these parts.
    pub raw: u32,

    /// Opcode for this instruction.
    ///
    /// This is used to determine the instruction format and operation.
    pub opcode: u8,

    /// Destination register for this instruction.
    ///
    /// Used by the R, I, U, J instruction formats.
    pub rd: u8,

    /// First source register for this instruction.
    ///
    /// Used by the R, I, S, B instruction formats.
    pub rs1: u8,

    /// Second source register for this instruction.
    ///
    /// Used by the R, S, B instruction formats.
    pub rs2: u8,

    /// Three-bit minor opcode.
    ///
    /// Used by the R, I, S, B instruction formats.
    pub funct3: u8,

    /// Seven-bit minor opcode.
    ///
    /// Used by the R instruction format.
    pub funct7: u8,

    /// 32-bit immediate value encoded by this instruction when interpreted in I format.
    pub imm_i: i32,

    /// 32-bit immediate value encoded by this instruction when interpreted in S format.
    pub imm_s: i32,

    /// 32-bit immediate value encoded by this instruction when interpreted in B format.
    pub imm_b: i32,

    /// 32-bit immediate value encoded by this instruction when interpreted in U format.
    pub imm_u: i32,

    /// 32-bit immediate value encoded by this instruction when interpreted in J format.
    pub imm_j: i32,
}

impl InstructionWordParts {
    /// Extract the raw component parts of this 32-bit instruction.
    pub fn new(raw: u32) -> Self {
        let opcode = raw & 0b00000000_00000000_00000000_01111111;

        // Registers
        let rd = (raw & 0b00000000_00000000_00001111_10000000) >> 7;
        let rs1 = (raw & 0b00000000_00001111_10000000_00000000) >> 15;
        let rs2 = (raw & 0b00000001_11110000_00000000_00000000) >> 20;

        // Minor opcode
        let funct3 = (raw & 0b00000000_00000000_01110000_00000000) >> 12;
        let funct7 = (raw & 0b11111110_00000000_00000000_00000000) >> 25;

        // Immediate data
        let imm_i = ((raw & 0b11111111_11110000_00000000_00000000) as i32) >> 20;
        let imm_s = ((raw & 0b11111110_00000000_00000000_00000000)
            | ((raw & 0b00000000_00000000_00001111_10000000) << 13)) as i32
            >> 20;
        let imm_b = ((raw & 0b10000000_00000000_00000000_00000000)
            | ((raw & 0b00000000_00000000_00000000_10000000) << 23)
            | ((raw & 0b01111110_00000000_00000000_00000000) >> 1)
            | ((raw & 0b00000000_00000000_00001111_00000000) << 12)) as i32
            >> 19;
        let imm_u = (raw & 0b11111111_11111111_11110000_00000000) as i32;
        let imm_j = ((raw & 0b10000000_00000000_00000000_00000000)
            | ((raw & 0b00000000_00001111_11110000_00000000) << 11)
            | ((raw & 0b00000000_00010000_00000000_00000000) << 2)
            | ((raw & 0b01111111_11100000_00000000_00000000) >> 9)) as i32
            >> 11;

        Self {
            raw,
            opcode: opcode as u8,
            rd: rd as u8,
            rs1: rs1 as u8,
            rs2: rs2 as u8,
            funct3: funct3 as u8,
            funct7: funct7 as u8,
            imm_i: imm_i as i32,
            imm_s: imm_s as i32,
            imm_b: imm_b as i32,
            imm_u: imm_u as i32,
            imm_j: imm_j as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InstructionParts, InstructionWordParts};
    use crate::instruction::InstructionLength;

    #[test]
    fn identify_instruction_length() {
        assert_eq!(
            InstructionParts::identify_instruction_length(0xfce0_8793),
            InstructionLength::Word
        );
        assert_eq!(
            // 32-bit instruction with a long, all-zero immediate value
            InstructionParts::identify_instruction_length(0x0000_02b7),
            InstructionLength::Word
        );
        assert!(matches!(
            InstructionParts::new(0xfce0_8793).unwrap(),
            InstructionParts::Word(_),
        ));

        assert_eq!(
            InstructionParts::identify_instruction_length(0x0000_12a9),
            InstructionLength::HalfWord,
        );
        assert_eq!(
            // 16-bit instruction immediately followed by a 32-bit instruction
            InstructionParts::identify_instruction_length(0x2423_12a9),
            InstructionLength::HalfWord,
        );
        /*
        // TODO: Uncomment when supported
        assert!(matches!(
            InstructionParts::new(0x0000_12a9),
            InstructionParts::HalfWord(_)
        ));
         */
    }

    // From https://inst.eecs.berkeley.edu/~cs61c/resources/su18_lec/Lecture7.pdf
    #[test]
    fn split_r_format() {
        let instruction = InstructionWordParts::new(0x0073_02b3); // add x5, x6, x7
        assert_eq!(instruction.opcode, 0x33);
        assert_eq!(instruction.rd, 5);
        assert_eq!(instruction.funct3, 0);
        assert_eq!(instruction.rs1, 6);
        assert_eq!(instruction.rs2, 7);
        assert_eq!(instruction.funct7, 0);
    }

    // From https://inst.eecs.berkeley.edu/~cs61c/resources/su18_lec/Lecture7.pdf
    #[test]
    fn split_i_format() {
        let instruction = InstructionWordParts::new(0xfce0_8793); // addi x15, x1, -50
        assert_eq!(instruction.opcode, 0x13);
        assert_eq!(instruction.rd, 15);
        assert_eq!(instruction.funct3, 0);
        assert_eq!(instruction.rs1, 1);
        assert_eq!(instruction.imm_i, -50);

        let instruction = InstructionWordParts::new(0x0081_2783); // lw, x15, 8(x2)
        assert_eq!(instruction.opcode, 0x03);
        assert_eq!(instruction.rd, 15);
        assert_eq!(instruction.funct3, 2);
        assert_eq!(instruction.rs1, 2);
        assert_eq!(instruction.imm_i, 8);
    }

    // From https://inst.eecs.berkeley.edu/~cs61c/resources/su18_lec/Lecture7.pdf
    #[test]
    fn split_s_format() {
        let instruction = InstructionWordParts::new(0x00e1_2423); // sw x14, 8(x2)
        assert_eq!(instruction.opcode, 0x23);
        assert_eq!(instruction.funct3, 2);
        assert_eq!(instruction.rs1, 2);
        assert_eq!(instruction.rs2, 14);
        assert_eq!(instruction.imm_s, 8);
    }

    // From https://inst.eecs.berkeley.edu/~cs61c/resources/su18_lec/Lecture7.pdf
    #[test]
    fn split_b_format() {
        let instruction = InstructionWordParts::new(0x00a9_8863); // beq x19, x10, offset = 16 bytes
        assert_eq!(instruction.opcode, 0x63);
        assert_eq!(instruction.funct3, 0);
        assert_eq!(instruction.rs1, 19);
        assert_eq!(instruction.rs2, 10);
        assert_eq!(instruction.imm_b, 16);
    }

    #[test]
    fn split_u_format() {
        let instruction = InstructionWordParts::new(0x8765_4537); // lui x10, 0x87654
        assert_eq!(instruction.opcode, 0x37);
        assert_eq!(instruction.rd, 10);
        assert_eq!(instruction.imm_u as u32, 0x8765_4000);

        let instruction = InstructionWordParts::new(0xdead_b797); // auipc x15, 0xdeadb
        assert_eq!(instruction.opcode, 0x17);
        assert_eq!(instruction.rd, 15);
        assert_eq!(instruction.imm_u as u32, 0xdead_b000);
    }

    #[test]
    fn split_j_format() {
        let instruction = InstructionWordParts::new(0x0a40_02ef); // jal x5, offset = 164 bytes
        assert_eq!(instruction.opcode, 0x6f);
        assert_eq!(instruction.rd, 5);
        assert_eq!(instruction.imm_j, 164);
    }
}
