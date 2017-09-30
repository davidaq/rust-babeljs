use util::MaybeOption;
use syntax::tokenize::token_type;

pub fn all (source: &str, ended: bool) -> MaybeOption<( u16, u32, usize, Option<String> )> {
  let mut chars = source.chars();
  let mut len = 0;
  let mut maybe = true;
  loop {
    match chars.next() {
      Some (c) => {
        match c {
          ' ' | '\n' | '\r' | '\t' => {
            len += 1;
          },
          _ => {
            maybe = false;
            break;
          },
        }
      },
      None => {
        if ended {
          maybe = false;
        }
        break;
      },
    };
  }
  if maybe {
    return MaybeOption::Maybe;
  }
  if len > 0 {
    println!("{}", len);
    return MaybeOption::Some(( token_type::WhiteSpace, 0, len, Option::None ));
  }
  return MaybeOption::None;
}
