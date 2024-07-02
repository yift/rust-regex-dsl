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
use crate::group::parse_group;
use crate::ident_parser::parse_single_word;
use crate::predefined_class::PredefineClass;

#[derive(Debug)]
pub struct UserClass {
    contains: UserClassInternal,
    use_me: bool,
}
#[derive(Debug)]
pub struct UserClassInternal {
    contains: String,
}
#[derive(Debug)]
struct Range {
    from: char,
    to: char,
}
#[derive(Debug)]
enum ClassOperationType {
    Intersection,
    Subtraction,
    SymmetricDifference,
}
#[derive(Debug)]
struct ClassOperation {
    operation_type: ClassOperationType,
    cls: UserClassInternal,
}
#[derive(Debug)]
enum UserClassElement {
    Char(char),
    String(String),
    PredefineClass(PredefineClass),
    SingleWord(String),
    Range(Range),
    ClassOperation(ClassOperation),
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
impl Escape for ClassOperation {
    fn escape(&self) -> String {
        match self.operation_type {
            ClassOperationType::Intersection => format!("&&[{}]", self.cls.contains),
            ClassOperationType::Subtraction => format!("&&[^{}]", self.cls.contains),
            ClassOperationType::SymmetricDifference => format!("~~[{}]", self.cls.contains),
        }
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
            UserClassElement::ClassOperation(op) => op.escape(),
        }
    }
}

impl UserClass {
    pub fn to_dsl(&self) -> Dsl {
        let prefix = if self.use_me { "" } else { "^" };
        let regex = format!("[{}{}]", prefix, self.contains.contains);
        Dsl::new(&regex, false)
    }
    pub fn parse(input: ParseStream, use_me: bool) -> Result<Self> {
        let contains = input.parse()?;
        Ok(UserClass { contains, use_me })
    }
}

impl Parse for UserClassInternal {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Punctuated<UserClassElement, syn::Token![,]> =
            Punctuated::parse_terminated(input)?;
        if items.is_empty() {
            return Err(input.error("Empty class is not supported"));
        }
        let contains = items.iter().map(Escape::escape).collect();
        Ok(UserClassInternal { contains })
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
            match ident.to_string().as_str() {
                "from" => {
                    let range: Range = input.parse()?;
                    Ok(UserClassElement::Range(range))
                }
                "intersect" => {
                    let operation = ClassOperation::parse(ClassOperationType::Intersection, input)?;
                    Ok(UserClassElement::ClassOperation(operation))
                }
                "subtract" => {
                    let operation = ClassOperation::parse(ClassOperationType::Subtraction, input)?;
                    Ok(UserClassElement::ClassOperation(operation))
                }
                "xor" => {
                    let operation =
                        ClassOperation::parse(ClassOperationType::SymmetricDifference, input)?;
                    Ok(UserClassElement::ClassOperation(operation))
                }
                _ => {
                    let dsl = parse_single_word(ident)?;
                    Ok(UserClassElement::SingleWord(
                        dsl.non_capturing_group_if_needed(),
                    ))
                }
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

impl ClassOperation {
    fn parse(operation_type: ClassOperationType, input: ParseStream) -> Result<Self> {
        let Some(group) = parse_group(input)? else {
            return Err(input.error("Missing content"));
        };
        let cls = group.parse()?;
        Ok(ClassOperation {
            operation_type,
            cls,
        })
    }
}
