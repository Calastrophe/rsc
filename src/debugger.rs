use crate::emulator::{util::Register, Assembler, Emulator};
use std::collections::HashMap;

pub struct Debugger {
    assembler: Assembler,
    emulator: Emulator,
    breakpoints: HashMap<u32, bool>,
}

impl Debugger {
    pub fn new(assembler: Assembler) -> Self {
        Self {
            emulator: Emulator::new(assembler.instructions()),
            breakpoints: HashMap::new(),
            assembler,
        }
    }

    /// Runs the loaded program until a breakpoint is hit or halted.
    pub fn start(&mut self) {
        while !self.should_stop() {
            self.emulator.cycle();
        }
    }

    /// Used to step over a breakpoint without disabling it.
    pub fn step_over(&mut self) {
        if !self.emulator.halted() {
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

    /// Sets an enabled breakpoint at the provided address.
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

    /// Determines if the debugger should yield execution.
    pub fn should_stop(&self) -> bool {
        self.emulator.halted() || self.query(self.emulator.registers.get(Register::PC))
    }
}
