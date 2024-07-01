pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_eq() {
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

#[test]
fn two_with_chars() {
    let regex = regex_dsl! {
        't',
        'e',
        's',
        't'
    };
    assert!(regex.is_match("test"));
    assert!(!regex.is_match("t"));
}

#[test]
fn test_eq_with_nl() {
    let regex = regex_dsl! {
        "hello\nworld"
    };
    assert!(regex.is_match("hello\nworld"));
    assert!(!regex.is_match("test world"));
}

#[test]
fn test_eq_with_quote() {
    let regex = regex_dsl! {
        "\"test\""
    };

    assert!(regex.is_match("\"test\""));
    assert!(!regex.is_match("test"));
}
