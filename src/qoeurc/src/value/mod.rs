pub mod instruction;
pub mod primitive;

#[cfg(test)]
mod tests;

use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::transformer::transpiler::Transpiler;
use crate::value::instruction::IKind;

use std::fmt;

use downcast_rs::{impl_downcast, Downcast};

pub trait VClone {
  fn boxed(&self) -> Box<dyn Value>;
  fn cloned(&self) -> Box<dyn Value>;
}

impl<T> VClone for T
where
  T: 'static + Value + Clone,
{
  fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  fn cloned(&self) -> Box<dyn Value> {
    Box::new(self.clone())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VKind {
  Expression,
  Statement,
}

pub type Values = Vec<Box<dyn Value>>;

pub trait Value: fmt::Debug + VClone + Downcast {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>>;
  fn ikind(&self) -> IKind;
  fn is_truthy(&self) -> bool {
    match self.ikind() {
      IKind::Bool(false) => false,
      _ => true,
    }
  }
  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>>;
  fn print(&self);
  fn text(&self) -> String;
  fn transpile(&mut self, transpiler: &mut Transpiler) -> String;
  fn vkind(&self) -> VKind;
}

impl fmt::Display for dyn Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl_downcast!(Value);

impl Clone for Box<dyn Value> {
  fn clone(&self) -> Self {
    self.cloned()
  }
}

impl PartialEq for Box<dyn Value> {
  fn eq(&self, rhs: &Box<dyn Value>) -> bool {
    self.ikind() == rhs.ikind()
  }
}
