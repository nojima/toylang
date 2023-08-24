use std::fmt::{self, Display, Formatter};

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
    String(String),
    Identifier(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
