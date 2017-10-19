use syntax::tokenize::token_type;
use syntax::tokenize::context::Context;

pub fn all (context: &mut Context) -> Option<( u16, u32, usize )> {
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
    return Option::Some((token_type::WHITE_SPACE, 0, len));
  }
  return Option::None;
}
