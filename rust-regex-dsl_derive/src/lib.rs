use dsl::Dsl;
use error_factory::ErrorFactory;
use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, LitStr};
mod dsl;
mod error_factory;
mod functions;
mod group;
mod ident_parser;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let dsl = Dsl::new(&input.value(), input.value().len() > 1);
    let error_factory = ErrorFactory::new_obj(input.span());
    dsl.build(error_factory).into()
}
#[proc_macro]
pub fn regex_dsl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<Dsl, syn::Token![,]>::parse_terminated);
    let dsls: Vec<_> = args.iter().collect();
    let dsl = Dsl::concat(&dsls);
    let error_factory = ErrorFactory::new_root();
    dsl.build(error_factory).into()
}
