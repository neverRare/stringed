use std::io::{self, BufRead, Write};

use crate::gen_interpreter::{GenInterpreter, InterpreterError, Output};

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
    I: BufRead,
    O: Write,
{
    pub fn run(&mut self, src: String) -> Result<(), InterpreterOrIoError> {
        let mut interpreter = GenInterpreter::start(src);
        let mut result = interpreter.next(None);
        loop {
            let input;
            if let Some(result) = result? {
                match result {
                    Output::Output(output) => {
                        write!(&mut self.output, "{}", output)?;
                        self.output.write(output.as_bytes())?;
                        input = None;
                    }
                    Output::Input => {
                        self.output.flush()?;
                        let mut str = String::new();
                        self.input.read_line(&mut str)?;
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
#[derive(Debug)]
pub enum InterpreterOrIoError {
    Interpreter(InterpreterError),
    Io(io::Error),
}
impl From<io::Error> for InterpreterOrIoError {
    fn from(value: io::Error) -> Self {
        InterpreterOrIoError::Io(value)
    }
}
impl From<InterpreterError> for InterpreterOrIoError {
    fn from(value: InterpreterError) -> Self {
        InterpreterOrIoError::Interpreter(value)
    }
}
