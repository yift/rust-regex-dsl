use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::parse::Parse;
use syn::token::Comma;

use crate::dsl::Dsl;
use crate::error_factory::ErrorFactory;
use crate::functions::parse_list::parse_list_to_vec;
use syn::parse::ParseStream;
use syn::{Ident, Index, LitStr, Result};

pub struct CreateCapture {
    struct_name: String,
    regex: String,
}
impl Parse for CreateCapture {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name: Ident = input.parse()?;
        let struct_name = struct_name.to_string();
        let lookahead = input.lookahead1();
        if lookahead.peek(Comma) {
            let _: Comma = input.parse()?;
        }
        let lookahead = input.lookahead1();
        let regex = if lookahead.peek(LitStr) {
            let regex: LitStr = input.parse()?;
            regex.value()
        } else {
            let items: Vec<Dsl> = parse_list_to_vec(input)?;
            if items.is_empty() {
                return Err(input.error("Nothing to capture"));
            }

            let dsl = Dsl::concat(&items);
            dsl.regex().to_string()
        };
        Ok(CreateCapture { struct_name, regex })
    }
}
impl CreateCapture {
    pub fn build(&self, error_factory: ErrorFactory) -> TokenStream {
        let regex_str = self.regex.as_str();
        let regex = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(e) => {
                return error_factory.error(format!("Invalid regular expression: {}", e));
            }
        };
        if regex
            .capture_names()
            .flatten()
            .any(|nm| nm == "get_capture")
        {
            return error_factory.error("The name get_capture in not supported".to_string());
        }
        let struct_name = format_ident!("{}", self.struct_name);
        let names = regex.capture_names();
        let len = names.len();
        let regex_name = format_ident!("{}_REGEX", self.struct_name.to_case(Case::UpperSnake));
        let define_regex = quote! {
            rust_regex_dsl::lazy_static! {
                static ref #regex_name: rust_regex_dsl::Regex = rust_regex_dsl::Regex::new(#regex_str).unwrap();
            }
        };

        let args: Vec<_> = (1..len).map(|_| quote! {, Option<&'h str>}).collect();
        let define_struct = quote! {
            #[derive(Debug)]
            struct #struct_name<'h>(&'h str #(#args)*);
        };

        let get_capture = quote! {
            pub fn get_capture(&self) -> &'h str {
                 self.0
            }
        };

        let get_names: Vec<_> = names
            .into_iter()
            .enumerate()
            .filter_map(|(i, cap)| cap.map(|c| (i, c)))
            .map(|(i, name)| {
                let name = format_ident!("{}", name.to_case(Case::Snake));
                let i = Index::from(i);
                quote! {
                    pub fn #name(&self) -> Option<&'h str> {
                         self.#i
                    }
                }
            })
            .collect();
        let capture_args: Vec<_> = (1..len)
            .map(|i| {
                let i = Index::from(i);
                quote! {
                    , captures.get(#i).map(|c| c.as_str())
                }
            })
            .collect();

        let new = quote! {
            fn new(captures: rust_regex_dsl::Captures<'h>) -> Self {
                Self(captures.get(0).unwrap().as_str() #(#capture_args)*)
            }
        };
        let capture = quote! {
            pub fn catch(haystack: &'h str) -> Option<#struct_name<'h>> {
                #regex_name.captures(haystack).map(Self::new)
            }
        };
        let capture_all = quote! {
            pub fn catch_all(haystack: &'h str) -> impl Iterator<Item = #struct_name<'h>> {
                #regex_name.captures_iter(haystack).map(Self::new)
            }
        };
        let get_regex = quote! {
            pub fn regex() -> &'static rust_regex_dsl::Regex {
                &*#regex_name
            }
        };
        let impl_getters = quote! {
            impl <'h> #struct_name<'h> {
                #new
                #get_capture
                #(#get_names)*
                #capture
                #capture_all
                #get_regex
            }
        };
        quote! {
            #define_regex
            #define_struct
            #impl_getters
        }
    }
}
