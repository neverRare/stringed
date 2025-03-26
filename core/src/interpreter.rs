use std::io::{self, Read, Write};

use crate::gen_interpreter::{GenInterpreter, Output};

pub struct Interpreter<I, O> {
    input: I,
    output: O,
}
impl<I, O> Interpreter<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
}
impl<I, O> Interpreter<I, O>
where
    I: Read,
    O: Write,
{
    pub fn run(&mut self, src: &str) -> Result<(), io::Error> {
        let mut interpreter = GenInterpreter::start(src);
        let mut result = None;
        loop {
            let input;
            if let Some(result) = result {
                match result {
                    Output::Output(output) => {
                        self.output.write(output.as_bytes())?;
                        input = None;
                    }
                    Output::Input => {
                        self.output.flush()?;
                        let mut str = String::new();
                        self.input.read_to_string(&mut str)?;
                        input = Some(str);
                    }
                    Output::Done => {
                        self.output.flush()?;
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
