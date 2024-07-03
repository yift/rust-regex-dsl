pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_exactly() {
    let regex = regex_dsl! {
        "-",
        times {
            exactly: 3
            "Ab"
        },
        "-",
    };

    assert!(regex.is_match("-AbAbAb-"));
    assert!(!regex.is_match("-AbAb-"));
    assert!(!regex.is_match("-AbAbAbAb-"));
}
#[test]
fn test_at_least() {
    let regex = regex_dsl! {
        "-",
        times {
            at_least: 3
            "Ab"
        },
        "-",
    };

    assert!(regex.is_match("-AbAbAb-"));
    assert!(!regex.is_match("-AbAb-"));
    assert!(regex.is_match("-AbAbAbAb-"));
    assert!(regex.is_match("-AbAbAbAbAb-"));
}
#[test]
fn test_at_least_at_most() {
    let regex = regex_dsl! {
        "-",
        times {
            at_most: 5,
            at_least: 3,
            digit
        },
        "-",
    };

    assert!(!regex.is_match("-1-"));
    assert!(!regex.is_match("-12-"));
    assert!(regex.is_match("-123-"));
    assert!(regex.is_match("-1234-"));
    assert!(regex.is_match("-12345-"));
    assert!(!regex.is_match("-123456-"));
}
#[test]
fn test_greedy() {
    let regex = regex_dsl! {
        "-",
        repeat {
            greedy,
            at_most: 5,
            at_least: 3,
            digit
        },
        "-",
    };

    assert!(!regex.is_match("-1-"));
    assert!(!regex.is_match("-12-"));
    assert!(regex.is_match("-123-"));
    assert!(regex.is_match("-1234-"));
    assert!(regex.is_match("-12345-"));
    assert!(!regex.is_match("-123456-"));
}
#[test]
fn test_lazy() {
    let regex = regex_dsl! {
        "-",
        repeat {
            lazy,
            at_most: 5,
            at_least: 3,
            digit
        },
        "-",
    };

    assert!(!regex.is_match("-1-"));
    assert!(!regex.is_match("-12-"));
    assert!(regex.is_match("-123-"));
    assert!(regex.is_match("-1234-"));
    assert!(regex.is_match("-12345-"));
}
