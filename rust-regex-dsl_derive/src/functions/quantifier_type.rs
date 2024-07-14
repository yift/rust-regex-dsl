use syn::parse::ParseStream;
use syn::Result;

use super::parse_ident::parse_ident;

#[derive(Default)]
pub enum QuantifierType {
    #[default]
    Greedy,
    Lazy,
}
impl QuantifierType {
    pub fn parse(input: ParseStream) -> Result<Option<Self>> {
        if parse_ident(input, "greedy")? {
            Ok(Some(QuantifierType::Greedy))
        } else if parse_ident(input, "lazy")? {
            Ok(Some(QuantifierType::Lazy))
        } else {
            Ok(None)
        }
    }

    pub fn postfix(&self) -> &str {
        match self {
            QuantifierType::Greedy => "",
            QuantifierType::Lazy => "?",
        }
    }
}
