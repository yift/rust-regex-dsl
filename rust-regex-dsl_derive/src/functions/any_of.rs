use syn::{parse::ParseBuffer, Result};

use crate::{dsl::Dsl, user_class::UserClass};

pub fn parse_any_of(group: &ParseBuffer, to_use: bool) -> Result<Dsl> {
    let cls: UserClass = UserClass::parse(group, to_use)?;
    let dsl = cls.to_dsl();
    Ok(dsl)
}
