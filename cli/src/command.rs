use std::{
    env, fs,
    io::{BufReader, stdin, stdout},
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
    fn new(name: &Option<&str>) -> Result<Self, String> {
        Ok(match name {
            None => Self::None,
            Some("run") => Self::Run,
            Some("help") => Self::Help,
            Some("version") => Self::Version,
            Some(command) => return Err(format!("unknown command {}", command)),
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
    pub fn new(mut args: env::Args) -> Result<Self, String> {
        args.next();
        Ok(match args.next() {
            Some(command) => {
                let name = CommandName::new(&Some(&command))?;
                let next = args.next();
                if next == Some("--help".to_string()) {
                    Self::Help(name)
                } else {
                    match &name {
                        CommandName::None => Self::None,
                        CommandName::Run => Self::Run(match next {
                            None => return Err("expected file name, please specify it".to_string()),
                            Some(path) => path,
                        }),
                        CommandName::Help => Self::Help(CommandName::new(&match &next {
                            Some(command) => Some(&command),
                            None => None,
                        })?),
                        CommandName::Version => Self::Version,
                    }
                }
            }
            None => Self::None,
        })
    }
    pub fn run(&self) -> Result<(), String> {
        match self {
            Self::None => {
                println!("STRINGED {}", VERSION);
                println!();
                println!("{}", HELP);
            }
            Self::Run(path) => {
                let content = match fs::read_to_string(path) {
                    Ok(content) => content,
                    Err(reason) => return Err(reason.to_string()),
                };
                let mut interpreter = Interpreter::new(BufReader::new(stdin()), stdout());
                // TODO: handle error
                interpreter.run(content).unwrap();
            }
            Self::Help(command) => command.print_help(),
            Self::Version => println!("STRINGED {}", VERSION),
        }
        Ok(())
    }
}
