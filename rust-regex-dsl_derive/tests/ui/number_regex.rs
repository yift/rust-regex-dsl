pub use rust_regex_dsl::regex;

fn main() {
    let regex = regex!(122);
    println!("{}", regex);
}