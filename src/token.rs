use std::fmt::{Display, Formatter, self};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Semicolon,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,

    Let,
    In,

    Number(f64),
    Identifier(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
