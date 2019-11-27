use std::fmt::Debug;
use std::io::{Read, Write};

use crate::ioctx;

#[derive(Debug)]
enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    ProgramError(T),
    InternalError(T),
}
