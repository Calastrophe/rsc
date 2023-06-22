use crate::{emulator::Memory, lexer::Token};
use std::collections::HashMap;
use std::io::Write;

pub struct Assembler<'a> {
    instructions: HashMap<u32, u32>,
    label_map: HashMap<&'a str, u32>,
    variable_map: HashMap<&'a str, u32>,
    replaced_instructions: HashMap<u32, &'a str>,
}

impl<'a> Assembler<'a> {
    pub fn assemble(tokens: Vec<Token<'a>>) -> Self {
        let mut token_iter = tokens.into_iter();
        let mut current_address: u32 = 0;
        let mut instructions = HashMap::new();
        let mut label_map = HashMap::new();
        let mut variable_map = HashMap::new();
        let mut label_replace = HashMap::new();
        let mut variable_replace = HashMap::new();
        let mut replaced_instructions = HashMap::new();
        while let Some(token) = token_iter.next() {
            match token {
                Token::Instruction(instruction) => {
                    instructions.insert(current_address, instruction);
                    current_address += 1;
                }
                Token::Label(label) => {
                    label_map.insert(label, current_address);
                }
                Token::Variable(var) => {
                    variable_map.insert(var, current_address);
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
                    if let Some(label) = label_map.get(&label) {
                        instructions.insert(current_address, *label);
                    } else {
                        label_replace.insert(current_address, label);
                    }
                    current_address += 1;
                }
                Token::VariableRef(var) => {
                    if let Some(var) = variable_map.get(&var) {
                        instructions.insert(current_address, *var);
                    } else {
                        variable_replace.insert(current_address, var);
                    }
                    current_address += 1;
                }
                _ => unreachable!(),
            }
        }
        for (address, label) in label_replace {
            if let Some(label_pos) = label_map.get(&label) {
                instructions.insert(address, *label_pos);
                replaced_instructions.insert(address, label);
            } else {
                panic!("failed to replace referenced label `{label}`");
            }
        }

        for (address, var) in variable_replace {
            if let Some(var_pos) = variable_map.get(&var) {
                instructions.insert(address, *var_pos);
                replaced_instructions.insert(address, var);
            } else {
                panic!("failed to replace referenced variable `{var}`");
            }
        }

        Assembler {
            instructions,
            label_map,
            variable_map,
            replaced_instructions,
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
