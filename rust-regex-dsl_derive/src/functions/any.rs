use crate::dsl::Dsl;
use itertools::Itertools;
use syn::{parse::ParseBuffer, Result};

use super::parse_list::parse_list_to_vec;

pub fn parse_any(group: &ParseBuffer) -> Result<Dsl> {
    let items = parse_list_to_vec(group)?;
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
