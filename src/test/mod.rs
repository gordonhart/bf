mod interpreter;

use crate::token::Token;

const CHARS: [char; 10] = ['>', '<', '+', '-', '.', ',', '[', ']', '?', '!'];
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
    for (&c, &t) in CHARS.into_iter().zip(TOKENS.into_iter()) {
        let decoded: Result<Token, String> = Token::decode(c);
        assert_eq!(decoded.is_ok(), true);
        assert_eq!(decoded.unwrap(), t);
    }
}

#[test]
fn encoding() {
    for (&c, &t) in CHARS.into_iter().zip(TOKENS.into_iter()) {
        assert_eq!(Token::encode(t), c);
    }
}
