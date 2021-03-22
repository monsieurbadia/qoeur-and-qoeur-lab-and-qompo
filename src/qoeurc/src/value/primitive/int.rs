use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Int {
  pub value: i64,
}

impl Default for Int {
  fn default() -> Self {
    Int::new(0)
  }
}

impl fmt::Display for Int {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Int {
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
    IKind::Int(self.value)
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    match parser.token.literal.replace("_", "").parse() {
      Err(_) => Err(format!("parse int error")),
      Ok(expression) => Ok(Int::new(expression).boxed()),
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

impl Int {
  pub fn new(value: i64) -> Self {
    Int { value }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
