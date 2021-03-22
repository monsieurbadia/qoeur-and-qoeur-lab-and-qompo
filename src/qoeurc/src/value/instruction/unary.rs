use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::tokenizer::token::Token;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::array::Array;
use crate::value::instruction::closure::Closure;
use crate::value::instruction::comment::Comment;
use crate::value::instruction::control::If;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::group::Group;
use crate::value::instruction::hash::Hash;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::primitive::bool::Bool;
use crate::value::primitive::float::Float;
use crate::value::primitive::int::Int;
use crate::value::primitive::str::Str;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Unary {
  operand: Box<Token>,
  rhs: Box<dyn Value>,
}

impl Default for Unary {
  fn default() -> Self {
    Unary::new(Box::new(Token::default()), void!())
  }
}

impl fmt::Display for Unary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Unary {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    match self.operand.kind {
      TokenKind::Symbol(Bang) => match self.rhs.ikind() {
        IKind::Bool(b) => Ok(Bool::new(!b).boxed()),
        IKind::Int(i) => Ok(Bool::new(i == 0).boxed()),
        _ => Err(format!("eval_unary_expression bang operand error")),
      },
      TokenKind::Operator(Minus) => match self.rhs.ikind() {
        IKind::Int(i) => Ok(Int::new(-i).boxed()),
        IKind::Float(i) => Ok(Float::new(-i).boxed()),
        _ => Err(format!("eval_unary_expression minus operand error")),
      },
      _ => Err(format!("eval_unary_expression error")),
    }
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Unary
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let operand = parser.token.boxed();

    parser.next_token();

    let rhs = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Unary,
    )?;

    Ok(self.add_operand(operand).add_rhs(rhs).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("({}{})", self.operand, self.rhs)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let operand = self.operand.text();

    let rhs = self
      .rhs
      .eval(&mut transpiler.interpreter)
      .unwrap()
      .transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "argument": {},
          "operator": "{}",
          "prefix": "true",
          "type": "UnaryExpression"
        }}"#,
        rhs, operand,
      ),
      TKind::Inline => self.text(),
      TKind::Pretty => self.text(),
    }
  }
}

impl Unary {
  pub fn new(operand: Box<Token>, rhs: Box<dyn Value>) -> Self {
    Unary { operand, rhs }
  }

  pub fn add_operand(&mut self, operand: Box<Token>) -> &mut Self {
    self.operand = operand;
    self
  }

  pub fn add_rhs(&mut self, rhs: Box<dyn Value>) -> &mut Self {
    self.rhs = rhs;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  pub fn to_unary(
    parser: &mut Parser,
    kind: TokenKind,
  ) -> ValueResult<Box<dyn Value>> {
    match kind {
      TokenKind::Comment(_) => Comment::default().parse(parser),
      TokenKind::GroupStart(Brace) => Hash::default().parse(parser),
      TokenKind::GroupStart(Bracket) => Array::default().parse(parser),
      TokenKind::GroupStart(Parenthesis) => Group::default().parse(parser),
      TokenKind::Identifier => Identifier::default().parse(parser),
      TokenKind::Keyword(False) | TokenKind::Keyword(True) => {
        Bool::default().parse(parser)
      }
      TokenKind::Keyword(If) => If::default().parse(parser),
      TokenKind::Literal(Float) => Float::default().parse(parser),
      TokenKind::Literal(Int) => Int::default().parse(parser),
      TokenKind::Literal(Str) => Str::default().parse(parser),
      TokenKind::Operator(Or) => Closure::default().parse(parser),
      TokenKind::Operator(Minus) | TokenKind::Symbol(Bang) => {
        Unary::default().parse(parser)
      }
      _ => Err(format!("unary error: {}", parser.token.literal)),
    }
  }
}
