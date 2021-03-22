use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::expression::Expression;
use crate::value::instruction::function::Function;
use crate::value::instruction::loop_for_range::LoopForRange;
use crate::value::instruction::loop_infinite::LoopInfinite;
use crate::value::instruction::loop_while::LoopWhile;
use crate::value::instruction::return_value::Return;
use crate::value::instruction::val::Val;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Statement {
  pub node: Box<dyn Value>,
}

impl Default for Statement {
  fn default() -> Self {
    Statement::new(void!())
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Statement {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.node.eval(interpreter)?)
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::Statement
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    match parser.token.kind {
      TokenKind::Keyword(For) => LoopForRange::default().parse(parser),
      TokenKind::Keyword(Function) => Function::default().parse(parser),
      TokenKind::Keyword(Loop) => LoopInfinite::default().parse(parser),
      TokenKind::Keyword(Return) => Return::default().parse(parser),
      TokenKind::Keyword(Val) => Val::default().parse(parser),
      TokenKind::Keyword(While) => LoopWhile::default().parse(parser),
      _ => Expression::default().parse(parser),
    }
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{}", self.node.text())
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(r#"{{ {} }}"#, self.node.transpile(transpiler)),
      _ => self.text(),
    }
  }
}

impl Statement {
  pub fn new(node: Box<dyn Value>) -> Self {
    Statement { node }
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
