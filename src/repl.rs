extern crate rustyline;

use rustyline::Editor;

use crate::interpreter;

pub fn run(context: &mut interpreter::ExecutionContext) {
    let mut rl = Editor::<()>::new();

    println!(
        "\
You have entered an interactive session. All regular commands are available.

Commands:
    'c' : Continue execution at the command following this breakpoint
    'q' : Exit interpreter
"
    );

    'repl: loop {
        let input_line = rl.readline("bfi $ ").expect("bfi: unable to read input");
        if input_line == "q" {
            context.status = interpreter::ExecutionStatus::Terminated;
            break 'repl;
        } else if input_line == "c" {
            break 'repl;
        } else {
            rl.add_history_entry(input_line.as_str());
            let new_program = interpreter::parse_program(input_line.as_str());
            match new_program {
                Ok(program) => {
                    let prev_program_ptr = context.program_ptr;
                    let prev_execution_status = context.status.clone();
                    context.program_ptr = 0;
                    interpreter::run_program(context, &program);
                    context.program_ptr = prev_program_ptr;
                    context.status = prev_execution_status;
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
