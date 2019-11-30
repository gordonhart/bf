extern crate clap;

use std::cell::RefCell;

use clap::{App, Arg, ArgMatches};

mod ioctx;
mod interpreter;
mod repl;
mod token;

use interpreter::{ExecutionStatus, ExecutionContext};


static PROGRAM_ARG: &str = "program";
static VERBOSE_ARG: &str = "verbose";
static FILE_ARG: &str = "file";
static UTF8_FLAG: &str = "utf8";
static UNBUFFERED_FLAG: &str = "unbuffered";


// explicitly specify 'static lifetime
fn get_command_line_args() -> ArgMatches<'static> {
    App::new("bfi")
        .version("0.1")
        .about("BrainF*ck language interpreter")
        .arg(Arg::with_name(PROGRAM_ARG)
            .help("Program to execute")
            .conflicts_with(FILE_ARG)
            .index(1))
        .arg(Arg::with_name(VERBOSE_ARG)
            .short("v")
            .long("verbose")
            .help("Toggle high verbosity"))
        .arg(Arg::with_name(FILE_ARG)
            .short("f")
            .long("file")
            .takes_value(true)
            .value_name("FILE")
            .conflicts_with(PROGRAM_ARG)
            .help("Program file to execute"))
        .arg(Arg::with_name(UTF8_FLAG)
            .short("u")
            .long("utf8")
            .takes_value(false)
            .help("Use 8-bit Unicode output encoding"))
        .arg(Arg::with_name(UNBUFFERED_FLAG)
            .long("unbuffered")
            .takes_value(false)
            .help("Do not buffer output"))
        .get_matches()
}


fn get_io_context(use_utf8: bool, use_unbuffered: bool) -> Box<dyn ioctx::IoCtx> {
    match (use_utf8, use_unbuffered) {
        // TODO: address
        // (true, _) => Box::new(ioctx::StdUTF8IOContext::default()),
        // (_, true) => Box::new(ioctx::StdIOContext::default()),
        _ => Box::new(ioctx::StdIoCtx::default()),
    }
}


fn main() {
    let opts = get_command_line_args();

    let program_string: String = match (opts.value_of(PROGRAM_ARG), opts.value_of(FILE_ARG)) {
        (Some(s), None) => s.to_string(),
        (None, Some(filename)) => match std::fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("bfi: file '{}' could not be read ({})", filename, e);
                std::process::exit(1);
            }
        },
        // default to REPL if no program provided
        (None, None) => "!".to_string(),
        // final arm should never be reached due to mutual `conflicts_with`
        _ => panic!("bfi: argument error"),
    };

    // Creating the io_context inside a block like this ensures that it is dropped before the call
    // to std::process::exit, necessary to flush output buffer for stdout
    let retcode: i32 = {
        let io_context = RefCell::new(
            get_io_context(opts.is_present(UTF8_FLAG), opts.is_present(UNBUFFERED_FLAG)));

        let execution_status: ExecutionStatus<String> =
            ExecutionContext::new(io_context.borrow_mut(), program_string.as_str()).execute();

        match execution_status {
            ExecutionStatus::Terminated => {
                if opts.is_present(VERBOSE_ARG) {
                    eprintln!("bfi: terminated without errors");
                };
                0
            }
            ExecutionStatus::ProgramError(err) => {
                eprintln!("bfi: exited with error: {}", err);
                1
            }
            _ => panic!("bfi: internal error"),
        }
    };

    std::process::exit(retcode);
}
