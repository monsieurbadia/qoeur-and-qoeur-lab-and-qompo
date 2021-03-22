use crate::kind::cmd::CmdKind;
use crate::kind::input::InputKind;
use crate::kind::mode::ModeKind;
use crate::kind::path::PathKind;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ArgKind {
  Cmd(CmdKind),
  Input(InputKind),
  Mode(ModeKind),
  Path(PathKind),
}

impl fmt::Display for ArgKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ArgKind::Path(PathKind::Exist(path)) => write!(f, "{}", path),
      _ => write!(f, "{:?}", self),
    }
  }
}
