use std::collections::HashMap;

pub mod types {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Unknown keyword '{0}' used on line {1}")]
        UnknownKeyword(String, usize),
        #[error("Expected an operand after '{0}' on line {1}")]
        ExpectedOperand(String, usize),
        #[error("Invalid operand defined for variable '{0}' on line {1}")]
        InvalidOperand(String, usize),
        #[error("Unknown variable '{0}' used as operand")]
        UnknownVariable(String),
        #[error("IO Error: {0}")]
        IO(#[from] std::io::Error),
    }

    /// All registers in the RSC architecture.
    #[derive(Debug, Clone, Copy)]
    pub enum Register {
        S,
        Z,
        IR,
        AR,
        DR,
        PC,
        OUTR,
        ACC,
        R,
    }

    impl Register {
        pub fn iter() -> std::slice::Iter<'static, Register> {
            [
                Register::S,
                Register::Z,
                Register::IR,
                Register::AR,
                Register::DR,
                Register::PC,
                Register::OUTR,
                Register::ACC,
                Register::R,
            ]
            .iter()
        }
    }

    #[derive(Debug, Clone, Copy)]
    /// All instructions in the RSC architecture.
    pub enum Instruction {
        HALT,
        LDAC,
        STAC,
        MVAC,
        MOVR,
        JMP,
        JMPZ,
        OUT,
        SUB,
        ADD,
        INC,
        CLAC,
        AND,
        OR,
        ASHR,
        NOT,
    }

    impl Instruction {
        pub fn has_operand(&self) -> bool {
            matches!(self, Self::LDAC | Self::STAC | Self::JMP | Self::JMPZ)
        }

        pub fn as_str(&self) -> &'static str {
            match self {
                Instruction::HALT => "HALT",
                Instruction::LDAC => "LDAC",
                Instruction::STAC => "STAC",
                Instruction::MVAC => "MVAC",
                Instruction::MOVR => "MOVR",
                Instruction::JMP => "JMP",
                Instruction::JMPZ => "JMPZ",
                Instruction::OUT => "OUT",
                Instruction::SUB => "SUB",
                Instruction::ADD => "ADD",
                Instruction::INC => "INC",
                Instruction::CLAC => "CLAC",
                Instruction::AND => "AND",
                Instruction::OR => "OR",
                Instruction::ASHR => "ASHR",
                Instruction::NOT => "NOT",
            }
        }
    }

    impl TryFrom<&str> for Instruction {
        type Error = ();

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            match s {
                "HALT" => Ok(Self::HALT),
                "LDAC" => Ok(Self::LDAC),
                "STAC" => Ok(Self::STAC),
                "MVAC" => Ok(Self::MVAC),
                "MOVR" => Ok(Self::MOVR),
                "JMP" => Ok(Self::JMP),
                "JMPZ" => Ok(Self::JMPZ),
                "OUT" => Ok(Self::OUT),
                "SUB" => Ok(Self::SUB),
                "ADD" => Ok(Self::ADD),
                "INC" => Ok(Self::INC),
                "CLAC" => Ok(Self::CLAC),
                "AND" => Ok(Self::AND),
                "OR" => Ok(Self::OR),
                "ASHR" => Ok(Self::ASHR),
                "NOT" => Ok(Self::NOT),
                _ => Err(()),
            }
        }
    }

    impl From<u32> for Instruction {
        fn from(value: u32) -> Self {
            match value {
                0 => Self::HALT,
                1 => Self::LDAC,
                2 => Self::STAC,
                3 => Self::MVAC,
                4 => Self::MOVR,
                5 => Self::JMP,
                6 => Self::JMPZ,
                7 => Self::OUT,
                8 => Self::SUB,
                9 => Self::ADD,
                10 => Self::INC,
                11 => Self::CLAC,
                12 => Self::AND,
                13 => Self::OR,
                14 => Self::ASHR,
                15 => Self::NOT,
                _ => unreachable!(),
            }
        }
    }
}

use crate::tracing::RSC;
use ctrlflow::{tracer::EventSender, Event};
use types::Register;

pub struct Registers {
    registers: [u32; 9],
    sender: EventSender<RSC>,
}

impl Registers {
    pub fn new(sender: EventSender<RSC>) -> Self {
        Registers {
            registers: [0; 9],
            sender,
        }
    }

    /// Retrieves the given registers current value.
    pub fn get(&self, reg: Register) -> u32 {
        let _ = self.sender.send(Event::RegRead(reg)).unwrap();
        self.registers[reg as usize]
    }

    /// Sets the registers content to the passed value.
    pub fn set(&mut self, reg: Register, val: u32) {
        let _ = self
            .sender
            .send(Event::RegWrite(reg, Box::from(val.to_le_bytes())))
            .unwrap();

        self.registers[reg as usize] = val
    }

    /// Transfers the source register contents to the destination register.
    pub fn transfer(&mut self, src: Register, dest: Register) {
        self.set(dest, self.get(src));
    }
}

pub struct Memory {
    underlying: HashMap<u32, u32>,
    sender: EventSender<RSC>,
}

impl Memory {
    pub fn new(memory: HashMap<u32, u32>, sender: EventSender<RSC>) -> Self {
        Memory {
            underlying: memory,
            sender,
        }
    }

    /// Retrieves the value at the given address.
    pub fn get(&self, address: u32) -> u32 {
        // Avoid the needless insertion, just keep returning zero until its set.
        let _ = self.sender.send(Event::MemRead(address)).unwrap();

        match self.underlying.get(&address) {
            Some(v) => *v,
            None => 0,
        }
    }

    /// Sets the value at the given address.
    pub fn set(&mut self, address: u32, val: u32) {
        let _ = self.sender.send(Event::MemWrite(address, val)).unwrap();

        self.underlying
            .entry(address)
            .and_modify(|v| *v = val)
            .or_insert(val);
    }
}
