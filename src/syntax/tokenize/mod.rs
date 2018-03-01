mod token;
pub use self::token::*;

mod context;


pub mod tt {
  use syntax::context::Context;

  pub type Flag = usize;
  pub type TokenType = usize;
  pub struct Token {
    pub token_type : TokenType,
    pub context : *const Context,
    pub start : usize,
    pub end : usize,
  }

  impl Token {
    pub fn content (&self) -> &str {
      unsafe {
        self.context.as_ref().unwrap().source.get(self.start..self.end).unwrap()
      }
    }
  }

  include!(concat!(env!("OUT_DIR"), "/token_type.rs"));

  pub fn stringify (ty: TokenType) -> Option<&'static str> {
    TOK_STRING[ty]
  }

  pub fn type_is (ty: TokenType, flag: Flag) -> bool {
    TOK_FLAG[ty] & flag != 0
  }
}

mod tokenizer;
pub use self::tokenizer::*;

mod rule_whitespace;
mod rule_comment;
mod rule_identifier;
mod rule_operator;
mod rule_literal;
mod rule_tplstr;
mod rule_regex;

