mod token;
pub use self::token::*;

mod context;

pub mod tt {
  pub struct TokenType {
    pub id: u32,
    pub flag: u32,
  }
 
  include!(concat!(env!("OUT_DIR"), "/token_type.rs"));

  impl PartialEq for TokenType {
    fn eq (&self, other: &Self) -> bool {
      self.id == other.id
    }
  }
  
  impl TokenType {
    pub fn stringify (&self) -> Option<&'static str> {
      stringify(self.id)
    }
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

