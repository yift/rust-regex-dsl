pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        apply,
    };
    let regex = regex_dsl! {
        apply(),
    };
    let regex = regex_dsl! {
        apply("one"),
    };
    let regex = regex_dsl! {
        apply(case_insensitive, "one"),
    };
    let regex = regex_dsl! {
        apply(+nop, "one"),
    };
    let regex = regex_dsl! {
        apply(+case_insensitive, -case_insensitive, "one"),
    };
    let regex = regex_dsl! {
        apply(-case_insensitive, +case_insensitive, "one"),
    };
    let regex = regex_dsl! {
        apply(+case_insensitive, "one", "two"),
    };
    println!("{}", regex);
}