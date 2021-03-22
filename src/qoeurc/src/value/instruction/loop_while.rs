use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::block::Block;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct LoopWhile {
  pub block: Box<dyn Value>,
  pub condition: Box<dyn Value>,
}

impl Default for LoopWhile {
  fn default() -> Self {
    LoopWhile::new(void!(), void!())
  }
}

impl fmt::Display for LoopWhile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for LoopWhile {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let mut block = void!().boxed();
    let condition = self.condition.eval(interpreter)?;

    while condition.is_truthy() {
      block = self.block.eval(interpreter)?;
    }

    Ok(self.add_condition(condition).add_block(block).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::LoopWhile
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let condition = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(self.add_condition(condition).add_block(block).boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!("while {} {{ {} }}", self.condition, self.block.text())
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "test": {},
          "type": "WhileStatement"
        }}"#,
        self.block.transpile(transpiler),
        self.condition.transpile(transpiler),
      ),
      TKind::Inline => format!(
        "while ({}) {{ {} }}",
        self.condition.transpile(transpiler),
        self.block.transpile(transpiler),
      ),
      TKind::Pretty => format!(
        "while ({}) {{\n\t{}\n}}",
        self.condition.transpile(transpiler),
        self.block.transpile(transpiler),
      ),
    }
  }
}

impl LoopWhile {
  pub fn new(block: Box<dyn Value>, condition: Box<dyn Value>) -> Self {
    LoopWhile { block, condition }
  }

  pub fn add_block(&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = block;
    self
  }

  pub fn add_condition(&mut self, condition: Box<dyn Value>) -> &mut Self {
    self.condition = condition;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
