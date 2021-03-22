use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Group {
  pub expression: Box<dyn Value>,
}

impl Default for Group {
  fn default() -> Self {
    Group::new(void!())
  }
}

impl fmt::Display for Group {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Group {
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
    IKind::Group
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let expression = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    parser.expect_first(TokenKind::GroupEnd(Parenthesis))?;

    Ok(self.add_expression(expression).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("({})", self.expression.text())
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      _ => self.text(),
    }
  }
}

impl Group {
  pub fn new(expression: Box<dyn Value>) -> Self {
    Group { expression }
  }

  pub fn add_expression(&mut self, expression: Box<dyn Value>) -> &mut Self {
    self.expression = expression;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
