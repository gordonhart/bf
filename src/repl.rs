extern crate rustyline;

use rustyline::{Editor, error::ReadlineError};

use crate::token::Token;
use crate::interpreter;

static HISTORY_FILE: &'static str = ".bf_history";

pub fn run(state: &mut interpreter::State) {
    let mut rl = Editor::<()>::new();

    println!("\
You have entered an interactive session. All regular commands are available.

Commands:
    'c' : Continue execution at the command following this breakpoint
    'q' : Exit interpreter
");

    'repl: loop {
        let input_line = rl.readline("bf $ ");
        match input_line {
            Ok(line) if line == "q" => {
                state.status = interpreter::ExecutionStatus::Terminated;
                break 'repl;
            },
            Ok(line) if line == "c" => break 'repl,
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let new_program = interpreter::parse_program(line.as_str());
                match new_program {
                    Ok(program) => {
                        let prev_program_ptr = state.program_ptr;
                        state.program_ptr = 0;
                        interpreter::run_program(state, &program);
                        state.program_ptr = prev_program_ptr;
                    },
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }

    state.program_ptr += 1;
}
