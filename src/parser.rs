use crate::{
    error::{expect, unexpect},
    lexer::{lex, Token},
};
#[derive(Debug)]
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
            Err(unexpect(tokens[node.count].describe()))
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
                lower: match lower {
                    Some(thing) => Some(Box::new(thing)),
                    None => None,
                },
                upper: match upper {
                    Some(thing) => Some(Box::new(thing)),
                    None => None,
                },
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
                Self::Concat(vec) => {
                    let mut vec = vec;
                    if let PartialNode::Concat(right) = partial_node {
                        vec.push(right);
                    } else {
                        let last = vec.pop().unwrap();
                        vec.push(last.merge(partial_node));
                    }
                    Self::Concat(vec)
                }
                Self::Slice { .. } => unreachable!(),
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
            Err(expect("expression", "EOF"))
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
                        None => Err(expect("expression", ")")),
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
                token => Err(expect("expression", token.describe())),
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
            Some(Err(unexpect("EOF")))
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
                            Some(Err(expect(closing.describe(), next.describe())))
                        }
                    } else {
                        Some(Err(expect(closing.describe(), "EOF")))
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
    fn from_tokens(parse_colon: bool, tokens: &[Token<'a>]) -> Option<Result<Self, String>> {
        if tokens.is_empty() {
            None
        } else {
            match &tokens[0] {
                Token::Colon => {
                    if parse_colon {
                        match CountedNode::from_tokens(true, &tokens[1..]) {
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
                Token::Plus => match CountedNode::from_tokens(parse_colon, &tokens[1..]) {
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
                            lower: match lower {
                                None => None,
                                Some(node) => Some(node.node),
                            },
                            upper: match upper {
                                None => None,
                                Some(node) => Some(node.node),
                            },
                        },
                    }))
                }
                Token::Equal => match CountedNode::from_tokens(parse_colon, &tokens[1..]) {
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
                match CountedPartialNode::from_tokens(self.parse_colon, tokens) {
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
