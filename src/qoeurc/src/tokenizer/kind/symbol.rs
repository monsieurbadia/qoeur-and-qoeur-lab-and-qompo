pub use crate::tokenizer::kind::symbol::SymbolKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolKind {
  ColonColon,
  Arrow,
  ArrowFunction,
  Attribute,
  Comma,
  Colon,
  Semicolon,
  Question,
  At,
  Dot,
  Dollar,
  Bang,
  Shebang,
  DollarDotDot,
}
