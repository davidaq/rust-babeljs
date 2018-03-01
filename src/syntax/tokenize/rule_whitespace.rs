use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn try (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
  let mut len = 0;
  loop {
    match context.next() {
      ' ' | '\n' | '\r' | '\t' => {
        len += 1;
      },
      _ => {
        break;
      },
    };
  }
  if len > 0 {
    return Option::Some((tt::WHITE_SPACE, len));
  }
  return Option::None;
}
