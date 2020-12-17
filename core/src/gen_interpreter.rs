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
    pub fn step(&mut self, input: Option<&str>) -> Output {
        let var = &mut self.var;
        let val = &mut self.val;
        let op = &mut self.op;
        let current = op.pop();
        if input.is_some() {
            assert!(matches!(current, Some(OpCode::Prompt)))
        }
        let current = match current {
            Some(value) => value,
            None => {
                debug_assert!(val.is_empty());
                debug_assert!(var.is_empty());
                return Output::Done;
            }
        };
        match current {
            OpCode::Output => {
                return Output::Output(val.pop().unwrap());
            }
            OpCode::Exec => todo!(),
            OpCode::Concat => todo!(),
            OpCode::Prompt => match input {
                Some(input) => val.push(input.to_string()),
                None => op.push(OpCode::Prompt),
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
        Output::None
    }
    pub fn next(&mut self, input: Option<&str>) -> Output {
        loop {
            let output = self.step(input);
            if let Output::None = output {
                continue;
            } else {
                return output;
            }
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    None,
    Output(String),
    Input,
    Error(String),
    Done,
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn hello_world() {
        let mut program = GenInterpreter::start(r#""Hello world""#);
        let result = program.next(None);
        assert_eq!(Output::Output("Hello world".to_string()), result);
        let result = program.next(None);
        assert_eq!(Output::Done, result);
    }
    #[test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
