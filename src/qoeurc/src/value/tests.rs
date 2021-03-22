extern crate qutonium;

use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("pub mod value", { "test empty input" || { Ok(()) } });
}
