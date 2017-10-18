use util::Queue;
use syntax::basic_types::{ SourceLoc };
use syntax::tokenize::*;
use self::input_reader::InputReader;

pub struct Tokenizer<'a> {
  output: &'a Queue<Token>,
  reader: InputReader<'a>,
}

impl<'a> Tokenizer<'a> {
  pub fn new (input: &'a Queue<String>, output: &'a Queue<Token>) -> Self {
    let inst = Tokenizer {
      output: output,
      reader: InputReader::new(input),
    };
    return inst;
  }

  pub fn run (&'a mut self) {
    let mut prev_type = token_type::UNEXPECTED;
    while !self.reader.ended() {
      let mut token_type = token_type::UNEXPECTED;
      let mut flag : u32 = 0;
      let mut len : usize = 0;
      let mut content = Option::None;
      let start = self.reader.pos();

      macro_rules! match_token_rule {
        ( $rule:path ) => {
          if token_type == token_type::UNEXPECTED {
            match $rule (&mut self.reader) {
              None => self.reader.reset(),
              Some (result) => {
                token_type = result.0;
                flag = result.1;
                len = result.2;
                if (token_type::COPY_SOURCE & token_type) == token_type::COPY_SOURCE {
                  content = Option::Some(String::from(self.reader.content(len)));
                }
                self.reader.commit(len);
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
        content = Option::Some(String::from(self.reader.content(5)));
        self.reader.commit(1);
      }
      self.output.push(Token {
        token_type: token_type,
        flag: flag,
        loc: SourceLoc {
          start: start,
          end: self.reader.pos(),
        },
        content: content,
      });
      if token_type == token_type::UNEXPECTED {
        break;
      }
      if token_type != token_type::WHITE_SPACE {
        if (token_type == token_type::IDENTIFIER || (token_type & token_type::KEYWORD) > 0) && match prev_type { token_type::QUESTION_DOT | token_type::DOT => true, _ => false } {
          self.reader.state.expr_allowed = true;
        } else {
          self.reader.state.expr_allowed = (token_type & token_type::BEFORE_EXPR) > 0;
        }
        prev_type = token_type;
      }
    }
    self.output.end();
  }
}
