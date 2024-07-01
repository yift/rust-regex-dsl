use syn::{parse::ParseBuffer, LitStr, Result};

use crate::dsl::Dsl;

pub fn parse_eq(group: &ParseBuffer) -> Result<Dsl> {
    let str: LitStr = group.parse()?;
    let dsl = Dsl::eq(&str.value());
    Ok(dsl)
}
