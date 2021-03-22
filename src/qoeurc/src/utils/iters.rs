use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::IKind;
use crate::value::{Value, Values};
use crate::void;

pub fn eval_expressions(
  interpreter: &mut Interpreter,
  exprs: Vec<Box<dyn Value>>,
) -> ValueResult<Values> {
  let values = exprs
    .to_owned()
    .iter()
    .map(|expr| -> ValueResult<Box<dyn Value>> {
      Ok(expr.to_owned().eval(interpreter)?)
    })
    .collect::<ValueResult<Values>>()?;

  Ok(values)
}

pub fn eval_statements(
  interpreter: &mut Interpreter,
  stmts: Vec<Box<dyn Value>>,
) -> ValueResult<Box<dyn Value>> {
  let mut value = void!().boxed();

  for stmt in &stmts {
    value = stmt.to_owned().eval(interpreter)?;

    if let IKind::Return = value.ikind() {
      return Ok(value);
    }
  }

  Ok(value)
}

pub fn strip_exprs(expr: &Values, delimiter: &str) -> String {
  expr
    .iter()
    .map(|a| a.text())
    .collect::<Vec<String>>()
    .join(delimiter)
}

pub fn transpile_exprs(
  transpiler: &mut Transpiler,
  data: &Values,
  delimiter: &str,
) -> String {
  data
    .iter()
    .map(|d| d.to_owned().transpile(transpiler))
    .collect::<Vec<String>>()
    .join(delimiter)
}
