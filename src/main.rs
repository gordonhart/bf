use std::env;

mod bf;

fn main() {
    let err_str = "usage: bf <program>";
    let args: Vec<String> = env::args().collect();
    // assert_eq!(args.len(), 2);
    match &args[..] {
        [_, program] => match bf::run_interpreter(program) {
            Ok(message) => println!("bf interpreter exiting: {}", message),
            Err(message) => eprintln!("bf interpreter exiting: {}", message),
        },
        [_] => eprintln!("missing program\n{}", err_str),
        _ => eprintln!("too many arguments\n{}", err_str),
    }
}
