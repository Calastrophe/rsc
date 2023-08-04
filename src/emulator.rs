use std::collections::HashMap;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum EmulatorErr {
    #[error("Failure to retrieve specified breakpoint")]
    BreakpointRetrievalFailure,
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

pub struct Registers {
    registers: [u32; 9],
    engine: TimelessEngine<(usize, u32)>,
}

impl Registers {
    pub fn new() -> Self {
        Registers{ registers: [0; 9], engine: TimelessEngine::new() }
    }

    /// Retrieves the given registers current value.
    #[inline(always)]
    pub fn get(&self, reg: Register) -> u32 {
        self.registers[reg as usize]
    }

    /// Sets the registers content to the passed value.
    #[inline(always)]
    pub fn set(&mut self, reg: Register, val: u32) {
        self.engine.add_change((reg as usize, self.registers[reg as usize]));
        self.registers[reg as usize] = val
    }

    /// Transfers the source register contents to the destination register.
    #[inline(always)]
    pub fn transfer(&mut self, src: Register, dest: Register) {
        self.engine.add_change((dest as usize, self.registers[dest as usize]));
        self.registers[dest as usize] = self.registers[src as usize]
    }

    fn revert(&mut self) {
        if let Some((_, changes)) = self.engine.step_backwards() {
            for (reg, val) in changes {
                self.registers[reg] = val
            }
        }
    }
}

pub struct Memory {
    memory: HashMap<u32, u32>,
    engine: TimelessEngine<(u32, u32)>,
}

impl Memory {
    pub fn new(instructions: &[u32]) -> Self {
        let mut memory = HashMap::new();
        for (count, instruction) in instructions.into_iter().enumerate() {
            let count = count as u32;
            memory.insert(count, *instruction);
        }
        Memory{ memory, engine: TimelessEngine::new() }
    }

    /// Retrieves the value at the given address.
    fn get(&self, address: u32) -> u32 {
        // Avoid the needless insertion, just keep returning zero until its set.
        match self.memory.get(&address) {
            Some(v) => *v,
            None => 0,
        }
    }

    /// Sets the value at the given address.
    fn set(&mut self, address: u32, val: u32) {
        self.engine.add_change((address, val));
        *self.memory.entry(address).or_default() = val
    }
    
    fn revert(&mut self) {
        if let Some((_, changes)) = self.engine.step_backwards() {
            for (addr, val) in changes {
                *self.memory.entry(addr).or_default() = val
            }
        }
    }
}


pub struct TimelessEngine<T> {
    step_counter: usize,
    changes: HashMap<usize, Vec<T>>,
}

impl<T> TimelessEngine<T> {
    pub fn new() -> Self {
        TimelessEngine { step_counter: 0, changes: HashMap::new() }
    }

    pub fn step_forward(&mut self) {
        self.step_counter += 1
    }

    pub fn step_backwards(&mut self) -> Option<(usize, Vec<T>)>{
        if self.step_counter > 0 {
            self.step_counter -= 1;
        }
        self.changes.remove_entry(&self.step_counter)
    }

    pub fn add_change(&mut self, c: T) {
       self.changes.entry(self.step_counter).or_insert(Vec::new()).push(c);
    }

}


pub struct Emulator {
    registers: Registers,
    memory: Memory,
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

    /// One cycle of execution for the emulator
    pub fn cycle(&mut self) {
        self.check_z();
        let instruction = self.fetch();
        self.execute(instruction);
        self.registers.engine.step_forward();
        self.memory.engine.step_forward();
    }

    pub fn step_back(&mut self) {
        self.registers.revert();
        self.memory.revert();
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
        self.registers.transfer(Register::ACC, Register::R)
    }

    fn movr(&mut self) {
        self.registers.transfer(Register::R, Register::ACC)
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
        let new_val = self.registers.get(Register::ACC) - self.registers.get(Register::R);
        self.registers.set(Register::ACC, new_val)
    }

    fn add(&mut self) {
        let new_val = self.registers.get(Register::ACC) + self.registers.get(Register::R);
        self.registers.set(Register::ACC, new_val)
    }

    fn inc(&mut self) {
        self.registers.set(Register::ACC, self.registers.get(Register::ACC) + 1)
    }

    fn clac(&mut self) {
        self.registers.set(Register::ACC, 0)
    }

    fn and(&mut self) {
        let new_val = self.registers.get(Register::ACC) & self.registers.get(Register::R);
        self.registers.set(Register::ACC, new_val)
    }

    fn or(&mut self) {
        let new_val = self.registers.get(Register::ACC) | self.registers.get(Register::R);
        self.registers.set(Register::ACC, new_val)
    }

    fn ashr(&mut self) {
        self.registers.set(Register::ACC, self.registers.get(Register::ACC) >> 1)
    }

    fn not(&mut self) {
        self.registers.set(Register::ACC, !self.registers.get(Register::ACC))
    }

    fn inc_pc(&mut self) {
        self.registers.set(Register::PC, self.registers.get(Register::PC) + 1)
    }

    fn get_memory_from_register(&self, r: Register) -> u32 {
        let var_address = self.registers.get(r);
        self.memory.get(var_address)
    }

    fn check_z(&mut self) -> bool {
        let acc = self.registers.get(Register::ACC);
        if acc == 0 {
            self.registers.set(Register::Z, 1);
            true
        } else {
            self.registers.set(Register::Z, 0);
            false
        }
    }

    pub fn halted(&self) -> bool {
        self.registers.get(Register::S) == 1
    }
}
