pub use rust_regex_dsl::regex;

fn main() {
    let regex = regex!("hello", "world");
    println!("{}", regex);
}