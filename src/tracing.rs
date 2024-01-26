use crate::util::types::{Instruction, Register};
use ctrlflow::{instruction::JumpKind, register::Info, Architecture, InsnInfo, RegInfo};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RSC;

pub enum InsnWrapper {
    NoOp(Instruction),
    Op(Instruction, u32),
}

impl Serialize for InsnWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::NoOp(ref insn) => serializer.serialize_str(insn.as_str()),
            Self::Op(ref insn, ref op) => {
                let insn = format!("{} {}", insn.as_str(), op);
                serializer.serialize_str(&insn)
            }
        }
    }
}

impl Architecture for RSC {
    type Register = Register;
    type Instruction = InsnWrapper;
    type AddressWidth = u32;
}

pub const REGISTER_INFOS: [Info<Register>; 9] = [
    Info::new("S", Register::S, Register::S, None, 1),
    Info::new("Z", Register::Z, Register::S, None, 1),
    Info::new("IR", Register::IR, Register::S, None, 4),
    Info::new("AR", Register::AR, Register::S, None, 4),
    Info::new("DR", Register::DR, Register::S, None, 4),
    Info::new("PC", Register::PC, Register::S, None, 4),
    Info::new("OUTR", Register::OUTR, Register::S, None, 4),
    Info::new("ACC", Register::ACC, Register::S, None, 4),
    Info::new("R", Register::R, Register::S, None, 4),
];

impl RegInfo for Register {
    fn info(&self) -> &'static Info<Self> {
        &REGISTER_INFOS[*self as usize]
    }

    fn iter() -> std::slice::Iter<'static, Self> {
        Register::iter()
    }
}

impl Serialize for Register {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

impl InsnInfo for InsnWrapper {
    fn size(&self) -> Option<u16> {
        match self {
            InsnWrapper::Op(Instruction::JMP | Instruction::JMPZ, ..) => Some(8),
            _ => None,
        }
    }

    fn kind(&self) -> Option<JumpKind> {
        match self {
            InsnWrapper::Op(Instruction::JMP, ..) => Some(JumpKind::Unconditional),
            InsnWrapper::Op(Instruction::JMPZ, ..) => Some(JumpKind::Conditional),
            _ => None,
        }
    }
}

impl Serialize for Instruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
