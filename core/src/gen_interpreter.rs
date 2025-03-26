use std::{error, fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum OpCode {
    Output,
    Exec,
    Concat,
    Prompt,
    LastVar,
    PushVar,
    PopVar,
    Slice(bool, bool),
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
            var: vec!["".to_owned()],
            op: vec![OpCode::PopVar, OpCode::Exec],
        }
    }
    pub fn next(&mut self, mut input: Option<String>) -> Result<Option<Output>, Error> {
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
                    return Ok(None);
                }
            };
            match current {
                OpCode::Output => return Ok(Some(Output::Output(val.pop().unwrap()))),
                OpCode::Exec => todo!(),
                OpCode::Concat => {
                    let second = val.pop().unwrap();
                    let first = val.pop().unwrap();
                    val.push(first + &second);
                }
                OpCode::Prompt => match input.take() {
                    Some(input) => val.push(input),
                    None => {
                        op.push(OpCode::Prompt);
                        return Ok(Some(Output::Input));
                    }
                },
                OpCode::LastVar => val.push(var.last().unwrap().clone()),
                OpCode::PushVar => var.push(val.pop().unwrap()),
                OpCode::PopVar => {
                    var.pop().unwrap();
                }
                OpCode::Slice(left, right) => {
                    let right = if right {
                        Some(val.pop().unwrap().parse()?)
                    } else {
                        None
                    };
                    let left = if left { val.pop().unwrap().parse()? } else { 0 };
                    let str = val.pop().unwrap();
                    let right = right.unwrap_or(str.len());
                    if str.len() < right || right < left {
                        return Err(Error::InvalidIndex);
                    }
                    val.push(str[left..right].to_owned());
                }
                OpCode::Equal => {
                    let second = val.pop().unwrap();
                    let first = val.pop().unwrap();
                    val.push(format!("{}", first == second));
                }
                OpCode::Length => {
                    let len = val.pop().unwrap().len();
                    val.push(format!("{}", len))
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
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IntParseError(<usize as FromStr>::Err),
    InvalidIndex,
}
impl From<ParseIntError> for Error {
    fn from(value: <usize as FromStr>::Err) -> Self {
        Error::IntParseError(value)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IntParseError(err) => write!(f, "{}", err)?,
            Error::InvalidIndex => write!(f, "invalid index")?,
        }
        Ok(())
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IntParseError(err) => Some(err),
            Error::InvalidIndex => None,
        }
    }
}
