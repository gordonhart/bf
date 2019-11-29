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
    }
}
