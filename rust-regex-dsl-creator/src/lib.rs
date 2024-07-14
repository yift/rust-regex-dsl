//! This crate can be used to create a regular expression DSL (see [here](https://crates.io/crates/rust-regex-dsl)).
//! To use it, run something like:
//! ```rust
//! use rust_regex_dsl_creator::ToDsl;
//!
//! let dsl = "[a-z]+".to_dsl().unwrap();
//! assert_eq!(dsl, "repeat {\n  any_of {\n    from: 'a', to: 'z',\n  },\n}\n")
//! ```

mod ast_impl;
mod basic_impls;
mod printer;

/// Import this trait to enable the `to_dsl` function for anything that implements the [ToString] trait.
pub trait ToDsl {
    /// This function (implemented by default for anything that implement the [ToString] trait) convert a regular expression to a DSL
    /// that can be used by the [regex_dsl](https://docs.rs/rust-regex-dsl/latest/rust_regex_dsl/macro.regex_dsl.html)
    /// or [create_capture](https://docs.rs/rust-regex-dsl/latest/rust_regex_dsl/macro.create_capture.html) macros.
    /// For example:
    /// ```rust
    /// use rust_regex_dsl_creator::ToDsl;
    ///
    /// let dsl = "[a-z]+[0-9]{2,3}$".to_dsl().unwrap();
    /// assert_eq!(dsl, "concat {\n  repeat {\n    any_of {\n      from: 'a', to: 'z',\n    },\n  },\n  times {\n    at_least: 2, at_most: 3,\n    any_of {\n      from: '0', to: '9',\n    },\n  },\n  end_of_line,\n}\n")
    /// ```
    fn to_dsl(&self) -> Result<String, regex::Error>;
}
