use compact_str::CompactString;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Semicolon,
    Comma,
    Equal,
    EqEq,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,

    If,
    Then,
    Else,
    Def,
    Let,
    In,

    Number(f64),
    String(CompactString),
    Identifier(CompactString),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
