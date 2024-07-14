use rust_regex_dsl::regex;

#[test]
fn it_works() {
    let regex = regex!("test");

    assert!(regex.is_match("test"));
    assert!(!regex.is_match("abcd"));
}

#[test]
fn test_with_newlinet() {
    let regex = regex!("test\nthis");

    assert!(regex.is_match("test\nthis"));
    assert!(!regex.is_match("test this"));
}

#[test]
fn test_with_letters() {
    let regex = regex!("[a-z]+[A-Z]+");

    assert!(regex.is_match("abcdABCD"));
    assert!(!regex.is_match("ACD"));
    assert!(!regex.is_match("abc"));
}
