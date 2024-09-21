use brainrusty::bfmachine::BfMachine;
use brainrusty::cliargs::Cli;

use clap::Parser;

use std::fs;

fn main() {
    let cli = Cli::parse();
    let program_bytes: Vec<u8> = fs::read_to_string(cli.bfpath)
        .expect("Couldn't read from bfpath")
        .as_bytes()
        .to_vec();

    let bfm = BfMachine::parse(program_bytes).unwrap_or_else(|e| {
        panic!("BfParseError: {e:?}");
    });

    println!("{:?}", bfm.program);
    println!("{:?}", bfm.loop_lookup);

    if cli.disassembly {
        println!("disassembly...");

        std::process::exit(0);
    }

    println!("normal...");
}
