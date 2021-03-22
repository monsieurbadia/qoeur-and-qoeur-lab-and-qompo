#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
  pub column: usize,
  pub line: usize,
}

impl Location {
  pub fn new(line: usize, column: usize) -> Self {
    Location { column, line }
  }
}
