use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Str {
  pub value: String,
}

impl Default for Str {
  fn default() -> Self {
    Str::new("")
  }
}

impl fmt::Display for Str {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Str {
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
    IKind::Str(self.value.to_owned())
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(Str::new(&parser.token.literal).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("\"{}\"", self.value)
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

impl Str {
  pub fn new(value: &str) -> Self {
    Str {
      value: value.into(),
    }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
