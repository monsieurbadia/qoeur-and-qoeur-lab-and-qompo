use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Comment {
  pub kind: TokenKind,
  pub value: String,
}

impl Default for Comment {
  fn default() -> Self {
    Comment::new(TokenKind::Comment(Line), "")
  }
}

impl fmt::Display for Comment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Comment {
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
    IKind::Comment
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    match parser.token.kind {
      TokenKind::Comment(Line) => {
        let kind = parser.token.kind;
        let value = &parser.token.literal;

        Ok(self.add_kind(kind).add_value(value).boxed())
      }
      _ => Err(format!("comment error: {}", parser.token.literal)),
    }
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("# {}", self.value)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match (self.kind, transpiler.mode()) {
      _ => format!(""),
    }
  }
}

impl Comment {
  pub fn new(kind: TokenKind, value: &str) -> Self {
    Comment {
      kind,
      value: value.into(),
    }
  }

  pub fn add_kind(&mut self, kind: TokenKind) -> &mut Self {
    self.kind = kind;
    self
  }

  pub fn add_value(&mut self, value: &str) -> &mut Self {
    self.value = value.into();
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
