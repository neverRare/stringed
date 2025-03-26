use crate::gen_interpreter::{GenInterpreter, Output};

pub struct Interpreter<I, O>
where
    I: FnMut() -> String,
    O: FnMut(&str),
{
    input: I,
    output: O,
}
impl<I, O> Interpreter<I, O>
where
    I: FnMut() -> String,
    O: FnMut(&str),
{
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
    pub fn run(&mut self, src: &str) -> Result<(), String> {
        let mut interpreter = GenInterpreter::start(src);
        let mut result = None;
        loop {
            let input;
            if let Some(result) = result {
                match result {
                    Output::Output(output) => {
                        (self.output)(&output);
                        input = None;
                    }
                    Output::Input => {
                        input = Some((self.input)());
                    }
                    Output::Done => {
                        break Ok(());
                    }
                }
            } else {
                input = None;
            }
            result = Some(interpreter.next(match &input {
                Some(x) => Some(&x),
                None => None,
            }));
        }
    }
}
