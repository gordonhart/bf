use std::cell::RefCell;

pub mod ioctx;
pub mod interpreter;
pub mod token;
pub mod repl;

use ioctx::{IoCtx, InMemoryIoCtx};
use interpreter::{ExecutionStatus, ExecutionContext};


#[derive(Debug)]
pub enum Error<T> {
    ProgramError(T),
    InternalError(T),
}


#[no_mangle]
pub extern fn bf_execute(program: &str, input: &[u8]) -> Result<Vec<u8>, Error<String>> {
    let ictx = RefCell::new(Box::new(InMemoryIoCtx::default()) as Box<dyn IoCtx>);
    let mut ictx_ref = ictx.borrow_mut();
    if let Err(_) = ictx_ref.write_input(&input[..]) {
        return Err(Error::InternalError("unable to open buffer".to_string()));
    };
    let status = ExecutionContext::new(ictx_ref, program).execute();
    let mut ictx_ref = ictx.borrow_mut();
    match status {
        ExecutionStatus::Terminated => {
            let mut output: Vec<u8> = Vec::new();
            let mut buf: [u8; 256] = [0; 256];
            loop {
                match ictx_ref.read_output(&mut buf) {
                    Ok(n) => {
                        if n == 0 { break };
                        output.extend_from_slice(&buf[..n]);
                    },
                    Err(_) => break,
                };
            };
            Ok(output)
        },
        ExecutionStatus::ProgramError(e) => Err(Error::ProgramError(e)),
        ExecutionStatus::InternalError(e) => Err(Error::InternalError(e)),
        _ => Err(Error::InternalError("unknown error occurred".to_string())),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addition() {
        let program: &str = ",>,<[->+<]>.";
        let input: Vec<u8> = vec![3, 4];
        let output: Vec<u8> = bf_execute(program, &input[..]).unwrap();
        assert_eq!(output, vec![7]);
    }
}
