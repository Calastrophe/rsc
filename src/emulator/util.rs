use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An unknown keyword '{0}' was used on line {1}")]
    UnknownKeyword(String, usize),
    #[error("An operand was expected after '{0}' on line {1}")]
    MissingOperand(String, usize),
    #[error("An invalid initializer was used on '{0}' on line {1}")]
    InvalidInitializer(String, usize),
    #[error("An undefined variable '{0}' was used on line {1}")]
    UndefinedVariable(String, usize),
    #[error("An attempt to redefine '{0}' occurred on line {1}")]
    Redefinition(String, usize),
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
