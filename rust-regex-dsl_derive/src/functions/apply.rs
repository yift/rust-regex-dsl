use std::collections::HashSet;

use crate::dsl::Dsl;
use itertools::Itertools;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token::Minus;
use syn::token::Plus;
use syn::Result;

use super::parse_ident::parse_ident;
use super::parse_list::parse_list;

enum Flag {
    CaseInsensitive,
    MultiLine,
    AllowDot,
    EnablesCrlfMode,
}
impl Flag {
    fn symbol(&self) -> char {
        match self {
            Flag::CaseInsensitive => 'i',
            Flag::AllowDot => 's',
            Flag::EnablesCrlfMode => 'R',
            Flag::MultiLine => 'm',
        }
    }
}
impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        if parse_ident(input, "case_insensitive")? {
            Ok(Flag::CaseInsensitive)
        } else if parse_ident(input, "multi_line")? {
            Ok(Flag::MultiLine)
        } else if parse_ident(input, "allow_dot")? {
            Ok(Flag::AllowDot)
        } else if parse_ident(input, "enables_crlf_mode")? {
            Ok(Flag::EnablesCrlfMode)
        } else {
            Err(input.error("Unknown flag"))
        }
    }
}
enum Element {
    Dsl(Dsl),
    FlagToAdd(Flag),
    FlagToRemove(Flag),
}
impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Plus) {
            let _: Plus = input.parse()?;
            let flag: Flag = input.parse()?;
            Ok(Element::FlagToAdd(flag))
        } else if lookahead.peek(Minus) {
            let _: Minus = input.parse()?;
            let flag: Flag = input.parse()?;
            Ok(Element::FlagToRemove(flag))
        } else {
            Ok(Element::Dsl(input.parse()?))
        }
    }
}
struct Apply {
    flags_to_add: HashSet<char>,
    flags_to_remove: HashSet<char>,
    dsl: Option<Dsl>,
}
impl Apply {
    fn to_dsl(&self) -> Dsl {
        let flags_to_add: String = self.flags_to_add.iter().join("");
        let flags_to_remove: String = if self.flags_to_remove.is_empty() {
            "".into()
        } else {
            format!("-{}", self.flags_to_remove.iter().join(""))
        };
        let dsl = self
            .dsl
            .as_ref()
            .map(|d| format!(":{}", d.regex()))
            .unwrap_or_default();
        let regex = format!("(?{}{}{})", flags_to_add, flags_to_remove, dsl);
        Dsl::new(&regex, false)
    }
}
impl Parse for Apply {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut flags_to_add = HashSet::new();
        let mut flags_to_remove = HashSet::new();
        let mut dsl = None;
        parse_list(input, |element| {
            match element {
                Element::Dsl(regex) => {
                    if dsl.is_some() {
                        return Err(input.error("Can not apply on two regular expressions"));
                    }
                    dsl = Some(regex);
                }
                Element::FlagToAdd(flag) => {
                    let flag = flag.symbol();
                    if flags_to_remove.contains(&flag) {
                        return Err(input.error("Can not add and remove the same flag"));
                    }
                    flags_to_add.insert(flag);
                }
                Element::FlagToRemove(flag) => {
                    let flag = flag.symbol();
                    if flags_to_add.contains(&flag) {
                        return Err(input.error("Can not add and remove the same flag"));
                    }
                    flags_to_remove.insert(flag);
                }
            }
            Ok(())
        })?;
        if flags_to_add.is_empty() && flags_to_remove.is_empty() {
            return Err(input.error("No flags to apply"));
        }
        Ok(Apply {
            flags_to_add,
            flags_to_remove,
            dsl,
        })
    }
}
pub fn parse_apply(group: ParseStream) -> Result<Dsl> {
    let apply: Apply = group.parse()?;

    Ok(apply.to_dsl())
}
