use std::fmt;

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
    cells: Vec<u8>,
    ptr: u64,
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
        cells: Vec::new(),
        ptr: 0,
    };
    for c in program.chars() {
        match encode_command(c) {
            Ok(command) => run_command(&mut state, command),
            Err(message) => panic!(message),
        };
        println!("state: {:?}", state);
    }
}

fn run_command(state: &mut State, cmd: Command) {
    match cmd {
        Command::PtrInc => state.ptr += 1,
        Command::PtrDec => state.ptr -= 1,
        Command::ValInc =>
        _ => panic!("unsupported command: {}", cmd),
    }
}
