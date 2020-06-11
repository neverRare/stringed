use std::env;
use stringed::Command;

fn main() {
    if let Err(reason) = Command::new(env::args()).and_then(|command| command.run()) {
        eprintln!("error: {}", reason);
    };
}
