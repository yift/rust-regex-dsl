pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        any_of,
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of(),
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of(""),
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            not_a_word
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            #not_a_class
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            from, 'a'
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            from: 'a' tof: 'b'
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            from: 'a' to, 'b'
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            from: 'a' to: 1
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        any_of {
            from: 1 to: 'a'
        },
    };
    println!("{}", regex);
}