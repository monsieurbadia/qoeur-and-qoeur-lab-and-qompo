pub use crate::tokenizer::kind::group::GroupKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GroupKind {
  Brace,
  Bracket,
  NoGroup,
  Parenthesis,
}
