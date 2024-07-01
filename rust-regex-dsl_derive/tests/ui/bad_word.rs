pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        nop
    };
    println!("{}", regex);
}