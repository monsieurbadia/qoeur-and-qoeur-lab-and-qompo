use crate::reader::{Reader, ReaderResult};

use std::fs;
use std::path::Path;

impl Reader {
  pub fn readfile(&mut self) -> ReaderResult<()> {
    let arg = &self.arg.clone().unwrap();
    let path = &arg.path.as_ref().unwrap().to_string();

    match Reader::advance_file(path) {
      Err(error) => Err(format!("{}", error)),
      Ok(file) => self.advance_mode(&file),
    }
  }

  fn advance_file(url: &str) -> ReaderResult<String> {
    let pathname = Path::new(url);

    match fs::read_to_string(pathname) {
      Err(error) => Err(format!("{}", error)),
      Ok(file) => Ok(file),
    }
  }
}
