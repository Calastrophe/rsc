use super::util::Register;

struct Change(usize, u32);

pub struct TimelessEngine {
    time_step: usize,
    changes: Vec<Vec<Change>>,
}

impl TimelessEngine {
    pub fn new() -> Self {
        TimelessEngine {
            time_step: 0,
            changes: vec![vec![]],
        }
    }

    /// Increases our current step in time by 1.
    pub fn step_forward(&mut self) {
        self.time_step += 1;

        if self.changes.get(self.time_step).is_none() {
            self.changes.push(Vec::new())
        }
    }

    /// Steps backward one step in time, only drains the contents of the vector holding the previous step's changes.
    pub fn step_backward(&mut self) -> Option<std::vec::Drain<'_, Change>> {
        if self.time_step > 0 {
            self.time_step -= 1;
        }

        self.changes.get_mut(self.time_step).map(|v| v.drain(..))
    }

    /// Adds a change to the existing vector of changes or creates a new one for the current time step.
    pub fn add_change(&mut self, idx: usize, value: u32) {
        self.changes[self.time_step].push(Change(idx, value));
    }
}

pub struct Registers {
    registers: [u32; 9],
    engine: TimelessEngine,
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
        self.engine
            .add_change(reg as usize, self.registers[reg as usize]);
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
            for Change(reg, val) in changes.rev() {
                self.registers[reg as usize] = val
            }
            true
        })
    }
}

pub struct Memory {
    underlying: Vec<u32>,
    engine: TimelessEngine,
}

impl Memory {
    pub fn new(instructions: &[u32]) -> Self {
        Memory {
            underlying: instructions.to_vec(),
            engine: TimelessEngine::new(),
        }
    }

    /// Retrieves the value at the given address.
    pub fn get(&self, address: u32) -> u32 {
        self.underlying[address as usize]
    }

    /// Sets the value at the given address.
    pub fn set(&mut self, address: u32, val: u32) {
        self.engine.add_change(address as usize, val);

        self.underlying[address as usize] = val;
    }

    pub fn step_forward(&mut self) {
        self.engine.step_forward()
    }

    // Steps backwards and indicates if any changes were undone.
    pub fn step_backward(&mut self) -> bool {
        self.engine.step_backward().map_or(false, |changes| {
            for Change(address, val) in changes.rev() {
                self.underlying[address] = val;
            }
            true
        })
    }
}
