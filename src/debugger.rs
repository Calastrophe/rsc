use crate::{
    assembler::Assembler,
    emulator::{Emulator, Register},
};

// Wrap an emulator instance and retain information from lexing/parsing stage.
// Implement efficient backtracking of instructions and saving state.
pub struct Debugger<'a> {
    emulator: Emulator,
    assembler: Assembler<'a>,
    breakpoints: Vec<u32>,
}

impl<'a> Debugger<'a> {
    pub fn new(emulator: Emulator, assembler: Assembler<'a>) -> Self {
        Debugger {
            emulator,
            assembler,
            breakpoints: vec![0],
        }
    }

    pub fn query_breakpoints(&self) -> bool {
        self.breakpoints
            .iter()
            .any(|v| *v == self.emulator.registers.get(Register::PC))
    }

    pub fn breakpoint_handler(&mut self) {}

    pub fn start(&mut self) {
        while !self.emulator.halted() {
            if self.query_breakpoints() {
                self.breakpoint_handler()
            }
            self.emulator.cycle()
        }
    }
}
