use syn::{
    parse::ParseBuffer,
    punctuated::Punctuated,
    token::{Colon, Comma},
    Ident, Result,
};

use crate::dsl::Dsl;

pub fn parse_capture_group(group: &ParseBuffer) -> Result<Dsl> {
    let lookahead = group.lookahead1();
    if lookahead.peek(Ident) {
        let fork = group.fork();
        let ident: Ident = fork.parse()?;
        if ident == "name" {
            let _: Ident = group.parse()?;
            let _: Colon = group.parse()?;
            let name: Ident = group.parse()?;
            let name = format!("?<{}>", name);
            let lookahead = group.lookahead1();
            if lookahead.peek(Comma) {
                let _: Comma = group.parse()?;
            }
            return parse_group_with_name(&name, group);
        }
    }
    parse_group_with_name("", group)
}
fn parse_group_with_name(name_prefix: &str, group: &ParseBuffer) -> Result<Dsl> {
    let items: Punctuated<Dsl, syn::Token![,]> = Punctuated::parse_terminated(group)?;
    if items.is_empty() {
        return Err(group.error("Nothing to capture in the group".to_string()));
    }
    let items: Vec<_> = items.iter().collect();
    let dsl = Dsl::concat(&items).group(name_prefix);
    Ok(dsl)
}
