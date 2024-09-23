use std::collections::HashMap;
use std::io::{self, Write};

pub const TAPE_SIZE: usize = 30000;

#[derive(Debug, PartialEq)]
pub enum BfError {
    OutOfBounds,
    UnmatchedLoopStart,
    UnmatchedLoopEnd,
    InvalidProgram,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BfInstruction {
    NoInstruction,
    MovePointerToRight,
    MovePointerToLeft,
    IncreaseCell,
    DecreaseCell,
    LoopStart,
    LoopEnd,
    Getchar,
    Putchar,
}

#[derive(Default, Debug, PartialEq)]
pub struct BfMachine {
    pub pc: u16,
    pub tp: u16,
    pub tape: Vec<u8>,
    pub program: Vec<BfInstruction>,
    pub loop_lookup: HashMap<usize, usize>,
}

impl From<u8> for BfInstruction {
    fn from(val: u8) -> BfInstruction {
        match val {
            62 => BfInstruction::MovePointerToRight,
            60 => BfInstruction::MovePointerToLeft,
            43 => BfInstruction::IncreaseCell,
            45 => BfInstruction::DecreaseCell,
            91 => BfInstruction::LoopStart,
            93 => BfInstruction::LoopEnd,
            44 => BfInstruction::Getchar,
            46 => BfInstruction::Putchar,
            _ => BfInstruction::NoInstruction,
        }
    }
}

pub trait Disassembly {
    fn disassembly(&self) -> Vec<String>;
}

impl Disassembly for BfMachine {
    fn disassembly(&self) -> Vec<String> {
        let mut disas_list = Vec::new();

        for instr in &self.program {
            let disas = match instr {
                BfInstruction::MovePointerToRight => {
                    format!("MOVE_R\t\t[{instr:?}]")
                }
                BfInstruction::MovePointerToLeft => {
                    format!("MOVE_L\t\t[{instr:?}]")
                }
                BfInstruction::IncreaseCell => {
                    format!("INC\t\t[{instr:?}]")
                }
                BfInstruction::DecreaseCell => {
                    format!("DEC\t\t[{instr:?}]")
                }
                BfInstruction::LoopStart => {
                    format!("LOOP_S\t\t[{instr:?}]")
                }
                BfInstruction::LoopEnd => {
                    format!("LOOP_E\t\t[{instr:?}]")
                }
                BfInstruction::Getchar => {
                    format!("GETC\t\t[{instr:?}]")
                }
                BfInstruction::Putchar => {
                    format!("PUTC\t\t[{instr:?}]")
                }
                BfInstruction::NoInstruction => "".to_string(),
            };

            disas_list.push(disas);
        }

        disas_list
    }
}

impl BfMachine {
    pub fn parse(program_as_bytes: Vec<u8>) -> Result<Self, BfError> {
        let mut program = Vec::new();

        for b in program_as_bytes {
            let instruction: BfInstruction = b.into();
            if instruction != BfInstruction::NoInstruction {
                program.push(instruction);
            }
        }

        if program.is_empty() {
            return Err(BfError::InvalidProgram);
        }

        let mut stack = Vec::new();
        let mut loop_lookup = HashMap::new();

        for (index, instr) in program.iter().enumerate() {
            if *instr == BfInstruction::LoopStart {
                stack.push(index);
            } else if *instr == BfInstruction::LoopEnd {
                if !stack.is_empty() {
                    let start_index = stack.pop().unwrap();
                    let end_index = index;

                    loop_lookup.insert(start_index, end_index);
                    loop_lookup.insert(end_index, start_index);
                } else {
                    return Err(BfError::UnmatchedLoopEnd);
                }
            }
        }

        if !stack.is_empty() {
            return Err(BfError::UnmatchedLoopStart);
        }

        Ok(Self {
            pc: 0,
            tp: 0,
            tape: vec![0; TAPE_SIZE],
            loop_lookup,
            program,
        })
    }

    pub fn exec(&mut self) -> Result<(), BfError> {
        while self.program.len() > self.pc as usize {
            let instr = self.program[self.pc as usize];

            match instr {
                BfInstruction::MovePointerToRight => {
                    if self.tp + 1 > TAPE_SIZE as u16 {
                        return Err(BfError::OutOfBounds);
                    }

                    self.tp += 1;
                }
                BfInstruction::MovePointerToLeft => {
                    if self.tp == 0 {
                        return Err(BfError::OutOfBounds);
                    }

                    self.tp -= 1;
                }
                BfInstruction::IncreaseCell => {
                    let tv = self.tape[self.tp as usize];
                    self.tape[self.tp as usize] = tv.wrapping_add(1);
                }
                BfInstruction::DecreaseCell => {
                    let tv = self.tape[self.tp as usize];
                    self.tape[self.tp as usize] = tv.wrapping_sub(1);
                }
                BfInstruction::LoopStart => {
                    if self.tape[self.tp as usize] == 0 {
                        let new_pc = self.loop_lookup.get(&(self.pc as usize));

                        self.pc = *new_pc.unwrap() as u16;
                        continue;
                    }
                }
                BfInstruction::LoopEnd => {
                    if self.tape[self.tp as usize] != 0 {
                        let new_pc = self.loop_lookup.get(&(self.pc as usize));

                        self.pc = *new_pc.unwrap() as u16;
                        continue;
                    }
                }
                BfInstruction::Getchar => {
                    let mut buf = String::new();
                    io::stdin()
                        .read_line(&mut buf)
                        .expect("Couldn't read from stdin");

                    if !buf.is_empty() {
                        let c = buf.as_bytes().first().unwrap();
                        self.tape[self.tp as usize] = *c;
                    }
                }
                BfInstruction::Putchar => {
                    let _ = io::stdout()
                        .write(&[self.tape[self.tp as usize]])
                        .expect("Couldn't print to stdout");
                }

                _ => (),
            }

            self.pc += 1;
        }

        Ok(())
    }
}
