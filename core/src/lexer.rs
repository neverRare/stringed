use std::{error, fmt::Display};

#[derive(Eq, PartialEq, Debug)]
pub enum Token<'a> {
    Literal(&'a str),
    OpenParen,
    CloseParen,
    QuestionMark,
    Underscore,
    Colon,
    Plus,
    OpenBracket,
    CloseBracket,
    Equal,
    Hash,
    Dollar,
}
impl<'a> Token<'a> {
    fn get_symbol(src: char) -> Option<Self> {
        Some(match src {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '?' => Token::QuestionMark,
            '_' => Token::Underscore,
            ':' => Token::Colon,
            '+' => Token::Plus,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '=' => Token::Equal,
            '#' => Token::Hash,
            '$' => Token::Dollar,
            _ => return None,
        })
    }
    fn get_quote_str(src: &'a str) -> Option<(Self, usize)> {
        debug_assert_eq!(src.get(0..1), Some("\""));
        let i = src[1..].find('"')?;
        Some((Token::Literal(&src[1..i + 1]), i + 2))
    }
    fn get_brace_str(src: &'a str) -> Option<(Self, usize)> {
        debug_assert_eq!(src.get(0..1), Some("{"));
        let mut count: usize = 1;
        for (i, ch) in src[1..].char_indices() {
            match ch {
                '{' => count += 1,
                '}' => {
                    count -= 1;
                    if count == 0 {
                        return Some((Token::Literal(&src[1..i + 1]), i + 2));
                    }
                }
                _ => (),
            }
        }
        None
    }
}
pub struct Lexer<'a>(&'a str);
impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer(src)
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let src = self.0.trim_start();
        let ch = src.chars().next()?;
        match ch {
            '{' => {
                let (token, len) = match Token::get_brace_str(&src[1..]) {
                    Some(result) => result,
                    None => return Some(Err(Error::UnclosedLiteral)),
                };
                self.0 = &src[len..];
                Some(Ok(token))
            }
            '"' => {
                let (token, len) = match Token::get_quote_str(&src[1..]) {
                    Some(result) => result,
                    None => return Some(Err(Error::UnclosedLiteral)),
                };
                self.0 = &src[len..];
                Some(Ok(token))
            }
            item => match Token::get_symbol(item) {
                Some(token) => Some(Ok(token)),
                None => return Some(Err(Error::UnknownSymbol(item))),
            },
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnclosedLiteral,
    UnknownSymbol(char),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnclosedLiteral => write!(f, "unclosed literal")?,
            Error::UnknownSymbol(ch) => write!(f, "unknown symbol {}", ch)?,
        }
        Ok(())
    }
}
impl error::Error for Error {}
#[cfg(test)]
mod lexer_test {
    #[test]
    fn get_quote_str() {
        use super::*;
        assert_eq!(
            Token::get_quote_str("\"in\"out"),
            Some((Token::Literal("in"), 4)),
        );
    }
    #[test]
    fn get_brace_str() {
        use super::*;
        assert_eq!(
            Token::get_brace_str("{{}{{}}}out"),
            Some((Token::Literal("{}{{}}"), 8)),
        );
    }
}
