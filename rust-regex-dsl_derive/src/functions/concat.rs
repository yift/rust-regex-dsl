use syn::{parse::ParseBuffer, Result};

use crate::dsl::Dsl;

use super::parse_list::parse_list_to_vec;

pub fn parse_concat(group: &ParseBuffer) -> Result<Dsl> {
    let items = parse_list_to_vec(group)?;
    if items.len() < 2 {
        return Err(group.error("Concat must have at least two elements".to_string()));
    }

    let dsl = Dsl::concat(&items);
    Ok(dsl)
}
