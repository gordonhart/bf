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
    DebugDump,
    DebugBreakpoint,
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
        Command::DebugDump => '?',
        Command::DebugBreakpoint => '!',
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
        '?' => Ok(Command::DebugDump),
        '!' => Ok(Command::DebugBreakpoint),
        other => Err(format!("unsupported character: {}", other)),
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f, "{}", encode_command(*self))
    }
}

#[derive(Debug)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    Error(T),
}

#[derive(Debug)]
pub struct State {
    pub data: Vec<u8>,
    pub data_ptr: usize,
    pub program: Vec<Command>,
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
}

pub fn run_interpreter(program: &str) -> State {
    const HEAP_SIZE: usize = 100;
    let mut state = State {
        data: vec![0], // Vec::with_capacity(HEAP_SIZE),
        data_ptr: 0,
        program: vec![],
        program_ptr: 0,
        loop_stack: vec![],
        status: ExecutionStatus::NotStarted,
    };
    match parse_program(program) {
        Ok(program) => {
            state.program = program;
            run_program(&mut state);
        },
        Err(err) => state.status = ExecutionStatus::Error(err),
    };
    state
}

fn parse_program(program: &str) -> Result<Vec<Command>, String> {
    program
        .chars()
        .map(|c| decode_command(c))
        .collect()
}

fn run_program(state: &mut State) {
    // TODO: surely there is a better way to structure this main control flow
    state.status = ExecutionStatus::InProgress;
    loop {
        match state.program.get(state.program_ptr) {
            Some(Command::PtrInc) => pointer_increment(state),
            Some(Command::PtrDec) => pointer_decrement(state),
            Some(Command::ValInc) => value_increment(state),
            Some(Command::ValDec) => value_decrement(state),
            Some(Command::PutChar) => put_character(state),
            Some(Command::GetChar) => get_character(state),
            Some(Command::LoopBeg) => loop_enter(state),
            Some(Command::LoopEnd) => loop_exit(state),
            Some(Command::DebugDump) => println!("{:?}", state),
            Some(Command::DebugBreakpoint) => panic!(),
            None => break,
        };
        state.program_ptr += 1;
    };
    match state.status {
        ExecutionStatus::Error(_) => {},
        _ => state.status = ExecutionStatus::Terminated,
    }
}

fn pointer_increment(state: &mut State) {
    state.data_ptr += 1;
    match state.data.get(state.data_ptr) {
        Some(_) => {},
        None => state.data.push(0),
    }
}

fn pointer_decrement(state: &mut State) {
    match state.data_ptr {
        0 => state.data.insert(0, 0),
        _ => state.data_ptr -= 1,
    }
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

fn put_character(state: &mut State) {
    print!("{}", state.data[state.data_ptr] as char);
}

fn get_character(state: &mut State) {
    match std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
    {
        Some(c) => state.data[state.data_ptr] = c,
        None => state.status = ExecutionStatus::Terminated,
    }
}

fn find_loop_end(ptr: usize, program: &Vec<Command>) -> Result<usize, ()> {
    match program.get(ptr) {
        Some(Command::LoopEnd) => Ok(ptr),
        Some(Command::LoopBeg) => {
            find_loop_end(ptr + 1, program)
                .and_then(|i| find_loop_end(i + 1, program))
        },
        Some(_) => find_loop_end(ptr + 1, program),
        None => Err(())
    }
}

fn loop_enter(state: &mut State) {
    match state.data[state.data_ptr] {
        0 => match find_loop_end(state.program_ptr + 1, &state.program) {
            Ok(i) => state.program_ptr = i,
            Err(_) => {
                state.status = ExecutionStatus::Error(
                    "'[' missing corresponding ']'".to_string()
                )
            },
        }
        _ => state.loop_stack.push(state.program_ptr),
    }
}

fn loop_exit(state: &mut State) {
    match (state.loop_stack.pop(), state.data[state.data_ptr]) {
        (Some(ptr_loc), 0) => {},
        // account for the fact that the program pointer is going to be incremented
        (Some(ptr_loc), _) => state.program_ptr = ptr_loc - 1,
        (None, _) => {
            state.status = ExecutionStatus::Error(
                "']' missing corresponding '['".to_string()
            )
        },
    }
}
