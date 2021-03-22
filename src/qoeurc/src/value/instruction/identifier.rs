use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

use inflector::cases::camelcase::to_camel_case;

#[derive(Clone, Debug)]
pub struct Identifier {
  pub name: String,
}

impl Default for Identifier {
  fn default() -> Self {
    Identifier::new("")
  }
}

impl fmt::Display for Identifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Identifier {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let name = &self.name;

    if let Some(function) = interpreter.scope().get_function(name) {
      Ok(function.boxed())
    } else if let Some(variable) = interpreter.scope().get_variable(name) {
      Ok(variable.boxed())
    } else {
      Err(format!(
        "error eval unknown identifier expression: {}",
        name
      ))
    }
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Identifier
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    let name = &parser.token.literal;

    match parser.token.kind {
      TokenKind::Identifier => Ok(self.add_name(name).boxed()),
      _ => Err(format!(
        "unexpected error on identifier parse with {}",
        name
      )),
    }
  }

  fn print(&self) {
    println!("{}", self.text());
  }

  fn text(&self) -> String {
    format!("{}", self.name)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let name = self.text();

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "name": "{}",
          "type": "Identifier"
        }}"#,
        to_camel_case(&name),
      ),
      _ => name,
    }
  }
}

impl Identifier {
  pub fn new(name: &str) -> Self {
    Identifier { name: name.into() }
  }

  pub fn add_name(&mut self, name: &str) -> &mut Self {
    self.name = name.into();
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
