use util::Queue;
use syntax::tokenize::*;

pub struct Tokenizer<'a> {
  parse_pos: usize,
  ended: bool,
  error: bool,
  updated: bool,
  input: &'a Queue<String>,
  output: &'a Queue<Token>,
  source: String,
}

impl<'a> Tokenizer<'a> {
  pub fn new (input: &'a Queue<String>, output: &'a Queue<Token>) -> Self {
    let inst = Tokenizer {
      parse_pos: 0,
      ended: false,
      error: false,
      updated: false,
      input: input,
      output: output,
      source: String::from(""),
    };
    return inst;
  }

  pub fn run (&mut self) {
    while !self.ended {
      if !self.input.hasMore() && self.updated {
        self.updated = false;
        self.parse();
      }
      match self.input.pop() {
        Some (chunk) => {
          self.updated = true;
          self.source += &chunk;
        },
        None => {
          self.ended = true;
          break;
        },
      }
    }
    if !self.error {
      self.parse();
    }
    self.output.end();
  }

  fn parse (&mut self) {
    while self.source.len() > self.parse_pos {
      let mut token_type : u16 = token_type::Unexpected;
      let mut flag : u32 = 0;
      let mut len : usize = 0;
      let mut maybe = false;

      macro_rules! match_token_rule {
        ( $rule:path ) => {
          match $rule (&self.source[self.parse_pos..]) {
            None => {},
            Some (result) => {
              token_type = result.0;
              flag = result.1;
              len = result.2;
              if len + self.parse_pos >= self.source.len() {
                maybe = true;
              }
            },
          }
          if maybe {
            if !self.ended {
              break;
            }
          }
        };
      }
      match_token_rule!(rule_whitespace::all);
      match_token_rule!(rule_literal::string);
      match_token_rule!(rule_identifier::all);
      match_token_rule!(rule_singlechar::all);

      if token_type == token_type::Unexpected {
        self.output.push(Token {
          token_type: token_type::Unexpected,
          flag: 0,
          start: self.parse_pos,
          end: self.parse_pos + 1,
          content: Option::None,
        });
        self.ended = true;
        self.error = true;
        self.input.interrupt();
        break;
      }
      let end = self.parse_pos + len;
      self.output.push(Token {
        token_type: token_type,
        flag: flag,
        start: self.parse_pos,
        end: end,
        content: if token_type::CopySource & token_type > 0 {
          Option::Some(String::from(&self.source[self.parse_pos..end]))
        } else {
          Option::None
        },
      });
      self.parse_pos += len;
    }
  }

  pub fn token_content<'b> (&'a self, token: &'b Token) -> &'a str {
    return &self.source[token.start..token.end];
  }
}
