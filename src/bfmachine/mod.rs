use std::collections::HashMap;

const TAPE_SIZE: usize = 30000;

#[derive(Debug)]
pub enum BfError {
    OutOfBounds,
    UnmatchedLoopStart,
    UnmatchedLoopEnd,
    InvalidProgram,
}

#[derive(PartialEq, Debug)]
pub enum BfInstruction {
    NoInstruction = 255,
    IncreasePointerToRight = 62,
    IncreasePointerToLeft = 60,
    IncreaseCell = 43,
    DecreaseCell = 45,
    LoopStart = 91,
    LoopEnd = 93,
    Getchar = 44,
    Putchar = 46,
}

pub struct BfMachine {
    /// Program Counter
    pub pc: u16,

    /// Tape pointer
    pub tp: u16,

    /// The memory tape
    pub tape: Vec<u8>,

    /// The program itself
    pub program: Vec<BfInstruction>,

    /// The lookup loop
    pub loop_lookup: HashMap<usize, usize>,
}

impl Into<BfInstruction> for u8 {
    fn into(self) -> BfInstruction {
        match self {
            62 => BfInstruction::IncreasePointerToRight,
            60 => BfInstruction::IncreasePointerToLeft,
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

impl BfMachine {
    pub fn parse(program_as_bytes: Vec<u8>) -> Result<Self, BfError> {
        let mut program = Vec::new();

        for b in program_as_bytes {
            let instruction: BfInstruction = b.into();
            if instruction != BfInstruction::NoInstruction {
                program.push(instruction);
            }
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
}
