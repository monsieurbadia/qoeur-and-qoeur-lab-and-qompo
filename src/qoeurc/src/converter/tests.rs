extern crate qutonium;

use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("qoeurc::parser", {
    "test syntax of comments" || { Ok(()) }
  });
}
