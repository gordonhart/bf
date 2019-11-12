use std::fmt;
use std::io::{Write, Read};

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
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
}

pub fn run_interpreter(program: &str) -> State {
    let mut state = State {
        data: vec![0], // Vec::with_capacity(HEAP_SIZE),
        data_ptr: 0,
        program_ptr: 0,
        loop_stack: vec![],
        status: ExecutionStatus::NotStarted,
    };
    match parse_program(program) {
        Ok(parsed_program) => run_program(&mut state, &parsed_program),
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

fn run_program(state: &mut State, program: &Vec<Command>) {
    // TODO: surely there is a better way to structure this main control flow
    state.status = ExecutionStatus::InProgress;
    loop {
        match state.status {
            ExecutionStatus::Terminated | ExecutionStatus::Error(_) => break,
            _ => {},
        };
        match program.get(state.program_ptr) {
            Some(command) => run_command(state, &command, program),
            None => break,
        };
        state.program_ptr += 1;
    };
    match state.status {
        ExecutionStatus::Error(_) => {},
        _ => state.status = ExecutionStatus::Terminated,
    }
}

fn run_command(state: &mut State, command: &Command, program: &Vec<Command>) {
    match command {
        Command::PtrInc => pointer_increment(state),
        Command::PtrDec => pointer_decrement(state),
        Command::ValInc => value_increment(state),
        Command::ValDec => value_decrement(state),
        Command::PutChar => put_character(state),
        Command::GetChar => get_character(state),
        Command::LoopBeg => loop_enter(state, program),
        Command::LoopEnd => loop_exit(state),
        Command::DebugDump => eprintln!("{:?}", state),
        Command::DebugBreakpoint => debug_repl(state),
    };
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
    match std::io::stdout().flush() { _ => {} };
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

fn loop_enter(state: &mut State, program: &Vec<Command>) {
    match state.data[state.data_ptr] {
        0 => match find_loop_end(state.program_ptr + 1, program) {
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
        (Some(_), 0) => {},
        // account for the fact that the program pointer is going to be incremented
        (Some(ptr_loc), _) => state.program_ptr = ptr_loc - 1,
        (None, _) => {
            state.status = ExecutionStatus::Error(
                "']' missing corresponding '['".to_string()
            )
        },
    }
}

fn debug_repl(state: &mut State) {
    // don't quite have this working yet
    // TODO: de-uglify -- this needs to be thought through
    let prompt: &'static str = "$ ";
    println!(
"You have entered an interactive session. All regular commands are available.
Enter 'q' to exit the session and resume program execution.");
    let mut buffer = String::new();
    'repl: loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        buffer.retain(|_| false); // empty buffer
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let mut new_program: Vec<Command> = Vec::new();
                let prev_program_ptr = state.program_ptr;
                state.program_ptr = 0;
                for c in buffer.trim().chars() {
                    if c == 'q' {
                        for command in &new_program {
                            run_command(state, &command, &new_program);
                        }
                        state.program_ptr = prev_program_ptr;
                        break 'repl;
                    } else {
                        match decode_command(c) {
                            Ok(command) => new_program.push(command),
                            Err(err) => eprintln!("{}", err),
                        };
                    };
                };
                for command in &new_program {
                    run_command(state, &command, &new_program);
                }
                state.program_ptr = prev_program_ptr;
            },
            Err(_) => println!("invalid input"),
        }
    }
}
