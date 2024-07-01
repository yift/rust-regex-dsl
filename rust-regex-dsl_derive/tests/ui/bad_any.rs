pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        any,
    };
    let regex = regex_dsl! {
        any(),
    };
    let regex = regex_dsl! {
        any("one"),
    };
    let regex = regex_dsl! {
        any(regex("["), "two"),
    };
    println!("{}", regex);
}