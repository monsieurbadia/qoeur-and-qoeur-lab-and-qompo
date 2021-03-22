use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::array::Array;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub enum TyKind {
  Array,
  Bool,
  Char,
  Float,
  Hash,
  Int,
  Str,
}

impl fmt::Display for TyKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      TyKind::Array => write!(f, "[]"),
      TyKind::Bool => write!(f, "bool"),
      TyKind::Char => write!(f, "char"),
      TyKind::Float => write!(f, "float"),
      TyKind::Hash => write!(f, "hash"),
      TyKind::Int => write!(f, "int"),
      TyKind::Str => write!(f, "str"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Ty {
  kind: TyKind,
}

impl Default for Ty {
  fn default() -> Self {
    Ty::new(TyKind::Array)
  }
}

impl fmt::Display for Ty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Ty {
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
    IKind::Ty
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let mut ty = Ty::default();
    let kind;

    if parser.token_is(TokenKind::GroupStart(Bracket)) {
      kind = Array::default().parse(parser)?.boxed().text();
    } else {
      kind = parser.token.literal.to_string();
    }

    match kind.as_str() {
      "[]" => Ok(ty.add_kind(TyKind::Array).boxed()),
      "bool" => Ok(ty.add_kind(TyKind::Bool).boxed()),
      "char" => Ok(ty.add_kind(TyKind::Char).boxed()),
      "float" => Ok(ty.add_kind(TyKind::Float).boxed()),
      "hash" => Ok(ty.add_kind(TyKind::Hash).boxed()),
      "int" => Ok(ty.add_kind(TyKind::Int).boxed()),
      "str" => Ok(ty.add_kind(TyKind::Str).boxed()),
      _ => Err(format!("type error expression")),
    }
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{}", self.kind)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      _ => self.text(),
    }
  }
}

impl Ty {
  pub fn new(kind: TyKind) -> Self {
    Ty { kind }
  }

  pub fn add_kind(&mut self, kind: TyKind) -> &mut Self {
    self.kind = kind;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
