use std::collections::HashMap;
use crate::emulator::{Emulator, EmulatorErr};
use crate::parser::Assembler;

// The debugger is used by web targets.

// The purpose of our debugger is to plainly allow for seamless introspection of the emulator.
// We want to populate a memory view through egui with their table, need to figure that out.
// There should be two options, a pretty (annotated) view and just number view.

// Allow for easy symbol lookup and find location of variables in memory view.
// Implement backtracking in the debugger, need to keep track of all register/memory changes.
// NEED ABSOLUTE PROGRAM COUNTER

// Provide a current view of the registers in binary, hexadecimal, or decimal.
// Set breakpoints on certain program counters or labels.
// Be able to enable and disable said breakpoints.


pub struct Debugger<'a> {
    emulator: Emulator,
    assembler: Assembler<'a>,
    breakpoints: HashMap<u32, bool>,
}

impl<'a> Debugger<'a> {
    pub fn set_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address, true);
    }

    pub fn query(&mut self, address: u32) {
        self.breakpoints.iter().any(|(k,v)| *k == address);
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
        while !self.emulator.halted() {
            for step in 0..steps {
                self.emulator.cycle()
            }
        }
    }

}
