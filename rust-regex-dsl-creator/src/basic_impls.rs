use crate::ast_impl::Builder;
use crate::printer::Printer;
use crate::ToDsl;
use regex::Error;
use regex::Regex;
use regex_syntax::ast::parse::Parser;

impl<T: ToString> ToDsl for T {
    fn to_dsl(&self) -> Result<String, Error> {
        let str = self.to_string();
        Regex::new(&str)?;
        let mut parser = Parser::new();
        // This must pass since the regular expression is valid
        let ast = parser.parse(&str).unwrap();
        let mut printer = Printer::new();
        ast.print_ast(&mut printer);
        Ok(printer.to_string())
    }
}
