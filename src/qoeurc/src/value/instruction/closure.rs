use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::transpile_exprs;
use crate::value::instruction::block::Block;
use crate::value::instruction::function_arg::FunctionArg;
use crate::value::instruction::ty::Ty;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Closure {
  pub args: Option<Vec<Box<dyn Value>>>,
  pub block: Option<Box<dyn Value>>,
  pub kind: Option<Box<dyn Value>>,
}

impl Default for Closure {
  fn default() -> Self {
    Closure::new(vec![], void!(), void!())
  }
}

impl fmt::Display for Closure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Closure {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn ikind(&self) -> IKind {
    IKind::Closure
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let args = FunctionArg::parse_args(parser)?;

    parser.expect_first(TokenKind::Symbol(Arrow))?;
    parser.next_token();

    let ty = Ty::default().parse(parser)?;

    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(self.add_args(args).add_kind(ty).add_block(block).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    match &self.kind {
      None => format!("|{:?}| {{ {:?} }}", self.args, self.block),
      Some(k) => format!("|{:?}| -> {:?} {{ {:?} }}", self.args, k, self.block),
    }
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let args = transpile_exprs(transpiler, &self.args.as_ref().unwrap(), ", ");
    let block = self
      .block
      .as_ref()
      .unwrap()
      .to_owned()
      .transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "params": [{}],
          "type": "ArrowFunctionExpression"
        }}"#,
        block, args,
      ),
      TKind::Inline => format!("({}) => {{ {} }}", args, block),
      TKind::Pretty => format!("({}) => {{\n\t{}\n}}", args, block),
    }
  }
}

impl Closure {
  pub fn new(
    args: Vec<Box<dyn Value>>,
    kind: Box<dyn Value>,
    block: Box<dyn Value>,
  ) -> Self {
    Closure {
      args: Some(args),
      block: Some(block),
      kind: Some(kind),
    }
  }

  pub fn add_args(&mut self, args: Vec<Box<dyn Value>>) -> &mut Self {
    self.args = Some(args);
    self
  }

  pub fn add_block(&mut self, block: Box<dyn Value>) -> &mut Self {
    self.block = Some(block);
    self
  }

  pub fn add_kind(&mut self, kind: Box<dyn Value>) -> &mut Self {
    self.kind = Some(kind);
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
