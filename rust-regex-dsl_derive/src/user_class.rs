use itertools::Itertools;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::token::Comma;
use syn::LitChar;
use syn::LitStr;
use syn::{Error, Ident, Result, Token};

use crate::dsl::Dsl;
use crate::ident_parser::parse_single_word;
use crate::predefined_class::PredefineClass;

pub struct UserClass {
    contains: String,
    use_me: bool,
}

struct Range {
    from: char,
    to: char,
}
enum UserClassElement {
    Char(char),
    String(String),
    PredefineClass(PredefineClass),
    SingleWord(String),
    Range(Range),
}
trait Escape {
    fn escape(&self) -> String;
}
impl Escape for char {
    fn escape(&self) -> String {
        match self {
            '-' | ']' | '[' | '^' | '&' | '.' | '\\' => format!("\\{}", self),
            _ => format!("{}", self),
        }
    }
}
impl Escape for String {
    fn escape(&self) -> String {
        self.chars().unique().map(|c| c.escape()).join("")
    }
}
impl Escape for Range {
    fn escape(&self) -> String {
        format!("{}-{}", self.from.escape(), self.to.escape())
    }
}
impl Escape for UserClassElement {
    fn escape(&self) -> String {
        match self {
            UserClassElement::Char(chr) => chr.escape(),
            UserClassElement::String(str) => str.escape(),
            UserClassElement::PredefineClass(cls) => cls.regex.clone(),
            UserClassElement::SingleWord(regex) => regex.clone(),
            UserClassElement::Range(range) => range.escape(),
        }
    }
}

impl UserClass {
    pub fn to_dsl(&self) -> Dsl {
        let prefix = if self.use_me { "" } else { "^" };
        let regex = format!("[{}{}]", prefix, self.contains);
        Dsl::new(&regex, false)
    }
    pub fn parse(input: ParseStream, use_me: bool) -> Result<Self> {
        let items: Punctuated<UserClassElement, syn::Token![,]> =
            Punctuated::parse_terminated(input)?;
        if items.is_empty() {
            return Err(input.error("Empty class is not supported"));
        }
        let contains = items.iter().map(Escape::escape).collect();
        Ok(UserClass { contains, use_me })
    }
}

impl Parse for UserClassElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let str: LitStr = input.parse()?;
            if str.value().is_empty() {
                Err(Error::new(str.span(), "Empty string is not supported"))
            } else {
                Ok(UserClassElement::String(str.value()))
            }
        } else if lookahead.peek(LitChar) {
            let chr: LitChar = input.parse()?;
            Ok(UserClassElement::Char(chr.value()))
        } else if lookahead.peek(Token![#]) || lookahead.peek(Token![~]) {
            let cls: PredefineClass = input.parse()?;
            Ok(UserClassElement::PredefineClass(cls))
        } else if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            if ident == "from" {
                let range: Range = input.parse()?;
                Ok(UserClassElement::Range(range))
            } else {
                let dsl = parse_single_word(ident)?;
                Ok(UserClassElement::SingleWord(
                    dsl.non_capturing_group_if_needed(),
                ))
            }
        } else {
            Err(lookahead.error())
        }
    }
}
impl Parse for Range {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Colon = input.parse()?;
        let from: LitChar = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Comma) {
            let _: Comma = input.parse()?;
        }
        let ident: Ident = input.parse()?;
        if ident != "to" {
            return Err(Error::new(ident.span(), "Expecting to"));
        }
        let _: Colon = input.parse()?;
        let to: LitChar = input.parse()?;
        Ok(Range {
            from: from.value(),
            to: to.value(),
        })
    }
}
