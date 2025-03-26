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
    fn get_quote_str(src: &str) -> Option<(Token, usize)> {
        debug_assert_eq!(src.get(0..1), Some("\""));
        let i = src[1..].find('"')?;
        Some((Token::Literal(&src[1..i + 1]), i + 2))
    }
    fn get_brace_str(src: &str) -> Option<(Token, usize)> {
        assert_eq!(src.get(0..1), Some("{"));
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
// TODO: use iterators instead
pub fn lex(src: &str) -> Result<Vec<Token>, LexerError> {
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < src.len() {
        match src.chars().next() {
            Some('{') => {
                let (token, len) = Token::get_brace_str(&src[i..])
                    .ok_or(LexerError::UnclosedLiteral(&src[i..]))?;
                tokens.push(token);
                i += len;
            }
            Some('"') => {
                let (token, len) = Token::get_quote_str(&src[i..])
                    .ok_or(LexerError::UnclosedLiteral(&src[i..]))?;
                tokens.push(token);
                i += len;
            }
            Some(item) => {
                i += 1;
                if item.is_whitespace() {
                    continue;
                }
                match Token::get_symbol(item) {
                    Some(token) => tokens.push(token),
                    None => return Err(LexerError::UnknownSymbol(item)),
                }
            }
            None => {
                i += 1;
            }
        }
    }
    Ok(tokens)
}
pub enum LexerError<'a> {
    UnclosedLiteral(&'a str),
    UnknownSymbol(char),
}
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
