pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_any_character() {
    let regex = regex_dsl! {
        "test",
        any_character,
    };
    assert!(regex.is_match("test1"));
    assert!(regex.is_match("test2"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_digit() {
    let regex = regex_dsl! {
        "test",
        digit,
    };
    assert!(regex.is_match("test1"));
    assert!(regex.is_match("test2"));
    assert!(!regex.is_match("testA"));
}

#[test]
fn test_not_digit() {
    let regex = regex_dsl! {
        "test",
        not_digit,
    };
    assert!(regex.is_match("testA"));
    assert!(regex.is_match("testB"));
    assert!(!regex.is_match("test2"));
}

#[test]
fn test_white_space() {
    let regex = regex_dsl! {
        "test",
        white_space,
    };
    assert!(regex.is_match("test "));
    assert!(regex.is_match("test\t"));
    assert!(!regex.is_match("test2"));
}

#[test]
fn test_not_white_space() {
    let regex = regex_dsl! {
        "test",
        not_white_space,
    };
    assert!(regex.is_match("test2"));
    assert!(regex.is_match("testA"));
    assert!(!regex.is_match("test "));
}

#[test]
fn test_word_character() {
    let regex = regex_dsl! {
        "test",
        word_character,
    };
    assert!(regex.is_match("testA"));
    assert!(regex.is_match("testB"));
    assert!(!regex.is_match("test\t"));
}
#[test]
fn test_not_word_character() {
    let regex = regex_dsl! {
        "test",
        not_word_character,
    };
    assert!(regex.is_match("test\t"));
    assert!(regex.is_match("test "));
    assert!(!regex.is_match("testA"));
}
#[test]
fn test_beginning_and_end_of_line() {
    let regex = regex_dsl! {
        beginning_of_line,
        "test",
        end_of_line,
    };
    assert!(regex.is_match("test"));
    assert!(!regex.is_match("Atest"));
    assert!(!regex.is_match("testA"));
}

#[test]
fn test_word_boundary() {
    let regex = regex_dsl! {
        word_boundary,
        "test",
        word_boundary,
    };
    assert!(regex.is_match("this is a test right"));
    assert!(!regex.is_match("Atest"));
    assert!(!regex.is_match("testA"));
}

#[test]
fn test_not_word_boundary() {
    let regex = regex_dsl! {
        not_word_boundary,
        "A",
    };
    assert!(regex.is_match("aA"));
    assert!(!regex.is_match(" A"));
}

#[test]
fn test_beginning_and_end_of_input() {
    let regex = regex_dsl! {
        beginning_of_input,
        "test",
        end_of_input,
    };
    assert!(regex.is_match("test"));
    assert!(!regex.is_match("Atest"));
    assert!(!regex.is_match("testA"));
}
