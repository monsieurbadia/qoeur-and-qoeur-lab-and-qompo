pub use crate::kind::cmd::CmdKind::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CmdKind {
  Copyright,
  Help,
  License,
  Repl,
  Version,
}
