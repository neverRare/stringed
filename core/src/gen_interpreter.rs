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
    pub fn start(code: String) -> Self {
        Self {
            val: vec![code],
            var: vec!["".to_string()],
            op: vec![OpCode::PopVar, OpCode::Exec],
        }
    }
    pub fn next(&mut self, mut input: Option<String>) -> Option<Output> {
        let var = &mut self.var;
        let val = &mut self.val;
        let op = &mut self.op;
        loop {
            let current = op.pop();
            if input.is_some() {
                assert!(matches!(current, Some(OpCode::Prompt)))
            }
            let current = match current {
                Some(value) => value,
                None => {
                    debug_assert!(val.is_empty());
                    debug_assert!(var.is_empty());
                    return None;
                }
            };
            match current {
                OpCode::Output => {
                    return Some(Output::Output(val.pop().unwrap()));
                }
                OpCode::Exec => todo!(),
                OpCode::Concat => todo!(),
                OpCode::Prompt => match input.take() {
                    Some(input) => val.push(input),
                    None => {
                        op.push(OpCode::Prompt);
                        return Some(Output::Input);
                    }
                },
                OpCode::LastVar => {
                    val.push(var.last().unwrap().clone());
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
                    val.push(format!("{}", first == second));
                }
                OpCode::Length => {
                    let len = format!("{}", val.pop().unwrap().len());
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
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn hello_world() {
        let mut program = GenInterpreter::start(r#""Hello world""#.to_string());
        let result = program.next(None);
        assert_eq!(Some(Output::Output("Hello world".to_string())), result);
        let result = program.next(None);
        assert_eq!(None, result);
    }
    #[test]
    #[ignore]
    fn hello_you() {
        let mut program = GenInterpreter::start(
            r#""Please enter your name:
Hello " + ? + "!""#
                .to_string(),
        );
        let result = program.next(None);
        assert_eq!(
            Some(Output::Output(
                "Please enter your name:
Hello "
                    .to_string()
            )),
            result,
        );
        let result = program.next(None);
        assert_eq!(Some(Output::Input), result);
        let result = program.next(Some("Random Rustacean".to_string()));
        assert_eq!(Some(Output::Output("Random Rustacean".to_string())), result);
        let result = program.next(None);
        assert_eq!(Some(Output::Output("!".to_string())), result);
        let result = program.next(None);
        assert_eq!(None, result);
    }
    #[test]
    #[ignore]
    fn r#loop() {
        let mut program = GenInterpreter::start(
            r#"{"loop
" + $ _}: $ "{" + _ + "}: " + _"#
                .to_string(),
        );
        for _ in 0..10 {
            let result = program.next(None);
            assert_eq!(Some(Output::Output("loop\n".to_string())), result);
        }
    }
    #[test]
    #[ignore]
    fn counter() {
        let mut program = GenInterpreter::start(
            r#""
": ($ "{#(_[{" + #(_ + "--------------------------------") + "}:]) + {" + _ + "} + (_ + { }: $ _)}"): $ "{" + _ + "}: " + _"#.to_string(),
        );
        for i in 1..=10 {
            let result = program.next(None);
            assert_eq!(Some(Output::Output(i.to_string())), result);
            let result = program.next(None);
            assert_eq!(Some(Output::Output("\n".to_string())), result);
        }
    }
    #[test]
    #[ignore]
    fn multiple_input() {
        let mut program = GenInterpreter::start(r#"(? + ?)[:]"#.to_string());
        let result = program.next(None);
        assert_eq!(Some(Output::Input), result);
        let result = program.next(Some("cherry".to_string()));
        assert_eq!(Some(Output::Input), result);
        let result = program.next(Some("donut".to_string()));
        assert_eq!(Some(Output::Output("cherrydonut".to_string())), result);
        let result = program.next(None);
        assert_eq!(None, result);
    }
    #[test]
    #[ignore]
    fn banana() {
        let mut program =
            GenInterpreter::start(r#""a": "b" + _ + ("n" + _ + "n": _) + _"#.to_string());
        for a in &["b", "a", "nan", "a"] {
            let result = program.next(None);
            assert_eq!(Some(Output::Output(a.to_string())), result);
        }
        let result = program.next(None);
        assert_eq!(None, result);
    }
}
