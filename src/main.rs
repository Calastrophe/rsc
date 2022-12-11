#[allow(non_camel_case_types)]
pub mod emulator;
pub mod tokenizer;
use crate::emulator::Emulator;
use std::{env, fs};


// TODO: Refactor the command line arguments
fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg1) = args.get(1) {
        match arg1.as_str() {
            "assembler" => {
                if args.len() < 4 {
                    println!("usage: rustrsc assembler [input] [output]")
                } else {
                    assembler(args.get(2).unwrap().to_string(), args.get(3).unwrap().to_string())
                }
            }
            "run" => {
                if args.len() < 3 {
                    println!("usage: rustrsc run [input]")
                } else {
                    run(args.get(2).unwrap().to_string())
                }
            }
            _ => {
                println!("usage: rustrsc [run|assembler] [input] [output]")
            }
        }
    }
    else {
        println!("usage: rustrsc [run|assembler] [input] [output]")
    }
}

fn assembler(input: String, output: String) {
    let input = fs::read_to_string(input).expect("Failure to read the file.");
    let mut tokenizer_obj = tokenizer::Tokenizer::new();
    tokenizer_obj.parse(input.as_str());
    tokenizer_obj.export(output.as_str())
}

fn run(input: String) {
    let input = fs::read_to_string(input).expect("Failure to read the file.");
    let mut tokenizer_obj = tokenizer::Tokenizer::new();
    tokenizer_obj.parse(input.as_str());
    let mut main_emu = Emulator::new(tokenizer_obj.instructions);
    while !main_emu.halted() {
        main_emu.fetch();
        main_emu.execute();
    }
    main_emu.display_contents();
}


// TODO: Write a bunch of test cases to catch any errors that arouse from later changes.