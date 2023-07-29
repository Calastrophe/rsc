use crate::emulator::Instruction;
use std::str::{FromStr, Lines};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Token<'a> {
    Number(u32),
    Instruction(u32),
    Variable(&'a str),
    VariableRef(&'a str),
    Label(&'a str),
    LabelRef(&'a str),
}

pub struct Lexer<'a> {
    lines: Lines<'a>,
    line_number: usize,
}

impl<'a> Lexer<'a> {
    pub fn tokenize(input: &'a str) -> Vec<Token> {
        let mut lexer = Lexer {
            lines: input.lines(),
            line_number: 0,
        };
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(new_tokens) = lexer.next_tokens() {
            tokens.extend(new_tokens)
        }
        tokens
    }

    fn next_line(&mut self) -> Option<&'a str> {
        let mut line = self.lines.next()?;
        self.line_number += 1;
        while line.is_empty() {
            line = self.lines.next()?;
            self.line_number += 1;
        }
        Some(line)
    }

    fn next_tokens(&mut self) -> Option<Vec<Token<'a>>> {
        let line = self.next_line()?;
        // We are doing this because we want to chop off the comment if there is one.
        let mut modified_line = line
            .split_once(";")
            .map(|(split_line, _c)| split_line)
            .unwrap_or(line)
            .split_whitespace();
        let first = modified_line.next()?;
        let mut tokens = Vec::with_capacity(2);
        if first.ends_with(":") {
            // It is either a label or variable
            if let Some(number) = modified_line.next() {
                // The next string should be able to be parsed as a hexadecimal number.
                let variable = &first[..first.len() - 1];
                let number = u32::from_str_radix(number, 16).unwrap_or_else(|_| {
                    panic!("expected hexadecimal number on line {}", self.line_number)
                });
                tokens.push(Token::Variable(variable));
                tokens.push(Token::Number(number));
            } else {
                let label = &first[..first.len() - 1];
                tokens.push(Token::Label(label));
            }
        } else {
            // Check to see if its an instruction, if not throw a parsing error.
            let instruction = Instruction::from_str(first)
                .unwrap_or_else(|e| panic!("{:?} on line {}", e, self.line_number));
            tokens.push(Token::Instruction(instruction as u32));
            // If its an instruction that has an operand, we need to handle it.
            match instruction {
                Instruction::LDAC | Instruction::STAC => {
                    let operand = modified_line.next()?;
                    tokens.push(Token::VariableRef(operand));
                }
                Instruction::JMP | Instruction::JMPZ => {
                    let label = modified_line.next()?;
                    tokens.push(Token::LabelRef(label));
                }
                _ => {}
            }
        }
        Some(tokens)
    }


}
