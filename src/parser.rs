enum Node<'a> {
    Literal(&'a str),
    Prompt,
    Var,
    Closure {
        left: Box<Node<'a>>,
        right: Box<Node<'a>>,
    },
    Concat(Vec<Node<'a>>),
    Slice {
        src: Box<Node<'a>>,
        lower: Box<Node<'a>>,
        upper: Box<Node<'a>>,
    },
    Equal {
        left: Box<Node<'a>>,
        right: Box<Node<'a>>,
    },
    Length(Box<Node<'a>>),
    Eval(Box<Node<'a>>),
}
