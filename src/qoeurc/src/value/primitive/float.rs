use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Float {
  pub value: f64,
}

impl Default for Float {
  fn default() -> Self {
    Float::new(0.0)
  }
}

impl fmt::Display for Float {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Float {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Float(self.value)
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    match parser.token.literal.parse() {
      Err(error) => Err(format!("{}", error)),
      Ok(expression) => Ok(Box::new(Float::new(expression))),
    }
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{}", self.value)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "type": "Literal",
          "raw": "{}",
          "value": {}
        }}"#,
        self.text(),
        self.text(),
      ),
      _ => self.text(),
    }
  }
}

impl Float {
  pub fn new(value: f64) -> Self {
    Float { value }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
