use std::io::{Read, Write};

use crate::repl::{self, REPLResult};
use crate::token::Token;

// #[derive(Debug, PartialEq)]
pub struct State {
    pub data: Vec<u8>,
    pub data_ptr: usize,
    pub program: Vec<Token>,
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
}

impl std::fmt::Debug for State {
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

impl State {
    pub fn new() -> State {
        State {
            data: vec![0],
            data_ptr: 0,
            program: vec![],
            program_ptr: 0,
            loop_stack: vec![],
            status: ExecutionStatus::NotStarted,
        }
    }
}

#[derive(Debug)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Interactive(repl::Instance),
    Terminated,
    ProgramError(T),
    InternalError(T),
}

pub fn run(program: &str, mut buffer: impl Write) -> State {
    let mut state = State::new();
    state.status = ExecutionStatus::InProgress;
    match Token::parse_str(program) {
        Ok(parsed_program) => {
            state.program = parsed_program;
            run_program(&mut state, buffer);
        },
        Err(err) => state.status = ExecutionStatus::ProgramError(err),
    };
    state
}

pub fn run_program(state: &mut State, mut buffer: impl Write) {
    match &mut state.status {
        ExecutionStatus::Terminated | ExecutionStatus::ProgramError(_) | ExecutionStatus::InternalError(_) => return,
        ExecutionStatus::Interactive(repl_instance) => {
            match &mut repl_instance.get() {
                REPLResult::Program(p) => {
                    state.program = Token::parse_str(p.as_str()).unwrap();
                    run_program(state, &mut buffer);
                },
                REPLResult::Break => state.status = ExecutionStatus::InProgress,
                REPLResult::Terminate => state.status = ExecutionStatus::Terminated,
                REPLResult::Error(e) => state.status = ExecutionStatus::InternalError(e.to_string()),
            };
        },
        _ => state.execute_command(&mut buffer)
    };
    run_program(state, &mut buffer);
}

impl State {
    // TODO: return result
    pub fn execute_command(&mut self, mut buffer: impl Write) {
        if let Some(command) = self.program.get(self.program_ptr) {
            match command {
                Token::PtrInc => pointer_increment(self),
                Token::PtrDec => pointer_decrement(self),
                Token::ValInc => value_increment(self),
                Token::ValDec => value_decrement(self),
                Token::PutChar => put_character(self, &mut buffer),
                Token::GetChar => get_character(self),
                Token::LoopBeg => loop_enter(self),
                Token::LoopEnd => loop_exit(self),
                Token::DebugDump => eprintln!("{:?}", self),
                Token::DebugBreakpoint => self.status = ExecutionStatus::Interactive(repl::Instance::new()),
            };
            match command {
                Token::LoopEnd => {} // special case that sets the program pointer itself
                _ => self.program_ptr += 1,
            };
        } else {
            self.status = ExecutionStatus::Terminated;
        }
    }
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

fn put_character(state: &mut State, mut buffer: impl Write) {
    buffer.write(&state.data[state.data_ptr..state.data_ptr]);
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

fn loop_enter(state: &mut State) {
    match state.data[state.data_ptr] {
        0 => match find_loop_end(state.program_ptr + 1, &state.program) {
            Ok(i) => state.program_ptr = i,
            Err(_) => {
                state.status = ExecutionStatus::ProgramError("'[' missing corresponding ']'".to_string())
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
            state.status = ExecutionStatus::ProgramError("']' missing corresponding '['".to_string())
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
