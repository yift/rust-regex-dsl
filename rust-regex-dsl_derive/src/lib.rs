use create_capture::CreateCapture;
use dsl::Dsl;
use error_factory::ErrorFactory;
use functions::parse_list::parse_list_to_vec;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
mod create_capture;
mod dsl;
mod error_factory;
mod functions;
mod group;
mod ident_parser;
mod predefined_class;
mod user_class;

/// A simple regular expression macro.
///
/// This macro will validate the regular expression and will produce a compile time error if the expression
/// is invalid. This can allow a developer to know that the hard coded regular expressions are valid. The result will be a valid [`regex::Regex`].
///
/// For example:
/// ```rust
/// use rust_regex_dsl::regex;
/// use regex::Regex;
/// use lazy_static::lazy_static;
///
/// lazy_static! {
///     static ref VALID_NAME: Regex = regex!("[a-z][a-zA-Z_]*");
/// }
/// ```
/// Is equivalent to:
/// ```rust
/// use rust_regex_dsl::regex;
/// use regex::Regex;
/// use lazy_static::lazy_static;
///
/// lazy_static! {
///     static ref VALID_NAME: Regex = Regex::new("[a-z][a-zA-Z_]*").unwrap();
/// }
/// ```
///
/// But this:
/// ```compile_fail
/// use rust_regex_dsl::regex;
/// use regex::Regex;
/// use lazy_static::lazy_static;
///
/// lazy_static! {
///     static ref VALID_NAME: Regex = regex!("[a-z][a-zA-Z_*");
/// }
/// ```
/// will fail with compilation error (because of the missing closing square bracket) while using the same without the macro will fail in run time.
#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let dsl = Dsl::new(&input.value(), input.value().len() > 1);
    let error_factory = ErrorFactory::new_obj(input.span());
    dsl.build(error_factory).into()
}

