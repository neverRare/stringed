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
    pub fn run(&mut self, src: String) -> Result<(), io::Error> {
        let mut interpreter = GenInterpreter::start(src);
        let mut result = interpreter.next(None);
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
                }
            } else {
                self.output.flush()?;
                break Ok(());
            }
            result = interpreter.next(input);
        }
    }
}
