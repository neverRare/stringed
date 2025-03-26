use std::{
    error,
    fmt::Display,
    io::{self, BufRead, Write},
};

use crate::state::{self, Output, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn run(&mut self, src: String) -> Result<(), Error> {
        let mut interpreter = State::start(src);
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
pub enum Error {
    Interpreter(state::Error),
    Io(io::Error),
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}
impl From<state::Error> for Error {
    fn from(value: state::Error) -> Self {
        Error::Interpreter(value)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Interpreter(error) => write!(f, "{}", error)?,
            Error::Io(error) => write!(f, "{}", error)?,
        }
        Ok(())
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Interpreter(error) => Some(error),
            Error::Io(error) => Some(error),
        }
    }
}