/// A DSL for creating Regular Expressions.
///
/// This macro will introduce a domain specific language to create a regular expression that is easier to read.
///
/// The macro support a few types of arguments, if the macro has more than one argument, it will concat all the arguments.
///
/// The supported arguments are:
/// # A literal string.
/// That is - `"<something>"` - this is the same as equals, i.e. it will produce a regular expression to compare that the string is exactly the same as the argument. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!("Foo");
/// assert!(regex.is_match("Foo"));
/// assert!(!regex.is_match("Bar"));
/// ```
/// It will also escape any character that need to be escaped, for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!("Foo\\Bar");
/// assert!(regex.is_match("Foo\\Bar"));
/// assert!(!regex.is_match("Bar-Bar"));
/// ```
/// # A literal character.
/// That is - `'a'` - this is the same as the String but for a single character. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!(
///   "Foo",
///   '-',
///    "Bar"
/// );
/// assert!(regex.is_match("Foo-Bar"));
/// assert!(!regex.is_match("Bar=Bar"));
/// ```
/// # Class name
/// That is - `#<class_name>` - a character from a unicode class. See more details in [here](https://www.regular-expressions.info/unicode.html). This is equivalent to `\p{<class_name>}`. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!{
///     #latin,
///     #greek,
/// };
/// assert!(regex.is_match("aα"));
/// assert!(!regex.is_match("αa"));
/// ```
/// # Not a class name
/// That is - `~<class_name>` - anything but a character from a unicode class. See more details in [here](https://www.regular-expressions.info/unicode.html). This is equivalent to `\P{<class_name>}`. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!{
///     ~latin,
///     ~greek,
/// };
/// assert!(!regex.is_match("aα"));
/// assert!(regex.is_match("αa"));
/// ```
/// # special cases:
/// That is - `something` - a special regular expression type of character or boundary. Available types are:
/// * `any_character` - Any character (besides new line unless the `allow_dot` flag is set). This is equivalent to `.`
/// * `digit` - A digit (similar to `#digit` and `#Nd`). This is equivalent to `\d`
/// * `not_digit` - Not a digit. This is equivalent to `\D`.
/// * `white_space` - A white space. . This is equivalent to `\s`.
/// * `not_white_space` - Not a white space. This is equivalent to `\S`.
/// * `word_character` - A word character. This is equivalent to `\w`.
/// * `not_word_character` - Not a word character. This is equivalent to `\W`.
/// * `beginning_of_line` - A beginning of a line. This is equivalent to `^`.
/// * `end_of_line` - An end of a line. This is equivalent to `$`.
/// * `word_boundary` - A word boundary. This is equivalent to `\b`.
/// * `not_word_boundary` - Not a word boundary. This is equivalent to `\B`.
/// * `beginning_of_input` - A beginning of the input. This is equivalent to `\A`.
/// * `end_of_input` - An end of the input. This is equivalent to `\z`.
///
/// For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl!{
///     "Foo",
///     any_character
///     "Bar",
/// };
/// assert!(regex.is_match("Foo Bar"));
/// assert!(regex.is_match("Foo-Bar"));
/// assert!(!regex.is_match("FooBar"));
/// ```
/// # Function
/// That is - `func(args)` or `func { args }` -  a function DSL. The available functions are:
/// ## `eq`
/// Expect a single literal string arguments. behave the same as literal string above. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     eq {
///         "Foo"
///     }
/// };
/// assert!(regex.is_match("Foo"));
/// assert!(!regex.is_match("Bar"));
/// ```
/// ## `regex`
/// Expect a single literal string argument. Use it as raw regular expression. To be used for things that are not covered by this DSL or when the expression is trivial to understand. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     regex("[0-9]+")
/// };
/// assert!(regex.is_match("300"));
/// assert!(!regex.is_match("Bar"));
/// ```
/// Invalid regular expression will fail the compilation. For example:
/// ```compile_fail
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     regex("[0-9+")
/// };
/// ```
/// ## `any`
/// Must have at least two dsl arguments. Make sure that at least of of them is valid. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any {
///         "Foo",
///         eq("Bar"),
///         regex("[0-9]+")
///     }
/// };
/// assert!(regex.is_match("Foo"));
/// assert!(regex.is_match("Bar"));
/// assert!(regex.is_match("100"));
/// assert!(!regex.is_match("Bor"));
/// ```
/// ## `concat`
/// Must have at least two dsl arguments. Make sure that all of of them is valid and in the  correct order. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     concat {
///         "Foo",
///         eq("Bar"),
///         regex("[0-9]+")
///     }
/// };
/// assert!(regex.is_match("FooBar300"));
/// assert!(!regex.is_match("Bar300"));
/// assert!(!regex.is_match("FooBar"));
/// ```
/// ## `any_of`
/// Creates a user class. The arguments can be:
/// * A string - in that case, the class will include all the characters in the string. For example:
///
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         "Bar",
///     }
/// };
/// assert!(regex.is_match("B"));
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("r"));
/// assert!(!regex.is_match("F"));
/// ```
/// * A character - in that case, the class will include the character. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         '1',
///         '2',
///         "Bar",
///     }
/// };
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("2"));
/// assert!(regex.is_match("B"));
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("r"));
/// assert!(!regex.is_match("F"));
/// assert!(!regex.is_match("3"));
/// ```
/// * A predefined unicode class - in that case, the class will include the characters in that class. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         #digit,
///         "Bar",
///     }
/// };
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("8"));
/// assert!(regex.is_match("B"));
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("r"));
/// assert!(!regex.is_match("F"));
/// ```
/// * Not a predefined unicode class - in that case, the class will include anything by the characters in that class. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         ~digit,
///     }
/// };
/// assert!(!regex.is_match("1"));
/// assert!(!regex.is_match("8"));
/// assert!(regex.is_match("B"));
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("r"));
/// ```
/// * A range of characters (from: 'a' to: 'z') - in that case, the class will include all the charactes in that range. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         '1',
///         from: '4' to: '6'
///     }
/// };
/// assert!(!regex.is_match("0"));
/// assert!(regex.is_match("1"));
/// assert!(!regex.is_match("2"));
/// assert!(!regex.is_match("3"));
/// assert!(regex.is_match("4"));
/// assert!(regex.is_match("5"));
/// assert!(regex.is_match("6"));
/// assert!(!regex.is_match("7"));
/// ```
/// * A special case (see special cases above). For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         'a',
///         digit,
///     }
/// };
/// assert!(regex.is_match("0"));
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("a"));
/// assert!(!regex.is_match("b"));
/// ```
/// * `intersect(<user_class>)` - An intersection of two user created class. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         from: '2' to: '5',
///         intersect {
///            from: '3' to: '7',
///         }
///     }
/// };
/// assert!(!regex.is_match("2"));
/// assert!(regex.is_match("3"));
/// assert!(regex.is_match("4"));
/// assert!(regex.is_match("5"));
/// assert!(!regex.is_match("6"));
/// assert!(!regex.is_match("7"));
/// ```
/// * `subtract(<user_class>)` - Remove characters from the class. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         from: '2' to: '7',
///         subtract {
///            '3',
///            '5'
///         }
///     }
/// };
/// assert!(regex.is_match("2"));
/// assert!(!regex.is_match("3"));
/// assert!(regex.is_match("4"));
/// assert!(!regex.is_match("5"));
/// assert!(regex.is_match("6"));
/// assert!(regex.is_match("7"));
/// ```
/// * `xor(<user_class>)` - Symmetric difference between the classes. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     any_of {
///         from: '2' to: '5',
///         xor {
///            from: '3' to: '7',
///         }
///     }
/// };
/// assert!(regex.is_match("2"));
/// assert!(!regex.is_match("3"));
/// assert!(!regex.is_match("4"));
/// assert!(!regex.is_match("5"));
/// assert!(regex.is_match("6"));
/// assert!(regex.is_match("7"));
/// ```
/// ## `not_any_of`
/// Revert a user class. The arguments are the same as the user class above. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     not_any_of {
///         '1',
///         from: '4' to: '6'
///     }
/// };
/// assert!(regex.is_match("0"));
/// assert!(!regex.is_match("1"));
/// assert!(regex.is_match("2"));
/// assert!(regex.is_match("3"));
/// assert!(!regex.is_match("4"));
/// assert!(!regex.is_match("5"));
/// assert!(!regex.is_match("6"));
/// assert!(regex.is_match("7"));
/// ```
/// ## `group`
/// Create a capture group.
/// If the argument is a DSL, the group will be unnamed. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "|",
///     group {
///         digit,
///     },
///     "|"
/// };
/// let caps = regex.captures("|4|").unwrap();
/// assert_eq!(caps.get(1).unwrap().as_str(), "4");
/// ```
/// If there is a `name:<name>` argument, the group will be named `name`. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "|",
///     group {
///         name: a_digit,
///         digit,
///     },
///     "|"
/// };
/// let caps = regex.captures("|4|").unwrap();
/// assert_eq!(&caps["a_digit"], "4");
/// ```
///
/// One can have more than one group in an expression. For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
/// let regex = regex_dsl! {
///     "|",
///     group {
///         group {
///             name: number_1,
///             regex("[0-9]+"),
///         },
///         ",",
///         group {
///             name: number_2,
///             regex("[0-9]+"),
///         },
///     }
///     "|"
/// };
/// let caps = regex.captures("|100,400|").unwrap();
///
/// assert_eq!(&caps["number_1"], "100");
/// assert_eq!(&caps["number_2"], "400");
/// assert_eq!(caps.get(1).unwrap().as_str(), "100,400");
/// ```
///
/// ## `maybe`
/// Represent a regular expression that appears once or not at all (i.e. equivalent to the `?` operator).
/// For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     maybe {
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<>"));
/// assert!(regex.is_match("<4>"));
/// assert!(!regex.is_match("<a>"));
/// ```
/// To use a lazy quantifier, use the keyword `lazy`, that is, something like:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     maybe {
///         lazy,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<>"));
/// assert!(regex.is_match("<4>"));
/// assert!(!regex.is_match("<a>"));
/// ```
///
/// ## `repeat`
/// Represent a regular expression that appears once or more (i.e. equivalent to the `+` operator).
/// For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     repeat {
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1>"));
/// assert!(regex.is_match("<123>"));
/// assert!(!regex.is_match("<>"));
/// assert!(!regex.is_match("<a>"));
/// ```
/// To use a lazy quantifier, use the keyword `lazy`, that is, something like:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     repeat {
///         lazy,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1>"));
/// assert!(regex.is_match("<123>"));
/// assert!(!regex.is_match("<>"));
/// assert!(!regex.is_match("<a>"));
/// ```
///
/// ## `maybe_repeat`
/// Represent a regular expression that can appears a few times or never (i.e. equivalent to the `*` operator).
/// For example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     maybe_repeat {
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1>"));
/// assert!(regex.is_match("<123>"));
/// assert!(regex.is_match("<>"));
/// assert!(!regex.is_match("<a>"));
/// ```
/// To use a lazy quantifier, use the keyword `lazy`, that is, something like:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     maybe_repeat {
///         lazy,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1>"));
/// assert!(regex.is_match("<123>"));
/// assert!(regex.is_match("<>"));
/// assert!(!regex.is_match("<a>"));
/// ```
///
/// ## `times`
/// Represent a regular expression that can appears a few times.
/// * One can specify exactly how many times, for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     times {
///         exactly: 4
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1234>"));
/// assert!(!regex.is_match("<1>"));
/// assert!(!regex.is_match("<123>"));
/// assert!(!regex.is_match("<>"));
/// assert!(!regex.is_match("<a>"));
/// ```
/// * One can specify lowest limit of the number of times, for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     times {
///         at_least: 2,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<12>"));
/// assert!(regex.is_match("<123>"));
/// assert!(regex.is_match("<1234>"));
/// assert!(regex.is_match("<12345>"));
/// assert!(!regex.is_match("<>"));
/// assert!(!regex.is_match("<1>"));
/// ```
/// * One can specify a range of the number of times, for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     times {
///         at_least: 2,
///         at_most: 4,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<1234>"));
/// assert!(regex.is_match("<123>"));
/// assert!(regex.is_match("<12>"));
/// assert!(!regex.is_match("<1>"));
/// assert!(!regex.is_match("<>"));
/// assert!(!regex.is_match("<12345>"));
/// ```
/// To use a lazy quantifier, use the keyword `lazy`, that is, something like:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     "<",
///     times {
///         lazy,
///         exactly: 2,
///         digit,
///     },
///     ">"
/// };
/// assert!(regex.is_match("<12>"));
/// assert!(!regex.is_match("<1>"));
/// assert!(!regex.is_match("<>"));
/// ```
///
/// ## `apply`
/// Apply regular expression flags.
/// The available flags are:
/// * `case_insensitive` - When the flag is set, letters match both upper and lower case - equivalent to the `i` flag.
/// * `multi_line` - When the flag is set, `beginning_of_line` and end_of_line match begin/end of line and not input - equivalent to the `m` flag.
/// * `allow_dot` - When the flag is set, allow `any_character` to match a new line - equivalent to the `s` flag.
/// * `enables_crlf_mode` - When `multi_line` flag is set, `\r\n` is used - - equivalent to the `R` flag.
/// To set a flag, use: `+<flag>`, to unset it, use: `-<flag>`.
/// One can apply flags on a specific DSL, by adding the DSL as an argument to the apply function - for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     apply {
///         +case_insensitive,
///         -multi_line,
///         "foo"
///     }
/// };
/// assert!(regex.is_match("foo"));
/// assert!(regex.is_match("Foo"));
/// ```
/// One can apply flags on the rest of the DSLs, by leaving just the flags as a arguments to the apply function - for example:
/// ```rust
/// use rust_regex_dsl::regex_dsl;
///
/// let regex = regex_dsl! {
///     apply {
///         +case_insensitive,
///     },
///     eq("foo"),
///     apply {
///         -case_insensitive,
///     },
///     eq("Bar"),
/// };
/// assert!(regex.is_match("FooBar"));
/// assert!(regex.is_match("fooBar"));
/// assert!(!regex.is_match("fooBAR"));
/// ```
///
#[proc_macro]
pub fn regex_dsl(input: TokenStream) -> TokenStream {
    let dsls: Vec<Dsl> = parse_macro_input!(input with parse_list_to_vec);
    let dsl = Dsl::concat(&dsls);
    let error_factory = ErrorFactory::new_root();
    dsl.build(error_factory).into()
}

