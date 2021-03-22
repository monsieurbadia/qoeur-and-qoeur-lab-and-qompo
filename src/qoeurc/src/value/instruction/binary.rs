use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::tokenizer::token::Token;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::call::Call;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::index::Index;
use crate::value::instruction::IKind;
use crate::value::primitive::{bool::Bool, float::Float, int::Int, str::Str};
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Binary {
  pub lhs: Box<dyn Value>,
  pub operator: Box<Token>,
  pub rhs: Box<dyn Value>,
}

impl Default for Binary {
  fn default() -> Self {
    Binary::new(void!(), Box::new(Token::default()), void!())
  }
}

impl fmt::Display for Binary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Box<dyn Value>> for Binary {
  fn from(lhs: Box<dyn Value>) -> Binary {
    Binary::new(lhs, Box::new(Token::default()), void!())
  }
}

impl Value for Binary {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let binary_lhs = self.lhs.eval(interpreter)?;
    let binary_rhs = self.rhs.eval(interpreter)?;
    let operator = &self.operator.to_owned();

    match (binary_lhs.ikind(), binary_rhs.ikind()) {
      (IKind::Bool(lhs), IKind::Bool(rhs)) => {
        Binary::eval_binary_bool_expression(&lhs, operator, &rhs)
      }
      (IKind::Int(lhs), IKind::Int(rhs)) => {
        Binary::eval_binary_int_expression(&lhs, operator, &rhs)
      }
      (IKind::Float(lhs), IKind::Float(rhs)) => {
        Binary::eval_binary_float_expression(&lhs, operator, &rhs)
      }
      (IKind::Str(lhs), IKind::Str(rhs)) => {
        Binary::eval_binary_str_expression(&lhs, operator, &rhs)
      }
      (_, _) => Err(format!("error binary expression")),
    }
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Binary
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let mut precedence = parser.current_precedence();
    let operator = parser.token.to_owned();

    parser.next_token();

    let rhs =
      Expression::parse_expression_by_precedence(parser, &mut precedence)?;

    Ok(self.add_operator(operator).add_rhs(rhs).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("({} {} {})", self.lhs, self.operator.text(), self.rhs)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let lhs = self
      .lhs
      // .eval(&mut transpiler.interpreter)
      // .unwrap()
      .transpile(transpiler);

    let operator = self.operator.text();

    let rhs = self
      .rhs
      // .eval(&mut transpiler.interpreter)
      // .unwrap()
      .transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "left": {},
          "operator": "{}",
          "right": {},
          "type": "BinaryExpression"
        }}"#,
        lhs, operator, rhs,
      ),
      _ => self.text(),
    }
  }
}

impl Binary {
  pub fn new(
    lhs: Box<dyn Value>,
    operator: Box<Token>,
    rhs: Box<dyn Value>,
  ) -> Self {
    Binary { operator, lhs, rhs }
  }

  pub fn add_lhs(&mut self, lhs: Box<dyn Value>) -> &mut Self {
    self.lhs = lhs;
    self
  }

  pub fn add_operator(&mut self, operator: Box<Token>) -> &mut Self {
    self.operator = operator.to_owned();
    self
  }

  pub fn add_rhs(&mut self, rhs: Box<dyn Value>) -> &mut Self {
    self.rhs = rhs;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  pub fn to_binary(
    parser: &mut Parser,
    lhs: Box<dyn Value>,
  ) -> ValueResult<Box<dyn Value>> {
    match parser.token.kind {
      TokenKind::GroupStart(Bracket) => Index::from(lhs).parse(parser),
      TokenKind::GroupStart(Parenthesis) => Call::from(lhs).parse(parser),
      _ => Binary::from(lhs).parse(parser),
    }
  }

  fn eval_binary_bool_expression(
    lhs: &bool,
    operator: &Token,
    rhs: &bool,
  ) -> ValueResult<Box<dyn Value>> {
    match operator.kind {
      TokenKind::Operator(Equal) => Ok(Bool::new(lhs == rhs).boxed()),
      TokenKind::Operator(NotEqual) => Ok(Bool::new(lhs != rhs).boxed()),
      _ => Err(format!(
        "cannot use `{:?}` for `{}` and `{}`",
        operator, lhs, rhs
      )),
    }
  }

  fn eval_binary_int_expression(
    lhs: &i64,
    operator: &Token,
    rhs: &i64,
  ) -> ValueResult<Box<dyn Value>> {
    match operator.kind {
      TokenKind::Operator(Plus) => Ok(Int::new(lhs + rhs).boxed()),
      TokenKind::Operator(Minus) => Ok(Int::new(lhs - rhs).boxed()),
      TokenKind::Operator(Star) => Ok(Int::new(lhs * rhs).boxed()),
      TokenKind::Operator(Slash) => Ok(Int::new(lhs / rhs).boxed()),
      TokenKind::Operator(Equal) => Ok(Bool::new(lhs == rhs).boxed()),
      TokenKind::Operator(NotEqual) => Ok(Bool::new(lhs != rhs).boxed()),
      TokenKind::Operator(LessThan) => Ok(Bool::new(lhs < rhs).boxed()),
      TokenKind::Operator(LessThanOrEqual) => Ok(Bool::new(lhs <= rhs).boxed()),
      TokenKind::Operator(GreaterThan) => Ok(Bool::new(lhs > rhs).boxed()),
      TokenKind::Operator(GreaterThanOrEqual) => {
        Ok(Bool::new(lhs >= rhs).boxed())
      }
      _ => Err(format!("error binary int")),
    }
  }

  fn eval_binary_float_expression(
    lhs: &f64,
    operator: &Token,
    rhs: &f64,
  ) -> ValueResult<Box<dyn Value>> {
    match operator.kind {
      TokenKind::Operator(Plus) => Ok(Float::new(lhs + rhs).boxed()),
      TokenKind::Operator(Minus) => Ok(Float::new(lhs - rhs).boxed()),
      TokenKind::Operator(Star) => Ok(Float::new(lhs * rhs).boxed()),
      TokenKind::Operator(Slash) => Ok(Float::new(lhs / rhs).boxed()),
      TokenKind::Operator(Equal) => Ok(Bool::new(lhs == rhs).boxed()),
      TokenKind::Operator(NotEqual) => Ok(Bool::new(lhs != rhs).boxed()),
      TokenKind::Operator(LessThan) => Ok(Bool::new(lhs < rhs).boxed()),
      TokenKind::Operator(LessThanOrEqual) => Ok(Bool::new(lhs <= rhs).boxed()),
      TokenKind::Operator(GreaterThan) => Ok(Bool::new(lhs > rhs).boxed()),
      TokenKind::Operator(GreaterThanOrEqual) => {
        Ok(Bool::new(lhs >= rhs).boxed())
      }
      _ => Err(format!("error binary float")),
    }
  }

  fn eval_binary_str_expression(
    lhs: &str,
    operator: &Token,
    rhs: &str,
  ) -> ValueResult<Box<dyn Value>> {
    match operator.kind {
      TokenKind::Operator(Plus) => {
        Ok(Str::new(format!("{}{}", lhs, rhs).as_str()).boxed())
      }
      _ => Err(format!("error binary str expression")),
    }
  }
}
