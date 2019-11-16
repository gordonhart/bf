extern crate clap;

use clap::{App, Arg, ArgMatches};

mod buffer;
mod interpreter;
mod repl;
mod token;

static PROGRAM_ARG: &'static str = "program";
static VERBOSE_ARG: &'static str = "verbose";
static FILE_ARG: &'static str = "file";
static UTF8_FLAG: &'static str = "utf8";
static UNBUFFERED_FLAG: &'static str = "unbuffered";

// explicitly specify 'static lifetime
fn get_command_line_args() -> ArgMatches<'static> {
    App::new("bfi")
        .version("0.1")
        .about("BrainF*ck language interpreter")
        .arg(
            Arg::with_name(PROGRAM_ARG)
                .help("Program to execute")
                .conflicts_with(FILE_ARG)
                .index(1),
        )
        .arg(
            Arg::with_name(VERBOSE_ARG)
                .short("v")
                .long("verbose")
                .help("Toggle high verbosity"),
        )
        .arg(
            Arg::with_name(FILE_ARG)
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("FILE")
                .conflicts_with(PROGRAM_ARG)
                .help("Program file to execute"),
        )
        .arg(
            Arg::with_name(UTF8_FLAG)
                .short("u")
                .long("utf8")
                .takes_value(false)
                .help("Use 8-bit Unicode output encoding"),
        )
        .arg(
            Arg::with_name(UNBUFFERED_FLAG)
                .long("unbuffered")
                .takes_value(false)
                .help("Do not buffer output"),
        )
        .get_matches()
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
        (None, None) => "!".to_string(), // default to REPL if no program provided
        // final arm should never be reached due to mutual `conflicts_with`
        _ => panic!("bfi: argument error"),
    };

    let mut buffer: Box<dyn buffer::Buffer> = match
        (opts.is_present(UTF8_FLAG), opts.is_present(UNBUFFERED_FLAG))
    {
        (true, _) => Box::new(buffer::UTF8CharBuffer::new()),
        (_, true) => Box::new(buffer::ASCIICharBuffer {}),
        (_, false) => Box::new(buffer::ASCIILineBuffer {}),
    };

    // let program_state_after_execution = interpreter::run(program_string.as_str(), &mut Box::into_raw(buffer));
    let program_state_after_execution = interpreter::run(program_string.as_str(), &mut *buffer);

    let retcode: i32 = match program_state_after_execution.status {
        interpreter::ExecutionStatus::Terminated => {
            if opts.is_present(VERBOSE_ARG) {
                eprintln!("bfi: terminated without errors");
            };
            0
        }
        interpreter::ExecutionStatus::Error(err) => {
            eprintln!("bfi: exited with error: {}", err);
            1
        }
        _ => panic!("bfi: internal error"),
    };

    std::process::exit(retcode);
}
