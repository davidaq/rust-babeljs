use syntax::tokenize::token_type;
use syntax::tokenize::context::Context;

pub fn all (context: &mut Context) -> Option<( u16, u32, usize )> {
  match context.next() {
    '/' => {
      match context.next() {
        '/' => {
          let mut len = 2;
          loop {
            match context.next() {
              '\0' | '\r' | '\n' => break,
              _ => len += 1,
            }
          }
          Option::Some((token_type::COMMENT, token_type::comment::LINE, len))
        },
        '*' => {
          let mut len = 2;
          loop {
            match context.next() {
              '\0' => return Option::None,
              '*' => {
                len += 1;
                match context.next() {
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
    '#' => if context.allow_hashbang() {
      if context.next() == '!' {
        let mut len = 2;
        loop {
          match context.next() {
            '\0' | '\r' | '\n' => break,
            _ => len += 1,
          }
        }
        Option::Some((token_type::COMMENT, token_type::comment::HASHBANG, len))
      } else {
        Option::None
      }
    } else {
      Option::None
    },
    _ => Option::None,
  }
}
