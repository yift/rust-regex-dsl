use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::Result;

use crate::dsl::Dsl;

use super::parse_list::parse_list;
use super::quantifier_type::QuantifierType;

enum Element {
    QuantifierType(QuantifierType),
    Dsl(Dsl),
}
impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Some(qt) = QuantifierType::parse(input)? {
            Ok(Element::QuantifierType(qt))
        } else {
            Ok(Element::Dsl(input.parse()?))
        }
    }
}
struct Repeat {
    quantifier_type: QuantifierType,
    dsl: Dsl,
}
impl Repeat {
    fn to_dsl(&self, operator: &str) -> Dsl {
        let regex = format!(
            "{}{}{}",
            self.dsl.non_capturing_group_if_needed(),
            operator,
            self.quantifier_type.postfix(),
        );
        Dsl::new(&regex, false)
    }
}
impl Parse for Repeat {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut dsl = None;
        let mut quantifier_type = None;
        parse_list(input, |element| {
            match element {
                Element::QuantifierType(qt) => {
                    if quantifier_type.is_some() {
                        return Err(input.error("Can not set quantifier type twice"));
                    }
                    quantifier_type = Some(qt);
                }
                Element::Dsl(regex) => {
                    if dsl.is_some() {
                        return Err(input.error("Can not repeat two things"));
                    }
                    dsl = Some(regex);
                }
            }
            Ok(())
        })?;
        let Some(dsl) = dsl else {
            return Err(input.error("Nothing to repeat"));
        };
        let quantifier_type = quantifier_type.unwrap_or_default();
        Ok(Self {
            quantifier_type,
            dsl,
        })
    }
}
pub fn parse_repeat(group: &ParseBuffer, operator: &str) -> Result<Dsl> {
    let repeat: Repeat = group.parse()?;

    Ok(repeat.to_dsl(operator))
}
