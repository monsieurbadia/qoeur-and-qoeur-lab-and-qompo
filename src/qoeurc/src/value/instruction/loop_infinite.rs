use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::block::Block;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct LoopInfinite {
  pub block: Box<dyn Value>,
}

impl Default for LoopInfinite {
  fn default() -> Self {
    LoopInfinite::new(void!())
  }
}

impl fmt::Display for LoopInfinite {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for LoopInfinite {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let block = self.block.eval(interpreter)?;

    Ok(self.add_block(block).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::LoopInfinite
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(self.add_block(block).boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!("loop {{ {} }}", self.block.text())
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let block = self.block.transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "init": null,
          "test": null,
          "update": null,
          "type": "ForStatement"
        }}"#,
        block,
      ),
      TKind::Inline => format!("for (;) {{ {} }}", block),
      TKind::Pretty => format!("for (;) {{\n\t{}\n}}", block),
    }
  }
}

impl LoopInfinite {
  pub fn new(block: Box<dyn Value>) -> Self {
    LoopInfinite { block }
  }

  pub fn add_block(&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = block;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
