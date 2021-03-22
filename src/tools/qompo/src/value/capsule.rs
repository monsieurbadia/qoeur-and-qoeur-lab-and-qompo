use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::console_log;
use crate::value::tag::Tag;
use crate::value::tag::{KwKind::*, TagKind};
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};
use qoeurc::analyzer::interpreter::Interpreter;

use qoeurc::value::instruction::IKind;

use std::convert::From;
use std::fmt;

use rsass::compile_scss;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, PartialEq)]
pub struct Capsule {
  pub kind: TagKind,
  pub attrs: Vec<Box<dyn HtmlValue>>,
  pub children: Vec<Box<dyn HtmlValue>>,
}

impl HtmlValue for Capsule {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    Ok(self.boxed())
  }

  fn parse(
    &mut self,
    parser: &mut Parser,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    let source = self.children[0].vkind().to_string();

    let children = match self.vkind() {
      VKind::Tag(_, _, _) => match self.kind {
        TagKind::Kw(Script) => Ok(vec![Tag::new(
          TagKind::Script(source.into()),
          vec![],
          vec![],
        )
        .boxed()]),
        TagKind::Ui(_) => {
          Ok(vec![Tag::new(TagKind::Text(source), vec![], vec![]).boxed()])
        }
        TagKind::Kw(View) => Ok(Parser::new(&source).parse_vdom().unwrap()),
        _ => Err(format!("parse capsule unknown error.")),
      },
      _ => Err(format!("parse capsule tag error.")),
    };

    Ok(self.add_children(children?).boxed())
  }

  fn text(&self) -> String {
    format!(
      "use capsule {} {{ {} }}",
      self.kind.to_string(),
      self
        .children
        .iter()
        .map(|child| child.text())
        .collect::<Vec<String>>()
        .join("")
    )
  }

  fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    let children = self
      .children
      .iter()
      .map(|v| v.to_owned().transpile(transpiler))
      .collect::<HtmlValueResult<Vec<Box<dyn HtmlValue>>>>()?;

    Ok(self.add_children(children).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Tag(
      self.kind.to_owned(),
      self.attrs.to_vec(),
      self.children.to_vec(),
    )
  }
}

impl Capsule {
  pub fn new(
    kind: TagKind,
    attrs: Vec<Box<dyn HtmlValue>>,
    children: Vec<Box<dyn HtmlValue>>,
  ) -> Self {
    Capsule {
      kind,
      attrs,
      children,
    }
  }

  pub fn add_children(
    &mut self,
    children: Vec<Box<dyn HtmlValue>>,
  ) -> &mut Self {
    self.children = children;
    self
  }
}
