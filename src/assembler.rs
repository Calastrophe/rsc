use crate::util::types::{Error, Instruction};
use std::collections::HashMap;

pub struct Assembler;

impl Assembler {
    pub fn parse(input: String) -> Result<HashMap<u32, u32>, Error> {
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

            // Either there is a word or just whitespace.
            let word = match words.next() {
                Some(word) => word,
                None => continue,
            };

            // If its a comment, just continue.
            if word == ";" {
                continue;
            }

            // If the first word is an instruction, parse the operand if required.
            if let Ok(instruction) = TryInto::<Instruction>::try_into(word) {
                instructions.push(instruction as u32);

                // If it requires an operand, ensure one exists, add it to the map.
                if instruction.has_operand() {
                    let operand = words
                        .next()
                        .ok_or_else(|| Error::ExpectedOperand(word.to_owned(), ln))?;

                    to_replace.insert(instructions.len() as u32, operand.to_owned());
                    // Placeholder for operand, which is a variable.
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

        for (address, variable) in to_replace {
            instructions[address as usize] = *symbol_map
                .get(&variable)
                .ok_or_else(|| Error::UnknownVariable(variable.to_string()))?;
        }

        Ok((0..instructions.len())
            .map(|k| (k as u32, instructions[k]))
            .collect())
    }
}
