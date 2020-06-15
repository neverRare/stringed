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
    pub fn describe(&self) -> &str {
        match self {
            Token::Literal(_) => "literal",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::QuestionMark => "?",
            Token::Underscore => "_",
            Token::Colon => ":",
            Token::Plus => "+",
            Token::OpenBracket => "[",
            Token::CloseBracket => "]",
            Token::Equal => "=",
            Token::Hash => "#",
            Token::Dollar => "$",
        }
    }
}
fn get_symbol(src: &str) -> Option<Token> {
    assert_eq!(src.len(), 1);
    Some(match src {
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "?" => Token::QuestionMark,
        "_" => Token::Underscore,
        ":" => Token::Colon,
        "+" => Token::Plus,
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,
        "=" => Token::Equal,
        "#" => Token::Hash,
        "$" => Token::Dollar,
        _ => return None,
    })
}
#[derive(Eq, PartialEq, Debug)]
struct SizedToken<'a> {
    token: Token<'a>,
    len: usize,
}
fn get_quote_str(src: &str) -> Result<SizedToken, &str> {
    assert_eq!(src.get(0..1), Some("\""));
    for i in 1..src.len() {
        if let Some("\"") = src.get(i..i + 1) {
            return Ok(SizedToken {
                token: Token::Literal(&src[1..i]),
                len: i + 1,
            });
        }
    }
    Err("invalid literal")
}
fn get_brace_str(src: &str) -> Result<SizedToken, &str> {
    assert_eq!(src.get(0..1), Some("{"));
    let mut count: usize = 1;
    for i in 1..src.len() {
        match src.get(i..i + 1) {
            Some("{") => count += 1,
            Some("}") => {
                count -= 1;
                if count == 0 {
                    return Ok(SizedToken {
                        token: Token::Literal(&src[1..i]),
                        len: i + 1,
                    });
                }
            }
            _ => (),
        }
    }
    Err("invalid literal")
}
pub fn lex(src: &str) -> Result<Vec<Token>, &str> {
    if src.is_empty() {
        Err("code can't be empty")
    } else {
        let mut i = 0;
        let mut tokens = Vec::new();
        while i < src.len() {
            match src.get(i..i + 1) {
                Some("{") => {
                    let token = get_brace_str(&src[i..])?;
                    tokens.push(token.token);
                    i += token.len;
                }
                Some("\"") => {
                    let token = get_quote_str(&src[i..])?;
                    tokens.push(token.token);
                    i += token.len;
                }
                Some(item) => {
                    i += 1;
                    if item == " " || item == "\t" || item == "\n" || item == "\r" {
                        continue;
                    }
                    match get_symbol(item) {
                        Some(token) => tokens.push(token),
                        None => return Err("unidentified char"),
                    }
                }
                None => {
                    i += 1;
                }
            }
        }
        Ok(tokens)
    }
}
#[cfg(test)]
mod lexer_test {
    #[test]
    fn get_quote_str() {
        use super::*;
        assert_eq!(
            get_quote_str("\"in\"out"),
            Ok(SizedToken {
                token: Token::Literal("in"),
                len: 4,
            }),
        );
    }
    #[test]
    fn get_brace_str() {
        use super::*;
        assert_eq!(
            get_brace_str("{{}{{}}}out"),
            Ok(SizedToken {
                token: Token::Literal("{}{{}}"),
                len: 8,
            }),
        );
    }
}
