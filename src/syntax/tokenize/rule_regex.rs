use syntax::tokenize::token_type;
use syntax::tokenize::context::Context;

// Must placed after comment, interpretor detect following identifier as regex flags
pub fn all (context: &mut Context) -> Option<( u16, u32, usize )> {
  if !context.state.expr_allowed {
    return Option::None;
  }
  match context.next() {
    '/' => {
      let mut len = 1;
      let mut escaped = false;
      loop {
        if escaped {
          let c = context.next();
          match c {
            '\0' | '\n' | '\r' => {
              return Option::None;
            },
            _ => {
              escaped = false;
              len += 1;
            }
          }
        } else {
          let c = context.next();
          len += 1;
          match c {
            '\\' => {
              escaped = true;
            },
            '/' => {
              break;
            },
            '\0' | '\n' | '\r' => {
              return Option::None;
            },
            _ => (),
          }
        }
      }
      Option::Some(( token_type::REGEX_LITERAL, 0, len ))
    },
    _ => Option::None,
  }
}
