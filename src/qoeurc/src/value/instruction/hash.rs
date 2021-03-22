use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::expression::Expression;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::primitive::bool::Bool;
use crate::value::primitive::int::Int;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum HashKey {
  Bool(bool),
  Int(i64),
  Str(String),
}

impl fmt::Display for HashKey {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Box<dyn Value>> for HashKey {
  fn from(instruction: Box<dyn Value>) -> HashKey {
    match instruction.ikind() {
      IKind::Bool(value) => HashKey::Bool(value),
      IKind::Int(value) => HashKey::Int(value),
      IKind::Str(value) => HashKey::Str(value),
      _ => unreachable!(),
    }
  }
}

impl HashKey {
  pub fn text(&self) -> String {
    match self {
      HashKey::Bool(value) => format!("{}", value),
      HashKey::Int(value) => format!("{}", value),
      HashKey::Str(value) => format!("{}", value),
    }
  }

  pub fn transpile(&self, transpiler: &mut Transpiler) -> String {
    match self {
      HashKey::Bool(value) => Bool::new(*value).transpile(transpiler),
      HashKey::Int(value) => Int::new(*value).transpile(transpiler),
      HashKey::Str(value) => Identifier::new(value).transpile(transpiler),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Hash {
  data: Vec<(HashKey, Box<dyn Value>)>,
}

impl Default for Hash {
  fn default() -> Self {
    Hash::new(vec![])
  }
}

impl Value for Hash {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let mut data = vec![];

    for (key, value) in &self.data {
      let k = HashKey::from(key.to_owned());
      let v = value.to_owned().eval(interpreter)?;

      data.push((k, v));
    }

    Ok(self.add_data(data).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Hash(self.data.to_vec())
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    while !parser.first_is(TokenKind::GroupEnd(Brace)) {
      parser.next_token();

      let key = Expression::parse_expression_by_precedence(
        parser,
        &mut PrecedenceKind::Lowest,
      )?;

      parser.expect_first(TokenKind::Symbol(Colon))?;
      parser.next_token();

      let value = Expression::parse_expression_by_precedence(
        parser,
        &mut PrecedenceKind::Lowest,
      )?;

      self.add_item((HashKey::from(key), value));

      if !parser.first_is(TokenKind::GroupEnd(Brace)) {
        parser.expect_first(TokenKind::Symbol(Comma))?;
      }
    }

    parser.expect_first(TokenKind::GroupEnd(Brace))?;

    Ok(self.boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    let output = self
      .data
      .iter()
      .map(|(k, v)| format!("{}: {}", k.text(), v.text()))
      .collect::<Vec<String>>()
      .join(", ");

    format!("{{ {} }}", output)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "properties": [{}],
          "type": "ObjectExpression"
        }}"#,
        self
          .data
          .iter()
          .map(|(key, value)| format!(
            r#"{{
              "key": {},
              "kind": "init",

              "type": "Property",
              "value": {}
            }}"#,
            key.transpile(transpiler),
            value.to_owned().transpile(transpiler),
          ))
          .collect::<Vec<String>>()
          .join(", "),
      ),
      TKind::Pretty => format!(
        r#"{{ {} }}"#,
        self
          .data
          .iter()
          .map(|(key, value)| format!(
            "\n{}: {}\n",
            key.transpile(transpiler),
            value.to_owned().transpile(transpiler),
          ))
          .collect::<Vec<String>>()
          .join(", "),
      ),
      _ => self.text(),
    }
  }
}

impl Hash {
  pub fn new(data: Vec<(HashKey, Box<dyn Value>)>) -> Self {
    Hash { data }
  }

  pub fn add_data(
    &mut self,
    data: Vec<(HashKey, Box<dyn Value>)>,
  ) -> &mut Self {
    self.data = data.to_owned();
    self
  }

  pub fn add_item(&mut self, item: (HashKey, Box<dyn Value>)) -> &mut Self {
    self.data.push(item);
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
