use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::Error;

#[derive(Debug)]
pub struct Dsl<'h> {
    regex: &'h str,
}

impl<'h> Dsl<'h> {
    pub fn new(regex: &'h str) -> Self {
        Dsl { regex }
    }

    pub fn build(&self, span: Span) -> TokenStream {
        if self.regex.is_empty() {
            return Error::new(span, "Empty regex is not supported".to_string()).to_compile_error();
        }
        if let Err(e) = Regex::new(self.regex) {
            return Error::new(span, format!("{}", e)).to_compile_error();
        }

        let lit = Literal::string(self.regex);

        quote! {
            rust_regex_dsl::Regex::new(#lit).unwrap()
        }
    }
}
