use brainrusty::bfmachine::{BfMachine, Disassembly};
use brainrusty::cliargs::Cli;

use clap::Parser;

use std::fs;

fn main() {
    let cli = Cli::parse();
    let program_bytes: Vec<u8> = fs::read_to_string(cli.bfpath)
        .expect("Couldn't read from bfpath")
        .as_bytes()
        .to_vec();

    let mut bfm = match BfMachine::parse(program_bytes) {
        Ok(bfm) => bfm,
        Err(e) => {
            eprintln!("BfParseError: {e:?}");
            std::process::exit(1);
        }
    };

    if cli.disassembly {
        for disas_str in &bfm.disassembly() {
            println!("{disas_str}");
        }

        std::process::exit(0);
    }

    if let Err(res) = bfm.exec() {
        eprintln!("BfExecError: {res:?}");
        std::process::exit(1);
    }
}
