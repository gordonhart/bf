use std::fmt::Debug;
use std::io::{Read, Write};

use crate::ioctx;
use crate::repl;
use crate::token::Token;

#[derive(Debug, Copy, Clone)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    Interactive,
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
    pub fn new(ctx: Box<dyn ioctx::RW>, program: &str) -> Self {
        ExecutionContext {
            status: ExecutionStatus::NotStarted,
            ctx: ctx,
            data: vec![0],
            data_ptr: 0,
            program: Token::parse_str(program),
            program_ptr: 0,
            loop_stack: vec![],
        }
    }

    pub fn execute(&mut self) -> ExecutionStatus<String> {
        self.run();
        self.status.clone()
    }

    fn run(&mut self) -> &mut Self {
        loop {
            match self.status {
                ExecutionStatus::Terminated
                | ExecutionStatus::ProgramError(_)
                | ExecutionStatus::InternalError(_) => break,
                ExecutionStatus::NotStarted => self.status = ExecutionStatus::InProgress,
                ExecutionStatus::Interactive => {
                    for cmd in repl::ReplInstance::new() {
                        self.run_command(&cmd);
                    }
                },
                ExecutionStatus::InProgress => {
                    match self.program.get(self.program_ptr) {
                        Some(cmd) => self.run_command(&cmd.clone()),
                        None => self.status = ExecutionStatus::Terminated,
                    };
                },
            };
        };
        self
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
            Token::DebugBreakpoint => self.status = ExecutionStatus::Interactive,
        };
        match command {
            Token::LoopEnd => {} // special case that sets the program pointer itself
            _ => self.program_ptr += 1,
        };
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

    fn put_character(&mut self) {
        (*self.ctx).write(&self.data[self.data_ptr..self.data_ptr]);
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
