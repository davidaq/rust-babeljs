use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn try (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
  let mut len = 0;
  loop {
    let c = context.next();
    match c {
      ' ' | '\n' | '\r' | '\t' => {
        len += 1;
      },
      _ => {
        println!("break {}", c);
        break;
      },
    };
  }
  if len > 0 {
    return Option::Some((tt::WHITE_SPACE, len));
  }
  return Option::None;
}
