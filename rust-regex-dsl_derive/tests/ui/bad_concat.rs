pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        concat,
    };
    let regex = regex_dsl! {
        concat(),
    };
    let regex = regex_dsl! {
        concat("one"),
    };
    let regex = regex_dsl! {
        concat(regex("["), "two"),
    };
    println!("{}", regex);
}