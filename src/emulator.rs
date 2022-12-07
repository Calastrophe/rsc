use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
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

// impl Instruction {
//     fn as_str(&self) -> &'static str {
//         match self {
//             Instruction::HALT => "HALT",
//             Instruction::LDAC => "LDAC",
//             Instruction::STAC => "STAC",
//             Instruction::MVAC => "MVAC",
//             Instruction::MOVR => "MOVR",
//             Instruction::JMP => "JMP",
//             Instruction::JMPZ => "JMPZ",
//             Instruction::OUT => "OUT",
//             Instruction::SUB => "SUB",
//             Instruction::ADD => "ADD",
//             Instruction::INC => "INC",
//             Instruction::CLAC => "CLAC",
//             Instruction::AND => "AND",
//             Instruction::OR => "OR",
//             Instruction::ASHR => "ASHR",
//             Instruction::NOT => "NOT",
//         }
//     }
// }

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

// Seperate the instructions as a trait, so later, if you want to hot-change out instructions its easier!
// Add another trait and add your own instructions or rework current ones.
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

pub struct Emulator {
    pub registers: [i32; 9],
    memory: HashMap<i32, i32>,
}

impl Emulator {
    pub fn new(instructions: HashMap<i32, i32>) -> Self {
        Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            memory: instructions,
        }
    }

    // Read and write to registers, no need to check if valid position as it would fail at enum conversion.
    pub fn read_register(&self, reg: Register) -> i32 {
        self.registers[reg as usize]
    }

    pub fn write_register(&mut self, reg: Register, value: i32) {
        self.registers[reg as usize] = value;
    }

    // Transfers the current value in one register (source) to another (destination)
    pub fn transfer_value(&mut self, dest: Register, src: Register) {
        self.registers[dest as usize] = self.registers[src as usize]
    }

    // Reads the current value in one register as an address in memory, gets the value and sets to (source) to another (destination)
    pub fn set_from_memory(&mut self, dest: Register, src: Register) {
        let addr = self.read_register(src);
        let value = self.read_memory(addr);
        self.write_register(dest, value);
    }

    // This will read in memory a given address and return its stored value or assign the given address its normal value of 0.
    pub fn read_memory(&mut self, addr: i32) -> i32 {
        *self.memory.entry(addr).or_insert(0)
    }

    // Writes to the given address (key), if it doesn't yet exist (in the HashMap), create one and change its value.
    pub fn write_memory(&mut self, addr: i32, val: i32) {
        *self.memory.entry(addr).or_insert(0) = val;
    }

    // Increments the PC register by 1.
    pub fn increment_pc(&mut self) {
        let value = self.read_register(Register::pc) + 1;
        self.write_register(Register::pc, value)
    }

    // The typical fetch cycle, includes the check for Z.
    pub fn fetch(&mut self) {
        self.check_z();
        self.transfer_value(Register::ar, Register::pc);
        self.set_from_memory(Register::dr, Register::ar);
        self.increment_pc();
        self.transfer_value(Register::ir, Register::dr);
        self.transfer_value(Register::ar, Register::pc);
    }

    fn check_z(&mut self) {
        if self.read_register(Register::acc) == 0 {
            self.write_register(Register::z, 1)
        } else {
            self.write_register(Register::z, 0)
        }
    }

    // Determines if the program should exit by checking the register S.
    pub fn halted(&self) -> bool {
        if self.read_register(Register::s) == 1 {
            true
        } else {
            false
        }
    }

    // Matches the current value inside IR to its respective function or panics.
    pub fn execute(&mut self) {
        println!(
            "The instruction {:?} was executed.",
            Instruction::from(self.read_register(Register::ir))
        );
        match self.read_register(Register::ir) {
            0 => self.halt(),
            1 => self.ldac(),
            2 => self.stac(),
            3 => self.mvac(),
            4 => self.movr(),
            5 => self.jmp(),
            6 => self.jmpz(),
            7 => self.out(),
            8 => self.sub(),
            9 => self.add(),
            10 => self.inc(),
            11 => self.clac(),
            12 => self.and(),
            13 => self.or(),
            14 => self.ashr(),
            15 => self.not(),
            _ => unreachable!(),
        }
    }

    // TODO: Refactor
    pub fn display_contents(&self) {
        println!("S:    {:x}", self.registers[Register::s as usize]);
        println!("Z:    {:x}", self.registers[Register::z as usize]);
        println!("IR:   {:x}", self.registers[Register::ir as usize]);
        println!("AR:   {:x}", self.registers[Register::ar as usize]);
        println!("DR:   {:x}", self.registers[Register::dr as usize]);
        println!("PC:   {:x}", self.registers[Register::pc as usize]);
        println!("OUTR: {:x}", self.registers[Register::outr as usize]);
        println!("ACC:  {:x}", self.registers[Register::acc as usize]);
        println!("R:    {:x}", self.registers[Register::r as usize]);
    }
}

impl StandardInstructionDef for Emulator {
    fn add(&mut self) {
        let value = self.read_register(Register::acc) + self.read_register(Register::r);
        self.write_register(Register::acc, value)
    }
    fn sub(&mut self) {
        let value = self.read_register(Register::acc) - self.read_register(Register::r);
        self.write_register(Register::acc, value)
    }
    fn and(&mut self) {
        let value = self.read_register(Register::acc) & self.read_register(Register::r);
        self.write_register(Register::acc, value)
    }
    fn or(&mut self) {
        let value = self.read_register(Register::acc) | self.read_register(Register::r);
        self.write_register(Register::acc, value)
    }
    fn not(&mut self) {
        let value = !self.read_register(Register::acc);
        self.write_register(Register::acc, value)
    }
    fn ashr(&mut self) {
        let value = self.read_register(Register::acc) >> 1;
        self.write_register(Register::acc, value)
    }
    fn clac(&mut self) {
        self.write_register(Register::acc, 0)
    }
    fn inc(&mut self) {
        let value = self.read_register(Register::acc) + 1;
        self.write_register(Register::acc, value)
    }
    fn out(&mut self) {
        self.transfer_value(Register::outr, Register::acc)
    }
    fn mvac(&mut self) {
        self.transfer_value(Register::r, Register::acc)
    }
    fn movr(&mut self) {
        self.transfer_value(Register::acc, Register::r)
    }
    fn jmp(&mut self) {
        self.set_from_memory(Register::dr, Register::ar);
        self.transfer_value(Register::pc, Register::dr)
    }
    fn jmpz(&mut self) {
        if self.read_register(Register::z) == 1 {
            self.set_from_memory(Register::dr, Register::ar);
            self.transfer_value(Register::pc, Register::dr)
        } else {
            self.increment_pc()
        }
    }
    fn ldac(&mut self) {
        self.set_from_memory(Register::dr, Register::ar);
        self.increment_pc();
        self.transfer_value(Register::ar, Register::dr);
        self.set_from_memory(Register::dr, Register::ar);
        self.transfer_value(Register::acc, Register::dr)
    }
    fn stac(&mut self) {
        self.set_from_memory(Register::dr, Register::ar);
        self.increment_pc();
        self.transfer_value(Register::ar, Register::dr);
        self.transfer_value(Register::dr, Register::acc);
        self.write_memory(
            self.read_register(Register::ar),
            self.read_register(Register::dr),
        )
    }
    fn halt(&mut self) {
        self.write_register(Register::s, 1)
    }
}
