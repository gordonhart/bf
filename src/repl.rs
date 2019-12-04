extern crate rustyline;

use std::default::Default;
use std::iter::Iterator;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use crate::token::Token;


pub enum ReplResult<T> {
    Command(Token),
    // Continue,
    Quit,
    Error(T),
}


pub struct ReplInstance {
    editor: Editor<()>,
    queue: Vec<Token>,
}


impl Default for ReplInstance {
    fn default() -> Self {
        println!(
            "\
You have entered an interactive session. All regular commands are available.

Commands:
    'c' : Continue execution at the command following this breakpoint
    'q' : Exit interpreter
"
        );
        Self {
            editor: Editor::<()>::new(),
            queue: Vec::new(),
        }
    }
}

impl Iterator for ReplInstance {
    type Item = ReplResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            let input_line = self.editor.readline("bfi $ ");
            match input_line {
                // TODO: merge these two arms?
                Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => Some(ReplResult::Quit),
                Ok(line) if line == "q" => Some(ReplResult::Quit),
                // exits cleanly out of the REPL by ending iteration
                Ok(line) if line == "c" => None,
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str());
                    self.queue.extend(Token::parse_str(line.as_str()).iter());
                    self.next()
                },
                Err(e) => Some(ReplResult::Error(format!("{}", e))),
            }
        } else {
            Some(ReplResult::Command(self.queue.remove(0)))
        }
    }
}
