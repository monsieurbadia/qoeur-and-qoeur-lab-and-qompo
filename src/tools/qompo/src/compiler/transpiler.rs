use crate::compiler::parser::Parser;
use crate::computer::app::App;
use crate::console_log;
use crate::value::attr::{Attr, PropValue};
use crate::value::program::Program;
use crate::value::tag::{KwKind::*, Tag, TagKind};
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};

use qoeurc::analyzer::interpreter::Interpreter;

use rsass::compile_scss;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Transpiler {
  interpreter: Interpreter,
  program: Box<dyn HtmlValue>,
}

impl Transpiler {
  pub fn new(program: &Box<dyn HtmlValue>, interpreter: Interpreter) -> Self {
    Transpiler {
      program: program.to_owned(),
      interpreter,
    }
  }

  pub fn text(&self) -> String {
    format!("{}", self.program.text())
  }

  pub fn interpreter(&mut self) -> &mut Interpreter {
    &mut self.interpreter
  }

  pub fn transpile(&mut self) -> Result<Box<dyn HtmlValue>, String> {
    Ok(self.program.to_owned().transpile(self)?)
  }
}
