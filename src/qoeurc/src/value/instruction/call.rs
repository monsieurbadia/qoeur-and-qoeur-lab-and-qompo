use crate::analyzer::environment::scope::Scope;
use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::{eval_expressions, strip_exprs, transpile_exprs};
use crate::value::instruction::function::Function;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Call {
  pub callee: Box<dyn Value>,
  pub args: Option<Vec<Box<dyn Value>>>,
}

impl Default for Call {
  fn default() -> Self {
    Call::new(void!(), vec![])
  }
}

impl fmt::Display for Call {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl From<Box<dyn Value>> for Call {
  fn from(callee: Box<dyn Value>) -> Call {
    Call::new(callee, vec![])
  }
}

impl Value for Call {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let callee = self.callee.eval(interpreter)?;
    let function = callee.as_ref().downcast_ref::<Function>().unwrap();

    let args_expected =
      eval_expressions(interpreter, self.args.as_ref().unwrap().to_vec())?;

    let Function {
      args,
      block,
      kind: _,
      name: _,
      scope,
    } = function;

    if args.as_ref().unwrap().len() != args_expected.len() {
      return Err(format!("error parameter count"));
    }

    let scope = Scope::extend_scope(
      scope.as_ref().unwrap().to_owned(),
      args.as_ref().unwrap().to_vec(),
      args_expected,
    );

    let mut interpreter_with_outer = Interpreter::new_with_outer(scope);
    let mut value = void!().boxed();

    for stmt in block {
      value = stmt.to_owned().eval(&mut interpreter_with_outer)?;

      if let IKind::Return = value.ikind() {
        return Ok(value);
      }
    }

    Ok(value)
  }

  fn ikind(&self) -> IKind {
    IKind::Call
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let args = parser.parse_until(TokenKind::GroupEnd(Parenthesis))?;

    Ok(self.add_args(args).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let args = strip_exprs(&self.args.as_ref().unwrap(), ", ");
    let callee = self.callee.text();

    format!("{}({})", callee, args)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let args = transpile_exprs(transpiler, &self.args.as_ref().unwrap(), ", ");
    let callee = self.callee.transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          arguments: [{}],
          callee: {},
          "type": "CallExpression"
        }}"#,
        args, callee,
      ),
      _ => self.text(),
    }
  }
}

impl Call {
  pub fn new(callee: Box<dyn Value>, args: Vec<Box<dyn Value>>) -> Self {
    Call {
      args: Some(args),
      callee,
    }
  }

  pub fn add_args(&mut self, args: Vec<Box<dyn Value>>) -> &mut Self {
    self.args = Some(args);
    self
  }

  pub fn add_callee(&mut self, callee: Box<dyn Value>) -> &mut Self {
    self.callee = callee;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
