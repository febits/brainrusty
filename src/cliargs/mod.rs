use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Brainfuck source file
    pub bfpath: String,

    /// Disassembly the brainfuck file
    #[arg(short, long)]
    pub disassembly: bool
}
