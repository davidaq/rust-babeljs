use syntax::tokenize::token_type;

pub fn all (source: &str) -> Option<( u16, u32, usize )> {
  let mut chars = source.chars();
  let mut len = 0;
  loop {
    match chars.next() {
      Some (c) => {
        match c {
          ' ' | '\n' | '\r' | '\t' => {
            len += 1;
          },
          _ => {
            break;
          },
        }
      },
      None => {
        break;
      },
    };
  }
  if len > 0 {
    return Option::Some((token_type::WhiteSpace, 0, len));
  }
  return Option::None;
}
