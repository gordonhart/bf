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
    eprintln!("{:?}", state);
    // TODO: surely there is a better way to structure this main control flow
    match state.program.get(state.program_ptr) {
        Some(Command::PtrInc) => {
            state.data_ptr += 1;
            state.program_ptr += 1;
            run_program(state)
        },
        Some(Command::PtrDec) => {
            state.data_ptr -= 1;
            state.program_ptr += 1;
            run_program(state)
        },
        Some(Command::ValInc) => {
            state.data[state.data_ptr] += 1;
            state.program_ptr += 1;
            run_program(state)
        },
        Some(Command::ValDec) => {
            state.data[state.data_ptr] -= 1;
            state.program_ptr += 1;
            run_program(state)
        },
        Some(Command::PutChar) => {
            print!("{}", state.data[state.data_ptr]);
            state.program_ptr += 1;
            run_program(state)
        },
        Some(Command::GetChar) => {
            get_character(state);
            run_program(state)
        },
        Some(Command::LoopBeg) => {
            loop_enter(state);
            run_program(state)
        },
        Some(Command::LoopEnd) => {
            loop_exit(state);
            run_program(state)
        },
        None => Ok("success".to_string()),
    }
}

/*
fn run_command(state: &mut State, cmd: Command) {
    match cmd {
        Command::PtrInc => stateptr += 1,
        Command::PtrDec => state.ptr -= 1,
        Command::ValInc =>
        _ => panic!("unsupported command: {}", cmd),
    }
}
*/

/*
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
*/

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
    state.program_ptr += 1;
}

/*
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
*/

/*
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
*/

fn loop_enter(state: &mut State) {}

fn loop_exit(state: &mut State) {
    match state.loop_stack.pop() {
        Some(ptr_loc) => state.program_ptr = ptr_loc,
        None => panic!("bf error: ']' missing corresponding '['"),
    }
}
