use regex::Regex;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::{Error, Ident, Result, Token};

use crate::dsl::Dsl;

pub struct PredefineClass {
    pub regex: String,
}

impl PredefineClass {
    pub fn to_dsl(&self) -> Dsl {
        Dsl::new(&self.regex, false)
    }
}

impl Parse for PredefineClass {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let letter = if lookahead.peek(Token![#]) {
            let _: Token![#] = input.parse()?;
            'p'
        } else if lookahead.peek(Token![~]) {
            let _: Token![~] = input.parse()?;
            'P'
        } else {
            return Err(lookahead.error());
        };
        let ident: Ident = input.parse()?;
        let class_name = ident.to_string();
        let regex = format!("\\{}{{{}}}", letter, class_name);
        if Regex::new(regex.as_str()).is_err() {
            return Err(Error::new(
                ident.span(),
                format!("Unknown regex class {}", class_name),
            ));
        }
        Ok(PredefineClass { regex })
    }
}
