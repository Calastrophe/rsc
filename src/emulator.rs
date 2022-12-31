use rustrsc::instruction_set::StandardInstructionDef;
use rustrsc::types::Register::*;
use rustrsc::types::{Instruction, Register};
use rustrsc::error_types::*;
use std::collections::{HashMap, BTreeMap};


pub struct Emulator {
    pub registers: [i32; 9],
    memory: HashMap<i32, i32>,
    debug_mode: bool,
    symbol_table: Option<HashMap<String, i32>>,
    holder_table: Option<HashMap<String, Vec<i32>>>,
    label_table: Option<BTreeMap<i32, String>>,
    breakpoints: Option<HashMap<i32, bool>>,
}

impl Emulator {
    pub fn new(instructions: HashMap<i32, i32>) -> Self {
        Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            memory: instructions,
            debug_mode: false,
            symbol_table: None,
            holder_table: None,
            label_table: None,
            breakpoints: None,
        }
    }
    pub fn debug(
        &mut self,
        symbol_table: Option<HashMap<String, i32>>,
        holder_table: Option<HashMap<String, Vec<i32>>>,
        label_table: Option<BTreeMap<i32, String>>
    ) -> &mut Emulator {
        self.debug_mode = true;
        self.symbol_table = symbol_table;
        self.holder_table = holder_table;
        self.label_table = label_table;
        self.breakpoints = Some(HashMap::from([(0, true)])); // We need a breakpoint for the first instruction otherwise execution will just happen like normal.
        self.symbol_table.as_mut().unwrap().insert("end".to_string(), self.memory.len() as i32);
        self
    }

    // Read and write to registers, no need to check if valid position as it would fail at enum conversion.
    pub fn read_register(&self, reg: Register) -> i32 {
        self.registers[reg as usize]
    }

    pub fn write_register(&mut self, reg: Register, value: i32) {
        self.registers[reg as usize] = value;
    }

    // Transfers the current value in one register (source) to another (destination)
    pub fn transfer_value(&mut self, dest: Register, src: Register) {
        self.registers[dest as usize] = self.registers[src as usize]
    }

    // Reads the current value in one register as an address in memory, gets the value and sets to (source) to another (destination)
    pub fn set_from_memory(&mut self, dest: Register, src: Register) {
        let addr = self.read_register(src);
        let value = self.read_memory(addr);
        self.write_register(dest, value);
    }

    // This will read in memory a given address and return its stored value or assign the given address its normal value of 0.
    pub fn read_memory(&mut self, addr: i32) -> i32 {
        *self.memory.entry(addr).or_insert(0)
    }

    // Writes to the given address (key), if it doesn't yet exist (in the HashMap), create one and change its value.
    pub fn write_memory(&mut self, addr: i32, val: i32) {
        *self.memory.entry(addr).or_insert(0) = val;
    }

    // Increments the PC register by 1.
    pub fn increment_pc(&mut self) {
        let value = self.read_register(pc) + 1;
        self.write_register(pc, value)
    }

    // The typical fetch cycle, includes the check for Z.
    pub fn fetch(&mut self) {
        self.check_z();
        self.transfer_value(ar, pc);
        self.set_from_memory(dr, ar);
        self.increment_pc();
        self.transfer_value(ir, dr);
        self.transfer_value(ar, pc);
    }

    fn check_z(&mut self) {
        if self.read_register(acc) == 0 {
            self.write_register(z, 1)
        } else {
            self.write_register(z, 0)
        }
    }

    // Determines if the program should exit by checking the register S.
    pub fn halted(&self) -> bool {
        self.read_register(s) == 1
    }

    // Matches the current value inside IR to its respective function or panics.
    pub fn execute(&mut self) {
        println!(
            "The instruction {:?} was executed.",
            Instruction::from(self.read_register(ir))
        );
        match self.read_register(ir) {
            0 => self.halt(),
            1 => self.ldac(),
            2 => self.stac(),
            3 => self.mvac(),
            4 => self.movr(),
            5 => self.jmp(),
            6 => self.jmpz(),
            7 => self.out(),
            8 => self.sub(),
            9 => self.add(),
            10 => self.inc(),
            11 => self.clac(),
            12 => self.and(),
            13 => self.or(),
            14 => self.ashr(),
            15 => self.not(),
            _ => unreachable!(),
        }
    }

    pub fn start(&mut self) {
        if self.debug_mode {
            while !self.halted() {
                self.handler();
                self.execute();
                self.fetch();
            }
        } else {
            while !self.halted() {
                self.execute();
                self.fetch();
            }
        }
    }

    pub fn display_contents(&self) {
        for reg in Register::iterator() {
            println!("{}: 0x{:X}", reg.as_str(), self.registers[reg as usize]);
        }
    }
    
    pub fn disas(&mut self) {
        let mut cloned_table = self.label_table.as_ref().unwrap().clone();
        let mut right = cloned_table.split_off(&self.read_register(ir));
        if let Some((start_pos, start_label)) = cloned_table.pop_last() {
            println!("{}", start_label);
            if let Some((end_pos, end_label)) = right.pop_first() {
                for addr in start_pos..=end_pos {
                    if self.query_holder(addr) {
                        println!("{:0<8} {}", addr, self.read_memory(addr))
                    } else {
                        println!("{:0<8} {}", addr, Into::<&str>::into(Instruction::from(self.read_memory(addr))))
                    }
                }
            } else {
                for addr in 0..=self.symbol_table.as_ref().unwrap()["end"] {
                    if self.query_holder(addr) {
                        println!("{:0<8} {}", addr, self.read_memory(addr))
                    } else {
                        println!("{:0<8} {}", addr, Into::<&str>::into(Instruction::from(self.read_memory(addr))))
                    }
                }
            }
        } else {
            unreachable!()
        }
    }

    // An unoptimized solution for identifying if a instruction at a certain position is actually a symbol or not.
    fn query_holder(&mut self, addr: i32) -> bool {
        for (_symbol, positions) in self.holder_table.as_mut().unwrap() {
            for address in positions {
                if *address == addr {
                    return true
                }
            }
        }
        false
    }

    // Sets a breakpoint for the emulator to await for a command at...
    // The reason for the Option<bool> is to allow for easy checking with handler() and if let
    pub fn bp(&mut self, addr: i32) -> Result<(), BreakpointExists> {
        if self.breakpoints.as_ref().unwrap().contains_key(&addr) {
            Err(BreakpointExists)
        } else {
            self.breakpoints.as_mut().unwrap().insert(addr, true);
            Ok(())
        }
    }

    pub fn bp_symbol(&mut self, sym: String) -> Result<(), Box<dyn std::error::Error>> {
        // Identify if that symbol exists
        if let Some(pos) = self.symbol_table.as_ref().unwrap().get(&sym) {
            // Check if its position is already a breakpoint
            if self.breakpoints.as_ref().unwrap().contains_key(pos) {
                Err(BreakpointExists.into()) // is already present
            } else {
                self.breakpoints.as_mut().unwrap().insert(*pos, true); // is not present
                Ok(())
            }
        } else {
            Err(SymbolNotFound.into()) // Symbol does not exist
        }
    }

    pub fn enable(&mut self, addr: i32) -> Result<(), BreakpointNonexistent> {
        if let Some(status) = self.breakpoints.as_mut().unwrap().get_mut(&addr) {
            *status = true;
            Ok(())
        }
        else {
            Err(BreakpointNonexistent)
        }
    }

    pub fn disable(&mut self, addr: i32) -> Result<(), BreakpointNonexistent> {
        if let Some(status) = self.breakpoints.as_mut().unwrap().get_mut(&addr) {
            *status = false;
            Ok(())
        }
        else {
            Err(BreakpointNonexistent)
        }
    }

    fn handler(&mut self) {
        let mut command = String::new();
        if let Ok(bytes_read) = std::io::stdin().read_line(&mut command) {
            match command.as_str() {
                "disas" => self.disas(),
                &_ => todo!()
            }
        }
    }
}

