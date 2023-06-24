use std::collections::HashMap;
use thiserror::Error;

/// All registers in the RSC architecture.
#[derive(Debug)]
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

#[derive(Error, Debug)]
pub enum EmulatorErr {
    #[error("An invalid instruction `{0}` was parsed")]
    TranslationError(String),
}

impl std::str::FromStr for Instruction {
    type Err = EmulatorErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HALT" => Ok(Instruction::HALT),
            "LDAC" => Ok(Instruction::LDAC),
            "STAC" => Ok(Instruction::STAC),
            "MVAC" => Ok(Instruction::MVAC),
            "MOVR" => Ok(Instruction::MOVR),
            "JMP" => Ok(Instruction::JMP),
            "JMPZ" => Ok(Instruction::JMPZ),
            "OUT" => Ok(Instruction::OUT),
            "SUB" => Ok(Instruction::SUB),
            "ADD" => Ok(Instruction::ADD),
            "INC" => Ok(Instruction::INC),
            "CLAC" => Ok(Instruction::CLAC),
            "AND" => Ok(Instruction::AND),
            "OR" => Ok(Instruction::OR),
            "ASHR" => Ok(Instruction::ASHR),
            "NOT" => Ok(Instruction::NOT),
            _ => Err(EmulatorErr::TranslationError(s.to_owned())),
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
            _ => unreachable!("Invalid instruction translated"),
        }
    }
}

pub struct Registers([u32; 9]);

impl Registers {
    pub fn new() -> Self {
        Registers([0; 9])
    }

    /// Retrieves the given registers current value.
    pub fn get(&self, reg: Register) -> u32 {
        self.0[reg as usize]
    }

    /// Sets the registers content to the passed value.
    pub fn set(&mut self, reg: Register, val: u32) {
        self.0[reg as usize] = val
    }

    /// Retrieves a mutuable reference to a register for easy manipulation.
    pub fn get_mut(&mut self, reg: Register) -> &mut u32 {
        &mut self.0[reg as usize]
    }

    /// Transfers the source register contents to the destination register.
    pub fn transfer(&mut self, src: Register, dest: Register) {
        self.0[dest as usize] = self.0[src as usize]
    }
}

#[derive(Debug)]
pub struct Memory(HashMap<u32, u32>);

impl Memory {
    pub fn new(instructions: &[u32]) -> Self {
        let mut memory = HashMap::new();
        for (count, instruction) in instructions.into_iter().enumerate() {
            let count = count as u32;
            memory.insert(count, *instruction);
        }
        Memory(memory)
    }

    /// Retrieves the value at the given address.
    fn get(&self, address: u32) -> u32 {
        // Avoid the needless insertion, just keep returning zero until its set.
        match self.0.get(&address) {
            Some(v) => *v,
            None => 0,
        }
    }

    /// Sets the value at the given address.
    fn set(&mut self, address: u32, val: u32) {
        *self.0.entry(address).or_default() = val
    }
}

pub struct Emulator {
    pub registers: Registers,
    pub memory: Memory,
}

impl Emulator {
    pub fn new(mem: Memory) -> Self {
        Emulator {
            registers: Registers::new(),
            memory: mem,
        }
    }

    /// Starts the emulation of the given instructions to the emulator.
    pub fn start(&mut self) {
        while !self.halted() {
            self.cycle();
        }
    }

    /// Starts the emulation and executes until a given program counter is hit.
    pub fn execute_until(&mut self, target_pc: u32) {
        while !self.halted() || self.registers.get(Register::PC) == target_pc {
            self.cycle()
        }
    }

    pub fn cycle(&mut self) {
        self.check_z();
        let instruction = self.fetch();
        self.execute(instruction)
    }

