use std::io::Read;

use crate::buffer::Buffer;
use crate::repl;
use crate::token::Token;

// TODO: figure out why these derive macros fail with E0495:
// cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
// #[derive(Debug, PartialEq)]
pub struct State<'a> {
    pub data: Vec<u8>,
    pub data_ptr: usize,
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
    pub buffer: &'a mut dyn Buffer,
}

impl<'a> std::fmt::Debug for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State:\n\
            \tdata: {:?}\n\
            \tdata_ptr: {:?}\n\
            \tprogram_ptr: {:?}\n\
            \tloop_stack: {:?}\n\
            \tstatus: {:?}",
            self.data, self.data_ptr, self.program_ptr, self.loop_stack, self.status)
    }
}

impl<'a> State<'a> {
    pub fn new<'b>(buffer: &'b mut dyn Buffer) -> State<'b> {
        State {
            data: vec![0],
            data_ptr: 0,
            program_ptr: 0,
            loop_stack: vec![],
            status: ExecutionStatus::NotStarted,
            buffer: buffer,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    Error(T),
}

pub fn run<'a>(program: &str, buffer: &'a mut dyn Buffer) -> State<'a> {
    let mut state = State::new(buffer);
    match parse_program(program) {
        Ok(parsed_program) => run_program(&mut state, &parsed_program),
        Err(err) => state.status = ExecutionStatus::Error(err),
    };
    state
}

pub fn parse_program(program: &str) -> Result<Vec<Token>, String> {
    program
        .chars()
        .map(|c| Token::decode(c))
        .filter(|t_res| t_res.is_ok())
        .collect()
}

pub fn run_program(state: &mut State, program: &Vec<Token>) {
    state.status = ExecutionStatus::InProgress;
    loop {
        match state.status {
            ExecutionStatus::Terminated | ExecutionStatus::Error(_) => break,
            _ => {}
        };
        match program.get(state.program_ptr) {
            Some(command) => run_command(state, &command, program),
            None => break,
        };
    }
    match state.status {
        ExecutionStatus::Error(_) => {}
        _ => state.status = ExecutionStatus::Terminated,
    }
}

pub fn run_command(state: &mut State, command: &Token, program: &Vec<Token>) {
    match command {
        Token::PtrInc => pointer_increment(state),
        Token::PtrDec => pointer_decrement(state),
        Token::ValInc => value_increment(state),
        Token::ValDec => value_decrement(state),
        Token::PutChar => state.buffer.put_byte(state.data[state.data_ptr]),
        Token::GetChar => get_character(state),
        Token::LoopBeg => loop_enter(state, program),
        Token::LoopEnd => loop_exit(state),
        Token::DebugDump => eprintln!("{:?}", state),
        Token::DebugBreakpoint => repl::run(state),
    };
    match command {
        Token::LoopEnd => {} // special case that sets the program pointer itself
        _ => state.program_ptr += 1,
    };
}

fn pointer_increment(state: &mut State) {
    state.data_ptr += 1;
    match state.data.get(state.data_ptr) {
        Some(_) => {}
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

fn find_loop_end(ptr: usize, program: &Vec<Token>) -> Result<usize, ()> {
    match program.get(ptr) {
        Some(Token::LoopEnd) => Ok(ptr),
        Some(Token::LoopBeg) => {
            find_loop_end(ptr + 1, program).and_then(|i| find_loop_end(i + 1, program))
        }
        Some(_) => find_loop_end(ptr + 1, program),
        None => Err(()),
    }
}

fn loop_enter(state: &mut State, program: &Vec<Token>) {
    match state.data[state.data_ptr] {
        0 => match find_loop_end(state.program_ptr + 1, program) {
            Ok(i) => state.program_ptr = i,
            Err(_) => {
                state.status = ExecutionStatus::Error("'[' missing corresponding ']'".to_string())
            }
        },
        _ => state.loop_stack.push(state.program_ptr),
    }
}

fn loop_exit(state: &mut State) {
    match (state.loop_stack.pop(), state.data[state.data_ptr]) {
        (Some(_), 0) => state.program_ptr += 1,
        (Some(ptr_loc), _) => state.program_ptr = ptr_loc,
        (None, _) => {
            state.status = ExecutionStatus::Error("']' missing corresponding '['".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::buffer::ASCIICharBuffer;

    #[test]
    fn test_pointer_increment() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = State::new(&mut buffer);
        pointer_increment(&mut state);
        assert_eq!(1, state.data_ptr);
        assert_eq!(vec![0, 0], state.data);
    }

    #[test]
    fn test_pointer_decrement() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = State::new(&mut buffer);
        pointer_decrement(&mut state);
        assert_eq!(0, state.data_ptr);
        assert_eq!(vec![0, 0], state.data);
    }

    #[test]
    fn test_value_increment() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = State::new(&mut buffer);
        value_increment(&mut state);
        assert_eq!(1, state.data[state.data_ptr]);
    }

    #[test]
    fn test_value_increment_with_overflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = State::new(&mut buffer);
        state.data[state.data_ptr] = 255;
        value_increment(&mut state);
        assert_eq!(0, state.data[state.data_ptr]);
    }

    #[test]
    fn test_value_decrement_with_underflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = State::new(&mut buffer);
        value_decrement(&mut state);
        assert_eq!(255, state.data[state.data_ptr]);
    }

    #[test]
    fn test_find_loop_end() {
        let program = vec![Token::PtrInc, Token::LoopEnd];
        assert_eq!(Ok(1), find_loop_end(0, &program));
    }
}
