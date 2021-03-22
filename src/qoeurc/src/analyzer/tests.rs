use super::*;

extern crate qutonium;
use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("qoeurc::analyzer", { "test empty input" || { Ok(()) } });
}
