extern crate rustyline;

use rustyline::Editor;

// TODO: probably shouldn't use the word 'Result' here
pub enum REPLResult<T> {
    Break,
    Terminate,
    Program(String),
    Error(T),
}

#[derive(Debug)]
pub struct Instance {
    editor: Editor<()>,
}

impl Instance {
    pub fn new() -> Self {
        println!(
            "\
    You have entered an interactive session. All regular commands are available.

    Commands:
        'c' : Continue execution at the command following this breakpoint
        'q' : Exit interpreter
    "
        );

        Instance { editor: Editor::<()>::new() }
    }

    pub fn get(&mut self) -> REPLResult<String> {
        let input_line = self.editor.readline("bfi $ ");
        match input_line {
            Ok(line) if line == "q" => REPLResult::Terminate,
            Ok(line) if line == "c" => REPLResult::Break,
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                REPLResult::Program(line)
            },
            Err(e) => REPLResult::Error(format!("{}", e)),
        }
    }
}
