use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    token::Colon,
    Error, LitInt, Result,
};

use crate::{dsl::Dsl, functions::parse_list::parse_list};

use super::parse_ident;
enum Element {
    Greedy,
    Lazy,
    AtMost(usize),
    AtLeast(usize),
    Exactly(usize),
    Dsl(Dsl),
}
fn read_colon_num(input: &ParseBuffer) -> Result<usize> {
    let _: Colon = input.parse()?;
    let number: LitInt = input.parse()?;
    let Ok(number) = number.base10_parse() else {
        return Err(Error::new(number.span(), "Invalid number"));
    };
    Ok(number)
}
impl Parse for Element {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        if parse_ident::parse_ident(input, "greedy")? {
            Ok(Element::Greedy)
        } else if parse_ident::parse_ident(input, "lazy")? {
            Ok(Element::Lazy)
        } else if parse_ident::parse_ident(input, "exactly")? {
            Ok(Element::Exactly(read_colon_num(input)?))
        } else if parse_ident::parse_ident(input, "at_least")? {
            Ok(Element::AtLeast(read_colon_num(input)?))
        } else if parse_ident::parse_ident(input, "at_most")? {
            Ok(Element::AtMost(read_colon_num(input)?))
        } else {
            Ok(Element::Dsl(input.parse()?))
        }
    }
}
struct Times {
    quantifier_type: String,
    times: String,
    dsl: Dsl,
}
impl Times {
    fn to_dsl(&self) -> Dsl {
        let regex = format!(
            "{}{}{}",
            self.dsl.non_capturing_group_if_needed(),
            self.times,
            self.quantifier_type,
        );
        Dsl::new(&regex, false)
    }
}
impl Parse for Times {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut quantifier_type = None;
        let mut at_least = None;
        let mut at_most = None;
        let mut exactly = None;
        let mut dsl = None;
        parse_list(input, |element| {
            match element {
                Element::AtLeast(num) => {
                    if at_least.is_some() {
                        return Err(input.error("Can not set at least twice"));
                    }
                    at_least = Some(num);
                }
                Element::AtMost(num) => {
                    if at_most.is_some() {
                        return Err(input.error("Can not set at most twice"));
                    }
                    at_most = Some(num);
                }
                Element::Exactly(num) => {
                    if exactly.is_some() {
                        return Err(input.error("Can not set exactly twice"));
                    }
                    if num == 0 {
                        return Err(input.error("Exactly zero times is pointless"));
                    }
                    exactly = Some(num);
                }
                Element::Greedy => {
                    if quantifier_type.is_some() {
                        return Err(input.error("Can not set quantifier type twice"));
                    }
                    quantifier_type = Some("");
                }
                Element::Lazy => {
                    if quantifier_type.is_some() {
                        return Err(input.error("Can not set quantifier type twice"));
                    }
                    quantifier_type = Some("+");
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
        let times = if let Some(exactly) = exactly {
            if at_least.is_some() {
                return Err(input.error("Can not set exactly and at least together"));
            }
            if at_most.is_some() {
                return Err(input.error("Can not set exactly and at most together"));
            }
            format!("{{{}}}", exactly)
        } else if let Some(at_most) = at_most {
            if let Some(at_least) = at_least {
                format!("{{{},{}}}", at_least, at_most)
            } else {
                return Err(input.error("At most without at least is not supported"));
            }
        } else if let Some(at_last) = at_least {
            format!("{{{},}}", at_last)
        } else {
            return Err(input.error("Must set either exactly, at_most and at_least or at_least"));
        };
        let quantifier_type = quantifier_type.unwrap_or_default().to_string();
        Ok(Self {
            quantifier_type,
            times,
            dsl,
        })
    }
}
pub fn parse_times(group: &ParseBuffer) -> Result<Dsl> {
    let times: Times = group.parse()?;
    Ok(times.to_dsl())
}
