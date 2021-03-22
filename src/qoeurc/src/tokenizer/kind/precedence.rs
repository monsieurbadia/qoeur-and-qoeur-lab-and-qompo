pub use crate::tokenizer::kind::precedence::PrecedenceKind::*;
//
#[derive(Debug, PartialEq, PartialOrd)]
pub enum PrecedenceKind {
  Lowest,
  Assignement,
  Conditional,
  Sum,
  Exponent,
  Unary,
  Call,
  Index,
}
