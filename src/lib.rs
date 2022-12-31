
pub mod types {

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub enum Register {
        acc,
        r,
        ir,
        pc,
        dr,
        ar,
        outr,
        z,
        s,
    }

    // https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
    impl Register {
        pub fn iterator() -> impl Iterator<Item = Register> {
            [
                Register::acc,
                Register::r,
                Register::ir,
                Register::pc,
                Register::dr,
                Register::ar,
                Register::outr,
                Register::z,
                Register::s,
            ]
            .iter()
            .copied()
        }
        pub fn as_str(&self) -> &str {
            match self {
                acc => "ACC",
                r => "R",
                ir => "IR",
                pc => "PC",
                dr => "DR",
                ar => "AR",
                outr => "OUTR",
                z => "Z",
                s => "S",
            }
        }
    }

    #[derive(Debug)]
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

    impl<'a> From<&'a str> for Instruction {
        fn from(other: &'a str) -> Self {
            match other {
                "HALT" => Instruction::HALT,
                "LDAC" => Instruction::LDAC,
                "STAC" => Instruction::STAC,
                "MVAC" => Instruction::MVAC,
                "MOVR" => Instruction::MOVR,
                "JMP" => Instruction::JMP,
                "JMPZ" => Instruction::JMPZ,
                "OUT" => Instruction::OUT,
                "SUB" => Instruction::SUB,
                "ADD" => Instruction::ADD,
                "INC" => Instruction::INC,
                "CLAC" => Instruction::CLAC,
                "AND" => Instruction::AND,
                "OR" => Instruction::OR,
                "ASHR" => Instruction::ASHR,
                "NOT" => Instruction::NOT,
                _ => panic!("Invalid conversion of &str to Instruction enum."),
            }
        }
    }

    impl<'a> Into<&'a str> for Instruction {
        fn into(self) -> &'a str {
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

    impl From<i32> for Instruction {
        fn from(other: i32) -> Self {
            match other {
                0 => Instruction::HALT,
                1 => Instruction::LDAC,
                2 => Instruction::STAC,
                3 => Instruction::MVAC,
                4 => Instruction::MOVR,
                5 => Instruction::JMP,
                6 => Instruction::JMPZ,
                7 => Instruction::OUT,
                8 => Instruction::SUB,
                9 => Instruction::ADD,
                10 => Instruction::INC,
                11 => Instruction::CLAC,
                12 => Instruction::AND,
                13 => Instruction::OR,
                14 => Instruction::ASHR,
                15 => Instruction::NOT,
                _ => unreachable!(),
            }
        }
    }
}

pub mod instruction_set {
    pub trait StandardInstructionDef {
        fn not(&mut self);
        fn ashr(&mut self);
        fn or(&mut self);
        fn and(&mut self);
        fn clac(&mut self);
        fn inc(&mut self);
        fn add(&mut self);
        fn sub(&mut self);
        fn out(&mut self);
        fn jmp(&mut self);
        fn jmpz(&mut self);
        fn movr(&mut self);
        fn mvac(&mut self);
        fn stac(&mut self);
        fn ldac(&mut self);
        fn halt(&mut self);
    }
}

pub mod error_types { 
    
    macro_rules! error_type {
        ($name:ident, $resp:expr) => {
            #[derive(Debug)]
            pub struct $name;

            impl std::error::Error for $name {}

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, $resp)
                }
            }
        };
    }


    error_type!(BreakpointNonexistent, "This breakpoint does not exist.");
    error_type!(BreakpointExists, "This breakpoint already exists.");
    error_type!(SymbolNotFound, "The given symbol does not exist.");
}