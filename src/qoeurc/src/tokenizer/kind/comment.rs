pub use crate::tokenizer::kind::comment::CommentKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommentKind {
  Block,
  Doc,
  Line,
}
