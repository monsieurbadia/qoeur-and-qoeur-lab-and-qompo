use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::binary::Binary;
use crate::value::instruction::unary::Unary;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Expression {
  pub node: Box<dyn Value>,
}

impl Default for Expression {
  fn default() -> Self {
    Expression::new(void!())
  }
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Expression {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.node.eval(interpreter)?)
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Expression
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let node = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    if parser.first_is(Symbol(Semicolon)) {
      parser.next_token();
    }

    Ok(self.add_node(node).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{}", self.node.text())
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let node = self.node.transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "expression": {},
          "type": "ExpressionStatement"
        }}"#,
        node,
      ),
      _ => node,
    }
  }
}

impl Expression {
  pub fn new(node: Box<dyn Value>) -> Self {
    Expression { node }
  }

  pub fn add_node(&mut self, node: Box<dyn Value>) -> &mut Self {
    self.node = node;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  pub fn parse_expression_by_precedence(
    parser: &mut Parser,
    precedence: &mut PrecedenceKind,
  ) -> ParserResult<Box<dyn Value>> {
    let mut node = Unary::to_unary(parser, parser.token.kind)?;

    while !parser.first_is(Symbol(Semicolon))
      && parser.should_precedence_has_priority(precedence)
    {
      parser.next_token();

      node = Binary::to_binary(parser, node)?;
    }

    Ok(node)
  }
}
