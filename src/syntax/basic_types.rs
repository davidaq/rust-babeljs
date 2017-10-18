
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
