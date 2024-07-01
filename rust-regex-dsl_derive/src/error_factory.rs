use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;
pub enum ErrorFactory {
    ObjectErrorFactory(Span),
    RootErrorFactory,
}
impl ErrorFactory {
    pub fn new_root() -> Self {
        ErrorFactory::RootErrorFactory
    }
    pub fn new_obj(span: Span) -> Self {
        ErrorFactory::ObjectErrorFactory(span)
    }
    pub fn error(&self, message: String) -> TokenStream {
        match self {
            ErrorFactory::RootErrorFactory => quote! { compile_error!(#message) },
            ErrorFactory::ObjectErrorFactory(span) => Error::new(*span, message).to_compile_error(),
        }
    }
}
