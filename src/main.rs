pub mod emulator;
pub mod lexer;
use clap::{Parser, Subcommand};
use emulator::Emulator;
use lexer::Lexer;

#[derive(Parser)]
#[command(author="Calastrophe", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Emulates a given input file to the end of computation.
    Emulate { input: String },
    /// Assembles a given input file into Logisim compatiable format named with the provided output name.
    Assemble { input: String, output: String },
    /// Debugs a given input file, allowing for breakpoints, stepping through code and introspection.
    Debug { input: String },
}

// Create a parser with subcommands for emulating, assembling, and debugging.
// Add a verbose option.

fn main() {
    let cli = Cli::parse();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexer_test() -> Result<(), Box<dyn std::error::Error>> {
        let input = std::fs::read_to_string("tests/selection_sort.txt")?;
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        println!("{:?}", tokens);
        Ok(())
    }
}
