use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::{eval_expressions, strip_exprs, transpile_exprs};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Array {
  pub data: Vec<Box<dyn Value>>,
}

impl Default for Array {
  fn default() -> Self {
    Array::new(vec![])
  }
}

impl fmt::Display for Array {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Array {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let data = eval_expressions(interpreter, self.data.to_vec())?;

    Ok(self.add_data(data).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Array(self.data.to_vec())
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let data = parser.parse_until(TokenKind::GroupEnd(Bracket))?;

    Ok(self.add_data(data).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    format!("[{}]", strip_exprs(&self.data, ", "))
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let data =
      &eval_expressions(&mut transpiler.interpreter, self.data.to_vec())
        .unwrap();

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "elements": [{}],
          "type": "ArrayExpression"
        }}"#,
        transpile_exprs(transpiler, data, ", "),
      ),
      TKind::Inline => {
        format!("[{}]", transpile_exprs(transpiler, data, ", "),)
      }
      TKind::Pretty => {
        format!("[\n{}\n]", transpile_exprs(transpiler, data, "\n"),)
      }
    }
  }
}

impl Array {
  pub fn new(data: Vec<Box<dyn Value>>) -> Self {
    Array { data }
  }

  pub fn add_data(&mut self, data: Vec<Box<dyn Value>>) -> &mut Self {
    self.data = data;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
