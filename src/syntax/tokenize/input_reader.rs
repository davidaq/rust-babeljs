use util::Queue;
use std::str::Chars;

pub struct InputReader<'a> {
  input: &'a Queue<String>,
  buffer: String,
  cursor: usize,
  ended: bool,
}

impl<'a> InputReader<'a> {
  pub fn new (input: &'a Queue<String>) -> Self {
    InputReader {
      input: input,
      buffer: String::from(""),
      cursor: 0,
      ended: false,
    }
  }

  pub fn commit (&mut self, len: usize) {
    self.buffer.drain(0..len);
    self.reset();
  }

  pub fn reset (&mut self) {
    self.cursor = 0;
  }

  pub fn content (&mut self, len: usize) -> &str {
    return &self.buffer[0..len];
  }

  fn load_input (&mut self) {
    match self.input.pop() {
      Some (s) => {
        if s.len() == 0 {
          self.load_input();
        } else {
          self.buffer += &s;
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
    let ret = match self.buffer.chars().nth(self.cursor) {
      Some (c) => c,
      None => '\0',
    };
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
