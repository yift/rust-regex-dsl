use rust_regex_dsl::create_capture;
#[test]
fn no_group_test() {
    create_capture!(NoGroupName, "[0-9]+,");

    let one = NoGroupName::catch("1234,").unwrap();
    assert_eq!(one.0, "1234,");
    assert_eq!(one.get_capture(), "1234,");

    let two = NoGroupName::catch("ABC");
    assert!(two.is_none());

    let three: Vec<_> = NoGroupName::catch_all("1,2,3,4,").collect();
    assert_eq!(three.len(), 4);
    assert_eq!(three[0].0, "1,");
    assert_eq!(three[1].get_capture(), "2,");
    assert_eq!(three[2].0, "3,");
    assert_eq!(three[3].0, "4,");
}

#[test]
fn two_groups_one_named() {
    create_capture!(ThreeGroups, "([0-9]+)(?<letters>[a-z]+)");

    let one = ThreeGroups::catch("123abc").unwrap();
    assert_eq!(one.0, "123abc");
    assert_eq!(one.1, Some("123"));
    assert_eq!(one.2, Some("abc"));
    assert_eq!(one.get_capture(), "123abc");
    assert_eq!(one.letters(), Some("abc"));

    let two = ThreeGroups::catch("ABC");
    assert!(two.is_none());

    let three: Vec<_> = ThreeGroups::catch_all("1a22bb33cc").collect();
    assert_eq!(three.len(), 3);
    assert_eq!(three[0].0, "1a");
    assert_eq!(three[1].1, Some("22"));
    assert_eq!(three[2].letters(), Some("cc"));
}

#[test]
fn use_regex() {
    create_capture!(MyRegexCapture, eq("one "), group(name: a_name regex("[a-z]+")));

    let one = MyRegexCapture::catch("one abcd").unwrap();
    assert_eq!(one.0, "one abcd");
    assert_eq!(one.1, Some("abcd"));
    assert_eq!(one.a_name(), Some("abcd"));
    assert_eq!(one.get_capture(), "one abcd");

    let two = MyRegexCapture::catch("ABC");
    assert!(two.is_none());

    let three: Vec<_> = MyRegexCapture::catch_all("one a").collect();
    assert_eq!(three.len(), 1);
    assert_eq!(three[0].0, "one a");
    assert_eq!(three[0].1, Some("a"));
}

#[test]
fn none_group() {
    create_capture!(MyRegexCapture, any {
        group {
            name: letters,
            repeat{ any_of { #letter }}
        }, group{
            name: digits,
            repeat{ any_of{ #digit } }
        }
    }, maybe_repeat {
        #white_space
    });

    let one = MyRegexCapture::catch("Letters ").unwrap();

    assert_eq!(one.0, "Letters ");
    assert_eq!(one.get_capture(), "Letters ");
    assert_eq!(one.1, Some("Letters"));
    assert_eq!(one.2, None);
    assert_eq!(one.letters(), Some("Letters"));
    assert_eq!(one.digits(), None);

    let two = MyRegexCapture::catch("1234").unwrap();
    assert_eq!(two.0, "1234");
    assert_eq!(two.get_capture(), "1234");
    assert_eq!(two.1, None);
    assert_eq!(two.2, Some("1234"));
    assert_eq!(two.letters(), None);
    assert_eq!(two.digits(), Some("1234"));

    let three: Vec<_> = MyRegexCapture::catch_all("abc 1234 def 33").collect();
    assert_eq!(three.len(), 4);
    assert_eq!(three[0].1, Some("abc"));
    assert_eq!(three[1].1, None);
    assert_eq!(three[2].2, None);
    assert_eq!(three[3].2, Some("33"));
}
#[test]
fn get_regex() {
    create_capture!(MyRegexCapture, "[a-z]+");

    let regex = MyRegexCapture::regex();

    assert!(regex.is_match("test"));
    assert!(!regex.is_match("TEST"));
}
