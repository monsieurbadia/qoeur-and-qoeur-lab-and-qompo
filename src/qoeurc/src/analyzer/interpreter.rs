use crate::analyzer::environment::scope::{Scope, TScope};
use crate::value::Value;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub type InterpreterError = String;
pub type ValueResult<T> = Result<T, InterpreterError>;

#[derive(Clone, Debug)]
pub struct Interpreter {
  pub scope: TScope,
}

impl Interpreter {
  pub fn new() -> Self {
    Interpreter {
      scope: Rc::new(RefCell::new(Scope::new())),
    }
  }

  pub fn new_with_outer(outer: Scope) -> Self {
    Interpreter {
      scope: Rc::new(RefCell::new(outer)),
    }
  }

  pub fn eval(
    &mut self,
    program: &Box<dyn Value>,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(program.to_owned().eval(self)?)
  }

  pub fn scope(&self) -> Ref<'_, Scope> {
    self.scope.borrow()
  }

  pub fn scope_mut(&self) -> RefMut<'_, Scope> {
    self.scope.borrow_mut()
  }
}
