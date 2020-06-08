enum Token<'a> {
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
