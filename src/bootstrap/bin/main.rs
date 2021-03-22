use bootstrap::prelude::*;

fn main() -> CmdResult<()> {
  let args = std::env::args().skip(1).collect::<Vec<_>>();

  Cmd::new(Arg::new(&args)).run()
}
