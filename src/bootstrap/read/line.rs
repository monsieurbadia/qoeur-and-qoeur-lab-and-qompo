use crate::cmd::Cmd;
use crate::reader::{Reader, ReaderResult};

use std::io;
use std::io::Stdin;
use std::io::Stdout;

impl Reader {
  pub fn readline(&mut self) -> ReaderResult<()> {
    let mut line = String::new();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    Cmd::banner();

    loop {
      Reader::prompt();

      match Reader::advance_line(&mut line, &mut stdin, &mut stdout) {
        Err(_) => (),
        Ok(line) => self.advance_mode(&line)?,
      };
    }
  }

  fn advance_line<'a>(
    line: &'a mut String,
    stdin: &'a mut Stdin,
    stdout: &'a mut Stdout,
  ) -> ReaderResult<&'a mut str> {
    io::Write::flush(stdout).expect("flush failed!");
    line.clear();

    stdin.read_line(line).unwrap();
    line.truncate(line.trim_end().len());

    Ok(line)
  }
}
