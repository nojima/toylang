use crate::token::Token;
use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LexicalError {
    #[error("unexpected character: '{0}'")]
    UnexpectedCharacter(char),

    #[error("unexpected end of file")]
    UnexpectedEndOfFile,

    #[error("undefined escape: '\\{0}'")]
    UndefinedEscape(char),
}

// Success: Ok(Some((token, bytes_consumed)))
// Failure: Err(LexicalError)
// EOF:     Ok(None)
type LexResult = Result<Option<(Token, usize)>, LexicalError>;

fn ok(token: Token, bytes_consumed: usize) -> LexResult {
    Ok(Some((token, bytes_consumed)))
}

fn err(e: LexicalError) -> LexResult {
    Err(e)
}

fn eof() -> LexResult {
    Ok(None)
}

macro_rules! static_regex {
    ($pattern:expr) => {{
        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($pattern).unwrap())
    }};
}

// Cuts a single token from `input` and returns `(token, bytes_consumed)`.
fn lex(input: &str) -> LexResult {
    let Some(first) = input.chars().next() else {
        return eof();
    };
    match first {
        ';' => return ok(Token::Semicolon, 1),
        '=' => return ok(Token::Equal, 1),
        '+' => return ok(Token::Plus, 1),
        '-' => return ok(Token::Minus, 1),
        '*' => return ok(Token::Asterisk, 1),
        '/' => return ok(Token::Slash, 1),
        '(' => return ok(Token::LParen, 1),
        ')' => return ok(Token::RParen, 1),
        _ => {}
    }

    let re_identifier_or_reserved = static_regex!("^[a-zA-Z_][a-zA-Z0-9_]*");
    if let Some(m) = re_identifier_or_reserved.find(input) {
        let s = m.as_str();
        let token = match s {
            "let" => Token::Let,
            "in" => Token::In,
            _ => Token::Identifier(s.into()),
        };
        return ok(token, m.end());
    }

    #[rustfmt::skip]
    let re_number = static_regex!(r"(?x)^
        (0|[1-9][0-9]*)     # integer
        ([.][0-9]+)?        # fraction
        ([eE][-+]?[0-9]+)?  # exponent
    ");
    if let Some(m) = re_number.find(input) {
        let n = f64::from_str(m.as_str()).unwrap();
        return ok(Token::Number(n), m.end());
    }

    if input.starts_with('"') {
        return lex_string_literal(input);
    }

    err(LexicalError::UnexpectedCharacter(first))
}

fn lex_string_literal(input: &str) -> LexResult {
    let mut chars = input.chars();
    assert_eq!(chars.next(), Some('"'));

    let mut buffer = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let Some(c2) = chars.next() else {
                    return Err(LexicalError::UnexpectedEndOfFile);
                };
                match c2 {
                    '"' => buffer.push('"'),
                    '\\' => buffer.push('\\'),
                    '/' => buffer.push('/'),
                    'n' => buffer.push('\n'),
                    'r' => buffer.push('\r'),
                    't' => buffer.push('\t'),
                    _ => return Err(LexicalError::UndefinedEscape(c2)),
                }
            }
            '"' => break,
            _ => buffer.push(c),
        }
    }

    let bytes_consumed = input.len() - chars.as_str().len();
    ok(Token::String(buffer.into()), bytes_consumed)
}

// Same as `lex` except that it ignores leading whitespaces.
fn lex_strip(input: &str) -> LexResult {
    let re_whitespaces = static_regex!(r"^[\t\n\r ]+");
    match re_whitespaces.find(input) {
        None => lex(input),
        Some(m) => {
            let r = lex(&input[m.end()..]);
            match r {
                Ok(Some((token, bytes_consumed))) => ok(token, m.end() + bytes_consumed),
                _ => r,
            }
        }
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    bytes_consumed: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            bytes_consumed: 0,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        match lex_strip(&self.input[self.bytes_consumed..]) {
            // Success
            Ok(Some((token, bytes_consumed))) => {
                let span_start = self.bytes_consumed;
                let span_end = self.bytes_consumed + bytes_consumed;
                self.bytes_consumed = span_end;
                Some(Ok((span_start, token, span_end)))
            }
            // Failure
            Err(e) => Some(Err(e)),
            // EOF
            Ok(None) => None,
        }
    }
}
