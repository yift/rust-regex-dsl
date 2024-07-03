use dsl::Dsl;
use error_factory::ErrorFactory;
use functions::parse_list::parse_list_to_vec;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
mod dsl;
mod error_factory;
mod functions;
mod group;
mod ident_parser;
mod predefined_class;
mod user_class;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let dsl = Dsl::new(&input.value(), input.value().len() > 1);
    let error_factory = ErrorFactory::new_obj(input.span());
    dsl.build(error_factory).into()
}
#[proc_macro]
pub fn regex_dsl(input: TokenStream) -> TokenStream {
    let dsls: Vec<Dsl> = parse_macro_input!(input with parse_list_to_vec);
    let dsl = Dsl::concat(&dsls);
    let error_factory = ErrorFactory::new_root();
    dsl.build(error_factory).into()
}
