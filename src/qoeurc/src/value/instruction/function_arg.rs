use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct FunctionArg {
  pub name: Box<dyn Value>,
  pub kind: Box<dyn Value>,
}

impl fmt::Display for FunctionArg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Default for FunctionArg {
  fn default() -> Self {
    FunctionArg::new(void!(), void!())
  }
}

impl Value for FunctionArg {
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
    IKind::FunctionArg
  }

  fn parse(&mut self, _parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    Ok(self.boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("{} : {}", self.name, self.kind)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!("{}", self.name.transpile(transpiler)),
      _ => format!("{}", self.name),
    }
  }
}

impl FunctionArg {
  pub fn new(name: Box<dyn Value>, kind: Box<dyn Value>) -> Self {
    FunctionArg { name, kind }
  }

  pub fn add_kind(&mut self, kind: Box<dyn Value>) -> &mut Self {
    self.kind = kind;
    self
  }

  pub fn add_name(&mut self, name: Box<dyn Value>) -> &mut Self {
    self.name = name;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  pub fn parse_arg(parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let name = Identifier::default().parse(parser)?;

    parser.expect_first(TokenKind::Symbol(Colon))?;
    parser.next_token();

    let kind = Identifier::default().parse(parser)?;

    Ok(FunctionArg::default().add_name(name).add_kind(kind).boxed())
  }

  pub fn parse_args(parser: &mut Parser) -> ParserResult<Vec<Box<dyn Value>>> {
    let mut function_args = vec![];

    if parser.first_is(TokenKind::GroupEnd(Parenthesis)) {
      parser.next_token();
      return Ok(function_args);
    }

    function_args.push(FunctionArg::parse_arg(parser)?);

    while parser.first_is(TokenKind::Symbol(Comma)) {
      parser.next_token();
      function_args.push(FunctionArg::parse_arg(parser)?);
    }

    parser.expect_first(TokenKind::GroupEnd(Parenthesis))?;

    Ok(function_args)
  }
}
