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
    while !self.reader.ended() {
      let mut token_type : u16 = token_type::UNEXPECTED;
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
        self.parse_pos += len;
      }
    }
    self.output.end();
  }
}
