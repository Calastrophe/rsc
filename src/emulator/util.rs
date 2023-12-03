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
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Z => "Z",
                Self::S => "S",
                Self::IR => "IR",
                Self::AR => "AR",
                Self::DR => "DR",
                Self::PC => "PC",
                Self::OUTR => "OUTR",
                Self::ACC => "ACC",
                Self::R => "R",
            }
        }

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

use types::Register;

pub struct TimelessEngine<T> {
    step_counter: usize,
    changes: HashMap<usize, Vec<T>>,
}

impl<T> TimelessEngine<T> {
    pub fn new() -> Self {
        TimelessEngine {
            step_counter: 0,
            changes: HashMap::new(),
        }
    }

    pub fn step_forward(&mut self) {
        self.step_counter += 1
    }

    pub fn step_backwards(&mut self) -> Option<Vec<T>> {
        if self.step_counter > 0 {
            self.step_counter -= 1;
        }

        self.changes.remove(&self.step_counter)
    }

    pub fn add_change(&mut self, c: T) {
        self.changes.entry(self.step_counter).or_insert(vec![c]);
    }
}

pub struct RegisterChange {
    reg: Register,
    val: u32,
}

pub struct Registers {
    pub registers: [u32; 9],
    pub engine: TimelessEngine<RegisterChange>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: [0; 9],
            engine: TimelessEngine::new(),
        }
    }

    /// Retrieves the given registers current value.
    pub fn get(&self, reg: Register) -> u32 {
        self.registers[reg as usize]
    }

    /// Sets the registers content to the passed value.
    pub fn set(&mut self, reg: Register, val: u32) {
        self.engine.add_change(RegisterChange {
            reg,
            val: self.registers[reg as usize],
        });
        self.registers[reg as usize] = val
    }

    /// Transfers the source register contents to the destination register.
    pub fn transfer(&mut self, src: Register, dest: Register) {
        self.set(dest, self.get(src));
    }

    pub fn revert(&mut self) {
        if let Some(changes) = self.engine.step_backwards() {
            for RegisterChange { reg, val } in changes.iter().rev() {
                self.registers[*reg as usize] = *val
            }
        }
    }
}

pub struct MemoryChange {
    address: u32,
    val: u32,
}

pub struct Memory {
    pub underlying: HashMap<u32, u32>,
    pub engine: TimelessEngine<MemoryChange>,
}

impl Memory {
    pub fn new(instructions: &[u32]) -> Self {
        let mut memory = HashMap::new();
        for (count, instruction) in instructions.iter().enumerate() {
            memory.insert(count as u32, *instruction);
        }
        Memory {
            underlying: memory,
            engine: TimelessEngine::new(),
        }
    }

    /// Retrieves the value at the given address.
    pub fn get(&self, address: u32) -> u32 {
        // Avoid the needless insertion, just keep returning zero until its set.
        match self.underlying.get(&address) {
            Some(v) => *v,
            None => 0,
        }
    }

    /// Sets the value at the given address.
    pub fn set(&mut self, address: u32, val: u32) {
        self.engine.add_change(MemoryChange { address, val });

        self.underlying
            .entry(address)
            .and_modify(|v| *v = val)
            .or_insert(val);
    }

    pub fn revert(&mut self) {
        if let Some(changes) = self.engine.step_backwards() {
            for MemoryChange { address, val } in changes.iter().rev() {
                *self.underlying.entry(*address).or_default() = *val
            }
        }
    }
}
