use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::expression::Expression;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Return {
  pub value: Box<dyn Value>,
}

impl Default for Return {
  fn default() -> Self {
    Return::new(void!())
  }
}

impl fmt::Display for Return {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Return {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let value = self.value.eval(interpreter)?;

    Ok(self.add_value(value).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::Return
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let value = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    while parser.token.kind != TokenKind::Symbol(Semicolon)
      && parser.token.kind != TokenKind::NewLine
    {
      parser.next_token();
    }

    Ok(self.add_value(value).boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!("return {};", self.value)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    // let mut value = self.value.eval(&mut transpiler.interpreter).unwrap();

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "argument": {},
          "type": "ReturnStatement"
        }}"#,
        self.value.transpile(transpiler),
      ),
      _ => format!("return {};", self.value.transpile(transpiler)),
    }
  }
}

impl Return {
  pub fn new(value: Box<dyn Value>) -> Self {
    Return { value }
  }

  pub fn add_value(&mut self, value: Box<dyn Value>) -> &mut Self {
    self.value = value;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
