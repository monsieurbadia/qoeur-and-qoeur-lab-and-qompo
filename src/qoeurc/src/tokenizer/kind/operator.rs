pub use crate::tokenizer::kind::operator::OperatorKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperatorKind {
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  Caret,
  And,
  AndAnd,
  Or,
  OrOr,
  ShiftLeft,
  ShiftRight,
  AssignType,
  Assign,
  Equal,
  NotEqual,
  Range,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
}
