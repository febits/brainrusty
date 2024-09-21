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
        eprintln!("BfParseError: {e:?}");
        std::process::exit(1);
    });

    if cli.disassembly {
        for disas_str in &bfm.disassembly() {
           println!("{disas_str}");
        }

        std::process::exit(0);
    }

    println!("normal...");
}
