use std::{error, fmt::Display};

use crate::{
    lexer::{self, Lexer},
    state::OpCode,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}
impl<'a> Parser<'a> {
    pub fn exec(src: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(src),
        }
    }
    pub fn eval(src: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(src),
        }
    }
}
impl<'a> Iterator for Parser<'a> {
    type Item = Result<OpCode, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.lexer.next()?;
        let next = match next {
            Ok(next) => next,
            Err(error) => return Some(Err(error.into())),
        };
        match next {
            lexer::Token::Literal(_) => todo!(),
            lexer::Token::OpenParen => todo!(),
            lexer::Token::CloseParen => todo!(),
            lexer::Token::QuestionMark => todo!(),
            lexer::Token::Underscore => todo!(),
            lexer::Token::Colon => todo!(),
            lexer::Token::Plus => todo!(),
            lexer::Token::OpenBracket => todo!(),
            lexer::Token::CloseBracket => todo!(),
            lexer::Token::Equal => todo!(),
            lexer::Token::Hash => todo!(),
            lexer::Token::Dollar => todo!(),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    LexerError(lexer::Error),
}
impl From<lexer::Error> for Error {
    fn from(value: lexer::Error) -> Self {
        Error::LexerError(value)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LexerError(error) => write!(f, "{}", error)?,
        }
        Ok(())
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::LexerError(error) => Some(error),
        }
    }
}