/// A macro to create a Regular Expression capturing struct.
///
/// This macro will create an helper struct for capture regular expression groups for a hard coded regular expression.
///
/// The macro has two arguments, the first one should be the name of the struct to create and the second one is the regular expression,
/// either as raw regular expression or as a DSL (See [regex_dsl] for syntax).
///
/// The struct will be a tuple with the first filed as a `&str` with the entire capture text (i.e. capture group 0) and the rest will be
/// `Option<&str>` with the content of the capture group (for any capture group in the regular expression). That is, for `[a-z]+` the struct will have
/// only `(&str)` and for `([a-z]+)([0-9]+)([a-z]+)` the struct will have `(&str, Option<&str>, Option<&str>, Option<&str>)`. If the regular expression has any named
/// capture groups, the struct will have a public function to retrieve them with the same name as the group name (snake case). The struct will have a `get_capture` method that will return the first member of the tuple.
///
/// The struct will have two static public functions: `catch` that accept a string reference and return an Option with the struct if it matches the argument and
/// `catch_all` that accept a string reference and return an Iterator over all the places the expression was caught in the argument.
///
/// Please note, The capturing groups are optionals for regular expressions like: `([a-z]+)|([0-9]+)`.
///
/// For example:
/// ```rust
/// use rust_regex_dsl::create_capture;
///
/// create_capture!(MyRegexCapture, "([0-9]+)");
///
/// assert!(MyRegexCapture::catch("Foo").is_none());
/// let caught = MyRegexCapture::catch("33").unwrap();
/// assert_eq!(caught.0, "33");
/// assert_eq!(caught.get_capture(), "33");
/// assert_eq!(caught.1, Some("33"));
///
/// let all: Vec<_> = MyRegexCapture::catch_all("100 90 80").collect();
/// assert_eq!(all.len(), 3);
/// assert_eq!(all[0].0, "100");
/// assert_eq!(all[1].0, "90");
/// assert_eq!(all[2].0, "80");
/// ```
/// A more complex example:
/// ```rust
/// use rust_regex_dsl::create_capture;
///
/// create_capture!(MyRegexCapture,
///   any {
///     group {
///       name: Letters,
///       repeat {
///          any_of {
///            #letter
///          }
///        }
///     }, group {
///       name: Digits,
///       repeat {
///          any_of {
///            #digit
///          }
///        }
///    }
///   });
///
/// let caught = MyRegexCapture::catch("hello").unwrap();
/// assert_eq!(caught.0, "hello");
/// assert_eq!(caught.get_capture(), "hello");
/// assert_eq!(caught.1, Some("hello"));
/// assert_eq!(caught.letters(), Some("hello"));
/// assert_eq!(caught.2, None);
/// assert_eq!(caught.digits(), None);
///
/// let caught = MyRegexCapture::catch("321").unwrap();
/// assert_eq!(caught.0, "321");
/// assert_eq!(caught.get_capture(), "321");
/// assert_eq!(caught.1, None);
/// assert_eq!(caught.letters(), None);
/// assert_eq!(caught.2, Some("321"));
/// assert_eq!(caught.digits(), Some("321"));
///
/// let all: Vec<_> = MyRegexCapture::catch_all("A1234B33").collect();
/// assert_eq!(all.len(), 4);
/// assert_eq!(all[0].1, Some("A"));
/// assert_eq!(all[1].2, Some("1234"));
/// assert_eq!(all[2].letters(), Some("B"));
/// assert_eq!(all[3].digits(), Some("33"));
/// ```
///
#[proc_macro]
pub fn create_capture(input: TokenStream) -> TokenStream {
    let create_capture = parse_macro_input!(input as CreateCapture);
    let error_factory = ErrorFactory::new_root();
    create_capture.build(error_factory).into()
}
