use crate::emulator::Instruction;
use crate::lexer::Token;
use std::collections::HashMap;
use std::io::Write;

pub struct Assembler<'a> {
    pub instructions: Vec<u32>,
    symbol_map: HashMap<&'a str, u32>,
    replaced_instructions: HashMap<&'a str, Vec<u32>>,
}

impl<'a> Assembler<'a> {
    pub fn assemble(tokens: Vec<Token<'a>>) -> Self {
        let mut token_iter = tokens.into_iter();
        let mut instructions = Vec::new();
        let mut symbol_map = HashMap::new();
        let mut to_replace: HashMap<&'a str, Vec<u32>> = HashMap::new();
        while let Some(token) = token_iter.next() {
            let current_address = instructions.len() as u32;
            match token {
                Token::Instruction(instruction) => {
                    instructions.push(instruction);
                }
                Token::Label(label) => {
                    symbol_map.insert(label, current_address);
                }
                Token::Variable(var) => {
                    symbol_map.insert(var, current_address);
                    let var_num = token_iter
                        .next()
                        .unwrap_or_else(|| panic!("failed to retrieve number for {var}"));
                    match var_num {
                        Token::Number(num) => {
                            instructions.push(num);
                        }
                        _ => {
                            panic!("expected number after {var} initilization")
                        }
                    }
                }
                Token::LabelRef(label) => {
                    if let Some(label_pos) = symbol_map.get(&label) {
                        instructions.push(*label_pos);
                    } else {
                        instructions.push(0);
                        to_replace.entry(label).or_default().push(current_address);
                    }
                }
                Token::VariableRef(var) => {
                    if let Some(var_pos) = symbol_map.get(&var) {
                        instructions.push(*var_pos);
                    } else {
                        instructions.push(0);
                        to_replace.entry(var).or_default().push(current_address);
                    }
                }
                _ => unreachable!(),
            }
        }

        // Replaces the placeholders if any, as LabelRef and VariableRef may have not all been put in yet.
        for (symbol, replace_addresses) in &to_replace {
            for replace_address in replace_addresses {
                if let Some(address) = symbol_map.get(symbol) {
                    if let Some(value) = instructions.get_mut(*replace_address as usize) {
                        *value = *address;
                    } else {
                        panic!("failed to find queued replacement {symbol} at {replace_address}")
                    }
                } else {
                    panic!("failed to retrieve {symbol}")
                }
            }
        }

        Assembler {
            instructions,
            symbol_map,
            replaced_instructions: to_replace,
        }
    }

    pub fn as_logisim(&self, o: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(o)?;
        writeln!(file, "v2.0 raw")?;
        for instruction in &self.instructions {
            writeln!(file, "{}", format!("{:08X}", instruction))?;
        }
        Ok(())
    }
}
