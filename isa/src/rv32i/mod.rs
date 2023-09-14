//! The RV32I base instruction set.

pub mod auipc;
pub mod branch;
pub mod fence;
pub mod jal;
pub mod jalr;
pub mod load;
pub mod lui;
pub mod op;
pub mod op_imm;
pub mod store;
pub mod system;

use std::fmt;
use std::fmt::Write;
use z2l_core::extension::Extension;
use z2l_core::processor::hart::Hart;

/// An [`Extension`] defining the RV32I base instruction set.
pub struct RV32I;

impl Extension for RV32I {
    fn code(&self) -> &'static str {
        "RV32I"
    }

    fn name(&self) -> &'static str {
        "32-bit Base Integer Instruction Set"
    }

    fn register(&self, hart: &mut Hart) {
        hart.opcodes.insert(0x03, Box::new(load::LoadHandler));
        hart.opcodes.insert(0x0f, Box::new(fence::FenceHandler));
        hart.opcodes.insert(0x13, Box::new(op_imm::OpImmHandler));
        hart.opcodes.insert(0x17, Box::new(auipc::AUIPCHandler));
        hart.opcodes.insert(0x23, Box::new(store::StoreHandler));
        hart.opcodes.insert(0x33, Box::new(op::OpHandler));
        hart.opcodes.insert(0x37, Box::new(lui::LuiHandler));
        hart.opcodes.insert(0x63, Box::new(branch::BranchHandler));
        hart.opcodes.insert(0x67, Box::new(jalr::JalrHandler));
        hart.opcodes.insert(0x6f, Box::new(jal::JalHandler));
        hart.opcodes.insert(0x73, Box::new(system::SystemHandler));
    }
}

/// Behaviour of a right-shift instruction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum RightShiftBehaviour {
    /// Zero-extend.
    Logical,

    /// Sign-extend.
    Arithmetic,
}

impl fmt::Display for RightShiftBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RightShiftBehaviour::Logical => f.write_char('l'),
            RightShiftBehaviour::Arithmetic => f.write_char('a'),
        }
    }
}
