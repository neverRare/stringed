#[derive(Debug)]
enum OpCode {
    Output,
    Exec,
    Concat,
    Prompt,
    LastVar,
    PushVar,
    PopVar,
    SliceAll,
    SliceTo,
    SliceFrom,
    SliceFromTo,
    Equal,
    Length,
    Eval,
    Value(String),
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
            op: vec![OpCode::PopVar, OpCode::Exec],
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
                OpCode::Exec => todo!(),
                OpCode::Concat => todo!(),
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
                OpCode::LastVar => {
                    val.push(var.last().unwrap().to_string());
                }
                OpCode::PushVar => {
                    var.push(val.pop().unwrap());
                }
                OpCode::PopVar => {
                    var.pop().unwrap();
                }
                OpCode::SliceAll => todo!(),
                OpCode::SliceFrom => todo!(),
                OpCode::SliceTo => todo!(),
                OpCode::SliceFromTo => todo!(),
                OpCode::Equal => {
                    let first = val.pop().unwrap();
                    let second = val.pop().unwrap();
                    val.push((first == second).to_string());
                }
                OpCode::Length => {
                    let len = val.pop().unwrap().len().to_string();
                    val.push(len);
                }
                OpCode::Eval => todo!(),
                OpCode::Value(value) => val.push(value),
            }
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
