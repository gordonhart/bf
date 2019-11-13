use std::io::Write;

use crate::token::Token;
use crate::interpreter;

pub fn run(state: &mut interpreter::State) {
    // don't quite have this working yet
    // TODO: de-uglify -- this needs to be thought through
    let prompt: &'static str = "$ ";
    println!(
"You have entered an interactive session. All regular commands are available.
Enter 'q' to exit the session and resume program execution.");
    let mut buffer = String::new();
    'repl: loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        buffer.retain(|_| false); // empty buffer
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let mut new_program: Vec<Token> = Vec::new();
                let prev_program_ptr = state.program_ptr;
                state.program_ptr = 0;
                for c in buffer.trim().chars() {
                    if c == 'q' {
                        for command in &new_program {
                            interpreter::run_command(state, &command, &new_program);
                        }
                        state.program_ptr = prev_program_ptr;
                        break 'repl;
                    } else {
                        match Token::decode(c) {
                            Ok(command) => new_program.push(command),
                            Err(err) => eprintln!("{}", err),
                        };
                    };
                };
                for command in &new_program {
                    interpreter::run_command(state, &command, &new_program);
                }
                state.program_ptr = prev_program_ptr;
            },
            Err(_) => println!("invalid input"),
        }
    }
}
