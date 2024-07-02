pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        #english
    };
    println!("{}", regex);

    let regex = regex_dsl! {
        ~english
    };
    println!("{}", regex);

    let regex = regex_dsl! {
        -latin
    };
    println!("{}", regex);
}