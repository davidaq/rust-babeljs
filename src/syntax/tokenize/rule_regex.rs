use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;
use super::rule_identifier;

// Must placed before comment, intepretor detect following identifier as regex flags
pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    '/' => {
      let mut len = 1;
      let mut escaped = false;
      loop {
        if escaped {
          match reader.next() {
            '\0' | '\n' => {
              return Option::None;
            },
            _ => {
              escaped = false;
              len += 1;
            }
          }
        } else {
          len += 1;
          match reader.next() {
            '\\' => {
              escaped = true;
            },
            '/' => {
              break;
            },
            '\0' | '\n' => {
              return Option::None;
            },
            _ => (),
          }
        }
      }
      Option::Some(( token_type::REGEX_LITERAL, 0, len ))
    },
    _ => Option::None,
  }
}
