pub use rust_regex_dsl::regex_dsl;

#[test]
fn test_any_one() {
    let regex = regex_dsl! {
      any {
        eq("one"),
        eq("two"),
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_two() {
    let regex = regex_dsl! {
      any {
        eq("one"),
        "two",
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_three() {
    let regex = regex_dsl! {
      any {
        "one",
        eq("two"),
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_single_letter() {
    let regex = regex_dsl! {
      any {
        "o",
        eq("b"),
      }
    };
    assert!(regex.is_match("o"));
    assert!(regex.is_match("b"));
    assert!(!regex.is_match("a"));
}

#[test]
fn test_any_four() {
    let regex = regex_dsl! {
      any {
        "one",
        "two",
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_five() {
    let regex = regex_dsl! {
      any {
        "one",
        "two",
        "three",
        eq("four")
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(regex.is_match("three"));
    assert!(regex.is_match("four"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_with_any() {
    let regex = regex_dsl! {
      any {
        "one",
        "two",
        any(
            "three",
            eq("four"),
        ),
      }
    };

    assert!(regex.is_match("one"));
    assert!(regex.is_match("two"));
    assert!(regex.is_match("three"));
    assert!(regex.is_match("four"));
    assert!(!regex.is_match("test"));
}

#[test]
fn test_any_with_regex() {
    let regex = regex_dsl! {
      any {
        "one",
        regex { "[0-9]+" },
      }
    };
    assert!(regex.is_match("one"));
    assert!(regex.is_match("1"));
    assert!(regex.is_match("12"));
    assert!(!regex.is_match("two"));
}
