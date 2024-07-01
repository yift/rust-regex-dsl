pub use rust_regex_dsl::regex_dsl;

#[test]
fn string_default_to_equals() {
    let regex = regex_dsl! {
        "test"
    };
    assert!(regex.is_match("test"));
    assert!(!regex.is_match("nop"));
}

#[test]
fn two_eqs() {
    let regex = regex_dsl! {
        "test",
        " ",
        "one"
    };
    assert!(regex.is_match("test one"));
    assert!(!regex.is_match("test"));
}

#[test]
fn two_with_escape() {
    let regex = regex_dsl! {
        "test.me\\",
    };
    assert!(regex.is_match("test.me\\"));
    assert!(!regex.is_match("test-me\n"));
}
