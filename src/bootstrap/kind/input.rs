pub use crate::kind::input::InputKind::*;

#[derive(Clone, Debug, PartialEq)]
pub enum InputKind {
  File,
  Line,
}
