use crate::value::instruction::function::Function;
use crate::value::instruction::function_arg::FunctionArg;
use crate::value::instruction::identifier::Identifier;
use crate::value::instruction::val::Val;
use crate::value::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type ScopeError = String;
pub type ScopeResult<T> = Result<T, ScopeError>;
pub type TScope = Rc<RefCell<Scope>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
  functions: HashMap<String, Box<dyn Value>>,
  outer: Option<TScope>,
  variables: HashMap<String, Box<dyn Value>>,
}

impl Scope {
  pub fn new() -> Self {
    Scope {
      variables: HashMap::new(),
      outer: None,
      functions: HashMap::new(),
    }
  }

  pub fn new_with_outer(outer: TScope) -> Self {
    Scope {
      functions: HashMap::new(),
      outer: Some(outer),
      variables: HashMap::new(),
    }
  }

  pub fn add_function(
    &mut self,
    statement: &Box<dyn Value>,
  ) -> ScopeResult<()> {
    let func = statement.as_ref().downcast_ref::<Function>().unwrap();
    let fname = &func
      .name
      .as_ref()
      .downcast_ref::<Identifier>()
      .unwrap()
      .name;

    match self.functions.get(fname) {
      Some(_) => Err(format!("function already exist")),
      None => Ok(self.set_function(fname, func.boxed())),
    }
  }

  pub fn add_variable(
    &mut self,
    statement: &Box<dyn Value>,
  ) -> ScopeResult<()> {
    let val = statement.as_ref().downcast_ref::<Val>().unwrap();
    let vname = &val.name.as_ref().downcast_ref::<Identifier>().unwrap().name;
    let vvalue = val.value.as_ref().unwrap().boxed();

    match self.variables.get(vname) {
      Some(_) => Err(format!("variable already exist")),
      None => Ok(self.set_variable(vname, vvalue)),
    }
  }

  pub fn get_function(&self, name: &str) -> Option<&Box<dyn Value>> {
    self.functions.get(name)
  }

  pub fn get_variable(&self, name: &str) -> Option<&Box<dyn Value>> {
    self.variables.get(name)
  }

  fn set_function(&mut self, name: &str, function: Box<dyn Value>) {
    self.functions.insert(name.into(), function);
  }

  fn set_variable(&mut self, name: &str, value: Box<dyn Value>) {
    self.variables.insert(name.into(), value);
  }

  pub fn extend_scope(
    outer: TScope,
    lhs: Vec<Box<dyn Value>>,
    rhs: Vec<Box<dyn Value>>,
  ) -> Scope {
    let mut scope = Scope::new_with_outer(outer);

    for (x, lhs_arg) in lhs.iter().enumerate() {
      let arg = &lhs_arg.as_ref().downcast_ref::<FunctionArg>().unwrap();
      let name = &arg.name.as_ref().downcast_ref::<Identifier>().unwrap().name;

      scope.set_variable(name, rhs[x].to_owned());
    }

    scope
  }
}
