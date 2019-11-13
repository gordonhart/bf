use std::env;

mod token;
mod interpreter;
mod repl;

fn main() {
    let err_str: &'static str = "usage: bf <program>";
    let args: Vec<String> = env::args().collect();
    // assert_eq!(args.len(), 2);
    std::process::exit(match &args[..] {
        [_, program] => {
            let state = interpreter::run(program);
            match state.status {
                interpreter::ExecutionStatus::Terminated => {
                    eprintln!("bf interpreter terminated: 0");
                    0
                },
                interpreter::ExecutionStatus::Error(err) => {
                    eprintln!("bf interpreter exited with error: {}", err);
                    1
                },
                _ => {
                    eprintln!("{:?}", state);
                    1
                },
            }
        },
        [_] => {
            eprintln!("{}", err_str);
            0
        },
        _ => {
            eprintln!("{}", err_str);
            0
        },
    });
}
