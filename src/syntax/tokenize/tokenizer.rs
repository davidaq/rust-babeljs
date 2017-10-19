use util::Queue;
use syntax::basic_types::{ SourceLoc };
use syntax::tokenize::*;
use self::context::Context;

pub struct Tokenizer<'a> {
  output: &'a Queue<Token>,
  context: Context<'a>,
}

impl<'a> Tokenizer<'a> {
  pub fn new (input: &'a Queue<String>, output: &'a Queue<Token>) -> Self {
    let inst = Tokenizer {
      output: output,
      context: Context::new(input),
    };
    return inst;
  }

  pub fn run (&'a mut self) {
    let mut prev_type = token_type::UNEXPECTED;
    while !self.context.ended() {
      let mut token_type = token_type::UNEXPECTED;
      let mut flag : u32 = 0;
      let mut content = Option::None;
      let start = self.context.pos();

      macro_rules! match_token_rule {
        ( $rule:path ) => {
          if token_type == token_type::UNEXPECTED {
            match $rule (&mut self.context) {
              None => self.context.reset(),
              Some (result) => {
                token_type = result.0;
                flag = result.1;
                let len = result.2;
                if (token_type::COPY_SOURCE & token_type) == token_type::COPY_SOURCE {
                  content = Option::Some(String::from(self.context.content(len)));
                }
                self.context.commit(len);
              },
            }
          };
        }
      }
      match_token_rule!(rule_whitespace::all);
      match_token_rule!(rule_comment::all);
      match_token_rule!(rule_literal::string);
      match_token_rule!(rule_literal::number);
      match_token_rule!(rule_tplstr::all);
      match_token_rule!(rule_regex::all);
      match_token_rule!(rule_identifier::all);
      match_token_rule!(rule_operator::all);

      if token_type == token_type::UNEXPECTED {
        content = Option::Some(String::from(self.context.content(5)));
        self.context.commit(1);
      }
      self.output.push(Token {
        token_type: token_type,
        flag: flag,
        loc: SourceLoc {
          start: start,
          end: self.context.pos(),
        },
        content: content,
      });
      if token_type == token_type::UNEXPECTED {
        break;
      }
      if token_type != token_type::WHITE_SPACE {
        if (token_type == token_type::IDENTIFIER || (token_type & token_type::KEYWORD) > 0) && match prev_type { token_type::QUESTION_DOT | token_type::DOT => true, _ => false } {
          self.context.state.expr_allowed = true;
        } else {
          self.context.state.expr_allowed = (token_type & token_type::BEFORE_EXPR) > 0;
        }
        prev_type = token_type;
      }
    }
    self.output.end();
  }
}
