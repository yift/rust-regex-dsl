use syn::{LitStr, Result, parse::ParseBuffer};

use crate::dsl::Dsl;

pub fn parse_eq(group: &ParseBuffer) -> Result<Dsl> {
    let str: LitStr = group.parse()?;
    let dsl = Dsl::eq(&str.value());
    Ok(dsl)
}
