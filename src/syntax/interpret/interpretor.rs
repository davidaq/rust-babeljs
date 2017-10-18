use util::Queue;
use syntax::basic_types::SourceLoc;
use syntax::tokenize::{ Token, token_type };
use super::{ node };

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
        loc: SourceLoc::None(),
      },
      ended: false,
    }
  }

  pub fn run (&mut self) -> Option<node::Node> {
    let (body, directives) = self.parse_scope_body();
    let ret = node::program(body, directives);
    match self.input.pop() {
      Some (..) => {
        return None;
      },
      None => {
        return Some(ret);
      },
    };
  }

  fn next_token (&mut self) -> &Token {
    if self.ended {
      return &self.eof;
    }
    if self.cursor >= self.buffer.len() {
      match self.input.pop() {
        Some (x) => self.buffer.push(x),
        None => {
          self.ended = false;
          return &self.eof;
        },
      }
    }
    let ret = &self.buffer[self.cursor];
    self.cursor += 1;
    return ret;
  }

  fn stash (&mut self) {
    self.stash.push(self.cursor);
  }

  fn commit (&mut self) {
    self.stash.pop();
  }

  fn revert (&mut self) {
    match self.stash.pop() {
      Some (x) => self.cursor = x,
      None => panic!("Unmatching interpretor stash stack"),
    }
  }

  fn parse_scope_body (&mut self) -> (node::NodeList, node::NodeList) {
    let mut body: node::NodeList = vec![];
    let mut directives: node::NodeList = vec![];
    let mut allow_scope_directive = true;
    loop {
      match self.parse_statement(allow_scope_directive, true) {
        Some (( node, is_directive )) => {
          if is_directive {
            directives.push(Box::new(node));
          } else {
            allow_scope_directive = false;
            body.push(Box::new(node));
          }
        },
        None => break,
      }
    };
    return (body, directives);
  }

  fn parse_statement (&mut self, allow_scope_directive: bool, allow_module_decl: bool) -> Option<(node::Node, bool)> {
    if allow_scope_directive {
      self.stash();
      { let tok = self.next_token(); }
      self.revert();
    }
    if allow_module_decl {
      match self.parse_module_decl() {
        Some (node) => return Some(( node, false )),
        None => (),
      }
    }
    return None;
  }

  fn parse_module_decl (&mut self) -> Option<node::Node> {
    return None;
  }
}
