pub use crate::tokenizer::kind::token::TokenKind::*;

use crate::tokenizer::kind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
  EOF,
  Comment(CommentKind),
  GroupStart(GroupKind),
  GroupEnd(GroupKind),
  Identifier,
  Keyword(KeywordKind),
  Literal(LiteralKind),
  NewLine,
  Operator(OperatorKind),
  Symbol(SymbolKind),
  Unknown,
  Whitespace,
}

impl TokenKind {
  pub fn precedence(kind: TokenKind) -> PrecedenceKind {
    match kind {
      Operator(Star) | Operator(Slash) => PrecedenceKind::Exponent,
      Operator(Plus) | Operator(Minus) => PrecedenceKind::Sum,

      Operator(LessThan)
      | Operator(GreaterThan)
      | Operator(LessThanOrEqual)
      | Operator(GreaterThanOrEqual) => PrecedenceKind::Conditional,

      Operator(Equal) | Operator(NotEqual) => PrecedenceKind::Assignement,
      GroupStart(Parenthesis) => PrecedenceKind::Call,
      GroupStart(Bracket) => PrecedenceKind::Index,
      _ => PrecedenceKind::Lowest,
    }
  }
}
