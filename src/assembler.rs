use crate::{emulator::Memory, lexer::Token};
use std::collections::HashMap;
use std::io::Write;

pub struct Assembler<'a> {
    instructions: HashMap<u32, u32>,
    symbol_map: HashMap<Token<'a>, u32>,
    replaced_instructions: HashMap<Token<'a>, u32>,
}

impl<'a> Assembler<'a> {
    pub fn assemble(tokens: Vec<Token<'a>>) -> Self {
        let mut token_iter = tokens.into_iter();
        let mut current_address: u32 = 0;
        let mut instructions = HashMap::new();
        let mut symbol_map = HashMap::new();
        let mut to_replace = HashMap::new();
        while let Some(token) = token_iter.next() {
            match token {
                Token::Instruction(instruction) => {
                    instructions.insert(current_address, instruction);
                    current_address += 1;
                }
                Token::Label(label) => {
                    symbol_map.insert(token, current_address);
                }
                Token::Variable(var) => {
                    symbol_map.insert(token, current_address);
                    let var_num = token_iter
                        .next()
                        .unwrap_or_else(|| panic!("failed to retrieve number for {var}"));
                    match var_num {
                        Token::Number(num) => {
                            instructions.insert(current_address, num);
                            current_address += 1;
                        }
                        _ => {
                            panic!("expected number after {var} initilization")
                        }
                    }
                    current_address += 1;
                }
                Token::LabelRef(label) => {
                    if let Some(label) = symbol_map.get(&token) {
                        instructions.insert(current_address, *label);
                    } else {
                        to_replace.insert(token, current_address);
                    }
                    current_address += 1;
                }
                Token::VariableRef(var) => {
                    if let Some(var) = symbol_map.get(&token) {
                        instructions.insert(current_address, *var);
                    } else {
                        to_replace.insert(token, current_address);
                    }
                    current_address += 1;
                }
                _ => unreachable!(),
            }
        }
        for (token, replace_addr) in symbol_map {
            if let Some(address) = symbol_map.get(&token) {
                instructions.insert(replace_addr, *address);
            } else {
                panic!("failed to replace {:?}", token)
            }
        }

        Assembler {
            instructions,
            symbol_map,
            replaced_instructions: to_replace,
        }
    }

    pub fn as_memory(&self) -> Memory {
        Memory(self.instructions.clone())
    }

    pub fn as_logisim(&self, o: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(o)?;
        writeln!(file, "v2.0 raw")?;
        for (_, instruction) in &self.instructions {
            writeln!(file, "{}", format!("{:08X}", *instruction))?;
        }
        Ok(())
    }
}
