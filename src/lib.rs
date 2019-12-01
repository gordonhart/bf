pub mod ioctx;
pub mod interpreter;
pub mod token;
pub mod repl;

use ioctx::InMemoryIoCtx;
use interpreter::{ExecutionStatus, ExecutionContext};


pub fn test() {
    println!("something!");
}
