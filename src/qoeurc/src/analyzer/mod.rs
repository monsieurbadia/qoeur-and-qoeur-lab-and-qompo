pub mod environment;
pub mod interpreter;

#[cfg(test)]
mod tests;

use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::Parser;
use crate::value::Value;

pub fn evalify(
  input: &str,
  interpreter: &mut Interpreter,
) -> ValueResult<Box<dyn Value>> {
  let mut parser = Parser::new(input);
  let program = &parser.parse()?;

  interpreter.eval(program)
}
