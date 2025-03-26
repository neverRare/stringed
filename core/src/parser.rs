use crate::{gen_interpreter::OpCode, lexer::Lexer};

struct Parser<'a> {
    lexer: Lexer<'a>,
}
impl<'a> Iterator for Parser<'a> {
    type Item = OpCode;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
