//! An interactive [BrainF\*ck](https://en.wikipedia.org/wiki/Brainfuck)
//! interpreter with executable, library, and foreign interfaces.

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
mod repl;


/// The different ways the execution of a program can fail.
#[derive(Debug)]
pub enum Error<T> {
    /// A 'you' problem.
    ProgramError(T),

    /// A 'me' problem.
    InternalError(T),
}


/// Execute a program using the `bfi` interpreter. The output of the program (as placed by the `.`
/// command) is returned as a vector.
///
/// # Examples
///
/// ```rust
/// extern crate bfi;
///
/// fn main () {
///     match bfi::execute(",>,<[->+<]>.", b"\x03\x04") {
///         Ok(v) => println!("result of addition: {}", v[0]),
///         Err(e) => eprintln!("error: {:?}", e),
///     };
/// }
/// ```
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


/// Result structure communicating the status of a call to `bf_exec` in a foreign-friendly way.
#[repr(C)]
pub struct BfExecResult {
    /// Impoverished boolean (`match success { 1 => true, 0 => false }`) indicating  the final
    /// status of the program producing this `BfExecResult`.
    pub success: c_uchar, // u8

    /// Raw pointer to the start of the memory section containing the output of the program.
    pub output: *mut c_uchar, // u8

    /// Length of the program output, required as many valid programs will output `\x00` bytes,
    /// preventing the usage of a NUL-terminated string for program output.
    pub output_length: size_t, // usize
}

impl BfExecResult {
    fn default_failure() -> Self {
        Self {
            success: 0,
            output: std::ptr::null_mut() as *mut c_uchar,
            output_length: 0,
        }
    }
}


/// Interface to `bfi::execute` a program from foreign code.
///
/// # Safety
///
/// This function dereferences the raw pointers provided as inputs. A fatal memory error will occur
/// if either of these are invalid addresses or the specified `input_length` of `input` is not
/// correct.
#[no_mangle]
#[deny(improper_ctypes)]  // TODO: this deny currently does not work
pub unsafe extern "C" fn bf_exec(
    program: *const c_char,
    input: *const c_uchar,
    input_length: size_t,
) -> BfExecResult
{
    let program_str: &str = match CStr::from_ptr(program).to_str() { // unsafe
        Ok(s) => s,
        // return failure if the program provided is not valid unicode
        Err(_) => return BfExecResult::default_failure(),
    };

    let input_slice: &[u8] = slice::from_raw_parts(input, input_length as usize); //unsafe

    match execute(program_str, input_slice) {
        Ok(mut v) => {
            // ensure v.len() == v.capacity() such that the capacity of the vector does not need to
            // be shared with the foreign caller in order for the subsequent call to `bf_free` to
            // not leak -- both len and capacity are required when calling `Vec::from_raw_parts`
            v.shrink_to_fit();
            let l = v.len();
            let ptr = v.as_mut_ptr();
            // instruct rust to forget about this section of memory -- it will only be
            // deallocated if the vector is reassembled and dropped (see `bf_free`)
            mem::forget(v);
            BfExecResult {
                success: 1,
                output: ptr,
                output_length: l,
            }
        },
        // point to garbage -- will certainly crash the program if this location is returned to
        // `bf_free`, so it is up to the foreign caller to be responsible here (as always)
        Err(_) => BfExecResult::default_failure(),
    }
}


/// Deallocate the memory containing the output of a previous call to `bf_exec`.
///
/// The output returned from `bf_exec` represents a vector in memory that has been forgotten by Rust
/// and will thus not be automatically deallocated. In order to prevent this leakage, the caller o
/// `bf_exec` should return this pointer back to Rust here such that it can be consumed and dropped.
///
/// # Safety
///
/// This function dereferences the raw pointer provided as input. In order to not crash with a
/// message like `free(): invalid pointer`, the provided pointer and length must match the location
/// of a vector in memory that Rust has been told to forget about -- e.g., one returned as `output`
/// and `output_length` from `bf_exec`.
///
/// See the
/// [Safety section](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts) of
/// `Vec::from_raw_parts` for more information.
#[no_mangle]
pub unsafe extern "C" fn bf_free(
    to_free: *mut c_uchar,
    length: size_t,
) {
    Vec::from_raw_parts(to_free as *mut u8, length as usize, length as usize);
}


#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    static ADD_PROGRAM: &str = ",>,<[->+<]>.";

    // TODO: is it _really_ necessary to brute force through all u8 pairs _twice_ in these tests?
    #[test]
    fn test_addition() {
        for a in 0..=255 {
            for b in 0..=255 {
                let output: Vec<u8> = execute(ADD_PROGRAM, &[a, b]).unwrap();
                let expected_output: u8 = a.wrapping_add(b);
                assert_eq!(output.iter().nth(0).unwrap(), &expected_output);
            };
        };
    }

    #[test]
    fn test_foreign_addition() {
        let program: *const c_char = CString::new(ADD_PROGRAM).unwrap().into_raw();
        for a in 0..=255 {
            for b in 0..=255 {
                let result: BfExecResult = unsafe { bf_exec(program, [a, b].as_ptr(), 2) };
                assert_eq!(result.success, 1u8);
                let expected_output: u8 = a.wrapping_add(b);
                let actual_output: &[u8] = unsafe {
                    slice::from_raw_parts(result.output, result.output_length)
                };
                if actual_output.len() != 1 {
                    panic!("expected output of length 1, got {} ({} + {})",
                        actual_output.len(), a, b);
                };
                assert_eq!(actual_output[0] as u8, expected_output);
                unsafe { bf_free(result.output, result.output_length) };
            };
        };
    }

    #[test]
    fn test_foreign_program_invalid_unicode() {
        let program = b"\x81";
        let program_ptr = program.as_ptr() as *const c_char;
        let result = unsafe { bf_exec(program_ptr, [].as_ptr(), 0) };
        assert_eq!(result.success, 0u8);
    }

    #[test]
    fn test_foreign_program_error() {
        let program = b"[";
        let program_ptr = program.as_ptr() as *const c_char;
        let result = unsafe { bf_exec(program_ptr, [].as_ptr(), 0) };
        assert_eq!(result.success, 0u8);
    }
}
