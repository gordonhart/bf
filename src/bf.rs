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

pub fn encode_command(command: Command) -> char {
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

pub fn decode_command(c: char) -> Result<Command, String> {
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
        write!(f, "{}", encode_command(*self))
    }
}

#[derive(Debug)]
struct State {
    data: Vec<u8>,
    data_ptr: usize,
    program: Vec<Command>,
    program_ptr: usize,
    loop_stack: Vec<usize>,
}

pub fn run_interpreter(program: &str) -> Result<String, String> {
    const HEAP_SIZE: usize = 100;
    let mut state = State {
        data: Vec::with_capacity(HEAP_SIZE),
        data_ptr: HEAP_SIZE / 2,
        program: vec![],
        program_ptr: 0,
        loop_stack: vec![],
    };
    state.data.resize(HEAP_SIZE, 0); // probably not the right way to initialize
    parse_program(program).and_then(|prog| {
        state.program = prog;
        run_program(&mut state)
    })
}

fn parse_program(program: &str) -> Result<Vec<Command>, String> {
    program
        .chars()
        .map(|c| decode_command(c))
        .collect()
}

fn run_program(state: &mut State) -> Result<String, String> {
    // TODO: surely there is a better way to structure this main control flow
    loop {
        eprintln!("{:?}", state);
        match state.program.get(state.program_ptr) {
            Some(Command::PtrInc) => pointer_increment(state),
            Some(Command::PtrDec) => pointer_decrement(state),
            Some(Command::ValInc) => value_increment(state),
            Some(Command::ValDec) => value_decrement(state),
            Some(Command::PutChar) => print!("{}", state.data[state.data_ptr]),
            Some(Command::GetChar) => get_character(state),
            Some(Command::LoopBeg) => loop_enter(state),
            Some(Command::LoopEnd) => loop_exit(state),
            None => break,
        };
        state.program_ptr += 1;
    };
    Ok("success".to_string())
}

fn pointer_increment(state: &mut State) {
    state.data_ptr += 1;
    /* TODO: implement bounds checks
    match state.data_ptr {
        0 => panic!("bf error: segfault (below beginning)"),
        i if i > state.data.len() => panic!("bf error: segfault (past end)"),
        _ => match amount {
            a if a < 0 => state.data_ptr -= usize::from(-a),
            a => state.data_ptr += usize::from(a),
        }
    }
    */
}

fn pointer_decrement(state: &mut State) {
    state.data_ptr -= 1;
}

fn value_increment(state: &mut State) {
    match state.data[state.data_ptr].overflowing_add(1) {
        (v, _) => state.data[state.data_ptr] = v,
    }
}

fn value_decrement(state: &mut State) {
    match state.data[state.data_ptr].overflowing_sub(1) {
        (v, _) => state.data[state.data_ptr] = v,
    }
}

fn get_character(state: &mut State) {
    // TODO: inspect this copypasta
    let input: Option<u8> = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8);
    match input {
        Some(c) => state.data[state.data_ptr] = c,
        None => panic!("bf error: failed to read"),
    }
}

fn find_loop_end(ptr: usize, program: &Vec<Command>) -> usize {
    match program.get(ptr) {
        Some(Command::LoopEnd) => ptr,
        Some(Command::LoopBeg) => find_loop_end(find_loop_end(ptr + 1, program), program),
        Some(_) => find_loop_end(ptr + 1, program),
        None => panic!(""),
    }
}

fn loop_enter(state: &mut State) {
    match state.data[state.data_ptr] {
        0 => state.program_ptr = find_loop_end(state.program_ptr + 1, &state.program),
        _ => state.loop_stack.push(state.program_ptr),
    }
}

fn loop_exit(state: &mut State) {
    match state.loop_stack.pop() {
        // account for the fact that the program pointer is going to be incremented
        Some(ptr_loc) => state.program_ptr = ptr_loc - 1,
        None => panic!("bf error: ']' missing corresponding '['"),
    }
}
