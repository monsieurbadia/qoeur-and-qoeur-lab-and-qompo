use crate::arg::Arg;
use crate::kind::*;

pub type CmdError = String;
pub type CmdResult<T> = Result<T, CmdError>;

#[derive(Clone, Debug)]
pub struct Cmd {
  pub arg: Arg,
}

impl Cmd {
  pub fn new(arg: Arg) -> Self {
    Cmd { arg }
  }

  pub fn advance_cmd(&mut self, cmd: ArgKind) -> CmdResult<()> {
    match cmd {
      c if c == ArgKind::Cmd(Copyright) => self.run_copyright(),
      c if c == ArgKind::Cmd(Help) => self.run_help(),
      c if c == ArgKind::Cmd(License) => self.run_license(),
      c if c == ArgKind::Cmd(Repl) => self.run_repl(),
      c if c == ArgKind::Cmd(Version) => self.run_version(),
      _ => self.run_help(),
    }
  }

  pub fn cmd(&mut self) -> CmdKind {
    self.arg.cmd.to_owned().unwrap_or(CmdKind::Help)
  }

  pub fn run(&mut self) -> CmdResult<()> {
    let kind = ArgKind::Cmd(self.cmd());

    self.advance_cmd(kind)
  }
}
