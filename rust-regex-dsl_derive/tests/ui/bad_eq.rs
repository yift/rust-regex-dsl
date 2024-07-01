pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl!("");
    println!("{}", regex);
    let regex = regex_dsl!();
    println!("{}", regex);
    let regex = regex_dsl!(122);
    println!("{}", regex);
    let regex = regex_dsl!("test", "nop", 123);
    println!("{}", regex);
}