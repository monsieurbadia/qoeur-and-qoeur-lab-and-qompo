extern crate qutonium;

use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("pub mod utils", { "test empty input" || { Ok(()) } });
}
