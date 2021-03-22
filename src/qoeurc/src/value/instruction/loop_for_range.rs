use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::block::Block;
use crate::value::instruction::IKind;
use crate::value::primitive::int::Int;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct LoopForRange {
  pub block: Box<dyn Value>,
  pub end: Box<dyn Value>,
  pub start: Box<dyn Value>,
}

impl Default for LoopForRange {
  fn default() -> Self {
    LoopForRange::new(void!(), void!(), void!())
  }
}

impl fmt::Display for LoopForRange {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for LoopForRange {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let start = self.start.as_ref().downcast_ref::<Int>().unwrap().value;
    let end = self.end.as_ref().downcast_ref::<Int>().unwrap().value;

    for _ in start..end {
      self.block.eval(interpreter)?;
    }

    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::LoopForRange
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let start = Int::default().parse(parser)?;

    parser.expect_first(TokenKind::Operator(Range))?;
    parser.next_token();

    let end = Int::default().parse(parser)?;

    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(self.add_block(block).add_end(end).add_start(start).boxed())
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!(
      "for {}..{} {{ {} }}",
      self.start.text(),
      self.end.text(),
      self.block.text(),
    )
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "init": {{
            "declarations": [{{
              "id": {{
                "name": "$$x",
                "type": "Identifier"
              }},
              "init": {{
                "type": "Literal",
                "value": {}
              }},
              "type": "VariableDeclarator"
            }}],
            "kind": "var",
            "type": "VariableDeclaration"
          }},
          "test": {{
            "left": {{
              "name": "$$x",
              "type": "Identifier"
            }},
            "operator": "<",
            "right": {{
              "type": "Literal",
              "value": {}
            }},
            "type": "BinaryExpression"
          }},
          "type": "ForStatement",
          "update": {{
            "argument": {{
              "name": "$$x",
              "type": "Identifier"
            }},
            "operator": "++",
            "type": "UpdateExpression"
          }},
        }}"#,
        self.block.transpile(transpiler),
        self.start.text(),
        self.end.text(),
      ),
      TKind::Inline => format!(
        "for (var $$x = {}; $$x < {}; $$x++) {{ {} }}",
        self.start.text(),
        self.end.text(),
        self.block.transpile(transpiler),
      ),
      TKind::Pretty => format!(
        "for (var $$x = {}; $$x < {}; $$x++) {{\n\t{}\n}}",
        self.start.text(),
        self.end.text(),
        self.block.transpile(transpiler),
      ),
    }
  }
}

impl LoopForRange {
  pub fn new(
    start: Box<dyn Value>,
    end: Box<dyn Value>,
    block: Box<dyn Value>,
  ) -> Self {
    LoopForRange { block, end, start }
  }

  pub fn add_block(&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = block;
    self
  }

  pub fn add_end(&mut self, end: Box<dyn Value>) -> &mut Self {
    self.end = end;
    self
  }

  pub fn add_start(&mut self, start: Box<dyn Value>) -> &mut Self {
    self.start = start;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
