use super::util::types::{Error, Instruction};
use std::collections::HashMap;

// The assembler owns the input, but then parses it.
pub struct Assembler {
    instructions: Vec<u32>,
    symbol_map: HashMap<String, u32>,
    replaced: HashMap<u32, String>,
}

impl Assembler {
    pub fn parse(input: String) -> Result<Assembler, Error> {
        let mut instructions = Vec::new();
        let mut symbol_map = HashMap::new();
        let mut to_replace = HashMap::new();

        // Iterate over each line and provide a line number
        for (ln, line) in input.lines().enumerate() {
            // Skip over empty lines
            if line.is_empty() {
                continue;
            }

            // Split each line by whitespace
            let mut words = line.split_whitespace().peekable();

            // NOTE: This should never fail because it isn't an empty line.
            if let Some(word) = words.next() {
                // If its a comment, just continue.
                if word == ";" {
                    continue;
                }

                // If the first word is an instruction, parse the operand if needed.
                if let Ok(instruction) = TryInto::<Instruction>::try_into(word) {
                    instructions.push(instruction as u32);

                    // If it requires an operand, ensure one exists, add it to the map.
                    if instruction.has_operand() {
                        let operand = words
                            .next()
                            .ok_or_else(|| Error::ExpectedOperand(word.to_owned(), ln))?;
                        to_replace.insert(instructions.len() as u32, operand.to_owned());
                        instructions.push(0);
                    }
                } else {
                    // If not an instruction and it doesn't end with a ":", its an unknown keyword.
                    if !word.ends_with(':') {
                        return Err(Error::UnknownKeyword(word.to_owned(), ln));
                    }

                    let name = word.trim_end_matches(':');

                    match words.next() {
                        // No operand, its a label.
                        Some(";") | None => {
                            symbol_map.insert(name.to_owned(), instructions.len() as u32);
                        }
                        // It has a operand, its a variable.
                        Some(value) => {
                            let value = u32::from_str_radix(value, 16)
                                .map_err(|_| Error::InvalidOperand(name.to_owned(), ln))?;

                            symbol_map.insert(name.to_owned(), instructions.len() as u32);
                            instructions.push(value);
                        }
                    }
                }
            }
        }

        let mut replaced = HashMap::new();

        // Replace placeholders for variables with their actual location in the bytecode
        for (idx, var_name) in to_replace {
            instructions[idx as usize] = *symbol_map
                .get(&var_name)
                .ok_or_else(|| Error::UnknownVariable(var_name.to_string()))?;

            // Add the index to the replaced instructions map
            replaced.insert(idx, var_name);
        }

        Ok(Assembler {
            instructions,
            symbol_map,
            replaced,
        })
    }

    pub fn instructions(&self) -> &[u32] {
        &self.instructions
    }

    pub fn symbol_map(&self) -> &HashMap<String, u32> {
        &self.symbol_map
    }

    pub fn replaced(&self) -> &HashMap<u32, String> {
        &self.replaced
    }
}
