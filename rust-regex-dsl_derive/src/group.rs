use syn::parse::{ParseBuffer, ParseStream, Result};
use syn::token::{Brace, Bracket, Paren};
use syn::{braced, bracketed, parenthesized};

pub fn parse_group(input: ParseStream) -> Result<Option<ParseBuffer>> {
    let content;
    let lookahead = input.lookahead1();
    if lookahead.peek(Brace) {
        braced!(content in input);
    } else if lookahead.peek(Bracket) {
        bracketed!(content in input);
    } else if lookahead.peek(Paren) {
        parenthesized!(content in input);
    } else {
        return Ok(None);
    }
    Ok(Some(content))
}