impl StandardInstructionDef for Emulator {
    fn add(&mut self) {
        let value = self.read_register(acc) + self.read_register(r);
        self.write_register(acc, value)
    }
    fn sub(&mut self) {
        let value = self.read_register(acc) - self.read_register(r);
        self.write_register(acc, value)
    }
    fn and(&mut self) {
        let value = self.read_register(acc) & self.read_register(r);
        self.write_register(acc, value)
    }
    fn or(&mut self) {
        let value = self.read_register(acc) | self.read_register(r);
        self.write_register(acc, value)
    }
    fn not(&mut self) {
        let value = !self.read_register(acc);
        self.write_register(acc, value)
    }
    fn ashr(&mut self) {
        let value = self.read_register(acc) >> 1;
        self.write_register(acc, value)
    }
    fn clac(&mut self) {
        self.write_register(acc, 0)
    }
    fn inc(&mut self) {
        let value = self.read_register(acc) + 1;
        self.write_register(acc, value)
    }
    fn out(&mut self) {
        self.transfer_value(outr, acc)
    }
    fn mvac(&mut self) {
        self.transfer_value(r, acc)
    }
    fn movr(&mut self) {
        self.transfer_value(acc, r)
    }
    fn jmp(&mut self) {
        self.set_from_memory(dr, ar);
        self.transfer_value(pc, dr)
    }
    fn jmpz(&mut self) {
        if self.read_register(z) == 1 {
            self.set_from_memory(dr, ar);
            self.transfer_value(pc, dr)
        } else {
            self.increment_pc()
        }
    }
    fn ldac(&mut self) {
        self.set_from_memory(dr, ar);
        self.increment_pc();
        self.transfer_value(ar, dr);
        self.set_from_memory(dr, ar);
        self.transfer_value(acc, dr)
    }
    fn stac(&mut self) {
        self.set_from_memory(dr, ar);
        self.increment_pc();
        self.transfer_value(ar, dr);
        self.transfer_value(dr, acc);
        self.write_memory(self.read_register(ar), self.read_register(dr))
    }
    fn halt(&mut self) {
        self.write_register(s, 1)
    }
}
