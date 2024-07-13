use thiserror::Error;

mod ast_impl;
mod basic_impls;
mod printer;
pub trait ToDsl {
    fn to_dsl(&self) -> Result<String, CreatorError>;
}

#[derive(Error, Debug)]
pub enum CreatorError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    RegexError(#[from] regex::Error),
}
