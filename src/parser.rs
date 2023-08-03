use crate::emulator::Instruction;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, map_res, recognize, value},
    multi::many0_count,
    multi::separated_list0,
    sequence::{pair, terminated, tuple},
    IResult,
};
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Keyword(Instruction),
    KeywordOperand(Instruction, &'a str),
    Label(&'a str),
    Variable(&'a str, u32),
    LabelRef(&'a str),
    VariableRef(&'a str),
    Comment,
}

pub struct Assembler<'a> {
    pub instructions: Vec<u32>,
    pub symbol_map: HashMap<&'a str, u32>,
    pub replaced_instructions: HashMap<u32, &'a str>,
}

impl<'a> Assembler<'a> {
    pub fn parse(input: &'a str) -> Self {
        // TODO: Fix this error handling...
        let (_, tokens) = parse(input).expect("There was an issue parsing your file.");

        let mut symbol_map = HashMap::new();
        let mut to_replace = HashMap::new();
        let mut instructions = Vec::new();
        for token in tokens {
            // Could potentially mess up parsing if there are more than u32::MAX instructions.
            let current_address = instructions.len() as u32;
            match token {
                Token::Keyword(i) => {
                    instructions.push(i as u32);
                }
                Token::KeywordOperand(i, op) => {
                    instructions.extend([i as u32, 0]);
                    to_replace.insert(current_address - 1, op);
                }
                Token::Label(label) => {
                    symbol_map.insert(label, current_address);
                }
                Token::LabelRef(name) | Token::VariableRef(name) => {
                    to_replace.insert(current_address, name);
                    instructions.push(0);
                }
                Token::Variable(var, value) => {
                    symbol_map.insert(var, current_address);
                    instructions.push(value);
                }
                _ => {}
            }
        }

        for (addr, name) in &to_replace {
            if let Some(value) = symbol_map.get(name) {
                instructions[*addr as usize] = *value;
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

fn parse<'a>(input: &'a str) -> IResult<&str, Vec<Token>> {
    separated_list0(
        multispace0,
        alt((parse_ident, parse_instruction, parse_comment)),
    )(input)
}

// Parses out an instruction
fn parse_instruction(input: &str) -> IResult<&str, Token> {
    use Instruction::*;
    use Token::Keyword;

    let halt = value(Keyword(HALT), tag("HALT"));
    let ldac = map(
        tuple((tag("LDAC"), multispace0, identifier)),
        |(_, _, ident)| Token::KeywordOperand(Instruction::LDAC, ident),
    );
    let stac = map(
        tuple((tag("STAC"), multispace0, identifier)),
        |(_, _, ident)| Token::KeywordOperand(Instruction::STAC, ident),
    );
    let mvac = value(Keyword(MVAC), tag("MVAC"));
    let movr = value(Keyword(MOVR), tag("MOVR"));
    let jmp = map(
        tuple((tag("JMP"), multispace0, identifier)),
        |(_, _, ident)| Token::KeywordOperand(Instruction::JMP, ident),
    );
    let jmpz = map(
        tuple((tag("JMPZ"), multispace0, identifier)),
        |(_, _, ident)| Token::KeywordOperand(Instruction::JMPZ, ident),
    );
    let out = value(Keyword(OUT), tag("OUT"));
    let sub = value(Keyword(SUB), tag("SUB"));
    let add = value(Keyword(ADD), tag("ADD"));
    let inc = value(Keyword(INC), tag("INC"));
    let clac = value(Keyword(CLAC), tag("CLAC"));
    let and = value(Keyword(AND), tag("AND"));
    let or = value(Keyword(OR), tag("OR"));
    let ashr = value(Keyword(ASHR), tag("ASHR"));
    let not = value(Keyword(NOT), tag("NOT"));

    alt((
        halt, ldac, stac, mvac, movr, jmpz, jmp, out, sub, add, inc, clac, and, or, ashr, not,
    ))(input)
}

// Parsing the name of an identifer
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

// Parsing a hexadecimal number after the identifier
fn parse_hex_number(input: &str) -> IResult<&str, u32> {
    map_res(alphanumeric1, |s: &str| u32::from_str_radix(s, 16))(input)
}

// Combined parser of parsing an identifer, produces Label or Variable
fn parse_ident(input: &str) -> IResult<&str, Token> {
    alt((
        map(
            tuple((identifier, tag(":"), multispace0, parse_hex_number)),
            |(ident, _, _, num)| Token::Variable(ident, num),
        ),
        map(terminated(identifier, tag(":")), Token::Label),
    ))(input)
}

fn parse_comment(input: &str) -> IResult<&str, Token> {
    value(Token::Comment, pair(tag(";"), take_until("\n")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = include_str!("../tests/selection_sort.txt");

        let (res, tokens) = parse(file)?;
        println!("{:?}, {:?}", res, tokens);
        Ok(())
    }
}
