use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::value::tag::KwKind::*;
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};
use qoeurc::analyzer::interpreter::Interpreter;

use wasm_bindgen::JsValue;

use std::convert::From;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum AKind {
  Key(JsValue),
  Property(JsValue, PropValue),
  Style(JsValue, JsValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropValue {
  Bool(JsValue),
  String(JsValue),
}

impl fmt::Display for PropValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl PropValue {
  pub fn text(&self) -> String {
    match self {
      PropValue::Bool(ref value) => format!("{}", value.as_string().unwrap(),),
      PropValue::String(ref value) => {
        format!("{}", value.as_string().unwrap(),)
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attr {
  pub prop: JsValue,
  pub value: JsValue,
  pub kind: VKind,
}

impl HtmlValue for Attr {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    Ok(self.boxed())
  }

  fn parse(
    &mut self,
    _parser: &mut Parser,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    match self.kind {
      VKind::Attr(AKind::Key(ref value)) => {}
      VKind::Attr(AKind::Property(ref prop, ref value)) => {}
      VKind::Attr(AKind::Style(ref prop, ref value)) => {}
      _ => {}
    };

    Ok(self.boxed())
  }

  fn text(&self) -> String {
    match self.kind {
      VKind::Attr(AKind::Key(ref value)) => {
        format!("{}", value.as_string().unwrap(),)
      }
      VKind::Attr(AKind::Property(ref prop, ref value)) => {
        format!(r#"{}="{}""#, prop.as_string().unwrap(), value,)
      }
      VKind::Attr(AKind::Style(ref prop, ref value)) => format!(
        "{}:{}",
        prop.as_string().unwrap(),
        value.as_string().unwrap(),
      ),
      _ => format!(""),
    }
  }

  fn transpile(
    &mut self,
    _transpiler: &mut Transpiler,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    Ok(self.boxed())
  }

  fn vkind(&self) -> VKind {
    self.kind.to_owned()
  }
}

impl Attr {
  pub fn new(prop: JsValue, value: JsValue, kind: VKind) -> Self {
    Attr { prop, value, kind }
  }
}
