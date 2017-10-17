use util::Queue;
use syntax::tokenize::*;
use self::input_reader::InputReader;

pub struct Tokenizer<'a> {
  parse_pos: usize,
  updated: bool,
  output: &'a Queue<Token>,
  reader: InputReader<'a>,
}

impl<'a> Tokenizer<'a> {
  pub fn new (input: &'a Queue<String>, output: &'a Queue<Token>) -> Self {
    let inst = Tokenizer {
      parse_pos: 0,
      updated: false,
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

      let end = self.parse_pos + if token_type == token_type::UNEXPECTED {
        content = Option::Some(String::from(self.reader.content(5)));
        1
      } else { len };
      self.output.push(Token {
        token_type: token_type,
        flag: flag,
        start: self.parse_pos,
        end: end,
        content: content,
      });
      if token_type == token_type::UNEXPECTED {
        break;
      } else {
        if token_type != token_type::WHITE_SPACE {
          if (token_type == token_type::IDENTIFIER || (token_type & token_type::KEYWORD) > 0) && match prev_type { token_type::QUESTION_DOT | token_type::DOT => true, _ => false } {
            self.reader.state.expr_allowed = true;
          } else {
            self.reader.state.expr_allowed = (token_type & token_type::BEFORE_EXPR) > 0;
          }
          prev_type = token_type;
        }
        self.parse_pos += len;
      }
    }
    self.output.end();
  }
}
