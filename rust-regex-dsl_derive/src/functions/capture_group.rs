use syn::{
    parse::ParseBuffer,
    token::{Colon, Comma},
    Ident, Result,
};

use crate::dsl::Dsl;

use super::{parse_ident::parse_ident, parse_list::parse_list_to_vec};

pub fn parse_capture_group(group: &ParseBuffer) -> Result<Dsl> {
    let name = if parse_ident(group, "name")? {
        let _: Colon = group.parse()?;
        let name: Ident = group.parse()?;
        let lookahead = group.lookahead1();
        if lookahead.peek(Comma) {
            let _: Comma = group.parse()?;
        }
        &format!("?<{}>", name)
    } else {
        ""
    };
    parse_group_with_name(name, group)
}
fn parse_group_with_name(name_prefix: &str, group: &ParseBuffer) -> Result<Dsl> {
    let items = parse_list_to_vec(group)?;
    if items.is_empty() {
        return Err(group.error("Nothing to capture in the group".to_string()));
    }
    let dsl = Dsl::concat(&items).group(name_prefix);
    Ok(dsl)
}
