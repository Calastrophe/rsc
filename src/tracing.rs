use crate::util::types::{Instruction, Register};
use ctrlflow::{
    instruction::JumpKind, register::Info, Architecture, InstructionInfo, RegisterInfo,
};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RSC;

impl Architecture for RSC {
    type Register = Register;
    type Instruction = Instruction;
    type AddressWidth = u32;
}

pub const REGISTER_INFOS: [Info<Register>; 9] = [
    Info::new("S", Register::S, Register::S, None, 1),
    Info::new("Z", Register::Z, Register::S, None, 1),
    Info::new("IR", Register::IR, Register::S, None, 4),
    Info::new("AR", Register::AR, Register::S, None, 4),
    Info::new("DR", Register::DR, Register::S, None, 4),
    Info::new("PC", Register::OUTR, Register::S, None, 4),
    Info::new("OUTR", Register::OUTR, Register::S, None, 4),
    Info::new("ACC", Register::ACC, Register::S, None, 4),
    Info::new("R", Register::R, Register::S, None, 4),
];

impl RegisterInfo for Register {
    fn info(&self) -> &'static Info<Self> {
        &REGISTER_INFOS[*self as usize]
    }

    fn iter() -> std::slice::Iter<'static, Self> {
        Register::iter()
    }
}

impl InstructionInfo for Instruction {
    fn size(&self) -> Option<u16> {
        match self {
            Instruction::JMP => Some(4),
            Instruction::JMPZ => Some(4),
            _ => None,
        }
    }

    fn kind(&self) -> Option<JumpKind> {
        match self {
            Instruction::JMP => Some(JumpKind::Unconditional),
            Instruction::JMPZ => Some(JumpKind::Conditional),
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
