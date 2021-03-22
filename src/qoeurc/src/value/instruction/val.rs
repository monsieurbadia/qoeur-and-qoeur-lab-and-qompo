use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::expression::Expression;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::ty::Ty;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Val {
  pub immutable: bool,
  pub kind: Option<Box<dyn Value>>,
  pub name: Box<dyn Value>,
  pub value: Option<Box<dyn Value>>,
}

impl Default for Val {
  fn default() -> Self {
    Val::new(true, None, void!(), None)
  }
}

impl fmt::Display for Val {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Val {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let value = self.value.as_ref().unwrap().to_owned().eval(interpreter)?;

    self.add_value(value);

    interpreter.scope_mut().add_variable(&self.boxed())?;

    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Statement
  }

  fn ikind(&self) -> IKind {
    IKind::Val
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.expect_first(TokenKind::Identifier)?;

    let name = Identifier::default().parse(parser)?;

    parser.expect_first(TokenKind::Symbol(Colon))?;
    parser.next_token();

    let kind = Ty::default().parse(parser)?;

    parser.expect_first(TokenKind::Operator(Assign))?;
    parser.next_token();

    let value = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    parser.expect_first(TokenKind::Symbol(Semicolon))?;

    Ok(self.add_name(name).add_kind(kind).add_value(value).boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let name = &self.name;
    let value = self.value.as_ref().unwrap();

    match (self.immutable, self.kind.as_ref()) {
      (true, Some(kind)) => format!("val {} : {} = {};", name, kind, value),
      (true, None) => format!("val {} := {};", name, value),
      (false, Some(kind)) => {
        format!("val mut {} : {} = {};", name, kind, value)
      }
      (false, None) => format!("val mut {} := {};", name, value),
    }
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let value = self
      .value
      .as_ref()
      .unwrap()
      .to_owned()
      .transpile(transpiler);
    let name = self.name.transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => {
        format!(
          r#"{{
            "declarations": [{{
              "id": {},
              "init": {},
              "type": "VariableDeclarator"
            }}],
            "kind": "var",
            "type": "VariableDeclaration"
          }}"#,
          name, value,
        )
      }
      _ => {
        format!("var {} = {};\n", name, value)
      }
    }
  }
}

impl Val {
  pub fn new(
    immutable: bool,
    kind: Option<Box<dyn Value>>,
    name: Box<dyn Value>,
    value: Option<Box<dyn Value>>,
  ) -> Self {
    Val {
      immutable,
      kind,
      name,
      value,
    }
  }

  pub fn add_name(&mut self, name: Box<dyn Value>) -> &mut Self {
    self.name = name;
    self
  }

  pub fn add_immutable(&mut self, immutable: bool) -> &mut Self {
    self.immutable = immutable;
    self
  }

  pub fn add_kind(&mut self, kind: Box<dyn Value>) -> &mut Self {
    self.kind = Some(kind);
    self
  }

  pub fn add_value(&mut self, value: Box<dyn Value>) -> &mut Self {
    self.value = Some(value);
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}
