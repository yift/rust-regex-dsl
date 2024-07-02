pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_any_of_string() {
    let regex = regex_dsl! {
        any_of {
            "abcd"
        }
    };
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(regex.is_match("d"));
    assert!(!regex.is_match("e"));
}
#[test]
fn test_any_of_chars() {
    let regex = regex_dsl! {
        any_of {
            'a',
            'b',
            "bcd",
        }
    };
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(regex.is_match("d"));
    assert!(!regex.is_match("e"));
}
#[test]
fn test_any_of_escape() {
    let regex = regex_dsl! {
        any_of {
            '\\',
            '-',
            'd',
            "[]",
        }
    };
    assert!(regex.is_match("\\"));
    assert!(regex.is_match("-"));
    assert!(!regex.is_match("c"));
    assert!(regex.is_match("d"));
    assert!(!regex.is_match("e"));
    assert!(regex.is_match("["));
    assert!(regex.is_match("]"));
}
#[test]
fn test_any_of_class() {
    let regex = regex_dsl! {
        any_of {
            #digit
        }
    };
    assert!(regex.is_match("1"));
    assert!(regex.is_match("2"));
    assert!(regex.is_match("3"));
    assert!(!regex.is_match("a"));
    assert!(!regex.is_match("b"));
}
#[test]
fn test_any_of_not_class() {
    let regex = regex_dsl! {
        any_of {
            ~digit
        }
    };
    assert!(!regex.is_match("1"));
    assert!(!regex.is_match("2"));
    assert!(!regex.is_match("3"));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
}
#[test]
fn test_any_of_single_word() {
    let regex = regex_dsl! {
        any_of {
            digit
        }
    };
    assert!(regex.is_match("1"));
    assert!(regex.is_match("2"));
    assert!(regex.is_match("3"));
    assert!(!regex.is_match("a"));
    assert!(!regex.is_match("b"));
}

#[test]
fn test_not_any_of_string() {
    let regex = regex_dsl! {
        not_any_of {
            "abcd"
        }
    };
    assert!(!regex.is_match("a"));
    assert!(!regex.is_match("b"));
    assert!(!regex.is_match("c"));
    assert!(!regex.is_match("d"));
    assert!(regex.is_match("e"));
}
#[test]
fn test_any_of_range() {
    let regex = regex_dsl! {
        any_of {
            "abcd",
            from: '2' to: '6'
        }
    };
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(regex.is_match("d"));
    assert!(!regex.is_match("e"));
    assert!(!regex.is_match("1"));
    assert!(regex.is_match("2"));
    assert!(regex.is_match("3"));
    assert!(regex.is_match("6"));
    assert!(!regex.is_match("7"));
}
#[test]
fn test_any_of_range_with_comma() {
    let regex = regex_dsl! {
        any_of {
            "abcd",
            from: '2', to: '6',
        }
    };

    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(regex.is_match("d"));
    assert!(!regex.is_match("e"));
    assert!(!regex.is_match("1"));
    assert!(regex.is_match("2"));
    assert!(regex.is_match("3"));
    assert!(regex.is_match("6"));
    assert!(!regex.is_match("7"));
}
