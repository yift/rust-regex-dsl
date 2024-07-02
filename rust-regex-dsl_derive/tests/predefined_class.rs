pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_predefine_class_letter() {
    let regex = regex_dsl! {
        #letter
    };
    assert!(regex.is_match("a"));
    assert!(!regex.is_match("1"));
}

#[test]
fn test_predefine_class_lang() {
    let regex = regex_dsl! {
        #greek,
        #latin,
        #hebrew,
    };
    assert!(regex.is_match("Ψbג"));
    assert!(!regex.is_match("abc"));
}

#[test]
fn test_predefine_class_negate() {
    let regex = regex_dsl! {
        #greek,
        ~latin,
        #hebrew,
    };
    assert!(regex.is_match("Ψ4ג"));
    assert!(!regex.is_match("Ψtג"));
}
