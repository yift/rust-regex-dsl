pub use rust_regex_dsl::regex_dsl;

#[test]
fn simple_unnamed_group() {
    let regex = regex_dsl! {
        "A",
        group {
            "BCD",
        },
        "E",
    };

    let caps = regex.captures("This ABCDE A test").unwrap();
    assert_eq!(caps.get(1).unwrap().as_str(), "BCD");
}
#[test]
fn simple_named_group() {
    let regex = regex_dsl! {
        "A",
        group {
            name: bcd,
            "BCD",
        },
        "E",
    };

    let caps = regex.captures("This ABCDE A test").unwrap();
    assert_eq!(&caps["bcd"], "BCD");
}
#[test]
fn two_groups() {
    let regex = regex_dsl! {
        "|",
        group {
            name: one,
            "-",
            regex {
                "[a-z]+"
            },
            "-"
        },
        "|",
        group {
            name: two,
            "-",
            regex {
                "[0-9]+"
            },
            "-"
        },
        "|"
    };

    let caps = regex.captures("|-hello-|-100-|").unwrap();
    assert_eq!(&caps["one"], "-hello-");
    assert_eq!(&caps["two"], "-100-");
}
#[test]
fn name_with_no_comma() {
    let regex = regex_dsl! {
        "A",
        group {
            name: bcd
            "BCD",
        },
        "E",
    };

    let caps = regex.captures("This ABCDE A test").unwrap();
    assert_eq!(&caps["bcd"], "BCD");
}
#[test]
fn name_with_unname_ident() {
    let regex = regex_dsl! {
        "(",
        group {
            regex("[a-z]+"),
        },
        ")",
    };

    let caps = regex.captures("(test)").unwrap();
    assert_eq!(caps.get(1).unwrap().as_str(), "test");
}
