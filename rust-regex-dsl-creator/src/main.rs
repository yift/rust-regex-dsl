use rust_regex_dsl_creator::ToDsl;
use std::io::stdin;
use std::io::BufRead;
use std::io::Error as IoError;

fn main() -> Result<(), IoError> {
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        match line.to_dsl() {
            Ok(dsl) => {
                println!("For `{}` DSL would look like:\n```\n{}\n```\n", line, dsl);
            }
            Err(e) => {
                eprintln!("Error with `{}` : {}", line, e);
            }
        }
    }
    Ok(())
}
