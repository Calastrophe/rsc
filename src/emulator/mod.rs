pub mod assembler;
pub mod util;

pub use assembler::Assembler;
use std::collections::HashMap;
use util::{
    types::{Instruction, Register},
    Memory, Registers,
};

use Register as R;

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

    /// Steps over a breakpoint.
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

    /// Steps backward with a given amount of steps.
    pub fn backi(&mut self, steps: usize) {
        // There are checks when stepping back that we can't go further than our beginning step count, 0.
        for _ in 0..steps {
            self.registers.step_backward();
            self.memory.step_backward();
        }
    }

    // One cycle of execution
    fn cycle(&mut self) {
        self.check_z();

        // Fetch, decode, and execute next instruction.
        let instruction = self.fetch();
        self.execute(instruction);

        // Timeless engine steps forward one step in execution
        self.registers.step_forward();
        self.memory.step_forward();
    }

    fn execute(&mut self, i: Instruction) {
        use Instruction::*;

        match i {
            LDAC => self.ldac(),
            STAC => self.stac(),
            JMP => self.jmp(),
            JMPZ => self.jmpz(),
            INC => self.inc(),
            MVAC => self.mvac(),
            MOVR => self.movr(),
            OUT => self.out(),
            ADD => self.add(),
            SUB => self.sub(),
            ASHR => self.ashr(),
            NOT => self.not(),
            OR => self.or(),
            AND => self.and(),
            CLAC => self.clac(),
            HALT => self.halt(),
        }
    }

    fn fetch(&mut self) -> Instruction {
        self.registers.transfer(R::PC, R::AR);
        self.registers.set(R::DR, self.dereference(R::AR));
        self.inc_pc();
        self.registers.transfer(R::DR, R::IR);
        self.registers.transfer(R::PC, R::AR);
        self.registers.get(R::IR).into()
    }

    fn halt(&mut self) {
        self.registers.set(R::S, 1);
    }

    fn ldac(&mut self) {
        self.registers.set(R::DR, self.dereference(R::AR));
        self.inc_pc();
        self.registers.transfer(R::DR, R::AR);
        self.registers.set(R::DR, self.dereference(R::AR));
        self.registers.transfer(R::DR, R::ACC);
    }

    fn stac(&mut self) {
        self.registers.set(R::DR, self.dereference(R::AR));
        self.inc_pc();
        self.registers.transfer(R::DR, R::AR);
        self.registers.transfer(R::ACC, R::DR);
        self.memory
            .set(self.registers.get(R::AR), self.registers.get(R::DR));
    }

    fn mvac(&mut self) {
        self.registers.transfer(R::ACC, R::R)
    }

    fn movr(&mut self) {
        self.registers.transfer(R::R, R::ACC)
    }

    fn jmp(&mut self) {
        self.registers.set(R::DR, self.dereference(R::AR));
        self.registers.transfer(R::DR, R::PC);
    }

    fn jmpz(&mut self) {
        if self.registers.get(R::Z) == 1 {
            self.jmp()
        } else {
            self.inc_pc()
        }
    }

    fn out(&mut self) {
        self.registers.transfer(R::ACC, R::OUTR);
    }

    fn sub(&mut self) {
        let new_val = self.registers.get(R::ACC) - self.registers.get(R::R);
        self.registers.set(R::ACC, new_val)
    }

    fn add(&mut self) {
        let new_val = self.registers.get(R::ACC) + self.registers.get(R::R);
        self.registers.set(R::ACC, new_val)
    }

    fn inc(&mut self) {
        self.registers.set(R::ACC, self.registers.get(R::ACC) + 1)
    }

    fn clac(&mut self) {
        self.registers.set(R::ACC, 0)
    }

    fn and(&mut self) {
        let new_val = self.registers.get(R::ACC) & self.registers.get(R::R);
        self.registers.set(R::ACC, new_val)
    }

    fn or(&mut self) {
        let new_val = self.registers.get(R::ACC) | self.registers.get(R::R);
        self.registers.set(R::ACC, new_val)
    }

    fn ashr(&mut self) {
        self.registers.set(R::ACC, self.registers.get(R::ACC) >> 1)
    }

    fn not(&mut self) {
        self.registers.set(R::ACC, !self.registers.get(R::ACC))
    }

    fn inc_pc(&mut self) {
        self.registers.set(R::PC, self.registers.get(R::PC) + 1)
    }

    /// Dereferences the current address stored in the given register
    fn dereference(&self, r: Register) -> u32 {
        let address = self.registers.get(r);
        self.memory.get(address)
    }

    fn check_z(&mut self) -> bool {
        let acc = self.registers.get(R::ACC);
        if acc == 0 {
            self.registers.set(R::Z, 1);
            true
        } else {
            self.registers.set(R::Z, 0);
            false
        }
    }

    fn halted(&self) -> bool {
        self.registers.get(R::S) == 1
    }

    fn should_stop(&self) -> bool {
        self.halted() || self.query(self.registers.get(R::PC))
    }
}
