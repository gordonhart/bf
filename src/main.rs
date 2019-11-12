use std::env;

mod bf;

fn main() {
    let err_str: &'static str = "usage: bf <program>";
    let args: Vec<String> = env::args().collect();
    // assert_eq!(args.len(), 2);
    std::process::exit(match &args[..] {
        [_, program] => {
            let state = bf::run_interpreter(program);
            match state.status {
                bf::ExecutionStatus::Terminated => {
                    eprintln!("bf interpreter terminated: 0");
                    0
                },
                bf::ExecutionStatus::Error(err) => {
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
