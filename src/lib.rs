extern crate libc;

use std::cell::RefCell;
use std::ffi::{CStr, CString};

use libc::c_char;

use ioctx::{IoCtx, InMemoryIoCtx};
use interpreter::{ExecutionStatus, ExecutionContext};


pub mod ioctx;
pub mod interpreter;
pub mod token;
pub mod repl;


#[derive(Debug)]
pub enum Error<T> {
    ProgramError(T),
    InternalError(T),
}


pub fn execute(
    program: &str,
    input: &[u8],
) -> Result<Vec<u8>, Error<String>>
{
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
    output: *mut u8,
    output_length: usize,
}


#[no_mangle]
#[deny(improper_ctypes)]  // TODO: this deny currently does not work
pub extern "C" fn bf_exec(
    program: *const c_char,
    input: *const u8, // c_uchar,
    input_length: usize,
) -> BfExecResult
{
    let program_str: &str = unsafe { CStr::from_ptr(program).to_str().unwrap() };
    let input_slice: &[u8] = unsafe { std::slice::from_raw_parts(input, input_length) };

    let (success, output, output_length) = match execute(program_str, input_slice) {
        Ok(v) => unsafe {
            let l = v.len();
            (1, CString::from_vec_unchecked(v).into_raw() as *mut u8, l)
        },
        // ends up pointing to garbage as the created vector is deallocated immediately
        Err(_) => (0, Vec::new().as_mut_ptr(), 0),
    };

    BfExecResult {
        success: success,
        output: output,
        output_length: output_length,
    }
}


#[cfg(test)]
mod test {
    extern crate rand;

    use rand::RngCore;

    use super::*;

    static ADD_PROGRAM: &str = ",>,<[->+<]>.";

    #[test]
    fn test_addition() {
        let mut rng = rand::thread_rng();
        let mut input: [u8; 2] = [0; 2];
        for _ in 0..1000 {
            rng.fill_bytes(&mut input[..]);
            let output: Vec<u8> = execute(ADD_PROGRAM, &input[..]).unwrap();
            let expected_output: u8 = input[0].wrapping_add(input[1]);
            assert_eq!(output.iter().nth(0).unwrap(), &expected_output);
        }
    }

    #[test]
    fn test_foreign_addition() {
        let mut rng = rand::thread_rng();
        let mut input: [u8; 2] = [0; 2];
        let program: *const c_char = CString::new(ADD_PROGRAM).unwrap().into_raw();
        for _ in 0..1000 {
            rng.fill_bytes(&mut input[..]);
            let result: BfExecResult = bf_exec(program, input.as_ptr(), input.len());
            assert_eq!(result.success, 1u8);
            let expected_output: u8 = input[0].wrapping_add(input[1]);
            let actual_output: &[u8] = unsafe { std::slice::from_raw_parts(result.output, result.output_length) };
            if actual_output.len() != 1 {
                panic!("expected output of length 1, got {} ({} + {})",
                    actual_output.len(), input[0], input[1]);
            }
            assert_eq!(actual_output[0] as u8, expected_output);
        }
    }
}
