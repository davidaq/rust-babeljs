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

pub struct InputReader<'a> {
  pub state: State,
  input: &'a Queue<String>,
  buffer: String,
  buffer_len: usize,
  cursor: usize,
  ended: bool,
}

impl<'a> InputReader<'a> {
  pub fn new (input: &'a Queue<String>) -> Self {
    InputReader {
      state: State::new(),
      input: input,
      buffer: String::from(""),
      buffer_len: 0,
      cursor: 0,
      ended: false,
    }
  }

  pub fn commit (&mut self, len: usize) {
    self.buffer.drain(0..len);
    self.buffer_len = self.buffer.chars().count();
    self.cursor = 0;
  }

  pub fn reset (&mut self) {
    self.cursor = 0;
    if self.buffer_len > 0 {
      self.ended = false;
    }
  }

  pub fn content (&mut self, len: usize) -> &str {
    if len >= self.buffer_len {
      return &self.buffer;
    } else {
      return &self.buffer[..len];
    }
  }

  fn load_input (&mut self) {
    match self.input.pop() {
      Some (s) => {
        if s.len() == 0 {
          self.load_input();
        } else {
          self.buffer += &s;
          self.buffer_len = self.buffer.chars().count();
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
    if self.cursor >= self.buffer_len {
      self.load_input();
      if self.ended {
        return '\0';
      }
    }
    let ret = match self.buffer.chars().nth(self.cursor) {
      Some (c) => c,
      None => '\0',
    };
    self.cursor += 1;
    return ret;
  }

  pub fn back (&mut self, len: usize) {
    if self.cursor > 0 {
      self.cursor -= len;
    }
  }

  pub fn ended (&mut self) -> bool {
    if self.ended {
      return true;
    } else if self.cursor < self.buffer_len {
      return false;
    } else {
      self.load_input();
      return self.ended;
    }
  }
}
