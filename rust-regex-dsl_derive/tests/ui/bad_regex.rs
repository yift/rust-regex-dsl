pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        "|",
        regex {
            "[a-z+"
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        regex {
            ""
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        regex {
            123
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        regex {
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        regex,
        "|",
    };
    println!("{}", regex);
}