    fn execute(&mut self, i: Instruction) {
        match i {
            Instruction::LDAC => self.ldac(),
            Instruction::STAC => self.stac(),
            Instruction::JMP => self.jmp(),
            Instruction::JMPZ => self.jmpz(),
            Instruction::INC => self.inc(),
            Instruction::MVAC => self.mvac(),
            Instruction::MOVR => self.movr(),
            Instruction::OUT => self.out(),
            Instruction::ADD => self.add(),
            Instruction::SUB => self.sub(),
            Instruction::ASHR => self.ashr(),
            Instruction::NOT => self.not(),
            Instruction::OR => self.or(),
            Instruction::AND => self.and(),
            Instruction::CLAC => self.clac(),
            Instruction::HALT => self.halt(),
        }
    }

    fn fetch(&mut self) -> Instruction {
        self.registers.transfer(Register::PC, Register::AR);
        self.registers
            .set(Register::DR, self.get_memory_from_register(Register::AR));
        self.inc_pc();
        self.registers.transfer(Register::DR, Register::IR);
        self.registers.transfer(Register::PC, Register::AR);
        self.registers.get(Register::IR).into()
    }

    fn halt(&mut self) {
        self.registers.set(Register::S, 1);
    }

    fn ldac(&mut self) {
        let var = self.get_memory_from_register(Register::AR);
        self.registers.set(Register::DR, var);
        self.inc_pc();
        self.registers.transfer(Register::DR, Register::AR);
        let var = self.get_memory_from_register(Register::AR);
        self.registers.set(Register::DR, var);
        self.registers.transfer(Register::DR, Register::ACC);
    }

    fn stac(&mut self) {
        let var = self.get_memory_from_register(Register::AR);
        self.registers.set(Register::DR, var);
        self.inc_pc();
        self.registers.transfer(Register::DR, Register::AR);
        self.registers.transfer(Register::ACC, Register::DR);
        let var_address = self.registers.get(Register::AR);
        let value = self.registers.get(Register::DR);
        self.memory.set(var_address, value);
    }

    fn mvac(&mut self) {
        self.registers.transfer(Register::R, Register::ACC)
    }

    fn movr(&mut self) {
        self.registers.transfer(Register::ACC, Register::R)
    }

    fn jmp(&mut self) {
        let new_pc = self.get_memory_from_register(Register::AR);
        // This is only for emulation purposes, it is redundant here.
        self.registers.set(Register::DR, new_pc);

        self.registers.set(Register::PC, new_pc);
    }

    fn jmpz(&mut self) {
        if self.registers.get(Register::Z) == 1 {
            self.jmp()
        } else {
            self.inc_pc()
        }
    }

    fn out(&mut self) {
        self.registers.transfer(Register::ACC, Register::OUTR);
    }

    fn sub(&mut self) {
        *self.registers.get_mut(Register::ACC) -= self.registers.get(Register::R);
    }

    fn add(&mut self) {
        *self.registers.get_mut(Register::ACC) += self.registers.get(Register::R);
    }

    fn inc(&mut self) {
        *self.registers.get_mut(Register::ACC) += 1;
    }

    fn clac(&mut self) {
        *self.registers.get_mut(Register::ACC) = 0;
    }

    fn and(&mut self) {
        *self.registers.get_mut(Register::ACC) &= self.registers.get(Register::R);
    }

    fn or(&mut self) {
        *self.registers.get_mut(Register::ACC) |= self.registers.get(Register::R);
    }

    fn ashr(&mut self) {
        *self.registers.get_mut(Register::ACC) >>= 1;
    }

    fn not(&mut self) {
        *self.registers.get_mut(Register::ACC) = !self.registers.get(Register::ACC)
    }

    fn inc_pc(&mut self) {
        *self.registers.get_mut(Register::PC) += 1;
    }

    fn get_memory_from_register(&self, r: Register) -> u32 {
        let var_address = self.registers.get(r);
        self.memory.get(var_address)
    }

    fn check_z(&mut self) -> bool {
        let acc = self.registers.get(Register::ACC);
        let z = self.registers.get_mut(Register::Z);
        if acc == 0 {
            *z = 1;
            true
        } else {
            *z = 0;
            false
        }
    }

    pub fn halted(&self) -> bool {
        self.registers.get(Register::S) == 1
    }
}
