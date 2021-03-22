pub mod attr;
pub mod capsule;
pub mod program;
pub mod tag;

use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::value::attr::{AKind, Attr, PropValue};
use crate::value::tag::{Tag, TagKind};
use qoeurc::analyzer::interpreter::Interpreter;

use std::fmt;

use wasm_bindgen::JsValue;

pub type ValueError = String;
pub type HtmlValueResult<T> = Result<T, ValueError>;

#[derive(Clone, Debug, PartialEq)]
pub enum VKind {
  Attr(AKind),
  Program(Vec<Box<dyn HtmlValue>>),
  Script(JsValue),
  Ui(JsValue),
  Tag(TagKind, Vec<Box<dyn HtmlValue>>, Vec<Box<dyn HtmlValue>>),
  Text(JsValue),
}

impl fmt::Display for VKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl VKind {
  pub fn text(&self) -> String {
    match self {
      VKind::Attr(ref kind) => match kind {
        AKind::Key(ref value) => {
          format!("{}", value.as_string().unwrap())
        }
        AKind::Property(ref prop, PropValue::Bool(ref value))
        | AKind::Property(ref prop, PropValue::String(ref value)) => {
          format!(
            r#"{}={}"#,
            prop.as_string().unwrap(),
            value.as_string().unwrap(),
          )
        }
        AKind::Style(ref prop, ref value) => {
          format!(
            "{}:{}",
            prop.as_string().unwrap(),
            value.as_string().unwrap(),
          )
        }
      },
      VKind::Program(ref nodes) => {
        let n = nodes
          .iter()
          .map(|n| n.text())
          .collect::<Vec<String>>()
          .join("");

        format!("{}", n)
      }
      VKind::Tag(ref name, _, _) => format!("{}", name),
      VKind::Text(ref value) | VKind::Ui(ref value) => {
        format!("{}", value.as_string().unwrap())
      }
      VKind::Script(ref value) => {
        format!("{}", value.as_string().unwrap())
      }
    }
  }
}

pub trait VBehavior {
  fn boxed(&self) -> Box<dyn HtmlValue>;
  fn cloned(&self) -> Box<dyn HtmlValue>;
}

impl<T> VBehavior for T
where
  T: 'static + HtmlValue + Clone,
{
  fn boxed(&self) -> Box<dyn HtmlValue> {
    Box::new(self.to_owned())
  }

  fn cloned(&self) -> Box<dyn HtmlValue> {
    Box::new(self.clone())
  }
}

pub trait HtmlValue: fmt::Debug + VBehavior {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> HtmlValueResult<Box<dyn HtmlValue>>;
  fn parse(
    &mut self,
    parser: &mut Parser,
  ) -> HtmlValueResult<Box<dyn HtmlValue>>;
  fn text(&self) -> String;
  fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> HtmlValueResult<Box<dyn HtmlValue>>;
  fn vkind(&self) -> VKind;
}

impl Clone for Box<dyn HtmlValue> {
  fn clone(&self) -> Self {
    self.cloned()
  }
}

impl PartialEq for Box<dyn HtmlValue> {
  fn eq(&self, rhs: &Box<dyn HtmlValue>) -> bool {
    self.vkind() == rhs.vkind()
  }
}
