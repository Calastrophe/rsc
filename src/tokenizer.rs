use itertools::Itertools;
use rustrsc::types::Instruction;
use std::collections::{BTreeMap, HashMap};
use std::i32;
use std::io::Write;

pub struct Tokenizer {
    pub instructions: HashMap<i32, i32>,
    symbol_table: HashMap<String, i32>,
    holder_table: HashMap<String, Vec<i32>>,
    label_table: BTreeMap<i32, String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            instructions: HashMap::new(),
            symbol_table: HashMap::new(),
            holder_table: HashMap::new(),
            label_table: BTreeMap::new(),
        }
    }

    pub fn parse(&mut self, s: &str) {
        // Iterate over each line in the given input.
        for line in s.lines() {
            // Rip off the tabs/spaces before the juicy instructions.
            let line = line.trim_start();
            // Split up the line with the space delimiter.
            let line_split = line.split(' ');
            let mut peek_split = line_split.clone().peekable();
            while let Some(token) = peek_split.next() {
                if token.contains(';') {
                    break;
                }
                if token.is_empty() {
                    break;
                }

                // Match each token while the peekable iterator yields some value and find its respective needed function.
                match token {
                    "HALT" => self.add_instruction(token),
                    "LDAC" => {
                        self.add_instruction_op(token, peek_split.peek().copied());
                        break;
                    }
                    "STAC" => {
                        self.add_instruction_op(token, peek_split.peek().copied());
                        break;
                    }
                    "MVAC" => self.add_instruction(token),
                    "MOVR" => self.add_instruction(token),
                    "JMP" => {
                        self.add_instruction_op(token, peek_split.peek().copied());
                        break;
                    }
                    "JMPZ" => {
                        self.add_instruction_op(token, peek_split.peek().copied());
                        break;
                    }
                    "OUT" => self.add_instruction(token),
                    "SUB" => self.add_instruction(token),
                    "ADD" => self.add_instruction(token),
                    "INC" => self.add_instruction(token),
                    "CLAC" => self.add_instruction(token),
                    "AND" => self.add_instruction(token),
                    "OR" => self.add_instruction(token),
                    "ASHR" => self.add_instruction(token),
                    "NOT" => self.add_instruction(token),
                    _ => {
                        self.add_symbol_table(token, peek_split.peek().copied());
                        break;
                    }
                }
            }
        }
        self.replace_instructions();
    }

    // Replaces the holder variables currently in the instructions with the ones in the symbol table.
    fn replace_instructions(&mut self) {
        for (key, pos_vec) in &self.holder_table {
            for val in pos_vec {
                self.instructions.insert(*val, self.symbol_table[key]);
            }
        }
    }

    // This just adds the instruction, since it does not possess an operand; nor is it a label or variable.
    fn add_instruction(&mut self, instr: &str) {
        let instruction = Instruction::from(instr);
        self.instructions
            .insert(self.instructions.len() as i32, instruction as i32);
    }

    // Add the instruction and do some bookkeeping for the holder variable we've inserted and keep track of the name of the variable.
    fn add_instruction_op(&mut self, instr: &str, op: Option<&str>) {
        let instruction = Instruction::from(instr);
        let op = op.unwrap();
        self.instructions
            .insert(self.instructions.len() as i32, instruction as i32);
        // The operand may not yet defined, so we just put a holder and store the holder in the HashMap.
        self.instructions.insert(self.instructions.len() as i32, 0);

        // A variable may occur multiple times, so positions are one or many so we use a vector and push onto it with the variable's name as a key.
        let holder = self
            .holder_table
            .entry(op.to_string())
            .or_insert_with(Vec::new);
        holder.push((self.instructions.len() - 1) as i32);
    }

    // This is for when we've parsed either a label or variable.
    fn add_symbol_table(&mut self, identifier: &str, potential_num: Option<&str>) {
        // If the potential num option yields a number, continue.
        if let Some(num) = potential_num {
            // Add the number to the instructions HashMap, record its position and place in symbol table.
            let num: i32 = i32::from_str_radix(num, 16).unwrap();
            self.instructions
                .insert(self.instructions.len() as i32, num);
            self.symbol_table.insert(
                identifier[..identifier.len() - 1].to_string(),
                (self.instructions.len() - 1) as i32,
            );
        } else {
            // If it just a label, we need to insert the current instructions length
            // It will just be used as a pointer to the instruction.
            self.symbol_table.insert(
                identifier[..identifier.len() - 1].to_string(),
                self.instructions.len() as i32,
            );
            self.label_table.insert(
                self.instructions.len() as i32,
                identifier[..identifier.len() - 1].to_string(),
            );
        }
    }

    // OpenAI generated
    fn decimal_to_hex_string(n: i32) -> String {
        // Convert the decimal number to a hexadecimal string and return it
        let hex = format!("{:x}", n);
        format!("{:0>8}", hex)
    }

    // A somewhat messy export function for writing a logisim compatible file.
    pub fn export(self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();

        file.write_all(b"v2.0 raw\n").unwrap();
        for key in self.instructions.keys().sorted() {
            let mut hex_string = Tokenizer::decimal_to_hex_string(self.instructions[key]);
            hex_string.push('\n');
            file.write_all(hex_string.as_bytes()).unwrap();
        }
    }
}
