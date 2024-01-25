use rsc::{assembler::Assembler, util::types::Error, Emulator};

fn main() -> Result<(), Error> {
    let path = std::env::args().nth(1).expect("No input file");
    let output = std::env::args()
        .nth(2)
        .expect("Missing name for trace file");

    let input = std::fs::read_to_string(path)?;
    let assembler = Assembler::parse(input)?;
    let mut emulator = Emulator::new(&output, assembler.instructions());
    emulator.start();
    emulator.terminate();

    Ok(())
}
