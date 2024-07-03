use syn::{parse::ParseBuffer, Ident, Result};

pub fn parse_ident(group: &ParseBuffer, name: &str) -> Result<bool> {
    let lookahead = group.lookahead1();
    if lookahead.peek(Ident) {
        let fork = group.fork();
        let ident: Ident = fork.parse()?;
        if ident == name {
            let _: Ident = group.parse()?;
            return Ok(true);
        }
    }
    Ok(false)
}
