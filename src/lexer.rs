use crate::emulator::Instruction;
use std::str::{FromStr, Lines};

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Number(u32),
    Instruction(Instruction),
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
    pub fn new(input: &'a str) -> Self {
        Lexer {
            lines: input.lines(),
            line_number: 0,
        }
    }

    fn next_line(&mut self) -> Option<&str> {
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
            tokens.push(Token::Instruction(instruction));
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

    // TODO: Figure out why a lifetime is really needed here, why can't the compiler elide it?
    pub fn tokenize(&'a mut self) -> Vec<Token> {
        let mut tokens: Vec<Token<'a>> = Vec::new();
        while let Some(new_tokens) = self.next_tokens() {
            tokens.extend(new_tokens)
        }
        tokens
    }
}
