use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::expression::Expression;
use crate::value::instruction::hash::HashKey;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Index {
  pub lhs: Box<dyn Value>,
  pub rhs: Box<dyn Value>,
}

impl Default for Index {
  fn default() -> Self {
    Index::new(void!(), void!())
  }
}

impl fmt::Display for Index {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Box<dyn Value>> for Index {
  fn from(lhs: Box<dyn Value>) -> Index {
    Index::new(lhs, void!())
  }
}

impl Value for Index {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let lhs = self.lhs.eval(interpreter)?;
    let rhs = self.rhs.eval(interpreter)?;

    match (lhs.ikind(), rhs.ikind()) {
      (IKind::Array(data), IKind::Int(value)) => {
        Ok(self.eval_index_array_int(&data, value)?)
      }
      (IKind::Hash(data), IKind::Int(value)) => {
        Ok(self.eval_index_hash_int(&data, value)?)
      }
      (IKind::Hash(data), IKind::Bool(value)) => {
        Ok(self.eval_index_hash_bool(&data, value)?)
      }
      (IKind::Hash(data), IKind::Str(value)) => {
        Ok(self.eval_index_hash_str(&data, &value)?)
      }
      (_, _) => Err(format!(
        "error index expression: {}[{}]",
        lhs.text(),
        rhs.text()
      )),
    }
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Index
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let rhs = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    parser.expect_first(TokenKind::GroupEnd(Bracket))?;

    Ok(self.add_rhs(rhs).boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!("({}[{}])", self.lhs, self.rhs)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "computed": true,
          "object": {},
          "property": {},
          "type": "MemberExpression"
        }}"#,
        self.lhs.transpile(transpiler),
        self.rhs.transpile(transpiler),
      ),
      _ => self.text(),
    }
  }
}

impl Index {
  pub fn new(lhs: Box<dyn Value>, rhs: Box<dyn Value>) -> Self {
    Index { lhs, rhs }
  }

  pub fn add_lhs(&mut self, lhs: Box<dyn Value>) -> &mut Self {
    self.lhs = lhs;
    self
  }

  pub fn add_rhs(&mut self, rhs: Box<dyn Value>) -> &mut Self {
    self.rhs = rhs;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  fn eval_index_array_int(
    &self,
    data: &Vec<Box<dyn Value>>,
    value: i64,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(data[value as usize].boxed())
  }

  fn eval_index_hash_int(
    &self,
    data: &Vec<(HashKey, Box<dyn Value>)>,
    value: i64,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(data[value as usize].1.boxed())
  }

  fn eval_index_hash_bool(
    &self,
    data: &Vec<(HashKey, Box<dyn Value>)>,
    value: bool,
  ) -> ValueResult<Box<dyn Value>> {
    let x = data
      .iter()
      .enumerate()
      .find(|(_, data)| data.0.text().parse::<bool>().unwrap() == value)
      .unwrap()
      .0;

    Ok(data[x].1.boxed())
  }

  fn eval_index_hash_str(
    &self,
    data: &Vec<(HashKey, Box<dyn Value>)>,
    value: &str,
  ) -> ValueResult<Box<dyn Value>> {
    let x = data
      .iter()
      .enumerate()
      .find(|(_, data)| data.0.text() == value)
      .unwrap()
      .0;

    Ok(data[x].1.boxed())
  }
}
