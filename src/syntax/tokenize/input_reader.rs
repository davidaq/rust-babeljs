use util::Queue;
use std::str::Chars;

pub struct InputReader<'a> {
  input: &'a Queue<String>,
  buffer: String,
  buffer_len: usize,
  cursor: usize,
  ended: bool,
}

impl<'a> InputReader<'a> {
  pub fn new (input: &'a Queue<String>) -> Self {
    InputReader {
      input: input,
      buffer: String::from(""),
      buffer_len: 0,
      cursor: 0,
      ended: false,
    }
  }

  pub fn commit (&mut self, len: usize) {
    let byte_len = self.byte_len(len);
    self.buffer.drain(0..byte_len);
    self.buffer_len = self.buffer.chars().count();
    self.reset();
  }

  pub fn reset (&mut self) {
    self.cursor = 0;
  }

  pub fn content (&mut self, len: usize) -> &str {
    let byte_len = self.byte_len(len);
    return &self.buffer[..byte_len];
  }

  fn byte_len (&mut self, len: usize) -> usize {
    let mut byte_len = 0;
    let mut chars = self.buffer.chars();
    for x in 0..len {
      match chars.next() {
        Some (c) => byte_len += c.len_utf8(),
        None => break,
      }
    }
    return byte_len;
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

  pub fn back (&mut self) {
    if self.cursor > 0 {
      self.cursor -= 1;
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
