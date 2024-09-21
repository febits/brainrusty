use brainrusty::bfmachine::*;
use std::collections::HashMap;

#[test]
fn parsing_valid_program() {
    let program = "+++---".as_bytes().to_vec();
    let bfm = BfMachine::parse(program).unwrap_or_default();

    assert_ne!(bfm, BfMachine::default());
}

#[test]
fn parsing_invalid_program() {
    let program = "thingsthings yes".as_bytes().to_vec();
    let bfm = BfMachine::parse(program);

    assert_eq!(bfm.unwrap_err(), BfError::InvalidProgram);
}

#[test]
fn parsing_loops() {
    let program = "[[]]".as_bytes().to_vec();
    let bfm = BfMachine::parse(program).unwrap();

    assert_eq!(
        bfm.loop_lookup,
        HashMap::from([(0, 3), (3, 0), (1, 2), (2, 1)])
    );
}

#[test]
fn parsing_loops_unmatched() {
    let mut program = "[]]".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program);

    assert_eq!(bfm.unwrap_err(), BfError::UnmatchedLoopEnd);

    program = "[[]".as_bytes().to_vec();
    bfm = BfMachine::parse(program);

    assert_eq!(bfm.unwrap_err(), BfError::UnmatchedLoopStart);
}
