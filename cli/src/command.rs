use std::{
    env, error,
    fmt::Display,
    fs,
    io::{self, BufReader, stdin, stdout},
};
use stringed_core::Interpreter;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION_MAJOR"),
    ".",
    env!("CARGO_PKG_VERSION_MINOR"),
    ".",
    env!("CARGO_PKG_VERSION_PATCH"),
);
const HELP: &str = "\
available commands:
  run        executes stringed code
  help       prints help text
  version    prints version

note:
  run `stringed help <command>` for more information on specific command";

const HELP_RUN: &str = "\
RUN
executes stringed code

usage:
  stringed run <file>

options:
  file    a stringed file to be executed";

const HELP_HELP: &str = "\
HELP
displays help message of a command

usage:
  stringed help <command>

options:
  command    a command whose help message to be display";

const HELP_VERSION: &str = "\
VERSION
displays the version of stringed interpreter

usage:
  stringed version";

pub enum CommandName {
    None,
    Run,
    Help,
    Version,
}
impl CommandName {
    fn new(name: Option<&str>) -> Result<Self, Error> {
        Ok(match name {
            None => Self::None,
            Some("run") => Self::Run,
            Some("help") => Self::Help,
            Some("version") => Self::Version,
            Some(command) => return Err(Error::UnknownCommand(command.to_owned())),
        })
    }
    fn print_help(&self) {
        println!(
            "{}",
            match self {
                Self::None => HELP,
                Self::Run => HELP_RUN,
                Self::Help => HELP_HELP,
                Self::Version => HELP_VERSION,
            },
        );
    }
}
pub enum Command {
    None,
    Run(String),
    Help(CommandName),
    Version,
}
impl Command {
    pub fn new(mut args: env::Args) -> Result<Self, Error> {
        args.next();
        Ok(match args.next() {
            Some(command) => {
                let name = CommandName::new(Some(&command))?;
                let next = args.next();
                if next == Some("--help".to_string()) {
                    Self::Help(name)
                } else {
                    match &name {
                        CommandName::None => Self::None,
                        CommandName::Run => Self::Run(match next {
                            None => return Err(Error::NoFileName),
                            Some(path) => path,
                        }),
                        CommandName::Help => {
                            Self::Help(CommandName::new(next.as_ref().map(|string| &string[..]))?)
                        }
                        CommandName::Version => Self::Version,
                    }
                }
            }
            None => Self::None,
        })
    }
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Self::None => {
                println!("STRINGED {}", VERSION);
                println!();
                println!("{}", HELP);
            }
            Self::Run(path) => {
                let content = fs::read_to_string(path)?;
                let mut interpreter = Interpreter::new(BufReader::new(stdin()), stdout());
                interpreter.run(content)?;
            }
            Self::Help(command) => command.print_help(),
            Self::Version => println!("STRINGED {}", VERSION),
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum Error {
    Interpreter(stringed_core::Error),
    Io(io::Error),
    UnknownCommand(String),
    NoFileName,
}
impl From<stringed_core::Error> for Error {
    fn from(value: stringed_core::Error) -> Self {
        Error::Interpreter(value)
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Interpreter(error) => write!(f, "{}", error)?,
            Error::Io(error) => write!(f, "{}", error)?,
            Error::UnknownCommand(command) => write!(f, "unknown command: {}", command)?,
            Error::NoFileName => write!(f, "no file name")?,
        }
        Ok(())
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Interpreter(error) => Some(error),
            Error::Io(error) => Some(error),
            Error::UnknownCommand(_) => None,
            Error::NoFileName => None,
        }
    }
}
