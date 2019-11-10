use std::fmt;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    PtrInc,
    PtrDec,
    ValInc,
    ValDec,
    PutChar,
    GetChar,
    LoopBeg,
    LoopEnd,
}

#[derive(Debug)]
struct State {
    // data_cells: Vec<u8>,
    data_cells: [u8; 32],
    data_ptr: usize, // u64,  # usize length (32, 64) is platform-dependent
    loop_stack: Vec<usize>,
    program_ptr: usize,
}

// pub fn decode_command(command: Command) -> &'static str {
pub fn decode_command(command: Command) -> char {
    match command {
        Command::PtrInc => '>',
        Command::PtrDec => '<',
        Command::ValInc => '+',
        Command::ValDec => '-',
        Command::PutChar => '.',
        Command::GetChar => ',',
        Command::LoopBeg => '[',
        Command::LoopEnd => ']',
    }
}

pub fn encode_command(c: char) -> Result<Command, String> {
    match c {
        '>' => Ok(Command::PtrInc),
        '<' => Ok(Command::PtrDec),
        '+' => Ok(Command::ValInc),
        '-' => Ok(Command::ValDec),
        '.' => Ok(Command::PutChar),
        ',' => Ok(Command::GetChar),
        '[' => Ok(Command::LoopBeg),
        ']' => Ok(Command::LoopEnd),
        other => Err(format!("unsupported character: {}", other)),
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f, "{}", decode_command(*self))
    }
}

pub fn run_program(program: &str) {
    let mut state = State {
        // data_cells: vec![0],
        data_cells: [0; 32],
        data_ptr: 16,
        loop_stack: vec![],
        program_ptr: 0,
    };
    while state.program_ptr < program.len() {
        let c = program.chars().nth(state.program_ptr).unwrap();
        println!("executing: {:?}\tstate: {:?}", c, state);
        match encode_command(c) {
            // Ok(command) => run_command(&mut state, command),
            Ok(Command::PtrInc) => pointer_increment(&mut state),
            Ok(Command::PtrDec) => pointer_decrement(&mut state),
            Ok(Command::ValInc) => value_increment(&mut state),
            Ok(Command::ValDec) => value_decrement(&mut state),
            Ok(Command::PutChar) => put_character(&mut state),
            Ok(Command::GetChar) => get_character(&mut state),
            Ok(Command::LoopBeg) => loop_enter(&mut state, program),
            Ok(Command::LoopEnd) => loop_exit(&mut state),
            Err(message) => panic!(message),
        };
    }
}

/*
fn run_command(state: &mut State, cmd: Command) {
    match cmd {
        Command::PtrInc => state.ptr += 1,
        Command::PtrDec => state.ptr -= 1,
        Command::ValInc =>
        _ => panic!("unsupported command: {}", cmd),
    }
}
*/

fn pointer_increment(state: &mut State) {
    state.data_ptr += 1;
    state.program_ptr += 1;
    /*
    if state.data_cells.len() == state.data_ptr {
        state.data_cells.push(0);
    }
    */
}

fn pointer_decrement(state: &mut State) {
    match state.data_ptr {
        0 => panic!("bf error: segfault"),
        _ => state.data_ptr -= 1,
    }
    state.program_ptr += 1;
}

fn value_increment(state: &mut State) {
    match state.data_cells[state.data_ptr].overflowing_add(1) {
        (v, _) => state.data_cells[state.data_ptr] = v,
    }
    state.program_ptr += 1;
}

fn value_decrement(state: &mut State) {
    match state.data_cells[state.data_ptr].overflowing_sub(1) {
        (v, _) => state.data_cells[state.data_ptr] = v,
    }
    state.program_ptr += 1;
}

fn put_character(state: &mut State) {
    print!("{}", state.data_cells[state.data_ptr]);
    state.program_ptr += 1;
}

fn get_character(state: &mut State) {
    // TODO: inspect this copypasta
    let input: Option<u8> = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8);
    match input {
        Some(c) => state.data_cells[state.data_ptr] = c,
        None => panic!("bf error: failed to read"),
    }
    state.program_ptr += 1;
}

fn find_loop_end(ptr: usize, program: &str) -> usize {
    match program.chars().nth(ptr) {
    }
    match decode_command(program.chars().nth(ptr).unwrap()) {
        Ok(Command::LoopEnd) -> ptr,
        Ok(Command::LoopBegin) -> find_loop_end(
        Ok(_) -> find_loop_end(ptr + 1)
        Err(msg) => panic!(message),
    }
}

fn loop_enter(state: &mut State, program: &str) {
    state.loop_stack.push(state.program_ptr);
    match state.data_cells[state.data_ptr] {
        0 => {
            // TODO: fp-ify
            // scan for corresponding loop_exit command
            let mut n: u64 = 0;
            let mut scan_ptr: usize = state.program_ptr + 1;
            println!("{:?}", state);
            loop {
                match (n, encode_command(program.chars().nth(scan_ptr).unwrap())) {
                    (_, Err(message)) => panic!(message),
                    (0, Ok(Command::LoopEnd)) => {
                        state.loop_stack.pop();
                        state.program_ptr = scan_ptr + 1;
                        break;
                    },
                    (_, Ok(Command::LoopBeg)) => n += 1,
                    (_, Ok(Command::LoopEnd)) => n -= 1,
                    (_, _) => {},
                }
                scan_ptr += 1;
            }
        }
        _ => {}
    }
}

fn loop_exit(state: &mut State) {
    match state.loop_stack.pop() {
        Some(ptr_loc) => state.program_ptr = ptr_loc,
        None => panic!("bf error: ']' missing corresponding '['"),
    }
}
