pub use rust_regex_dsl::regex_dsl;

#[test]
fn maybe() {
    let regex = regex_dsl! {
        "-",
        maybe {
            "Ab"
        },
        "-",
    };

    assert!(regex.is_match("-Ab-"));
    assert!(regex.is_match("--"));
    assert!(!regex.is_match("-AbAb-"));
}
#[test]
fn maybe_greedy() {
    let regex = regex_dsl! {
        "-",
        maybe {
            greedy,
            "Ab"
        },
        "-",
    };

    assert!(regex.is_match("-Ab-"));
    assert!(regex.is_match("--"));
    assert!(!regex.is_match("-AbAb-"));
}
#[test]
fn maybe_repeat() {
    let regex = regex_dsl! {
        "-",
        group {
            name: letters
            maybe_repeat {
                #letter
            },
        },
        "-",
    };

    let caps = regex.captures("--").unwrap();
    assert_eq!(&caps["letters"], "");
    let caps = regex.captures("-hello-").unwrap();
    assert_eq!(&caps["letters"], "hello");
}
#[test]
fn maybe_repeat_lazy() {
    let regex = regex_dsl! {
        "-",
        group {
            name: letters
            maybe_repeat {
                lazy,
                #letter
            },
        },
        "-",
    };

    let caps = regex.captures("--").unwrap();
    assert_eq!(&caps["letters"], "");
    let caps = regex.captures("-hello-").unwrap();
    assert_eq!(&caps["letters"], "hello");
}

#[test]
fn repeat() {
    let regex = regex_dsl! {
        "-",
        group {
            name: number
            repeat {
                #digit
            },
        },
        "-",
    };

    assert!(!regex.is_match("--"));
    let caps = regex.captures("-123-").unwrap();
    assert_eq!(&caps["number"], "123");
}

#[test]
fn group_in_repeat() {
    let regex = regex_dsl! {
        "-",
        repeat {
            group {
                name: digit
                #digit
             }
        },
        "-",
    };

    assert!(!regex.is_match("--"));
    let caps = regex.captures("-123-").unwrap();
    assert_eq!(&caps["digit"], "3");
}
