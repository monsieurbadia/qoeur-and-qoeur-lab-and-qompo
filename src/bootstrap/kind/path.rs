pub use crate::kind::path::PathKind::*;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum PathKind {
  Exist(String),
}

impl fmt::Display for PathKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PathKind::Exist(path) => write!(f, "{}", path),
    }
  }
}
