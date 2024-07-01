pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_concat() {
    let regex = regex_dsl! {
        concat {
            "test1",
            "test2",
            "test3",
        }
    };
    assert!(regex.is_match("test1test2test3"));
    assert!(!regex.is_match("test1"));
}
#[test]
fn test_concat_with_regex_and_equal() {
    let regex = regex_dsl! {
        concat {
            "test",
            regex("[0-9]+"),
            eq("me"),
        }
    };
    assert!(regex.is_match("test1me"));
    assert!(regex.is_match("test21me"));
    assert!(!regex.is_match("test12"));
}
