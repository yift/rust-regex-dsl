use syn::{parse::ParseBuffer, punctuated::Punctuated, Result};

use crate::dsl::Dsl;

pub fn parse_concat(group: &ParseBuffer) -> Result<Dsl> {
    let items: Punctuated<Dsl, syn::Token![,]> = Punctuated::parse_terminated(group)?;
    if items.len() < 2 {
        return Err(group.error("Concat must have at least two elements".to_string()));
    }
    let items: Vec<_> = items.iter().collect();

    let dsl = Dsl::concat(&items);
    Ok(dsl)
}
