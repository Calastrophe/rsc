pub mod assembler;
pub mod memory;
pub mod util;

pub use assembler::Assembler;
use memory::{Memory, Registers};
use util::{Instruction, Register};

use Register as R;

pub struct Emulator {
    pub registers: Registers,
    pub memory: Memory,
}

impl Emulator {
    pub fn new(memory: &[u32]) -> Self {
        Emulator {
            registers: Registers::new(),
            memory: Memory::new(memory),
        }
    }

    pub fn halted(&self) -> bool {
        self.registers.get(R::S) == 1
    }

    /// One entire cycle of execution.
    pub fn cycle(&mut self) {
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

        self.update_z();
    }

    // Fetches the next instruction to be executed.
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

    // Loads the value of a given address into ACC.
    fn ldac(&mut self) {
        self.registers.set(R::DR, self.dereference(R::AR));
        self.inc_pc();
        self.registers.transfer(R::DR, R::AR);
        self.registers.set(R::DR, self.dereference(R::AR));
        self.registers.transfer(R::DR, R::ACC);
    }

    // Stores the current value of ACC at the given address.
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

    // Dereference the address in the address register and give it to the data register then transfer that value to the program counter.
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
        self.registers.set(
            R::ACC,
            self.registers
                .get(R::ACC)
                .wrapping_sub(self.registers.get(R::R)),
        )
    }

    fn add(&mut self) {
        self.registers.set(
            R::ACC,
            self.registers
                .get(R::ACC)
                .wrapping_add(self.registers.get(R::R)),
        )
    }

    fn inc(&mut self) {
        self.registers
            .set(R::ACC, self.registers.get(R::ACC).wrapping_add(1))
    }

    fn clac(&mut self) {
        self.registers.set(R::ACC, 0)
    }

    fn and(&mut self) {
        self.registers.set(
            R::ACC,
            self.registers.get(R::ACC) & self.registers.get(R::R),
        )
    }

    fn or(&mut self) {
        self.registers.set(
            R::ACC,
            self.registers.get(R::ACC) | self.registers.get(R::R),
        )
    }

    fn ashr(&mut self) {
        self.registers.set(R::ACC, self.registers.get(R::ACC) >> 1)
    }

    fn not(&mut self) {
        self.registers.set(R::ACC, !self.registers.get(R::ACC))
    }

    fn inc_pc(&mut self) {
        self.registers
            .set(R::PC, self.registers.get(R::PC).wrapping_add(1))
    }

    /// Dereferences the current address stored in the given register and retrieves the contents of said address from memory.
    fn dereference(&self, r: Register) -> u32 {
        self.memory.get(self.registers.get(r))
    }

    fn update_z(&mut self) -> bool {
        let z = self.registers.get(R::ACC) == 0;
        self.registers.set(R::Z, z as u32);
        z
    }
}
