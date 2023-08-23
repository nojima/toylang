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
