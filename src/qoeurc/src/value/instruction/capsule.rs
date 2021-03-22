use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
// use crate::utils::iters::{eval_expressions, strip_exprs, transpile_exprs};
use crate::value::instruction::block::Block;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Capsule {
  pub block: Box<dyn Value>,
  pub name: Box<dyn Value>,
}

impl Default for Capsule {
  fn default () -> Self {
    Capsule::new(void!(), void!())
  }
}

impl fmt::Display for Capsule {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Capsule {
  fn eval (&mut self, _interpreter: &mut Interpreter)
    -> ValueResult<Box<dyn Value>>
  {
    Ok(self.boxed())
  }

  fn vkind (&self) -> VKind {
    VKind::Expression
  }

  fn ikind (&self) -> IKind {
    IKind::Capsule
  }

  fn parse (&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn print (&self) {
    println!("{}", self.text())
  }

  fn text (&self) -> String {
    format!("{}", self.block.text())
  }

  fn transpile (&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "elements": [{}],
          "type": "ArrayExpression"
        }}"#,
        "",
      ),
      TKind::Inline => format!(
        "[{}]", "",
      ),
      TKind::Pretty => format!(
        "[\n{}\n]", "",
      ),
    }
  }
}

impl Capsule {
  pub fn new (name: Box<dyn Value>, block: Box<dyn Value>) -> Self {
    Capsule {
      block,
      name,
    }
  }

  pub fn add_block (&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = block;
    self
  }

  pub fn add_name (&mut self, name: Box<dyn Value>) -> &mut Self {
    self.name = name;
    self
  }

  pub fn boxed (&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
