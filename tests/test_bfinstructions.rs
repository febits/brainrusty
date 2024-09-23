use brainrusty::bfmachine::*;

#[test]
fn move_pointer_to_right() {
    let program = ">>>".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    let _ = bfm.exec();

    assert_eq!(bfm.tp, 3);
}

#[test]
fn move_pointer_to_left() {
    let program = ">>><<<".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    let _ = bfm.exec();

    assert_eq!(bfm.tp, 0);
}

#[test]
fn increase_cell() {
    let program = "+++".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    let _ = bfm.exec();

    assert_eq!(bfm.tape[bfm.tp as usize], 3);
}

#[test]
fn decrease_cell() {
    let program = "+++---".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    let _ = bfm.exec();

    assert_eq!(bfm.tape[bfm.tp as usize], 0);
}

#[test]
fn loop_start_and_end() {
    let program = "++++++[-]".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    let _ = bfm.exec();

    assert_eq!(bfm.tape[bfm.tp as usize], 0);
}

#[test]
fn out_of_bounds_to_right() {
    let mut program = String::new();
    for _ in 0..TAPE_SIZE + 1 {
        program.push('>');
    }
    let mut bfm = BfMachine::parse(program.as_bytes().to_vec()).unwrap();

    assert_eq!(bfm.exec().unwrap_err(), BfError::OutOfBounds);
}

#[test]
fn out_of_bounds_to_left() {
    let program = "<".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();

    assert_eq!(bfm.exec().unwrap_err(), BfError::OutOfBounds);
}

#[test]
fn wrap_around_to_0() {
    let mut program = String::new();
    for _ in 0..256 {
        program.push('+');
    }

    let mut bfm = BfMachine::parse(program.as_bytes().to_vec()).unwrap();
    let _ = bfm.exec();

    assert_eq!(bfm.tape[bfm.tp as usize], 0);
}

#[test]
fn wrap_around_to_255() {
    let program = "-".as_bytes().to_vec();
    let mut bfm = BfMachine::parse(program).unwrap();
    let _ = bfm.exec();

    assert_eq!(bfm.tape[bfm.tp as usize], 255);
}
