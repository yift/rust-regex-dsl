use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::Result;

pub fn parse_list<T: Parse>(
    input: ParseStream,
    mut added: impl FnMut(T) -> Result<()>,
) -> Result<()> {
    while !input.is_empty() {
        let lookahead = input.lookahead1();
        if lookahead.peek(Comma) {
            let _: Comma = input.parse()?;
        } else {
            let t: T = input.parse()?;
            added(t)?;
        }
    }
    Ok(())
}
pub fn parse_list_to_vec<T: Parse>(input: ParseStream) -> Result<Vec<T>> {
    let mut ret = vec![];
    parse_list(input, |t| {
        ret.push(t);
        Ok(())
    })?;
    Ok(ret)
}
