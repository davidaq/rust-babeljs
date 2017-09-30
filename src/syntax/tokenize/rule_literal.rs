use util::MaybeOption;
use syntax::tokenize::token_type;

pub fn string (source: &str, ended: bool) -> MaybeOption<( u16, u32, usize, Option<String> )> {
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
          if len == 0 {
            MaybeOption::None
          } else if maybe {
            MaybeOption::Maybe
          } else {
            MaybeOption::Some(( token_type::StringLiteral, 0, len, Option::Some(String::from(&source[0..len])) ))
          }
        },
        _ => MaybeOption::None,
      }
    },
    None => MaybeOption::None,
  }
}

pub fn number (source: &str, ended: bool) -> MaybeOption<( u16, u32, usize, Option<String> )> {
  let mut chars = source.chars();
  match chars.next() {
    Some (c) {

    },
    None => MaybeOption::None,
  }
}