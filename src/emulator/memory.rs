use super::util::Register;
use std::collections::HashMap;

pub struct TimelessEngine<T: Copy> {
    step_counter: usize,
    changes: HashMap<usize, Vec<T>>,
}

impl<T: Copy> TimelessEngine<T> {
    pub fn new() -> Self {
        TimelessEngine {
            step_counter: 0,
            changes: HashMap::new(),
        }
    }

    /// Increases our current step in time by 1.
    pub fn step_forward(&mut self) {
        self.step_counter += 1
    }

    /// Steps backward one step in time, only drains the contents of the vector holding the previous step's changes.
    pub fn step_backward(&mut self) -> Option<std::vec::Drain<'_, T>> {
        if self.step_counter > 0 {
            self.step_counter -= 1;
        }

        self.changes
            .get_mut(&self.step_counter)
            .map(|v| v.drain(..))
    }

    /// Adds a change to the existing vector of changes or creates a new one for the current time step.
    pub fn add_change(&mut self, c: T) {
        self.changes
            .entry(self.step_counter)
            .and_modify(|v| v.push(c))
            .or_insert(vec![c]);
    }
}

pub struct Registers {
    registers: [u32; 9],
    engine: TimelessEngine<(Register, u32)>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: [0, 1, 0, 0, 0, 0, 0, 0, 0],
            engine: TimelessEngine::new(),
        }
    }

    /// Retrieves the given registers current value.
    pub fn get(&self, reg: Register) -> u32 {
        self.registers[reg as usize]
    }

    /// Sets the registers content to the passed value.
    pub fn set(&mut self, reg: Register, val: u32) {
        self.engine.add_change((reg, self.registers[reg as usize]));
        self.registers[reg as usize] = val
    }

    /// Transfers the source register contents to the destination register.
    pub fn transfer(&mut self, src: Register, dest: Register) {
        self.set(dest, self.get(src));
    }

    pub fn step_forward(&mut self) {
        self.engine.step_forward();
    }

    // Steps backwards and indicates if any changes were undone.
    pub fn step_backward(&mut self) -> bool {
        self.engine.step_backward().map_or(false, |changes| {
            for (reg, val) in changes.rev() {
                self.registers[reg as usize] = val
            }
            true
        })
    }
}

pub struct Memory {
    underlying: HashMap<u32, u32>,
    engine: TimelessEngine<(u32, u32)>,
}

impl Memory {
    pub fn new(instructions: &[u32]) -> Self {
        let mut memory = HashMap::new();
        for (count, instruction) in instructions.iter().enumerate() {
            memory.insert(count as u32, *instruction);
        }
        Memory {
            underlying: memory,
            engine: TimelessEngine::new(),
        }
    }

    /// Retrieves the value at the given address.
    pub fn get(&self, address: u32) -> u32 {
        // Avoid the needless insertion, just keep returning zero until its set.
        *self.underlying.get(&address).unwrap_or(&0)
    }

    /// Sets the value at the given address.
    pub fn set(&mut self, address: u32, val: u32) {
        self.engine.add_change((address, val));

        self.underlying
            .entry(address)
            .and_modify(|v| *v = val)
            .or_insert(val);
    }

    pub fn step_forward(&mut self) {
        self.engine.step_forward()
    }

    // Steps backwards and indicates if any changes were undone.
    pub fn step_backward(&mut self) -> bool {
        self.engine.step_backward().map_or(false, |changes| {
            for (address, val) in changes.rev() {
                self.underlying
                    .entry(address)
                    .and_modify(|v| *v = val)
                    .or_insert(val);
            }
            true
        })
    }
}
