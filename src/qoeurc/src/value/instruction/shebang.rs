use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Shebang {
  value: String,
}

impl Default for Shebang {
  fn default() -> Self {
    Shebang::new("")
  }
}

impl fmt::Display for Shebang {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Shebang {
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
    IKind::Shebang
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(self.add_value(&parser.token.literal).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{}", self.value)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      _ => self.text(),
    }
  }
}

impl Shebang {
  pub fn new(value: &str) -> Self {
    Shebang {
      value: value.into(),
    }
  }

  pub fn add_value(&mut self, value: &str) -> &mut Self {
    self.value = value.into();
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
