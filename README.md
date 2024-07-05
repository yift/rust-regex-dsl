# Rust Regular Expression DSL

This crate was build to help using hard coded regular expressions.

## Why

While regular expressions is a powerful tool to parse and validate strings, they are hard to maintain from within a codebase. This is from a few reason reason:

* The compiler won't identify invalid regular expression. This means that one need to test the regular expression in run time
 and that one need to add code to handle an invalid regular expression (which can not happen for a valid hard coded regular expression).
* Long regular expression are hard to read for humans. This means that maintaining a regular expressions can be harder than rewriting them.
* Capture groups index access return an optional that might be none if the group is not define, or when the group was not caught (for example,
 for a regular expression like `([a-z]+)|([A-Z]+)` has two group but only one will be caught, so none for index 3 has different meaning from none in index 1).
* Named capture groups has access by string - so the compiler won't complain if one gt the string wrong.

## Getting Started

Add

```toml
[dependencies]
rust-regex-dsl = "0.1"
```

To the `Cargo.toml` manifest. And use either one of the available macros.

## Example

See [here](#example).
