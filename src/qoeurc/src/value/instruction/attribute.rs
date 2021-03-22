use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::Transpiler;
use crate::utils::iters::strip_exprs;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Attribute {
  pub args: Option<Vec<Box<dyn Value>>>,
  pub label: Box<dyn Value>,
}

impl Default for Attribute {
  fn default() -> Self {
    Attribute::new(void!(), vec![])
  }
}

impl fmt::Display for Attribute {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Attribute {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::Attribute
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.expect_first(TokenKind::Identifier)?;

    let label = Identifier::default().parse(parser)?;

    parser.expect_first(TokenKind::Symbol(Colon))?;

    let args = parser.parse_until(TokenKind::Symbol(Dot))?;

    Ok(self.add_label(label).add_args(args).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let args = &self.args.as_ref().unwrap();

    format!("|> {} : {}.", self.label, strip_exprs(args, ", "))
  }

  fn transpile(&mut self, _: &mut Transpiler) -> String {
    self.text()
  }
}

impl Attribute {
  pub fn new(label: Box<dyn Value>, args: Vec<Box<dyn Value>>) -> Self {
    Attribute {
      args: Some(args),
      label,
    }
  }

  pub fn add_args(&mut self, args: Vec<Box<dyn Value>>) -> &mut Self {
    self.args = Some(args);
    self
  }

  pub fn add_label(&mut self, label: Box<dyn Value>) -> &mut Self {
    self.label = label;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
