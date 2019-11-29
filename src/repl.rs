extern crate rustyline;

use std::iter::Iterator;

use rustyline::Editor;

use crate::token::Token;


pub struct ReplInstance {
    editor: Editor<()>,
    queue: Vec<Token>,
}


impl ReplInstance {
    pub fn new() -> Self {
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
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() == 0 {
            let input_line = self.editor.readline("bfi $ ");
            match input_line {
                // Ok(line) if line == "q" => None,
                Ok(line) if line == "c" => None,
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str());
                    self.queue.extend(Token::parse_str(line.as_str()).iter());
                    self.next()
                },
                // Err(e) => REPLResult::Error(format!("{}", e)),
                Err(e) => None,
            }
        } else {
            Some(self.queue.remove(0))
        }
    }
}
