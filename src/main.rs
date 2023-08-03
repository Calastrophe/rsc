pub mod emulator;
pub mod parser;
use clap::{Parser, Subcommand};
use emulator::{Emulator, Memory};
use parser::Assembler;

#[derive(Parser)]
#[command(author="Calastrophe", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Emulates a given input file to the end of computation.
    Run { input: String },
    /// Assembles a given input file into Logisim compatiable format named with the provided output name.
    Assemble { input: String, output: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { input } => {
            let input = std::fs::read_to_string(input)?;
            let assembler = Assembler::parse(&input);
            let memory = Memory::new(&assembler.instructions);
            let mut emulator = Emulator::new(memory);
            emulator.start();
        }
        Command::Assemble { input, output } => {
            let input = std::fs::read_to_string(input)?;
            let assembler = Assembler::parse(&input);
            assembler.as_logisim(&output)?;
        }
    }
    Ok(())
}
