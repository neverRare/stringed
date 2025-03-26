use std::{error, fmt::Display};

use crate::{gen_interpreter::OpCode, lexer::Lexer};

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
        todo!()
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl error::Error for Error {}
