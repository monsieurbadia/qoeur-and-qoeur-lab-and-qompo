pub mod parser;

#[cfg(test)]
mod tests;

use crate::converter::parser::{Parser, ParserResult};
use crate::reporter::diagnostic::DiagnosticBuilder;
use crate::value::Value;

pub fn astify(input: &str) -> ParserResult<Box<dyn Value>> {
  let mut parser = Parser::new(input);
  let program = parser.parse()?;

  match parser.errors.is_empty() {
    false => DiagnosticBuilder::print_errors(parser.errors),
    true => (),
  };

  Ok(program)
}
