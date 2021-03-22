use crate::cmd::{Cmd, CmdResult};

impl Cmd {
  pub fn copyright() -> CmdResult<()> {
    println!("\n\tcopyright (c) 2020 qoeur. all rights reserved.\n");
    Ok(())
  }

  pub fn run_copyright(&mut self) -> CmdResult<()> {
    Cmd::copyright()
  }
}
