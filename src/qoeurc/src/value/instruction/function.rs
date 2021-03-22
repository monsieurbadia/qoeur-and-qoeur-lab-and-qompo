use crate::analyzer::environment::scope::TScope;
use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::{strip_exprs, transpile_exprs};
use crate::value::instruction::block::Block;
use crate::value::instruction::function_arg::FunctionArg;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::ty::{Ty, TyKind};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Function {
  pub args: Option<Vec<Box<dyn Value>>>,
  pub block: Option<Box<dyn Value>>,
  pub kind: Option<Box<dyn Value>>,
  pub name: Box<dyn Value>,
  pub scope: Option<TScope>,
}

impl Default for Function {
  fn default() -> Self {
    Function::new(vec![], void!(), void!(), void!(), None)
  }
}

impl fmt::Display for Function {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Function {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    self.add_scope(interpreter.scope.to_owned());

    interpreter.scope_mut().add_function(&self.boxed())?;

    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::Function(
      self.args.clone(),
      self.block.clone(),
      self.kind.clone(),
      Some(self.name.boxed()),
      self.scope.clone(),
    )
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.expect_first(TokenKind::Identifier)?;

    let name = Identifier::default().parse(parser)?;

    parser.expect_first(TokenKind::GroupStart(Parenthesis))?;

    let args = FunctionArg::parse_args(parser)?;

    let mut ty = Ty::default();

    if parser.first_is(TokenKind::Symbol(Arrow)) {
      parser.next_token();
      parser.next_token();
      self.add_kind(ty.parse(parser)?);
    } else {
      self.add_kind(ty.add_kind(TyKind::Array).boxed());
    }

    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let block = Block::default().parse(parser)?;

    Ok(self.add_name(name).add_args(args).add_block(block).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let args = strip_exprs(&self.args.as_ref().unwrap(), ", ");
    let name = &self.name;
    let block = &self
      .block
      .as_ref()
      .unwrap()
      .downcast_ref::<Block>()
      .unwrap();
    let stmts = strip_exprs(&block.statements, " ");
    let kind = &self.kind.as_ref();

    match kind {
      None => format!("ƒ {} ({}) {{ {} }}", name, args, stmts),
      Some(k) => {
        format!("ƒ {} ({}) -> {} {{ {} }}", name, args, k.text(), stmts,)
      }
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
    let name = self.name.transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": {},
          "id": {},
          "params": [{}],
          "type": "FunctionDeclaration"
        }}"#,
        block, name, args,
      ),
      TKind::Inline => format!("function {} ({}) {{ {} }}", name, args, block,),
      TKind::Pretty => {
        format!("function {} ({}) {{\n\t{}\n}}", name, args, block,)
      }
    }
  }
}

impl Function {
  pub fn new(
    args: Vec<Box<dyn Value>>,
    block: Box<dyn Value>,
    kind: Box<dyn Value>,
    name: Box<dyn Value>,
    scope: Option<TScope>,
  ) -> Self {
    Function {
      args: Some(args),
      block: Some(block),
      kind: Some(kind),
      name,
      scope,
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

  pub fn add_name(&mut self, name: Box<dyn Value>) -> &mut Self {
    self.name = name;
    self
  }

  pub fn add_scope(&mut self, scope: TScope) -> &mut Self {
    self.scope = Some(scope);
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
