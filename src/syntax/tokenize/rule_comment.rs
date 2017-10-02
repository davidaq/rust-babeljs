use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    '/' => {
      match reader.next() {
        '/' => {
          let mut len = 2;
          loop {
            match reader.next() {
              '\0' | '\r' | '\n' => break,
              _ => len += 1,
            }
          }
          Option::Some((token_type::COMMENT, token_type::comment::LINE, len))
        },
        '*' => {
          let mut len = 2;
          loop {
            match reader.next() {
              '\0' => return Option::None,
              '*' => {
                len += 1;
                match reader.next() {
                  '\0' => return Option::None,
                  '/' => {
                    len += 1;
                    break;
                  },
                  _ => len += 1,
                }
              },
              _ => len += 1,
            }
          }
          Option::Some((token_type::COMMENT, token_type::comment::BLOCK, len))
        },
        _ => Option::None,
      }
    },
    _ => Option::None,
  }
}
