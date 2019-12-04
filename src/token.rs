//! (De)serialization tools for BrainF\*ck tokens.

use std::fmt;


/// All valid `bfi` program commands.
#[derive(Debug, Copy, Clone, PartialEq)]
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


impl Token {
    /// Transform a `Token` into its corresponding character.
    pub fn encode(t: Token) -> char {
        match t {
            Token::PtrInc => '>',
            Token::PtrDec => '<',
            Token::ValInc => '+',
            Token::ValDec => '-',
            Token::PutChar => '.',
            Token::GetChar => ',',
            Token::LoopBeg => '[',
            Token::LoopEnd => ']',
            Token::DebugDump => '#',
            Token::DebugBreakpoint => '%',
        }
    }

    /// Transform a character into a `Token`, returning the resulting `Token` member if the
    /// character is a valid command, otherwise returning the `Err`-wrapped unsupported character.
    pub fn decode(c: char) -> Result<Token, char> {
        match c {
            '>' => Ok(Token::PtrInc),
            '<' => Ok(Token::PtrDec),
            '+' => Ok(Token::ValInc),
            '-' => Ok(Token::ValDec),
            '.' => Ok(Token::PutChar),
            ',' => Ok(Token::GetChar),
            '[' => Ok(Token::LoopBeg),
            ']' => Ok(Token::LoopEnd),
            '#' => Ok(Token::DebugDump),
            '%' => Ok(Token::DebugBreakpoint),
            other => Err(other),
        }
    }

    /// Associated method to parse a `&str` into a `Vec<Token>`. Ignores any provided characters
    /// that do not yield valid `Token`s.
    pub fn parse_str(s: &str) -> Vec<Self> {
        s.chars().filter_map(|c| Token::decode(c).ok()).collect()
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Token::encode(*self))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const SYMBOLS: &str = "><+-.,[]#%";
    const TOKENS: [Token; 10] = [
        Token::PtrInc,
        Token::PtrDec,
        Token::ValInc,
        Token::ValDec,
        Token::PutChar,
        Token::GetChar,
        Token::LoopBeg,
        Token::LoopEnd,
        Token::DebugDump,
        Token::DebugBreakpoint,
    ];

    #[test]
    fn decoding() {
        for (c, &t) in SYMBOLS.chars().zip(TOKENS.into_iter()) {
            let decoded: Result<Token, char> = Token::decode(c);
            assert_eq!(decoded.is_ok(), true);
            assert_eq!(decoded.unwrap(), t);
        }
    }

    #[test]
    fn encoding() {
        for (c, &t) in SYMBOLS.chars().zip(TOKENS.into_iter()) {
            assert_eq!(Token::encode(t), c);
        }
    }
}
