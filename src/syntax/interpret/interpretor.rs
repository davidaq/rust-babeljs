use util::Queue;
use syntax::basic_types::SourceLoc;
use syntax::tokenize::{ Token, token_type };
use super::*;

pub struct Checkpoint {
  pos: usize,
  dealt: bool,
  ctx_cursor: *mut usize,
}

impl Checkpoint {
  fn new(ctx: &mut Interpretor) -> Self {
    Checkpoint {
      pos: ctx.cursor,
      dealt: false,
      ctx_cursor: &mut ctx.cursor as *mut usize,
    }
  }

  pub fn commit (&mut self) {
    if self.dealt {
      panic!("Double commitment");
    }
    self.dealt = true;
  }
}

impl Drop for Checkpoint {
  fn drop (&mut self) {
    if !self.dealt {
      unsafe { *self.ctx_cursor = self.pos };
    }
  }
}

pub struct Interpretor<'a> {
  input: &'a Queue<Token>,
  stash: Vec<usize>,
  buffer: Vec<Token>,
  cursor: usize,
  eof: Token,
  ended: bool,
}

impl<'a> Interpretor<'a> {
  pub fn new (input: &'a Queue<Token>) -> Self {
    Interpretor {
      input: input,
      stash: vec![],
      buffer: vec![],
      cursor: 0,
      eof: Token {
        token_type: token_type::EOF,
        flag: 0,
        content: None,
        loc: SourceLoc::new(),
      },
      ended: false,
    }
  }

  pub fn run (&mut self) -> node::NodeBox {
    self.load_token();
    return parse_scope::parse_program(self);
  }

  fn load_token (&mut self) {
    match self.input.pop() {
      Some (x) => self.buffer.push(x),
      None => self.ended = true,
    }
  }

  pub fn next_token (&mut self) {
    self.cursor += 1;
    if self.cursor >= self.buffer.len() {
      self.load_token();
    }
  }

  pub fn cur_token (&mut self) -> &Token {
    if self.ended {
      return &self.eof;
    } else {
      return &self.buffer[self.cursor];
    }
  }

  pub fn checkpoint (&mut self) -> Checkpoint {
    return Checkpoint::new(self);
  }
}
