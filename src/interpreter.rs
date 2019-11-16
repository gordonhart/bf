use std::io::Read;

use crate::buffer::Buffer;
use crate::repl;
use crate::token::Token;

// TODO: figure out why these derive macros fail with E0495:
// cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
// #[derive(Debug, PartialEq)]
pub struct ExecutionContext<'a> {
    pub data: Vec<u8>,
    pub data_ptr: usize,
    pub program_ptr: usize,
    pub loop_stack: Vec<usize>,
    pub status: ExecutionStatus<String>,
    pub buffer: &'a mut dyn Buffer,
}

impl<'a> std::fmt::Debug for ExecutionContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>11} {:?}\n", "data", self.data)?;
        write!(f, "{:>11} {:?}\n", "data_ptr", self.data_ptr)?;
        write!(f, "{:>11} {:?}\n", "program_ptr", self.program_ptr)?;
        write!(f, "{:>11} {:?}\n", "loop_stack", self.loop_stack)?;
        write!(f, "{:>11} {:?}", "status", self.status)
    }
}

impl<'a> ExecutionContext<'a> {
    pub fn new<'b>(buffer: &'b mut dyn Buffer) -> ExecutionContext<'b> {
        ExecutionContext {
            data: vec![0],
            data_ptr: 0,
            program_ptr: 0,
            loop_stack: vec![],
            status: ExecutionStatus::NotStarted,
            buffer: buffer,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExecutionStatus<T> {
    NotStarted,
    InProgress,
    Terminated,
    Error(T),
}

pub fn run<'a>(program: &str, buffer: &'a mut dyn Buffer) -> ExecutionContext<'a> {
    let mut context = ExecutionContext::new(buffer);
    match parse_program(program) {
        Ok(parsed_program) => run_program(&mut context, &parsed_program),
        Err(err) => context.status = ExecutionStatus::Error(err),
    };
    context
}

pub fn parse_program(program: &str) -> Result<Vec<Token>, String> {
    program
        .chars()
        .map(|c| Token::decode(c))
        .filter(|t_res| t_res.is_ok())
        .collect()
}

pub fn run_program(context: &mut ExecutionContext, program: &Vec<Token>) {
    context.status = ExecutionStatus::InProgress;
    loop {
        match context.status {
            ExecutionStatus::Terminated | ExecutionStatus::Error(_) => break,
            _ => {}
        };
        match program.get(context.program_ptr) {
            Some(command) => run_command(context, &command, program),
            None => {
                context.status = ExecutionStatus::Terminated;
                break;
            },
        };
    }
}

pub fn run_command(context: &mut ExecutionContext, command: &Token, program: &Vec<Token>) {
    match command {
        Token::PtrInc => pointer_increment(context),
        Token::PtrDec => pointer_decrement(context),
        Token::ValInc => value_increment(context),
        Token::ValDec => value_decrement(context),
        Token::PutChar => context.buffer.put_byte(context.data[context.data_ptr]),
        Token::GetChar => get_character(context),
        Token::LoopBeg => loop_enter(context, program),
        Token::LoopEnd => loop_exit(context),
        Token::DebugDump => eprintln!("{:?}", context),
        Token::DebugBreakpoint => repl::run(context),
    };
    match command {
        Token::LoopEnd => {} // special case that sets the program pointer itself
        _ => context.program_ptr += 1,
    };
}

fn pointer_increment(context: &mut ExecutionContext) {
    context.data_ptr += 1;
    match context.data.get(context.data_ptr) {
        Some(_) => {}
        None => context.data.push(0),
    }
}

fn pointer_decrement(context: &mut ExecutionContext) {
    match context.data_ptr {
        0 => context.data.insert(0, 0),
        _ => context.data_ptr -= 1,
    }
}

fn value_increment(context: &mut ExecutionContext) {
    match context.data[context.data_ptr].overflowing_add(1) {
        (v, _) => context.data[context.data_ptr] = v,
    }
}

fn value_decrement(context: &mut ExecutionContext) {
    match context.data[context.data_ptr].overflowing_sub(1) {
        (v, _) => context.data[context.data_ptr] = v,
    }
}

fn get_character(context: &mut ExecutionContext) {
    match std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
    {
        Some(c) => context.data[context.data_ptr] = c,
        None => context.status = ExecutionStatus::Terminated, // EOF
    }
}

fn find_loop_end(ptr: usize, program: &Vec<Token>) -> Result<usize, ()> {
    match program.get(ptr) {
        Some(Token::LoopEnd) => Ok(ptr),
        Some(Token::LoopBeg) => {
            find_loop_end(ptr + 1, program).and_then(|i| find_loop_end(i + 1, program))
        }
        Some(_) => find_loop_end(ptr + 1, program),
        None => Err(()),
    }
}

fn loop_enter(context: &mut ExecutionContext, program: &Vec<Token>) {
    if context.data[context.data_ptr] == 0 {
        match find_loop_end(context.program_ptr + 1, program) {
            Ok(i) => context.program_ptr = i,
            Err(_) => {
                context.status = ExecutionStatus::Error("'[' missing corresponding ']'".to_string())
            }
        };
    } else {
       context.loop_stack.push(context.program_ptr);
    }
}

fn loop_exit(context: &mut ExecutionContext) {
    match (context.loop_stack.pop(), context.data[context.data_ptr]) {
        (Some(_), 0) => context.program_ptr += 1,
        (Some(ptr_loc), _) => context.program_ptr = ptr_loc,
        (None, _) => {
            context.status = ExecutionStatus::Error("']' missing corresponding '['".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::buffer::ASCIICharBuffer;

    #[test]
    fn test_pointer_increment() {
        let mut buffer = ASCIICharBuffer {};
        let mut context = ExecutionContext::new(&mut buffer);
        pointer_increment(&mut context);
        assert_eq!(1, context.data_ptr);
        assert_eq!(vec![0, 0], context.data);
    }

    #[test]
    fn test_pointer_decrement() {
        let mut buffer = ASCIICharBuffer {};
        let mut context = ExecutionContext::new(&mut buffer);
        pointer_decrement(&mut context);
        assert_eq!(0, context.data_ptr);
        assert_eq!(vec![0, 0], context.data);
    }

    #[test]
    fn test_value_increment() {
        let mut buffer = ASCIICharBuffer {};
        let mut context = ExecutionContext::new(&mut buffer);
        value_increment(&mut context);
        assert_eq!(1, context.data[context.data_ptr]);
    }

    #[test]
    fn test_value_increment_with_overflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut context = ExecutionContext::new(&mut buffer);
        context.data[context.data_ptr] = 255;
        value_increment(&mut context);
        assert_eq!(0, context.data[context.data_ptr]);
    }

    #[test]
    fn test_value_decrement_with_underflow() {
        let mut buffer = ASCIICharBuffer {};
        let mut context = ExecutionContext::new(&mut buffer);
        value_decrement(&mut context);
        assert_eq!(255, context.data[context.data_ptr]);
    }

    #[test]
    fn test_find_loop_end() {
        let program = vec![Token::PtrInc, Token::LoopEnd];
        assert_eq!(Ok(1), find_loop_end(0, &program));
    }
}
