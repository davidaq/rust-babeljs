use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

// Must placed after comment, interpretor detect following identifier as regex flags
pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    '/' => {
      let mut len = 1;
      let mut escaped = false;
      loop {
        if escaped {
          let c = reader.next();
          match c {
            '\0' | '\n' | '\r' => {
              return Option::None;
            },
            _ => {
              escaped = false;
              len += c.len_utf8();
            }
          }
        } else {
          let c = reader.next();
          len += c.len_utf8();
          match c {
            '\\' => {
              escaped = true;
            },
            '/' => {
              break;
            },
            '\0' | '\n' | '\r' => {
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
