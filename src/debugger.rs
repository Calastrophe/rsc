use crate::emulator::{util::Register, Emulator};
use std::collections::HashSet;

pub mod event;
pub mod message;
pub mod state;

pub struct Debugger {
    pub instructions_per_second: u32,
    breakpoints: HashSet<u32>,
    pub emulator: Emulator,
}

impl Debugger {
    pub fn new(instructions: &[u32]) -> Self {
        Self {
            instructions_per_second: 5,
            emulator: Emulator::new(instructions),
            breakpoints: HashSet::new(),
        }
    }

    /// Steps over a breakpoint without disabling it.
    pub fn step_over(&mut self) {
        if !self.halted() {
            self.emulator.cycle();
        }
    }

    /// Steps forward through execution path by 'steps' amount at a time.
    pub fn stepi(&mut self, steps: usize) {
        for _ in 0..steps {
            if !self.should_stop() {
                self.emulator.cycle();
            }
        }
    }

    /// Traces back execution path by 'steps' amount at a time.
    pub fn backi(&mut self, steps: usize) {
        for _ in 0..steps {
            self.emulator.registers.step_backward();
            self.emulator.memory.step_backward();
        }
    }

    /// Traces back execution until we arrive back at the start.
    pub fn restart(&mut self) {
        loop {
            if !self.emulator.registers.step_backward() {
                return;
            }
            self.emulator.memory.step_backward();
        }
    }

    /// Sets an enabled breakpoint at the provided address.
    pub fn set_breakpoint(&mut self, address: u32) {
        self.breakpoints.insert(address);
    }

    /// Removes a breakpoint at a given address, returns if the removal acted on anything.
    pub fn remove_breakpoint(&mut self, address: u32) -> bool {
        self.breakpoints.remove(&address)
    }

    /// Indicates whether the underlying emulator is halted.
    pub fn halted(&mut self) -> bool {
        self.emulator.halted()
    }

    /// Returns if a given address is a breakpoint and is enabled.
    pub fn query(&mut self, address: u32) -> bool {
        self.breakpoints.get(&address).is_some()
    }

    /// Determines if the debugger should yield execution.
    pub fn should_stop(&mut self) -> bool {
        self.halted() || self.query(self.emulator.registers.get(Register::PC))
    }
}
