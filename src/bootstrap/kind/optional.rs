pub use crate::kind::optional::OptionalKind::*;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum OptionalKind {
  Inline,
  Json,
  Pretty,
}

impl fmt::Display for OptionalKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      OptionalKind::Inline => write!(f, "inline"),
      OptionalKind::Json => write!(f, "json"),
      OptionalKind::Pretty => write!(f, "pretty"),
    }
  }
}
