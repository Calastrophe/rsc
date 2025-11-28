use super::util::{Error, Instruction};
use std::collections::HashMap;

/// The assembler parses the assembly file and builds up relevant structures for our emulator and debugger.
///
/// The internal line map is used for our bytecode highlighter to match a given line number to a range of instructions.
/// Additionally, the assembler will look ahead for other errors to report for the editor to display.
pub struct Assembler {
    pub instructions: Vec<u32>,
    pub line_map: HashMap<usize, (usize, usize)>,
    pub symbol_map: HashMap<String, u32>,
    pub symbol_references: HashMap<u32, String>,
    pub errors: Option<Vec<Error>>,
}

impl Assembler {
    /// Parses a given file and produces bytecode for the emulator along with information for the debugger.
    pub fn parse(input: String) -> Assembler {
        let mut instructions = Vec::new();
        let mut line_map = HashMap::new();
        let mut symbol_map = HashMap::new();
        let mut to_replace = HashMap::new();
        let mut errors = Vec::new();

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
                    let current_idx = instructions.len();
                    line_map.insert(ln, (current_idx, current_idx));

                    instructions.push(instruction as u32);

                    // If it requires an operand, ensure one exists, add it to the map.
                    if instruction.has_operand() {
                        let Some(operand) = words.next() else {
                            errors.push(Error::MissingOperand(word.to_owned(), ln));
                            continue;
                        };

                        // Extend the bytecode highlight for the operand.
                        line_map.entry(ln).and_modify(|(_, end)| {
                            *end += 1;
                        });

                        // Add the current position in the bytecode to a map with the variable name.
                        to_replace.insert(instructions.len() as u32, (ln, operand.to_owned()));

                        // Insert a placeholder that is to be replaced.
                        instructions.push(0);
                    }
                } else {
                    // If not an instruction and it doesn't end with a ":", its an unknown keyword.
                    if !word.ends_with(':') {
                        errors.push(Error::UnknownKeyword(word.to_owned(), ln));
                        continue;
                    }

                    let name = word.trim_end_matches(':');

                    if symbol_map.contains_key(name) {
                        errors.push(Error::Redefinition(name.to_owned(), ln));
                        continue;
                    }

                    match words.next() {
                        // No operand, its a label.
                        Some(";") | None => {
                            symbol_map.insert(name.to_owned(), instructions.len() as u32);
                        }
                        // It has a operand, it is a variable declaration.
                        Some(value) => {
                            let Ok(value) = u32::from_str_radix(value, 16) else {
                                errors.push(Error::InvalidInitializer(name.to_owned(), ln));
                                continue;
                            };

                            let current_idx = instructions.len();
                            line_map.insert(ln, (current_idx, current_idx));

                            symbol_map.insert(name.to_owned(), current_idx as u32);
                            instructions.push(value);
                        }
                    }
                }
            }
        }

        // Replace the placeholders in our bytecode with the address of their variable from the symbol table.
        let symbol_references: HashMap<u32, String> = to_replace
            .into_iter()
            .filter_map(|(idx, (ln, var_name))| {
                // Identify if the variable name exists in our symbol map, error if not.
                match symbol_map.get(&var_name) {
                    Some(symbol) => {
                        instructions[idx as usize] = *symbol;
                        Some((idx, var_name))
                    }
                    None => {
                        errors.push(Error::UndefinedVariable(var_name.to_string(), ln));
                        None
                    }
                }
            })
            .collect();

        let errors = (!errors.is_empty()).then(|| errors);

        Assembler {
            instructions,
            line_map,
            symbol_map,
            symbol_references,
            errors,
        }
    }
}
