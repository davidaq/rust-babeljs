use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let mut len = 0;
  loop {
    match reader.next() {
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
