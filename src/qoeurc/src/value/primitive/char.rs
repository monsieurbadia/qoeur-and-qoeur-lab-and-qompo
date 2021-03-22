use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Char {
  pub value: char,
}

impl Default for Char {
  fn default() -> Self {
    Char::new(' ')
  }
}

impl fmt::Display for Char {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl Value for Char {
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
    IKind::Char(self.value)
  }

  fn parse(&mut self, _parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(Char::new(' ').boxed())
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

impl Char {
  pub fn new(value: char) -> Self {
    Char { value }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
