use crate::parser::Node;
use crate::utils::parse_uint;

#[derive(Debug)]
enum OpCode {
    Output,
    Exec,
    Concat(usize),
    Prompt,
    Var,
    Open,
    Close,
    Slice { lower: bool, upper: bool },
    Equal,
    Length,
    Eval,
    Value(String),
}
enum OpNode<'a> {
    OpCode(OpCode),
    Node(Node<'a>),
}
enum OpEvalExec<'a> {
    OpCode(OpCode),
    Eval(Node<'a>),
    Exec(Node<'a>),
}
pub struct GenInterpreter {
    var: Vec<String>,
    val: Vec<String>,
    op: Vec<OpCode>,
}
impl GenInterpreter {
    pub fn start(code: &str) -> Self {
        Self {
            val: vec![code.to_string()],
            var: vec!["".to_string()],
            op: vec![OpCode::Close, OpCode::Exec],
        }
    }
    pub fn next(&mut self, mut input: Option<&str>) -> Output {
        let var = &mut self.var;
        let val = &mut self.val;
        let op = &mut self.op;
        loop {
            let current = match op.pop() {
                Some(value) => value,
                None => {
                    if !val.is_empty() || !var.is_empty() {
                        panic!(
                            "\
generator interpreter exited with non-empty value or variable stack:
val.len() = {}
var.len() = {}",
                            val.len(),
                            var.len(),
                        );
                    } else {
                        break Output::Done;
                    }
                }
            };
            match current {
                OpCode::Output => {
                    let output = val.pop().unwrap();
                    break Output::Output(output);
                }
                OpCode::Exec => {
                    let code = val.pop().unwrap();
                    let mut op_nodes = match Node::parse(&code) {
                        Ok(node) => vec![OpEvalExec::Exec(node)],
                        Err(reason) => break Output::Error(reason),
                    };
                    while let Some(op_node) = op_nodes.pop() {
                        match op_node {
                            OpEvalExec::Exec(Node::Group(node)) => {
                                op_nodes.push(OpEvalExec::Exec(*node));
                            }
                            OpEvalExec::Exec(Node::Closure { left, right }) => {
                                op.push(OpCode::Close);
                                op_nodes.push(OpEvalExec::Eval(*left));
                                op_nodes.push(OpEvalExec::OpCode(OpCode::Open));
                                op_nodes.push(OpEvalExec::Exec(*right));
                            }
                            OpEvalExec::Exec(Node::Concat(nodes)) => {
                                for node in nodes {
                                    op_nodes.push(OpEvalExec::Exec(node));
                                }
                            }
                            OpEvalExec::Exec(Node::Eval(node)) => {
                                op.push(OpCode::Exec);
                                op_nodes.push(OpEvalExec::Eval(*node));
                            }
                            OpEvalExec::Exec(node) => {
                                op.push(OpCode::Output);
                                push_node(op, node);
                            }
                            OpEvalExec::Eval(node) => push_node(op, node),
                            OpEvalExec::OpCode(op_code) => op.push(op_code),
                        }
                    }
                }
                OpCode::Concat(len) => {
                    let mut result = String::new();
                    for _ in 0..len {
                        let more = result;
                        result = val.pop().unwrap();
                        result.push_str(&more);
                    }
                    val.push(result);
                }
                OpCode::Prompt => match input {
                    Some(value) => {
                        val.push(value.to_string());
                        input = None;
                    }
                    None => {
                        op.push(OpCode::Prompt);
                        break Output::Input;
                    }
                },
                OpCode::Var => {
                    val.push(var.last().unwrap().to_string());
                }
                OpCode::Open => {
                    var.push(val.pop().unwrap());
                }
                OpCode::Close => {
                    var.pop().unwrap();
                }
                OpCode::Slice { lower, upper } => {
                    let upper = if upper {
                        match parse_uint(&val.pop().unwrap()) {
                            Ok(val) => Some(val),
                            Err(reason) => break Output::Error(reason),
                        }
                    } else {
                        None
                    };
                    let lower = if lower {
                        match parse_uint(&val.pop().unwrap()) {
                            Ok(val) => val,
                            Err(reason) => break Output::Error(reason),
                        }
                    } else {
                        0
                    };
                    let src = val.pop().unwrap();
                    let upper = upper.unwrap_or_else(|| src.len());
                    if upper > src.len() {
                        break Output::Error(
                            "upper bound larger than the length of string".to_string(),
                        );
                    } else if lower > upper {
                        break Output::Error("lower bound larger than upper bound".to_string());
                    } else {
                        val.push(src[lower..upper].to_string());
                    }
                }
                OpCode::Equal => {
                    let first = val.pop().unwrap();
                    let second = val.pop().unwrap();
                    val.push((first == second).to_string());
                }
                OpCode::Length => {
                    let len = val.pop().unwrap().len().to_string();
                    val.push(len);
                }
                OpCode::Eval => match Node::parse(&val.pop().unwrap()) {
                    Ok(node) => push_node(op, node),
                    Err(reason) => break Output::Error(reason),
                },
                OpCode::Value(value) => val.push(value),
            }
        }
    }
}
fn push_node(op: &mut Vec<OpCode>, node: Node) {
    let mut op_nodes = vec![OpNode::Node(node)];
    while let Some(op_node) = op_nodes.pop() {
        match op_node {
            OpNode::Node(Node::Literal(content)) => op.push(OpCode::Value(content.to_string())),
            OpNode::Node(Node::Group(node)) => op_nodes.push(OpNode::Node(*node)),
            OpNode::Node(Node::Prompt) => op.push(OpCode::Prompt),
            OpNode::Node(Node::Var) => op.push(OpCode::Var),
            OpNode::Node(Node::Closure { left, right }) => {
                op.push(OpCode::Close);
                op_nodes.push(OpNode::Node(*left));
                op_nodes.push(OpNode::OpCode(OpCode::Open));
                op_nodes.push(OpNode::Node(*right));
            }
            OpNode::Node(Node::Concat(nodes)) => {
                op.push(OpCode::Concat(nodes.len()));
                for node in nodes {
                    op_nodes.push(OpNode::Node(node));
                }
            }
            OpNode::Node(Node::Slice { src, lower, upper }) => {
                op.push(OpCode::Slice {
                    lower: lower.is_some(),
                    upper: upper.is_some(),
                });
                op_nodes.push(OpNode::Node(*src));
                if let Some(node) = lower {
                    op_nodes.push(OpNode::Node(*node));
                }
                if let Some(node) = upper {
                    op_nodes.push(OpNode::Node(*node));
                }
            }
            OpNode::Node(Node::Equal { left, right }) => {
                op.push(OpCode::Equal);
                op_nodes.push(OpNode::Node(*left));
                op_nodes.push(OpNode::Node(*right));
            }
            OpNode::Node(Node::Length(node)) => {
                op.push(OpCode::Length);
                op_nodes.push(OpNode::Node(*node));
            }
            OpNode::Node(Node::Eval(node)) => {
                op.push(OpCode::Eval);
                op_nodes.push(OpNode::Node(*node));
            }
            OpNode::OpCode(op_code) => op.push(op_code),
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    Output(String),
    Input,
    Error(String),
    Done,
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hello_world() {
        let mut program = GenInterpreter::start(r#""Hello world""#);
        let result = program.next(None);
        assert_eq!(Output::Output("Hello world".to_string()), result);
        let result = program.next(None);
        assert_eq!(Output::Done, result);
    }
    #[test]
    fn hello_you() {
        let mut program = GenInterpreter::start(
            r#""Please enter your name:
Hello " + ? + "!""#,
        );
        let result = program.next(None);
        assert_eq!(
            Output::Output(
                "Please enter your name:
Hello "
                    .to_string()
            ),
            result,
        );
        let result = program.next(None);
        assert_eq!(Output::Input, result);
        let result = program.next(Some("Random Rustacean"));
        assert_eq!(Output::Output("Random Rustacean".to_string()), result);
        let result = program.next(None);
        assert_eq!(Output::Output("!".to_string()), result);
        let result = program.next(None);
        assert_eq!(Output::Done, result);
    }
    #[test]
    fn r#loop() {
        let mut program = GenInterpreter::start(
            r#"{"loop
" + $ _}: $ "{" + _ + "}: " + _"#,
        );
        for _ in 0..10 {
            let result = program.next(None);
            assert_eq!(Output::Output("loop\n".to_string()), result);
        }
    }
    #[test]
    fn counter() {
        let mut program = GenInterpreter::start(
            r#""
": ($ "{#(_[{" + #(_ + "--------------------------------") + "}:]) + {" + _ + "} + (_ + { }: $ _)}"): $ "{" + _ + "}: " + _"#,
        );
        for i in 1..=10 {
            let result = program.next(None);
            assert_eq!(Output::Output(i.to_string()), result);
            let result = program.next(None);
            assert_eq!(Output::Output("\n".to_string()), result);
        }
    }
    #[test]
    fn multiple_input() {
        let mut program = GenInterpreter::start(r#"(? + ?)[:]"#);
        let result = program.next(None);
        assert_eq!(Output::Input, result);
        let result = program.next(Some("cherry"));
        assert_eq!(Output::Input, result);
        let result = program.next(Some("donut"));
        assert_eq!(Output::Output("cherrydonut".to_string()), result);
        let result = program.next(None);
        assert_eq!(Output::Done, result);
    }
    #[test]
    fn banana() {
        let mut program = GenInterpreter::start(r#""a": "b" + _ + ("n" + _ + "n": _) + _"#);
        for a in &["b", "a", "nan", "a"] {
            let result = program.next(None);
            assert_eq!(Output::Output(a.to_string()), result);
        }
        let result = program.next(None);
        assert_eq!(Output::Done, result);
    }
}
