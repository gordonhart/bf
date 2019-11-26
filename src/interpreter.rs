use std::io::{Read, Write};

use crate::repl::{self, REPLResult};
use crate::token::Token;

// #[derive(Debug, PartialEq)]
pub struct ExecutionContext {
    pub data: Vec<u8>,
    pub data_ptr: usize,
    pub program: Vec<Token>,
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
}

impl std::fmt::Debug for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "data: {:?}\ndata_ptr: {:?}\nprogram_ptr: {:?}\nloop_stack: {:?}\nstatus: {:?}",
            self.data, self.data_ptr, self.program_ptr, self.loop_stack, self.status,
        )
    }
}

impl ExecutionContext {
    pub fn new() -> ExecutionContext {
        ExecutionContext {
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
    Interactive(Box<repl::Instance>),
    Terminated,
    ProgramError(T),
    InternalError(T),
}

pub fn run(program: &str, mut buffer: impl Write) -> ExecutionContext {
    let mut state = ExecutionContext::new();
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

pub fn run_program(state: &mut ExecutionContext, mut buffer: impl Write) {
    match &mut state.status {
        ExecutionStatus::Terminated
        | ExecutionStatus::ProgramError(_)
        | ExecutionStatus::InternalError(_) => return,
        ExecutionStatus::Interactive(repl_instance_box) => {
            match &mut (*repl_instance_box).get() {
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

impl ExecutionContext {
    // TODO: return result
    pub fn execute_command(&mut self, mut buffer: impl Write) {
        let command = self.program.get(self.program_ptr).unwrap().clone();

        match command {
            Token::PtrInc => self.pointer_increment(),
            Token::PtrDec => self.pointer_decrement(),
            Token::ValInc => self.value_increment(),
            Token::ValDec => self.value_decrement(),
            Token::PutChar => self.put_character(&mut buffer),
            Token::GetChar => self.get_character(),
            Token::LoopBeg => self.loop_enter(),
            Token::LoopEnd => self.loop_exit(),
            Token::DebugDump => eprintln!("{:?}", self),
            Token::DebugBreakpoint => self.status = ExecutionStatus::Interactive(Box::new(repl::Instance::new())),
        };
        match command {
            Token::LoopEnd => {} // special case that sets the program pointer itself
            _ => self.program_ptr += 1,
        };
        
        /*
        if let Some(command) = self.program.get(self.program_ptr).clone() {
            match command {
                Token::PtrInc => self.pointer_increment(),
                Token::PtrDec => self.pointer_decrement(),
                Token::ValInc => self.value_increment(),
                Token::ValDec => self.value_decrement(),
                Token::PutChar => self.put_character(&mut buffer),
                Token::GetChar => self.get_character(),
                Token::LoopBeg => self.loop_enter(),
                Token::LoopEnd => self.loop_exit(),
                Token::DebugDump => eprintln!("{:?}", self),
                Token::DebugBreakpoint => self.status = ExecutionStatus::Interactive(Box::new(repl::Instance::new())),
            };
            match command {
                Token::LoopEnd => {} // special case that sets the program pointer itself
                _ => self.program_ptr += 1,
            };
        } else {
            self.status = ExecutionStatus::Terminated;
        }
        */
    }

    fn pointer_increment(&mut self) {
        self.data_ptr += 1;
        match self.data.get(self.data_ptr) {
            Some(_) => {}
            None => self.data.push(0),
        }
    }

    fn pointer_decrement(&mut self) {
        match self.data_ptr {
            0 => self.data.insert(0, 0),
            _ => self.data_ptr -= 1,
        }
    }

    fn value_increment(&mut self) {
        match self.data[self.data_ptr].overflowing_add(1) {
            (v, _) => self.data[self.data_ptr] = v,
        }
    }

    fn value_decrement(&mut self) {
        match self.data[self.data_ptr].overflowing_sub(1) {
            (v, _) => self.data[self.data_ptr] = v,
        }
    }

    fn put_character(&mut self, mut buffer: impl Write) {
        buffer.write(&self.data[self.data_ptr..self.data_ptr]);
    }

    fn get_character(&mut self) {
        match std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8)
        {
            Some(c) => self.data[self.data_ptr] = c,
            None => self.status = ExecutionStatus::Terminated,
        }
    }

    fn find_loop_end(&self, ptr: usize, program: &Vec<Token>) -> Result<usize, ()> {
        match program.get(ptr) {
            Some(Token::LoopEnd) => Ok(ptr),
            Some(Token::LoopBeg) => {
                self.find_loop_end(ptr + 1, program).and_then(|i| self.find_loop_end(i + 1, program))
            }
            Some(_) => self.find_loop_end(ptr + 1, program),
            None => Err(()),
        }
    }

    fn loop_enter(&mut self) {
        match self.data[self.data_ptr] {
            0 => match self.find_loop_end(self.program_ptr + 1, &self.program) {
                Ok(i) => self.program_ptr = i,
                Err(_) => {
                    self.status = ExecutionStatus::ProgramError("'[' missing corresponding ']'".to_string())
                }
            },
            _ => self.loop_stack.push(self.program_ptr),
        }
    }

    fn loop_exit(&mut self) {
        match (self.loop_stack.pop(), self.data[self.data_ptr]) {
            (Some(_), 0) => self.program_ptr += 1,
            (Some(ptr_loc), _) => self.program_ptr = ptr_loc,
            (None, _) => {
                self.status = ExecutionStatus::ProgramError("']' missing corresponding '['".to_string())
            }
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
        let mut state = ExecutionContext::new(&mut buffer);
        pointer_increment(&mut state);
        assert_eq!(1, state.data_ptr);
        assert_eq!(vec![0, 0], state.data);
    }

    #[test]
    fn test_pointer_decrement() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = ExecutionContext::new(&mut buffer);
        pointer_decrement(&mut state);
        assert_eq!(0, state.data_ptr);
        assert_eq!(vec![0, 0], state.data);
    }

    #[test]
    fn test_value_increment() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = ExecutionContext::new(&mut buffer);
        value_increment(&mut state);
        assert_eq!(1, state.data[state.data_ptr]);
    }

    #[test]
    fn test_value_increment_with_overflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = ExecutionContext::new(&mut buffer);
        state.data[state.data_ptr] = 255;
        value_increment(&mut state);
        assert_eq!(0, state.data[state.data_ptr]);
    }

    #[test]
    fn test_value_decrement_with_underflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut state = ExecutionContext::new(&mut buffer);
        value_decrement(&mut state);
        assert_eq!(255, state.data[state.data_ptr]);
    }

    #[test]
    fn test_find_loop_end() {
        let program = vec![Token::PtrInc, Token::LoopEnd];
        assert_eq!(Ok(1), find_loop_end(0, &program));
    }
}
