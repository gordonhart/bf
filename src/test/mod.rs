use crate::token::Token;

#[test]
fn test_mapping() {
    let source: Vec<char> = vec!['>', '<', '+', '-', '.', ',', '[', ']', '?', '!'];
    let target: Vec<Token> = vec![
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
    for (src, tgt) in source.into_iter().zip(target.into_iter()) {
        let decoded: Result<Token, String> = Token::decode(src);
        assert_eq!(decoded.is_ok(), true);
        assert_eq!(decoded.unwrap(), tgt);
    }
}
