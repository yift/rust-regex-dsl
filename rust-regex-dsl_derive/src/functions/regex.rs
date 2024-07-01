use syn::{parse::ParseBuffer, Error, LitStr, Result};

use crate::dsl::Dsl;

pub fn parse_regex(group: &ParseBuffer) -> Result<Dsl> {
    let str: LitStr = group.parse()?;
    let regex = str.value();
    let dsl = Dsl::new(&regex, regex.len() > 1);
    if let Some(err) = dsl.validate() {
        return Err(Error::new(str.span(), err));
    }
    Ok(dsl)
}
