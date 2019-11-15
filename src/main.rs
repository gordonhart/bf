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

// explicitly specify 'static lifetime
fn get_command_line_args() -> ArgMatches<'static> {
    App::new("bf")
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
        .get_matches()
}

fn main() {
    let opts = get_command_line_args();

    let program_string: String = match (opts.value_of(PROGRAM_ARG), opts.value_of(FILE_ARG)) {
        (Some(s), None) => s.to_string(),
        (None, Some(filename)) => match std::fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("bf: file '{}' could not be read ({})", filename, e);
                std::process::exit(1);
            }
        },
        (None, None) => "!".to_string(), // default to REPL if no program provided
        // final arm should never be reached due to mutual `conflicts_with`
        _ => panic!("bf: argument error"),
    };

    let program_state_after_execution = interpreter::run(program_string.as_str());
    let retcode: i32 = match program_state_after_execution.status {
        interpreter::ExecutionStatus::Terminated => {
            if opts.is_present(VERBOSE_ARG) {
                eprintln!("bf: terminated without errors");
            };
            0
        }
        interpreter::ExecutionStatus::Error(err) => {
            eprintln!("bf: exited with error: {}", err);
            1
        }
        _ => panic!("bf: internal error"),
    };

    std::process::exit(retcode);
}
