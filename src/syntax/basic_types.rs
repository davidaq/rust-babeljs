
#[derive(Copy, Clone)]
pub struct Position {
  pub line: usize,
  pub col: usize,
}

#[derive(Copy, Clone)]
pub struct SourceLoc {
  pub start: Position,
  pub end: Position,
}

impl SourceLoc {
  pub fn new () -> Self {
    SourceLoc {
      start: Position {
        line: 0,
        col: 0,
      },
      end: Position {
        line: 0,
        col: 0,
      },
    }
  }
}
