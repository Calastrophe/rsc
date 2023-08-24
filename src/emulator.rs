use crate::util::{Memory, Registers, types::{Instruction, Register, EmulatorErr}};
use std::collections::HashMap;
use crate::parser::Assembler;

pub struct Emulator {
    assembler: Assembler,
    pub registers: Registers,
    pub memory: Memory,
    breakpoints: HashMap<u32, bool>,
}

impl Emulator {
    pub fn new(assembler: Assembler, memory: Memory) -> Self {
        Emulator {
            assembler,
            registers: Registers::new(),
            memory,
            breakpoints: HashMap::new(),
        }
    }

    /// Starts the emulation of the given instructions to the emulator.
    pub fn start(&mut self) {
        while !self.halted() && !self.query(self.registers.get(Register::PC)) {
            self.cycle();
        }
    }

    pub fn set_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address, true);
    }

    pub fn query(&mut self, address: u32) -> bool {
        self.breakpoints.iter().any(|(k,v)| *k == address && *v == true)
    }

    pub fn enable(&mut self, address: u32) -> Result<(), EmulatorErr>  {
        *self.breakpoints.get_mut(&address).ok_or(EmulatorErr::BreakpointRetrievalFailure)? = true;
        Ok(())
    }

    pub fn disable(&mut self, address: u32) -> Result<(), EmulatorErr> {
        *self.breakpoints.get_mut(&address).ok_or(EmulatorErr::BreakpointRetrievalFailure)? = false;
        Ok(())
    }
    
    pub fn stepi(&mut self, steps: usize) {
        for _ in 0..steps {
            if !self.halted() && !self.query(self.registers.get(Register::PC)) {
                self.cycle()
            }
        }
    }

    pub fn backi(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step_back()
        }
    }

    fn cycle(&mut self) {
        self.check_z();
        let instruction = self.fetch();
        self.execute(instruction);
        self.registers.engine.step_forward();
        self.memory.engine.step_forward();
    }

    fn step_back(&mut self) {
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
        self.registers.transfer(Register::PC, Register::AR);
        self.registers.get(Register::IR).into()
    }

    // All the implementations of the various instructions are below...

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

    fn halted(&self) -> bool {
        self.registers.get(Register::S) == 1
    }
}
