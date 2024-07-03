pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_apply_with_data() {
    let regex = regex_dsl! {
      apply {
        +case_insensitive
        "one"
      }
      apply {
        -case_insensitive
        "two"
      }
    };

    assert!(regex.is_match("ONEtwo"));
    assert!(regex.is_match("onetwo"));
    assert!(!regex.is_match("oneTWO"));
}
#[test]
fn test_apply_without_data() {
    let regex = regex_dsl! {
      apply {
        +case_insensitive
      }
      "one"
      apply {
        -case_insensitive
      }
      "two"
    };

    assert!(regex.is_match("ONEtwo"));
    assert!(regex.is_match("onetwo"));
    assert!(!regex.is_match("oneTWO"));
}
#[test]
fn test_apply_two_flags() {
    let regex = regex_dsl! {
      apply {
        +case_insensitive
        +multi_line
        -enables_crlf_mode
      }
      "one"
      apply {
        -case_insensitive
        -multi_line
      }
      "two"
    };

    assert!(regex.is_match("ONEtwo"));
    assert!(regex.is_match("onetwo"));
    assert!(!regex.is_match("oneTWO"));
}
