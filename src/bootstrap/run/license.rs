use crate::cmd::{Cmd, CmdResult};

impl Cmd {
  pub fn run_license(&mut self) -> CmdResult<()> {
    Cmd::license()
  }

  pub fn license() -> CmdResult<()> {
    println!("read LICENSE file.");
    Ok(())
  }
}
