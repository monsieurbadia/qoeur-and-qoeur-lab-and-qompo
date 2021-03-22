pub use crate::tokenizer::kind::keyword::KeywordKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeywordKind {
  As,
  Async,
  Await,
  BOX,
  Break,
  Capsule,
  Const,
  Continue,
  Do,
  Else,
  Enum,
  Extern,
  False,
  For,
  Function,
  If,
  Impl,
  Import,
  Loop,
  Macro,
  Match,
  Module,
  Move,
  Program,
  Public,
  Ref,
  Return,
  SelfLower,
  SelfUpper,
  Static,
  Struct,
  Super,
  True,
  Type,
  Typeof,
  Underscore,
  Unsafe,
  Use,
  Val,
  While,
}

#[macro_export]
macro_rules! symbols {
  { $type:tt { $($kind:ident: $value:expr,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
          $($kind => write!(f, "{}", $value),)*
        }
      }
    }
  }
}

symbols! {
  KeywordKind {
    As: "as",
    Async: "async",
    Await: "await",
    BOX: "box",
    Break: "break",
    Capsule: "capsule",
    Const: "const",
    Continue: "contiue",
    Do: "do",
    Else: "else",
    Enum: "enum",
    Extern: "extern",
    False: "false",
    For: "for",
    Function: "ƒ",
    If: "if",
    Impl: "impl",
    Import: "import",
    Loop: "loop",
    Macro: "macro",
    Match: "match",
    Module: "mod",
    Move: "move",
    Program: "program",
    Public: "pub",
    Ref: "ref",
    Return: "return",
    SelfLower: "self",
    SelfUpper: "Self",
    Static: "static",
    Struct: "struct",
    Super: "super",
    True: "true",
    Type: "type",
    Typeof: "typeof",
    Underscore: "_",
    Unsafe: "unsafe",
    Use: "use",
    Val: "val",
    While: "while",
  }
}
