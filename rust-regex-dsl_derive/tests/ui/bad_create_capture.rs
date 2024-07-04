pub use rust_regex_dsl::create_capture;

fn main() {
    let regex = create_capture! {
    };
    println!("{}", regex);
    let regex = create_capture! {
        "test",
        "test",
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        1,
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        "[",
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        regex("["),
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        "[",
    };
    println!("{}", regex);
    let regex = create_capture! {
        test,
        nop("[a-z]+"),
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        regex("(?<get_capture>[a-z]+)"),
    };
    println!("{}", regex);
    let regex = create_capture! {
        Test,
        regex("(?<one>[a-z]+)(?<one>[a-z]+)"),
    };
    println!("{}", regex);
}