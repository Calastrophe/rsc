mod assembler;
mod util;

use assembler::Assembler;
use std::collections::HashMap;
use util::{
    types::{Instruction, Register},
    Memory, Registers,
};

pub struct Emulator {
    pub registers: Registers,
    pub memory: Memory,
    assembler: Assembler,
    breakpoints: HashMap<u32, bool>,
}

impl Emulator {
    pub fn new(assembler: Assembler) -> Self {
        Emulator {
            registers: Registers::new(),
            memory: Memory::new(assembler.instructions()),
            assembler,
            breakpoints: HashMap::new(),
        }
    }

    /// Starts the emulation of the given instructions to the emulator.
    pub fn start(&mut self) {
        while !self.should_stop() {
            self.cycle();
        }
    }

    /// Sets an enabled breakpoint at the given address
    pub fn set_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address, true);
    }

    /// Removes a breakpoint at a given address, returns if the removal acted on anything.
    pub fn remove_breakpoint(&mut self, address: u32) -> bool {
        self.breakpoints.remove(&address).is_some()
    }

    /// Returns if a given address is a breakpoint and is enabled
    pub fn query(&self, address: u32) -> bool {
        self.breakpoints.get(&address).is_some_and(|v| *v)
    }

    /// Enables a given breakpoint, returns false if the breakpoint does not exist.
    pub fn enable(&mut self, address: u32) -> bool {
        self.breakpoints.get_mut(&address).is_some_and(|v| {
            *v = true;
            true
        })
    }

    /// Disables a given breakpoint, returns false if the breakpoint does not exist.
    pub fn disable(&mut self, address: u32) -> bool {
        self.breakpoints.get_mut(&address).is_some_and(|v| {
            *v = false;
            true
        })
    }

    /// Steps over a breakpoint, this button will only show when breakpoint has been hit.
    pub fn step_over(&mut self) {
        if !self.halted() {
            self.cycle()
        }
    }

    /// Steps forward with a given amount of steps, will stop progresing when S == 1 or a breakpoint is hit.
    pub fn stepi(&mut self, steps: usize) {
        for _ in 0..steps {
            if !self.should_stop() {
                self.cycle()
            }
        }
    }

    /// Steps backward with a given amount of steps - will go all the way back to the beginning of execution.
    pub fn backi(&mut self, steps: usize) {
        // There are checks when stepping back that we can't go further than our beginning step count, 0.
        for _ in 0..steps {
            self.step_back()
        }
    }

    // One cycle of execution
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
        self.registers
            .set(Register::ACC, self.registers.get(Register::ACC) + 1)
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
        self.registers
            .set(Register::ACC, self.registers.get(Register::ACC) >> 1)
    }

    fn not(&mut self) {
        self.registers
            .set(Register::ACC, !self.registers.get(Register::ACC))
    }

    fn inc_pc(&mut self) {
        self.registers
            .set(Register::PC, self.registers.get(Register::PC) + 1)
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

    fn should_stop(&self) -> bool {
        self.halted() || self.query(self.registers.get(Register::PC))
    }
}
