//! This crate was build to help using hard coded regular expressions. It provides those macros:
//! * The [`regex!`] macro - to verify a regular expression on compile time and remove the need to do it in run time and unwrap it.
//! * The [`regex_dsl!`] macro - to make a regular expression easier to read and maintain.
//! * The [`create_capture!`] macro  - to create a struct from a regular expression.
//!
//! To use, add:
//! ```toml
//! rust-regex-dsl = "0.1"
//! ```
//! To your `Cargo.toml` manifest.

#[doc(hidden)]
pub use lazy_static::lazy_static;
#[doc(hidden)]
pub use regex::{Captures, Regex};
pub use rust_regex_dsl_derive::create_capture;
pub use rust_regex_dsl_derive::regex;
pub use rust_regex_dsl_derive::regex_dsl;
