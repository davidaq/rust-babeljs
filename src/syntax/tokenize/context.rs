use syntax::basic_types::Position;
use util::Queue;

pub struct State {
  pub expr_allowed: bool,
  pub brace_stack: Vec<bool>,
}

impl State {
  fn new () -> Self {
    State {
      expr_allowed: true,
      brace_stack: vec![],
    }
  }
}

pub struct Context<'a> {
  pub state: State,
  input: &'a Queue<String>,
  buffer: Vec<char>,
  head: usize,
  cursor: usize,
  ended: bool,
  line: usize,
  col: usize,
  commit: usize,

  tmp_commit: usize,
  tmp_len: usize,
  tmp_line: usize,
  tmp_col: usize,
  tmp_content: String,
}

impl<'a> Context<'a> {
  pub fn new (input: &'a Queue<String>) -> Self {
    Context {
      state: State::new(),
      input: input,
      buffer: vec![],
      head: 0,
      cursor: 0,
      ended: false,
      line: 1,
      col: 0,
      commit: 0,

      tmp_commit: 0,
      tmp_len: 0,
      tmp_line: 1,
      tmp_col: 1,
      tmp_content: String::with_capacity(100),
    }
  }

  pub fn allow_hashbang (&self) -> bool {
    return self.commit == 0;
  }

  pub fn pos (&self) -> Position {
    Position {
      line: self.line,
      col: self.col,
    }
  }

  fn content_called_for_len (&self, len: usize) -> bool {
    return self.tmp_commit == self.commit && self.tmp_len == len;
  }

  pub fn commit (&mut self, len: usize) {
    if self.content_called_for_len(len) {
      self.col = self.tmp_col;
      self.line = self.tmp_line;
    } else {
      // HINT: any token containing line wrapping has varying content, no content() call no new line
      self.col += len;
    }
    self.commit += 1;
    self.head += len;
    self.cursor = self.head;
  }

  pub fn reset (&mut self) {
    self.cursor = self.head;
    if self.ended && self.buffer.len() > self.cursor {
      self.ended = false;
    }
  }

  pub fn content (&mut self, len: usize) -> &str {
    if !self.content_called_for_len(len) {
      self.tmp_commit = self.commit;
      self.tmp_len = len;
      self.tmp_content.clear();
      self.tmp_col = self.col;
      self.tmp_line = self.line;
      for i in self.head..(self.head + len) {
        let c = self.buffer[i];
        match c {
          '\n' => {
            self.tmp_line += 1;
            self.tmp_col = 0;
          },
          '\r' => (),
          _ => {
            self.tmp_col += 1;
          },
        }
        self.tmp_content.push(c);
      };
    }
    return &self.tmp_content;
  }

  fn load_input (&mut self) {
    match self.input.pop() {
      Some (s) => {
        if s.len() == 0 {
          self.load_input();
        } else {
          self.buffer.drain(0..self.head);
          self.cursor -= self.head;
          self.head = 0;
          let mut chars = s.chars();
          loop {
            match chars.next() {
              Some (c) => self.buffer.push(c),
              None => break,
            };
          };
        }
      },
      None => {
        self.ended = true;
      },
    };
  }

  pub fn next (&mut self) -> char {
    if self.ended {
      return '\0';
    }
    if self.cursor >= self.buffer.len() {
      self.load_input();
      if self.ended {
        return '\0';
      }
    }
    let ret = self.buffer[self.cursor];
    self.cursor += 1;
    return ret;
  }

  pub fn ended (&mut self) -> bool {
    if self.ended {
      return true;
    } else if self.cursor < self.buffer.len() {
      return false;
    } else {
      self.load_input();
      return self.ended;
    }
  }
}
