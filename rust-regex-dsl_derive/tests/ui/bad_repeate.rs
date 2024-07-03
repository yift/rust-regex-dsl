pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        "|",
        maybe,
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        maybe_repeat {
            12
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        repeat {
            lazy, lazy,
            "test"
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        repeat {
            lazy, greedy,
            "test"
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        repeat {
            greedy, greedy
            "test"
        },
        "|",
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        "|",
        repeat {
            greedy, lazy
            "test"
        },
        "|",
    };
    println!("{}", regex);
}