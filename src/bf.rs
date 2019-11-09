use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    PtrInc,
    PtrDec,
    ValInc,
    ValDec,
    Input,
    Output,
    LoopBegin,
    LoopEnd,
}


// pub fn decode_action(action: Action) -> &'static str {
pub fn decode_action(action: Action) -> char {
    match action {
        Action::PtrInc => '>',
        Action::PtrDec => '<',
        Action::ValInc => '+',
        Action::ValDec => '-',
        Action::Input => ',',
        Action::Output => '.',
        Action::LoopBegin => '[',
        Action::LoopEnd => ']',
    }
}

pub fn encode_action(c: char) -> Result<Action, &'static str> {
    match c {
        '>' => Ok(Action::PtrInc),
        '<' => Ok(Action::PtrDec),
        '+' => Ok(Action::ValInc),
        '-' => Ok(Action::ValDec),
        ',' => Ok(Action::Input),
        '.' => Ok(Action::Output),
        '[' => Ok(Action::LoopBegin),
        ']' => Ok(Action::LoopEnd),
        _ => Err("unrecognized"),
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f, "{}", decode_action(*self))
    }
}
