use syntax::tokenize::token_type;

pub fn string (source: &str) -> Option<( u16, u32, usize )> {
  let mut chars = source.chars();
  match chars.next() {
    Some(start_char) => {
      match start_char {
        '\'' | '"' => {
          let mut len = 1;
          let mut maybe = true;
          let mut escaped = false;
          loop {
            if escaped {
              escaped = false;
              if Option::None == chars.next() {
                break;
              }
              len += 1;
            } else {
              match chars.next() {
                Some(c) => {
                  len += 1;
                  if c == '\\' {
                    escaped = true;
                  } else if c == start_char {
                    maybe = false;
                    break;
                  }
                },
                None => {
                  break;
                },
              }
            }
          }
          if len > 0 {
            Option::Some(( token_type::StringLiteral, 0, len ))
          } else {
            Option::None
          }
        },
        _ => Option::None,
      }
    },
    None => Option::None,
  }
}

pub fn number (source: &str) -> Option<( u16, u32, usize )> {
  let len = numberAsLen(source);
  if len > 0 {
    Option::Some(( token_type::NumericLiteral, 0, len ))
  } else {
    Option::None
  }
}

fn numberAsLen (source: &str) -> usize {
  let mut chars = source.chars();
  match chars.next() {
    Some (c) => {
      match c {
        '0' => {
          match chars.next() {
            Some (c) => {
              1
            },
            None => 1,
          }
        },
        '.' => {
          1
        },
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          1
        },
        _ => 0,
      }
    },
    None => 0,
  }
}
