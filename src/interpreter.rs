use std::fmt::Debug;
use std::io::{Read, Write};

use crate::ioctx;

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
}

impl ExecutionContext {
    pub fn new() -> Self {
        ExecutionContext {
            status: ExecutionStatus::NotStarted,
        }
    }

    pub fn with_io_context(&mut self, ctx: Box<dyn ioctx::RW>) -> &mut Self {
        self
    }

    pub fn with_program(&mut self, program: &str) -> &mut Self {
        self
    }

    pub fn execute(&mut self) -> ExecutionStatus<String> {
        self.status.clone()
    }
}
