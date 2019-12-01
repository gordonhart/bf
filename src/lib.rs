extern crate libc;

use std::cell::RefCell;
use std::ffi::{CStr, CString};

use libc::c_char;

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


pub fn bf_execute(
    program: &str,
    input: &[u8],
) -> Result<Vec<u8>, Error<String>> {
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


#[repr(C)]
pub struct BfExecResult {
    success: u8,
    output: *mut c_char,
}


#[no_mangle]
#[deny(improper_ctypes)]  // TODO: this deny currently does not work
pub extern "C" fn bf_exec(
    program: *const c_char,
    input: *const c_char,
) -> BfExecResult {
    let program_str: &str = unsafe { CStr::from_ptr(program).to_str().unwrap() };
    let input_slice: &[u8] = unsafe { CStr::from_ptr(input).to_bytes() };

    let (success, output) = match bf_execute(program_str, input_slice) {
        Ok(v) => unsafe { (1, CString::from_vec_unchecked(v).into_raw()) },
        Err(_) => (0, CString::new("").unwrap().into_raw()),
    };

    BfExecResult {
        success: success,
        output: output,
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
