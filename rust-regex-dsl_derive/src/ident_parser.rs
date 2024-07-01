use syn::{
    parse::{ParseBuffer, ParseStream},
    Error, Ident, Result,
};

use crate::{
    dsl::Dsl,
    functions::{concat::parse_concat, eq::parse_eq, regex::parse_regex},
    group::parse_group,
};

pub fn parse_ident(input: ParseStream) -> Result<Dsl> {
    let ident: Ident = input.parse()?;
    let group = parse_group(input)?;
    match group {
        Some(group) => parse_function(ident, &group),
        None => parse_single_word(ident),
    }
}

fn parse_function(ident: Ident, group: &ParseBuffer) -> Result<Dsl> {
    match ident.to_string().as_str() {
        "regex" => parse_regex(group),
        "eq" => parse_eq(group),
        "concat" => parse_concat(group),
        unknown_function => Err(Error::new(
            ident.span(),
            format!("Unknown function: {}", unknown_function),
        )),
    }
}
fn parse_single_word(ident: Ident) -> Result<Dsl> {
    let regex = match ident.to_string().as_str() {
        "any_character" => ".",
        "digit" => "\\d",
        "not_digit" => "\\D",
        "white_space" => "\\s",
        "not_white_space" => "\\S",
        "word_character" => "\\w",
        "not_word_character" => "\\W",
        "beginning_of_line" => "^",
        "end_Of_line" => "$",
        "word_boundary" => "\\b",
        "not_word_boundary" => "\\B",
        "beginning_of_input" => "\\A",
        "end_of_input" => "\\z",

        unknown_word => {
            return Err(Error::new(
                ident.span(),
                format!("Unknown word: {}", unknown_word),
            ))
        }
    };
    Ok(Dsl::new(regex))
}
