pub use crate::kind::mode::ModeKind::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ModeKind {
  Ast,
  Eval,
  Js,
  Tokens,
}
