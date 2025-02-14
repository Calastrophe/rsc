use crate::emulator::{util::Register, Assembler, Emulator};
use std::{
    collections::HashMap,
    thread::sleep,
    time::{Duration, Instant},
};

pub enum ExecutionState {
    Start,
    Halted,
    Paused,
    Running,
    Stepping,
    BreakpointHit,
}

pub struct Debugger {
    pub instructions_per_second: u32,
    pub execution_state: ExecutionState,
    breakpoints: HashMap<u32, bool>,
    emulator: Emulator,
}

impl Debugger {
    pub fn new(instructions: &[u32]) -> Self {
        Self {
            execution_state: ExecutionState::Start,
            instructions_per_second: 5,
            emulator: Emulator::new(instructions),
            breakpoints: HashMap::new(),
        }
    }

    fn set_execution_state(&mut self, state: ExecutionState) {
        self.execution_state = state;
    }

    /// Runs the loaded program until a breakpoint is hit or halted.
    pub fn run(&mut self) {
        self.set_execution_state(ExecutionState::Running);

        let time_per_instruction =
            Duration::from_secs_f32(1.0 / self.instructions_per_second as f32);
        let mut last_time = Instant::now();
        let mut accumulated_time = Duration::ZERO;

        while !self.should_stop() {
            let now = Instant::now();
            let elapsed = now.duration_since(last_time);
            accumulated_time += elapsed;
            last_time = now;

            while accumulated_time >= time_per_instruction {
                self.emulator.cycle();
                accumulated_time -= time_per_instruction;
            }

            sleep(Duration::from_micros(500));
        }
    }

    /// Steps over a breakpoint without disabling it.
    pub fn step_over(&mut self) {
        self.set_execution_state(ExecutionState::Stepping);

        if !self.halted() {
            self.emulator.cycle();
        }
    }

    /// Steps forward through execution path by 'steps' amount at a time.
    pub fn stepi(&mut self, steps: usize) {
        self.set_execution_state(ExecutionState::Stepping);

        for _ in 0..steps {
            if !self.should_stop() {
                self.emulator.cycle();
            }
        }
    }

    /// Traces back execution path by 'steps' amount at a time.
    pub fn backi(&mut self, steps: usize) {
        self.set_execution_state(ExecutionState::Stepping);

        for _ in 0..steps {
            if !self.emulator.registers.step_backward() {
                self.set_execution_state(ExecutionState::Start);
            }
            self.emulator.memory.step_backward();
        }
    }

    /// Traces back execution until we arrive back at the start.
    pub fn restart(&mut self) {
        loop {
            if !self.emulator.registers.step_backward() {
                self.set_execution_state(ExecutionState::Start);
                return;
            }
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

    /// Indicates whether the underlying emulator is halted.
    pub fn halted(&mut self) -> bool {
        if self.emulator.halted() {
            self.set_execution_state(ExecutionState::Halted);
            true
        } else {
            false
        }
    }

    /// Returns if a given address is a breakpoint and is enabled.
    pub fn query(&mut self, address: u32) -> bool {
        if self.breakpoints.get(&address).is_some_and(|v| *v) {
            self.set_execution_state(ExecutionState::BreakpointHit);
            true
        } else {
            false
        }
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

    /// A helper function to easily read registers from the underlying emulator.
    pub fn read_reg(&self, reg: Register) -> u32 {
        self.emulator.registers.get(reg)
    }

    /// A helper function to easily read memory from the underlying emulator.
    pub fn read_mem(&self, addr: u32) -> u32 {
        self.emulator.memory.get(addr)
    }

    /// Determines if the debugger should yield execution.
    pub fn should_stop(&mut self) -> bool {
        self.halted() || self.query(self.emulator.registers.get(Register::PC))
    }
}
