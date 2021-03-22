pub use crate::tokenizer::kind::literal::LiteralKind::*;

use crate::tokenizer::kind::*;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
  Bool,
  Binary,
  Char,
  Int,
  Float,
  Str,
  Error,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Literal {
  pub kind: LiteralKind,
  pub suffix: Option<&'static str>,
  pub symbol: &'static str,
}

impl fmt::Display for Literal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Literal {
      kind,
      symbol,
      suffix,
    } = *self;

    match kind {
      Binary => write!(f, "b'{}'", symbol)?,
      Char => write!(f, "'{}'", symbol)?,
      Str => write!(f, "\"{}\"", symbol)?,
      Int | Float | Bool | Error => write!(f, "{}", symbol)?,
    }

    if let Some(suffix) = suffix {
      write!(f, "{}", suffix)?;
    }

    Ok(())
  }
}

impl Literal {
  pub fn new(
    kind: LiteralKind,
    symbol: &'static str,
    suffix: Option<&'static str>,
  ) -> Self {
    Literal {
      kind,
      symbol,
      suffix,
    }
  }

  pub fn keyword(name: &str) -> TokenKind {
    match name {
      "as" => Keyword(As),
      "async" => Keyword(Async),
      "await" => Keyword(Await),
      "box" => Keyword(BOX),
      "break" => Keyword(Break),
      "capsule" => Keyword(Capsule),
      "const" => Keyword(Const),
      "continue" => Keyword(Continue),
      "do" => Keyword(Do),
      "else" => Keyword(Else),
      "enum" => Keyword(Enum),
      "extern" => Keyword(Extern),
      "false" => Keyword(False),
      "for" => Keyword(For),
      "ƒ" => Keyword(Function),
      "if" => Keyword(If),
      "impl" => Keyword(Impl),
      "import" => Keyword(Import),
      "loop" => Keyword(Loop),
      "macro" => Keyword(Macro),
      "match" => Keyword(Match),
      "mod" => Keyword(Module),
      "move" => Keyword(Move),
      "program" => Keyword(Program),
      "pub" => Keyword(Public),
      "ref" => Keyword(Ref),
      "return" => Keyword(Return),
      "self" => Keyword(SelfLower),
      "Self" => Keyword(SelfUpper),
      "static" => Keyword(Static),
      "struct" => Keyword(Struct),
      "super" => Keyword(Super),
      "true" => Keyword(True),
      "type" => Keyword(Type),
      "typeof" => Keyword(Typeof),
      "_" => Keyword(Underscore),
      "unsafe" => Keyword(Unsafe),
      "use" => Keyword(Use),
      "val" => Keyword(Val),
      "while" => Keyword(While),
      _ => Identifier,
    }
  }
}
