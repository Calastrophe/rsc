pub mod assembler;
pub mod debugger;
pub mod emulator;
pub mod lexer;
use assembler::Assembler;
use clap::ArgAction::Count;
use clap::{Parser, Subcommand};
use emulator::Emulator;
use lexer::Lexer;

#[derive(Parser)]
#[command(author="Calastrophe", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(short, long, action = Count, help = "For debugging purposes, but allows for deciding how verbose you want the emulation.")]
    verbose: u8,
}

#[derive(Subcommand)]
enum Command {
    /// Emulates a given input file to the end of computation.
    Run { input: String },
    /// Assembles a given input file into Logisim compatiable format named with the provided output name.
    Assemble { input: String, output: String },
    /// Debugs a given input file, allowing for breakpoints, stepping through code and introspection.
    Debug { input: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Command::Run { input } => {
                let input = std::fs::read_to_string(input)?;
                let mut lexer = Lexer::new(&input);
                let assembler = Assembler::assemble(lexer.tokenize());
                let mut emulator = Emulator::new(assembler.as_memory());
                emulator.start();
            }
            Command::Assemble { input, output } => {
                let input = std::fs::read_to_string(input)?;
                let mut lexer = Lexer::new(&input);
                let assembler = Assembler::assemble(lexer.tokenize());
                assembler.as_logisim(&output)?;
            }
            Command::Debug { input } => {
                todo!()
            }
        },
        None => {
            // GUI
            todo!()
        }
    }
    Ok(())
}
