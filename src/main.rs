use std::io;
use stringed::Interpretter;

fn main() {
    let mut interpretter = Interpretter::new(
        || {
            let mut guess = String::new();
            if let Err(reason) = io::stdin().read_line(&mut guess) {
                Err(reason.to_string())
            } else {
                Ok(guess)
            }
        },
        |string| {
            println!("{}", string);
        },
    );
    let code = r#"
"Please enter your name:
" + "Hello " + ? + "!"
"#;
    if let Err(reason) = interpretter.run(code) {
        eprintln!("Error: {}", reason);
    }
}
