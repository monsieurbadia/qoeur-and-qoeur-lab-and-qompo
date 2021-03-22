use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Bool {
  pub value: bool,
}

impl Default for Bool {
  fn default() -> Self {
    Bool::new(false)
  }
}

impl fmt::Display for Bool {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Bool {
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
    IKind::Bool(self.value)
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(Bool::new(parser.token_is(Keyword(True))).boxed())
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
      _ => format!("{}", self.text()),
    }
  }
}

impl Bool {
  pub fn new(value: bool) -> Self {
    Bool { value }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
