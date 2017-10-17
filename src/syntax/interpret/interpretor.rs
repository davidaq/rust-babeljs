use util::Queue;
use syntax::tokenize::{ Token };
use syntax::interpret::*;

pub struct Interpretor<'a> {
  input: &'a Queue<Token>,
}

impl<'a> Interpretor<'a> {
  pub fn new (input: &'a Queue<Token>) -> Self {
    Interpretor {
      input: input,
    }
  }

  pub fn run (&mut self) {

  }
}
