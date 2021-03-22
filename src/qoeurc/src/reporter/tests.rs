extern crate qutonium;

use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("qoeurc::reporter", { "test empty input" || { Ok(()) } });
}
