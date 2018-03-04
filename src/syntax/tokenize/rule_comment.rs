use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn try (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
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
          Option::Some((tt::LINE_COMMENT, len))
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
          Option::Some((tt::BLOCK_COMMENT, len))
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
        Option::Some((tt::HASHBANG, len))
      } else {
        Option::None
      }
    } else {
      Option::None
    },
    _ => Option::None,
  }
}
