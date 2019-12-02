extern crate libc;

use std::cell::RefCell;
use std::ffi::CStr;
use std::mem;
use std::slice;

use libc::{c_char, size_t, c_uchar};

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
    if ictx_ref.write_input(&input[..]).is_err() {
        return Err(Error::InternalError("unable to open buffer".to_string()));
    };
    let status = ExecutionContext::new(ictx_ref, program).execute();
    let mut ictx_ref = ictx.borrow_mut();
    match status {
        ExecutionStatus::Terminated => {
            let mut output: Vec<u8> = Vec::new();
            let mut buf: [u8; 256] = [0; 256];
            while let Ok(n) = ictx_ref.read_output(&mut buf) {
                if n == 0 { break };
                output.extend_from_slice(&buf[..n]);
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
    success: c_uchar, // u8
    output: *mut c_uchar, // u8
    output_length: size_t, // usize
}


// TODO: better Safety section
/// # Safety
///
/// This function dereferences the raw pointers provided as inputs.
#[no_mangle]
#[deny(improper_ctypes)]  // TODO: this deny currently does not work
pub unsafe extern "C" fn bf_exec(
    program: *const c_char,
    input: *const c_uchar,
    input_length: size_t,
) -> BfExecResult
{
    let program_str: &str = CStr::from_ptr(program).to_str().unwrap(); // unsafe
    let input_slice: &[u8] = slice::from_raw_parts(input, input_length as usize); //unsafe

    let (success, output, output_length) = match execute(program_str, input_slice) {
        Ok(mut v) => {
            v.shrink_to_fit();
            let l = v.len();
            let ptr = v.as_mut_ptr();
            // instruct rust to forget about this section of memory -- it will not only be
            // deallocated if the vector is reassembled and dropped (see `bf_free`)
            mem::forget(v);
            (1, ptr, l)
        },
        // point to garbage -- will certainly crash the program if this location is returned to
        // `bf_free`, so it is up to the foreign caller to be responsible here (as always)
        Err(_) => (0, 0 as *mut c_uchar, 0),
    };

    BfExecResult {
        success,
        output,
        output_length,
    }
}


// TODO: better Safety section
/// `CString::into_raw` transfers ownership of its memory to the holder of the raw pointer. This
/// will leak unless this pointer is consumed by Rust back into a CString that is then dropped,
/// hence this `bf_free` function.
///
/// # Safety
///
/// This function dereferences the raw pointer provided as inputs.
#[no_mangle]
pub unsafe extern "C" fn bf_free(
    to_free: *mut c_uchar,
    length: size_t,
) {
    Vec::from_raw_parts(to_free as *mut u8, length as usize, length as usize);
}


#[cfg(test)]
mod test {
    extern crate rand;

    use std::ffi::CString;

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
            let result: BfExecResult = unsafe { bf_exec(program, input.as_ptr(), input.len()) };
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
