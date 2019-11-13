use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    PtrInc,
    PtrDec,
    ValInc,
    ValDec,
    PutChar,
    GetChar,
    LoopBeg,
    LoopEnd,
    DebugDump,
    DebugBreakpoint,
}

pub fn encode_token(t: Token) -> char {
    match t {
        Token::PtrInc => '>',
        Token::PtrDec => '<',
        Token::ValInc => '+',
        Token::ValDec => '-',
        Token::PutChar => '.',
        Token::GetChar => ',',
        Token::LoopBeg => '[',
        Token::LoopEnd => ']',
        Token::DebugDump => '?',
        Token::DebugBreakpoint => '!',
    }
}

pub fn decode_token(c: char) -> Result<Token, String> {
    match c {
        '>' => Ok(Token::PtrInc),
        '<' => Ok(Token::PtrDec),
        '+' => Ok(Token::ValInc),
        '-' => Ok(Token::ValDec),
        '.' => Ok(Token::PutChar),
        ',' => Ok(Token::GetChar),
        '[' => Ok(Token::LoopBeg),
        ']' => Ok(Token::LoopEnd),
        '?' => Ok(Token::DebugDump),
        '!' => Ok(Token::DebugBreakpoint),
        other => Err(format!("unsupported character: {}", other)),
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f, "{}", encode_token(*self))
    }
}
