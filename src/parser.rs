use crate::lexer::{lex, Token};
macro_rules! expect {
    ($expected:expr, $found:expr $(,)?) => {
        format!("expected {}, found {}", $expected, $found)
    };
}
macro_rules! unexpect {
    ($unexpected:expr $(,)?) => {
        format!("unexpected {}", $unexpected)
    };
}
#[derive(Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Literal(&'a str),
    Group(Box<Self>),
    Prompt,
    Var,
    Closure {
        left: Box<Self>,
        right: Box<Self>,
    },
    Concat(Vec<Self>),
    Slice {
        src: Box<Self>,
        lower: Option<Box<Self>>,
        upper: Option<Box<Self>>,
    },
    Equal {
        left: Box<Self>,
        right: Box<Self>,
    },
    Length(Box<Self>),
    Eval(Box<Self>),
}
impl<'a> Node<'a> {
    fn precedence(&self) -> u8 {
        match self {
            Self::Literal(_) => 7,
            Self::Prompt => 7,
            Self::Var => 7,
            Self::Group(_) => 6,
            Self::Length(_) => 5,
            Self::Slice { .. } => 4,
            Self::Concat(_) => 3,
            Self::Equal { .. } => 2,
            Self::Closure { .. } => 1,
            Self::Eval(_) => 0,
        }
    }
    pub fn from_tokens(tokens: &[Token<'a>]) -> Result<Self, String> {
        let node = CountedNode::from_tokens(true, &tokens)?;
        if node.count == tokens.len() {
            Ok(node.node)
        } else {
            Err(unexpect!(tokens[node.count].describe()))
        }
    }
    pub fn parse(src: &'a str) -> Result<Self, String> {
        Self::from_tokens(&lex(src)?)
    }
    fn merge_careless(self, partial_node: PartialNode<'a>) -> Self {
        match partial_node {
            PartialNode::Closure(right) => Self::Closure {
                left: Box::new(self),
                right: Box::new(right),
            },
            PartialNode::Concat(right) => Self::Concat(vec![self, right]),
            PartialNode::Slice { lower, upper } => Self::Slice {
                src: Box::new(self),
                lower: lower.map(Box::new),
                upper: upper.map(Box::new),
            },
            PartialNode::Equal(right) => Self::Equal {
                left: Box::new(self),
                right: Box::new(right),
            },
        }
    }
    fn merge(self, partial_node: PartialNode<'a>) -> Self {
        if self.precedence() <= partial_node.precedence() {
            match self {
                Self::Literal(_) => unreachable!(),
                Self::Group(_) => unreachable!(),
                Self::Prompt => unreachable!(),
                Self::Var => unreachable!(),
                Self::Closure { left, right } => Self::Closure {
                    left,
                    right: Box::new(right.merge(partial_node)),
                },
                Self::Concat(mut vec) => {
                    if let PartialNode::Concat(right) = partial_node {
                        vec.push(right);
                    } else {
                        let last = vec.pop().unwrap();
                        vec.push(last.merge(partial_node));
                    }
                    Self::Concat(vec)
                }
                Self::Slice { .. } => {
                    if let PartialNode::Slice { .. } = partial_node {
                        self.merge_careless(partial_node)
                    } else {
                        unreachable!()
                    }
                }
                Self::Equal { left, right } => {
                    if let PartialNode::Equal(another_right) = partial_node {
                        Self::Equal {
                            left: Box::new(Self::Equal { left, right }),
                            right: Box::new(another_right),
                        }
                    } else {
                        Self::Equal {
                            left,
                            right: Box::new(right.merge(partial_node)),
                        }
                    }
                }
                Self::Length(node) => Self::Length(Box::new(node.merge(partial_node))),
                Self::Eval(node) => Self::Eval(Box::new(node.merge(partial_node))),
            }
        } else {
            self.merge_careless(partial_node)
        }
    }
}

struct CountedNode<'a> {
    node: Node<'a>,
    count: usize,
}
impl<'a> CountedNode<'a> {
    fn simple_from_tokens(tokens: &[Token<'a>]) -> Result<Self, String> {
        if tokens.is_empty() {
            Err(expect!("expression", "EOF"))
        } else {
            match &tokens[0] {
                Token::Literal(content) => Ok(Self {
                    node: Node::Literal(content),
                    count: 1,
                }),
                Token::OpenParen => {
                    match Self::delimited_from_tokens(true, &tokens[1..], &Token::CloseParen) {
                        Some(node) => {
                            let node = node?;
                            Ok(Self {
                                node: Node::Group(Box::new(node.node)),
                                count: node.count + 2,
                            })
                        }
                        None => Err(expect!("expression", ")")),
                    }
                }
                Token::QuestionMark => Ok(Self {
                    node: Node::Prompt,
                    count: 1,
                }),
                Token::Underscore => Ok(Self {
                    node: Node::Var,
                    count: 1,
                }),
                Token::Hash => {
                    let node = Self::simple_from_tokens(&tokens[1..])?;
                    Ok(Self {
                        node: Node::Length(Box::new(node.node)),
                        count: node.count + 1,
                    })
                }
                Token::Dollar => {
                    let node = Self::simple_from_tokens(&tokens[1..])?;
                    Ok(Self {
                        node: Node::Eval(Box::new(node.node)),
                        count: node.count + 1,
                    })
                }
                token => Err(expect!("expression", token.describe())),
            }
        }
    }
    fn from_tokens(parse_colon: bool, tokens: &[Token<'a>]) -> Result<Self, String> {
        let node = Self::simple_from_tokens(tokens)?;
        let mut count = node.count;
        let mut node = node.node;
        for partial_node in CountedPartialNodes::new(parse_colon, &tokens[count..]) {
            let partial_node = partial_node?;
            node = node.merge(partial_node.node);
            count += partial_node.count;
        }
        Ok(Self { node, count })
    }
    fn delimited_from_tokens(
        parse_colon: bool,
        tokens: &[Token<'a>],
        closing: &Token,
    ) -> Option<Result<Self, String>> {
        if tokens.is_empty() {
            Some(Err(unexpect!("EOF")))
        } else if &tokens[0] == closing {
            None
        } else {
            match Self::from_tokens(parse_colon, tokens) {
                Ok(node) => {
                    if node.count < tokens.len() {
                        let next = &tokens[node.count];
                        if next == closing {
                            Some(Ok(Self {
                                node: node.node,
                                count: node.count,
                            }))
                        } else {
                            Some(Err(expect!(closing.describe(), next.describe())))
                        }
                    } else {
                        Some(Err(expect!(closing.describe(), "EOF")))
                    }
                }
                Err(reason) => Some(Err(reason)),
            }
        }
    }
}
enum PartialNode<'a> {
    Closure(Node<'a>),
    Concat(Node<'a>),
    Slice {
        lower: Option<Node<'a>>,
        upper: Option<Node<'a>>,
    },
    Equal(Node<'a>),
}
impl<'a> PartialNode<'a> {
    fn precedence(&self) -> u8 {
        match self {
            Self::Slice { .. } => 4,
            Self::Concat(_) => 3,
            Self::Equal(_) => 2,
            Self::Closure(_) => 1,
        }
    }
}
struct CountedPartialNode<'a> {
    node: PartialNode<'a>,
    count: usize,
}
impl<'a> CountedPartialNode<'a> {
    fn simple_from_tokens(parse_colon: bool, tokens: &[Token<'a>]) -> Option<Result<Self, String>> {
        if tokens.is_empty() {
            None
        } else {
            match &tokens[0] {
                Token::Colon => {
                    if parse_colon {
                        match CountedNode::simple_from_tokens(&tokens[1..]) {
                            Ok(node) => Some(Ok(Self {
                                node: PartialNode::Closure(node.node),
                                count: node.count + 1,
                            })),
                            Err(reason) => Some(Err(reason)),
                        }
                    } else {
                        None
                    }
                }
                Token::Plus => match CountedNode::simple_from_tokens(&tokens[1..]) {
                    Ok(node) => Some(Ok(Self {
                        node: PartialNode::Concat(node.node),
                        count: node.count + 1,
                    })),
                    Err(reason) => Some(Err(reason)),
                },
                Token::OpenBracket => {
                    let lower = match CountedNode::delimited_from_tokens(
                        false,
                        &tokens[1..],
                        &Token::Colon,
                    ) {
                        None => None,
                        Some(Ok(node)) => Some(node),
                        Some(Err(reason)) => return Some(Err(reason)),
                    };
                    let upper_index = match &lower {
                        None => 2,
                        Some(node) => node.count,
                    };
                    let upper = match CountedNode::delimited_from_tokens(
                        false,
                        &tokens[upper_index..],
                        &Token::CloseBracket,
                    ) {
                        None => None,
                        Some(Ok(node)) => Some(node),
                        Some(Err(reason)) => return Some(Err(reason)),
                    };
                    Some(Ok(Self {
                        count: 3
                            + match &lower {
                                None => 0,
                                Some(node) => node.count,
                            }
                            + match &upper {
                                None => 0,
                                Some(node) => node.count,
                            },
                        node: PartialNode::Slice {
                            lower: lower.map(|node| node.node),
                            upper: upper.map(|node| node.node),
                        },
                    }))
                }
                Token::Equal => match CountedNode::simple_from_tokens(&tokens[1..]) {
                    Ok(node) => Some(Ok(Self {
                        node: PartialNode::Equal(node.node),
                        count: node.count + 1,
                    })),
                    Err(reason) => Some(Err(reason)),
                },
                _ => None,
            }
        }
    }
}
struct CountedPartialNodes<'a, 'b> {
    parse_colon: bool,
    tokens: &'b [Token<'a>],
    i: usize,
    stopped: bool,
}
impl<'a, 'b> CountedPartialNodes<'a, 'b> {
    fn new(parse_colon: bool, tokens: &'b [Token<'a>]) -> Self {
        Self {
            parse_colon,
            tokens,
            i: 0,
            stopped: false,
        }
    }
}
impl<'a, 'b> Iterator for CountedPartialNodes<'a, 'b> {
    type Item = Result<CountedPartialNode<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            None
        } else {
            let tokens = &self.tokens[self.i..];
            if tokens.is_empty() {
                self.stopped = true;
                None
            } else {
                match CountedPartialNode::simple_from_tokens(self.parse_colon, tokens) {
                    None => {
                        self.stopped = true;
                        None
                    }
                    Some(Err(reason)) => {
                        self.stopped = true;
                        Some(Err(reason))
                    }
                    Some(Ok(node)) => {
                        self.i += node.count;
                        Some(Ok(node))
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod parser_tests {
    use super::Node;
    macro_rules! assert_node {
        ($src:expr, $node:expr $(,)?) => {{
            let src: &str = $src;
            let node: Node = $node;
            assert_eq!(Node::parse(src), Ok(node));
        }};
    }
    #[test]
    fn literal() {
        assert_node!(r#""hello""#, Node::Literal("hello"));
        assert_node!("{hello}", Node::Literal("hello"));
    }
    #[test]
    fn group() {
        assert_node!(
            r#"("hello")"#,
            Node::Group(Box::new(Node::Literal("hello"))),
        );
    }
    #[test]
    fn prompt() {
        assert_node!("?", Node::Prompt);
    }
    #[test]
    fn var() {
        assert_node!("_", Node::Var);
    }
    #[test]
    fn closure() {
        assert_node!(
            r#""test": _"#,
            Node::Closure {
                left: Box::new(Node::Literal("test")),
                right: Box::new(Node::Var),
            },
        );
    }
    #[test]
    fn closure_closure() {
        assert_node!(
            r#""A": "B": "C""#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Closure {
                    left: Box::new(Node::Literal("B")),
                    right: Box::new(Node::Literal("C")),
                }),
            },
        );
    }
    #[test]
    fn concat() {
        assert_node!(
            r#""A" + "B""#,
            Node::Concat(vec![Node::Literal("A"), Node::Literal("B")]),
        );
    }
    #[test]
    fn concat_concat() {
        assert_node!(
            r#""A" + "B" + "C""#,
            Node::Concat(vec![
                Node::Literal("A"),
                Node::Literal("B"),
                Node::Literal("C"),
            ]),
        );
    }
    #[test]
    fn slice() {
        assert_node!(
            r#""A"["B":"C"]"#,
            Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: Some(Box::new(Node::Literal("B"))),
                upper: Some(Box::new(Node::Literal("C"))),
            },
        );
        assert_node!(
            r#""A"["B":]"#,
            Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: Some(Box::new(Node::Literal("B"))),
                upper: None,
            },
        );
        assert_node!(
            r#""A"[:"B"]"#,
            Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: None,
                upper: Some(Box::new(Node::Literal("B"))),
            },
        );
        assert_node!(
            r#""A"[:]"#,
            Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: None,
                upper: None,
            },
        );
    }
    #[test]
    fn slice_slice() {
        assert_node!(
            r#""A"[:][:]"#,
            Node::Slice {
                src: Box::new(Node::Slice {
                    src: Box::new(Node::Literal("A")),
                    lower: None,
                    upper: None,
                }),
                lower: None,
                upper: None,
            },
        );
    }
    #[test]
    fn equal() {
        assert_node!(
            r#""A" = "B""#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Literal("B")),
            },
        );
    }
    #[test]
    fn equal_equal() {
        assert_node!(
            r#""A" = "B" = "C""#,
            Node::Equal {
                left: Box::new(Node::Equal {
                    left: Box::new(Node::Literal("A")),
                    right: Box::new(Node::Literal("B")),
                }),
                right: Box::new(Node::Literal("C")),
            },
        );
    }
    #[test]
    fn length() {
        assert_node!(r#"#"A""#, Node::Length(Box::new(Node::Literal("A"))));
    }
    #[test]
    fn length_length() {
        assert_node!(
            r#"##"A""#,
            Node::Length(Box::new(Node::Length(Box::new(Node::Literal("A"))))),
        );
    }
    #[test]
    fn eval() {
        assert_node!(r#"$ "A""#, Node::Eval(Box::new(Node::Literal("A"))));
    }
    #[test]
    fn eval_eval() {
        assert_node!(
            r#"$ $ "A""#,
            Node::Eval(Box::new(Node::Eval(Box::new(Node::Literal("A"))))),
        );
    }
    #[test]
    fn group_closure() {
        assert_node!(
            r#"("A": "B")"#,
            Node::Group(Box::new(Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Literal("B")),
            })),
        );
        assert_node!(
            r#"("A"): "B""#,
            Node::Closure {
                left: Box::new(Node::Group(Box::new(Node::Literal("A")))),
                right: Box::new(Node::Literal("B")),
            },
        );
        assert_node!(
            r#""A": ("B")"#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Group(Box::new(Node::Literal("B")))),
            },
        );
    }
    #[test]
    fn group_concat() {
        assert_node!(
            r#"("A" + "B")"#,
            Node::Group(Box::new(Node::Concat(vec![
                Node::Literal("A"),
                Node::Literal("B"),
            ]))),
        );
        assert_node!(
            r#"("A") + "B""#,
            Node::Concat(vec![
                Node::Group(Box::new(Node::Literal("A"))),
                Node::Literal("B"),
            ]),
        );
        assert_node!(
            r#""A" + ("B")"#,
            Node::Concat(vec![
                Node::Literal("A"),
                Node::Group(Box::new(Node::Literal("B"))),
            ]),
        );
    }
    #[test]
    fn group_slice() {
        assert_node!(
            r#"("A"[:])"#,
            Node::Group(Box::new(Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: None,
                upper: None,
            })),
        );
        assert_node!(
            r#"("A")[:]"#,
            Node::Slice {
                src: Box::new(Node::Group(Box::new(Node::Literal("A")))),
                lower: None,
                upper: None,
            },
        );
    }
    #[test]
    fn group_equal() {
        assert_node!(
            r#"("A" = "B")"#,
            Node::Group(Box::new(Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Literal("B")),
            })),
        );
        assert_node!(
            r#"("A") = "B""#,
            Node::Equal {
                left: Box::new(Node::Group(Box::new(Node::Literal("A")))),
                right: Box::new(Node::Literal("B")),
            },
        );
        assert_node!(
            r#""A" = ("B")"#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Group(Box::new(Node::Literal("B")))),
            },
        );
    }
    #[test]
    fn group_length() {
        assert_node!(
            r#"(#"A")"#,
            Node::Group(Box::new(Node::Length(Box::new(Node::Literal("A"))))),
        );
        assert_node!(
            r#"#("A")"#,
            Node::Length(Box::new(Node::Group(Box::new(Node::Literal("A"))))),
        );
    }
    #[test]
    fn group_eval() {
        assert_node!(
            r#"($ "A")"#,
            Node::Group(Box::new(Node::Eval(Box::new(Node::Literal("A"))))),
        );
        assert_node!(
            r#"$ ("A")"#,
            Node::Eval(Box::new(Node::Group(Box::new(Node::Literal("A"))))),
        );
    }
    #[test]
    fn closure_concat() {
        assert_node!(
            r#""A": "B" + "C""#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Concat(vec![Node::Literal("B"), Node::Literal("C")])),
            },
        );
        assert_node!(
            r#""A" + "B": "C""#,
            Node::Closure {
                left: Box::new(Node::Concat(vec![Node::Literal("A"), Node::Literal("B")])),
                right: Box::new(Node::Literal("C")),
            },
        );
    }
    #[test]
    fn closure_slice() {
        assert_node!(
            r#""A": "B"[:]"#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Slice {
                    src: Box::new(Node::Literal("B")),
                    lower: None,
                    upper: None,
                }),
            },
        );
        assert_node!(
            r#""A"[:]: "B""#,
            Node::Closure {
                left: Box::new(Node::Slice {
                    src: Box::new(Node::Literal("A")),
                    lower: None,
                    upper: None,
                }),
                right: Box::new(Node::Literal("B")),
            },
        );
    }
    #[test]
    fn closure_equal() {
        assert_node!(
            r#""A": "B" = "C""#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Equal {
                    left: Box::new(Node::Literal("B")),
                    right: Box::new(Node::Literal("C")),
                }),
            },
        );
        assert_node!(
            r#""A" = "B": "C""#,
            Node::Closure {
                left: Box::new(Node::Equal {
                    left: Box::new(Node::Literal("A")),
                    right: Box::new(Node::Literal("B")),
                }),
                right: Box::new(Node::Literal("C")),
            },
        );
    }
    #[test]
    fn closure_length() {
        assert_node!(
            r#""A": #"B""#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Length(Box::new(Node::Literal("B")))),
            },
        );
        assert_node!(
            r#"#"A": "B""#,
            Node::Closure {
                left: Box::new(Node::Length(Box::new(Node::Literal("A")))),
                right: Box::new(Node::Literal("B")),
            },
        );
    }
    #[test]
    fn closure_eval() {
        assert_node!(
            r#""A": $ "B""#,
            Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Eval(Box::new(Node::Literal("B")))),
            },
        );
        assert_node!(
            r#"$ "A": "B""#,
            Node::Eval(Box::new(Node::Closure {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Literal("B")),
            })),
        );
    }
    #[test]
    fn concat_slice() {
        assert_node!(
            r#""A" + "B"[:]"#,
            Node::Concat(vec![
                Node::Literal("A"),
                Node::Slice {
                    src: Box::new(Node::Literal("B")),
                    lower: None,
                    upper: None,
                },
            ]),
        );
        assert_node!(
            r#""A"[:] + "B""#,
            Node::Concat(vec![
                Node::Slice {
                    src: Box::new(Node::Literal("A")),
                    lower: None,
                    upper: None,
                },
                Node::Literal("B"),
            ]),
        );
    }
    #[test]
    fn concat_equal() {
        assert_node!(
            r#""A" + "B" = "C""#,
            Node::Equal {
                left: Box::new(Node::Concat(vec![Node::Literal("A"), Node::Literal("B")])),
                right: Box::new(Node::Literal("C")),
            },
        );
        assert_node!(
            r#""A" = "B" + "C""#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Concat(vec![Node::Literal("B"), Node::Literal("C")])),
            },
        );
    }
    #[test]
    fn concat_length() {
        assert_node!(
            r#""A" + #"B""#,
            Node::Concat(vec![
                Node::Literal("A"),
                Node::Length(Box::new(Node::Literal("B"))),
            ]),
        );
        assert_node!(
            r#"#"A" + "B""#,
            Node::Concat(vec![
                Node::Length(Box::new(Node::Literal("A"))),
                Node::Literal("B"),
            ]),
        );
    }
    #[test]
    fn concat_eval() {
        assert_node!(
            r#""A" + $ "B""#,
            Node::Concat(vec![
                Node::Literal("A"),
                Node::Eval(Box::new(Node::Literal("B"))),
            ]),
        );
        assert_node!(
            r#"$ "A" + "B""#,
            Node::Eval(Box::new(Node::Concat(vec![
                Node::Literal("A"),
                Node::Literal("B"),
            ]))),
        );
    }
    #[test]
    fn slice_equal() {
        assert_node!(
            r#""A"[:] = "B""#,
            Node::Equal {
                left: Box::new(Node::Slice {
                    src: Box::new(Node::Literal("A")),
                    lower: None,
                    upper: None,
                }),
                right: Box::new(Node::Literal("B")),
            },
        );
        assert_node!(
            r#""A" = "B"[:]"#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Slice {
                    src: Box::new(Node::Literal("B")),
                    lower: None,
                    upper: None,
                }),
            },
        );
    }
    #[test]
    fn slice_length() {
        assert_node!(
            r#"#"A"[:]"#,
            Node::Slice {
                src: Box::new(Node::Length(Box::new(Node::Literal("A")))),
                lower: None,
                upper: None,
            },
        );
    }
    #[test]
    fn slice_eval() {
        assert_node!(
            r#"$ "A"[:]"#,
            Node::Eval(Box::new(Node::Slice {
                src: Box::new(Node::Literal("A")),
                lower: None,
                upper: None,
            })),
        );
    }
    #[test]
    fn equal_length() {
        assert_node!(
            r#""A" = #"B""#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Length(Box::new(Node::Literal("B")))),
            },
        );
        assert_node!(
            r#"#"A" = "B""#,
            Node::Equal {
                left: Box::new(Node::Length(Box::new(Node::Literal("A")))),
                right: Box::new(Node::Literal("B")),
            },
        );
    }
    #[test]
    fn equal_eval() {
        assert_node!(
            r#""A" = $ "B""#,
            Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Eval(Box::new(Node::Literal("B")))),
            },
        );
        assert_node!(
            r#"$ "A" = "B""#,
            Node::Eval(Box::new(Node::Equal {
                left: Box::new(Node::Literal("A")),
                right: Box::new(Node::Literal("B")),
            })),
        );
    }
    #[test]
    fn length_eval() {
        assert_node!(
            r#"#$ "A""#,
            Node::Length(Box::new(Node::Eval(Box::new(Node::Literal("A"))))),
        );
        assert_node!(
            r#"$ #"A""#,
            Node::Eval(Box::new(Node::Length(Box::new(Node::Literal("A"))))),
        );
    }
}
