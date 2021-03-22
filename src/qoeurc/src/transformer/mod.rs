pub mod transpiler;

pub use crate::transformer::TKind::*;

use crate::analyzer::interpreter::Interpreter;
use crate::converter::parser::Parser;
use crate::transformer::transpiler::{TKind, Transpiler, TranspilerResult};

#[cfg(test)]
mod tests;

pub fn transformify(
  input: &str,
  mode: &str,
  interpreter: &Interpreter,
) -> TranspilerResult<String> {
  let mut parser = Parser::new(input);
  let program = &parser.parse()?;
  let mut transpiler =
    Transpiler::new(interpreter.to_owned(), TKind::from(mode));

  transpiler.transpile(program)
}
