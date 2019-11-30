use std::default::Default;
use std::fmt::{self, Debug};

use crate::ioctx;
use crate::repl;
use crate::token::Token;

#[derive(Debug, Copy, Clone)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    ProgramError(T),
    InternalError(T),
}

pub struct ExecutionContext {
    status: ExecutionStatus<String>,
    ctx: Box<dyn ioctx::RW>,
    data: Vec<u8>,
    data_ptr: usize,
    program: Vec<Token>,
    program_ptr: usize,
    loop_stack: Vec<usize>,
}

impl Debug for ExecutionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "data: {:?}\ndata_ptr: {:?}\nprogram_ptr: {:?}\nloop_stack: {:?}\nstatus: {:?}",
            self.data, self.data_ptr, self.program_ptr, self.loop_stack, self.status,
        )
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        ExecutionContext {
            status: ExecutionStatus::NotStarted,
            ctx: Box::new(ioctx::StdIOContext::new()),
            data: vec![0],
            data_ptr: 0,
            program: vec![],
            program_ptr: 0,
            loop_stack: vec![],
        }
    }
}

impl ExecutionContext {
    pub fn new(ctx: Box<dyn ioctx::RW>, program: &str) -> Self {
        ExecutionContext {
            ctx: ctx,
            program: Token::parse_str(program),
            ..ExecutionContext::default()
        }
    }

    pub fn execute(&mut self) -> ExecutionStatus<String> {
        self.run();
        self.status.clone()
    }

    fn run(&mut self) {
        loop {
            match self.status {
                ExecutionStatus::Terminated => return self.cleanup(),
                ExecutionStatus::ProgramError(_) | ExecutionStatus::InternalError(_) => return,
                ExecutionStatus::NotStarted => self.status = ExecutionStatus::InProgress,
                ExecutionStatus::InProgress => {
                    match self.program.get(self.program_ptr) {
                        Some(cmd) => self.run_command(&cmd.clone()),
                        None => self.status = ExecutionStatus::Terminated,
                    };
                },
            };
        }
    }

    fn run_command(&mut self, command: &Token) {
        match command {
            Token::PtrInc => self.pointer_increment(),
            Token::PtrDec => self.pointer_decrement(),
            Token::ValInc => self.value_increment(),
            Token::ValDec => self.value_decrement(),
            Token::PutChar => self.put_character(),
            Token::GetChar => self.get_character(),
            Token::LoopBeg => self.loop_enter(),
            Token::LoopEnd => self.loop_exit(),
            Token::DebugDump => eprintln!("{:?}", self),
            Token::DebugBreakpoint => self.run_interactive(),
        };
        match command {
            Token::LoopEnd => {} // special case that sets the program pointer itself
            _ => self.program_ptr += 1,
        };
    }

    fn run_interactive(&mut self) {
        let program_ptr_before = self.program_ptr;
        for cmd in repl::ReplInstance::new() {
            match cmd {
                repl::ReplResult::Command(cmd) => self.run_command(&cmd),
                repl::ReplResult::Quit => {
                    self.status = ExecutionStatus::Terminated;
                    return
                },
                repl::ReplResult::Error(e) => panic!(e), // TODO
            };
        }
        self.program_ptr = program_ptr_before;
    }

    fn cleanup(&mut self) {
        // Assert that all open loops have been terminated
        if self.loop_stack.len() > 0 {
            let e = format!("unmatched '[' at program position(s): {:?}", self.loop_stack);
            self.status = ExecutionStatus::ProgramError(e.to_string());
        };
    }

    fn pointer_increment(&mut self) {
        self.data_ptr += 1;
        if let None = self.data.get(self.data_ptr) {
            self.data.push(0);
        };
    }

    fn pointer_decrement(&mut self) {
        match self.data_ptr {
            0 => self.data.insert(0, 0),
            _ => self.data_ptr -= 1,
        }
    }

    fn value_increment(&mut self) {
        self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(1);
    }

    fn value_decrement(&mut self) {
        self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_sub(1);
    }

    fn put_character(&mut self) {
        (*self.ctx).write(&self.data[self.data_ptr..self.data_ptr+1]);
    }

    fn get_character(&mut self) {
        // let mut buffer: [u8; 1024] = [0; 1024];
        let mut buffer: [u8; 1] = [0; 1];
        (*self.ctx).read(&mut buffer[..]).unwrap(); // TODO: actually handle Result here
        self.data[self.data_ptr] = buffer[0];
    }

    fn find_loop_end(ptr: usize, program: &Vec<Token>) -> Result<usize, ()> {
        match program.get(ptr) {
            Some(Token::LoopEnd) => Ok(ptr),
            Some(Token::LoopBeg) => {
                ExecutionContext::find_loop_end(ptr + 1, program)
                    .and_then(|i| ExecutionContext::find_loop_end(i + 1, program))
            }
            Some(_) => ExecutionContext::find_loop_end(ptr + 1, program),
            None => Err(()),
        }
    }

    fn loop_enter(&mut self) {
        match self.data[self.data_ptr] {
            0 => match ExecutionContext::find_loop_end(self.program_ptr + 1, &self.program) {
                Ok(i) => self.program_ptr = i,
                Err(_) => {
                    let e = format!(
                        "'[' at program position {} missing corresponding ']'", self.program_ptr);
                    self.status = ExecutionStatus::ProgramError(e.to_string());
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
                let e = format!(
                    "']' at program position {} missing corresponding '['", self.program_ptr);
                self.status = ExecutionStatus::ProgramError(e.to_string())
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pointer_increment() {
        let mut ectx = ExecutionContext::default();
        ectx.pointer_increment();
        assert_eq!(1, ectx.data_ptr);
        assert_eq!(vec![0, 0], ectx.data);
    }

    #[test]
    fn test_pointer_decrement() {
        let mut ectx = ExecutionContext::default();
        ectx.pointer_decrement();
        assert_eq!(0, ectx.data_ptr);
        assert_eq!(vec![0, 0], ectx.data);
    }

    #[test]
    fn test_find_loop_end() {
        let program = vec![Token::PtrInc, Token::LoopEnd];
        assert_eq!(Ok(1), ExecutionContext::find_loop_end(0, &program));
    }
}
