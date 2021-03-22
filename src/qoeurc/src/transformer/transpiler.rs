use crate::analyzer::interpreter::Interpreter;
use crate::value::Value;

pub type TranspilerError = String;
pub type TranspilerResult<T> = Result<T, TranspilerError>;

#[derive(Clone, Debug)]
pub enum TKind {
  Json,
  Inline,
  Pretty,
}

impl From<&str> for TKind {
  fn from(lhs: &str) -> TKind {
    match lhs {
      "json" => TKind::Json,
      "pretty" => TKind::Pretty,
      _ => TKind::Inline,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Transpiler {
  pub mode: TKind,
  pub interpreter: Interpreter,
}

impl Transpiler {
  pub fn new(interpreter: Interpreter, mode: TKind) -> Self {
    Transpiler { mode, interpreter }
  }

  pub fn mode(&self) -> TKind {
    self.mode.to_owned()
  }

  pub fn transpile(
    &mut self,
    program: &Box<dyn Value>,
  ) -> TranspilerResult<String> {
    Ok(program.to_owned().transpile(self))
  }
}
