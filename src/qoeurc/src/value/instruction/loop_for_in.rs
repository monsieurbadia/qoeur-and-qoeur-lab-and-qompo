use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::array::Array;
use crate::value::instruction::block::Block;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct LoopForIn {
  pub block: Box<dyn Value>,
  pub iterable: Box<dyn Value>,
  pub variable: Box<dyn Value>,
}

impl Default for LoopForIn {
  fn default() -> Self {
    LoopForIn::new(void!(), void!(), void!())
  }
}

impl fmt::Display for LoopForIn {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for LoopForIn {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::LoopForIn
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.first_is(TokenKind::GroupStart(Bracket));

    let iterable = Array::default().parse(parser)?;

    parser.next_token();
    parser.expect_first(TokenKind::Operator(Or))?;
    parser.expect_first(TokenKind::Identifier)?;

    let variable = Identifier::default().parse(parser)?;

    parser.expect_first(TokenKind::Operator(Or))?;
    parser.next_token();
    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(
      self
        .add_block(block)
        .add_iterable(iterable)
        .add_variable(variable)
        .boxed(),
    )
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!(
      "for {} |{}| {{ {} }}",
      self.iterable.text(),
      self.variable.text(),
      self.block.text()
    )
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "left": {{
            "declarations": [{{
              "id": {},
              "init": null,
              "type": "VariableDeclarator"
            }}],
            "kind": "var",
            "type": "VariableDeclaration"
          }},
          "right": {},
          "type": "ForInStatement"
        }}"#,
        self.block.transpile(transpiler),
        self.variable.transpile(transpiler),
        self.iterable.transpile(transpiler),
      ),
      TKind::Inline => format!(
        "for (var {} in {}) {{ {} }}",
        self.variable.transpile(transpiler),
        self.iterable.transpile(transpiler),
        self.block.transpile(transpiler),
      ),
      TKind::Pretty => format!(
        "for (var {} in {}) {{\n\t{}\n}}",
        self.variable.transpile(transpiler),
        self.iterable.transpile(transpiler),
        self.block.transpile(transpiler),
      ),
    }
  }
}

impl LoopForIn {
  pub fn new(
    block: Box<dyn Value>,
    iterable: Box<dyn Value>,
    variable: Box<dyn Value>,
  ) -> Self {
    LoopForIn {
      block,
      iterable,
      variable,
    }
  }

  pub fn add_block(&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = block;
    self
  }

  pub fn add_iterable(&mut self, iterable: Box<dyn Value>) -> &mut Self {
    self.iterable = iterable;
    self
  }

  pub fn add_variable(&mut self, variable: Box<dyn Value>) -> &mut Self {
    self.variable = variable;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
