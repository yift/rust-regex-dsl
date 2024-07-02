pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl!(
        group
    );
    println!("{}", regex);
    let regex = regex_dsl!(
        group {

        }
    );
    println!("{}", regex);
    let regex = regex_dsl!(
        group {
            nameo,
            "test"
        }
    );
    println!("{}", regex);
    let regex = regex_dsl!(
        group {
            name, "terst",
            "test"
        }
    );
    println!("{}", regex);
    let regex = regex_dsl!(
        group {
            name: "terst",
            "test"
        }
    );
    println!("{}", regex);
}