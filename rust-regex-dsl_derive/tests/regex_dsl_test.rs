pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_regex() {
    let regex = regex_dsl! {
        regex("[a-z]+")
    };
    assert!(regex.is_match("test"));
    assert!(!regex.is_match("TEST"));
}

#[test]
fn test_regex_and_eq() {
    let regex = regex_dsl! {
        "|",
        regex("[a-z]+"),
        "|",
    };
    assert!(regex.is_match("|test|"));
    assert!(!regex.is_match("|TEST|"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_regex_qb() {
    let regex = regex_dsl! {
        "|",
        regex {
            "[a-z]+"
        },
        "|",
    };
    assert!(regex.is_match("|test|"));
    assert!(!regex.is_match("|TEST|"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_regex_sb() {
    let regex = regex_dsl! {
        "|",
        regex [
            "[a-z]+"
        ],
        "|",
    };
    assert!(regex.is_match("|test|"));
    assert!(!regex.is_match("|TEST|"));
    assert!(!regex.is_match("test"));
}
