use crate::cmd::{Cmd, CmdResult};

impl Cmd {
  pub fn run_version(&mut self) -> CmdResult<()> {
    println!("v{}", Cmd::version());
    Ok(())
  }

  pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
  }
}
