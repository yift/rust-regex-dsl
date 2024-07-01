use crate::dsl::Dsl;
use itertools::Itertools;
use syn::{parse::ParseBuffer, punctuated::Punctuated, Result};

pub fn parse_any(group: &ParseBuffer) -> Result<Dsl> {
    let items: Punctuated<Dsl, syn::Token![,]> = Punctuated::parse_terminated(group)?;
    if items.len() < 2 {
        return Err(group.error("Concat must have at least two elements".to_string()));
    }
    let regex = items
        .iter()
        .map(Dsl::non_capturing_group_if_needed)
        .join("|");

    let dsl = Dsl::new(&regex, true);
    Ok(dsl)
}
