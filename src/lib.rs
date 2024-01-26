pub mod assembler;
mod tracing;
pub mod util;

use ctrlflow::{Event, Tracer};
use std::collections::HashMap;
use tracing::{InsnWrapper, RSC};
use util::{
    types::{Instruction, Register},
    Memory, Registers,
};

use Register as R;

pub struct Emulator {
    pub registers: Registers,
    pub memory: Memory,
    tracer: Tracer<RSC>,
}

impl Emulator {
    pub fn new(log_name: &str, memory: HashMap<u32, u32>) -> Self {
        let tracer = Tracer::new(log_name, memory.iter()).unwrap();
        Emulator {
            registers: Registers::new(tracer.sender()),
            memory: Memory::new(memory, tracer.sender()),
            tracer,
        }
    }

    /// Starts the emulation of the given instructions to the emulator.
    pub fn start(&mut self) {
        while !self.halted() {
            self.cycle();
        }
    }

    // One cycle of execution
    pub fn cycle(&mut self) {
        self.check_z();

        // Grab the address of the next instruction and the instruction to be executed.
        let (addr, instruction) = self.fetch();

        // This is partially because we don't have a disassembler or pretty printer which can
        // disassemble the given instruction at an address. So rather than write that, this is just
        // a quick hack to the get the same functionality. However, it IS NOT required for this
        // tracing library.
        let wrapped = match instruction {
            Instruction::JMP | Instruction::JMPZ | Instruction::LDAC | Instruction::STAC => {
                InsnWrapper::Op(instruction, self.dereference(R::PC))
            }
            _ => InsnWrapper::NoOp(instruction),
        };

        // Tell the tracer we are now starting the given instruction.
        // It is important to remember that the instruction you pass over should be serialized as a
        // String. Almost as if it was just from a disassembler. However, its up to you as the
        // implementer what you want to show in the graph.
        let _ = self.tracer.send(Event::InsnStart(addr, wrapped));

        self.execute(instruction);
    }

    fn fetch(&mut self) -> (u32, Instruction) {
        self.registers.transfer(R::PC, R::AR);
        self.registers.set(R::DR, self.dereference(R::AR));
        let addr = self.registers.get(R::PC);
        self.inc_pc();
        self.registers.transfer(R::DR, R::IR);
        self.registers.transfer(R::PC, R::AR);
        (addr, self.registers.get(R::IR).into())
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

        self.tracer.send(Event::InsnEnd).unwrap()
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

    fn call(&mut self) {
        self.registers.set(R::DR, self.dereference(R::AR));
        self.registers.transfer(R::DR, R::PC);
    }

    fn ret(&mut self) {}

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

    pub fn terminate(self) {
        self.tracer.terminate().unwrap()
    }
}
