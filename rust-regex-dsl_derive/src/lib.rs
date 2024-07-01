use dsl::Dsl;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
mod dsl;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let regex = input.value();

    let dsl = Dsl::new(regex.as_str());
    dsl.build(input.span()).into()
}
