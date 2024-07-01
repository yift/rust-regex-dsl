use proc_macro2::{Literal, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, LitChar, LitStr,
};

use crate::{error_factory::ErrorFactory, ident_parser::parse_ident};

#[derive(Debug, Clone)]
pub struct Dsl {
    regex: String,
}

impl Dsl {
    pub fn new(regex: &str) -> Self {
        Dsl {
            regex: regex.into(),
        }
    }

    pub fn eq(string: &str) -> Self {
        let regex: String = string
            .chars()
            .map(|c| match c {
                '\\' | '\"' | '.' | ')' | '(' | '{' | '}' | '[' | ']' | '$' | '^' | '?' | '+'
                | '*' | '|' => {
                    format!("\\{}", c)
                }
                _ => format!("{}", c),
            })
            .collect();
        Dsl { regex }
    }
    pub fn validate(&self) -> Option<String> {
        if self.regex.is_empty() {
            return Some("Empty regex is not supported".to_string());
        }
        if let Err(e) = Regex::new(self.regex.as_str()) {
            return Some(format!("{}", e));
        }
        None
    }

    pub fn build(&self, error_factory: ErrorFactory) -> TokenStream {
        if let Some(err) = self.validate() {
            return error_factory.error(err);
        }

        let lit = Literal::string(self.regex.as_str());
        quote! {
            rust_regex_dsl::Regex::new(#lit).unwrap()
        }
    }

    pub fn concat(dsls: &[&Dsl]) -> Self {
        let regex: String = dsls.iter().map(|dsl| dsl.regex.as_str()).collect();
        Dsl { regex }
    }
}

impl Parse for Dsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let str: LitStr = input.parse()?;
            if str.value().is_empty() {
                Err(Error::new(str.span(), "Empty string is not supported"))
            } else {
                Ok(Dsl::eq(&str.value()))
            }
        } else if lookahead.peek(LitChar) {
            let chr: LitChar = input.parse()?;
            let str = format!("{}", chr.value());
            Ok(Dsl::eq(&str))
        } else if lookahead.peek(Ident) {
            parse_ident(input)
        } else {
            Err(lookahead.error())
        }
    }
}
