use util::MaybeOption;
use syntax::tokenize::token_type;

pub fn all (source: &str, ended: bool) -> MaybeOption<( u16, u32, usize, Option<String> )> {
  match source.chars().next() {
    Some(c) => {
      match c {
        ':' => MaybeOption::Some(( token_type::Colon, 0, 1, Option::None )),
        ';' => MaybeOption::Some(( token_type::Semicolon, 0, 1, Option::None )),
        ',' => MaybeOption::Some(( token_type::Comma, 0, 1, Option::None )),
        '(' => MaybeOption::Some(( token_type::Parenthesis, 0, 1, Option::None )),
        ')' => MaybeOption::Some(( token_type::Parenthesis, 1, 1, Option::None )),
        '[' => MaybeOption::Some(( token_type::Bracket, 0, 1, Option::None )),
        ']' => MaybeOption::Some(( token_type::Bracket, 1, 1, Option::None )),
        '{' => MaybeOption::Some(( token_type::Brace, 0, 1, Option::None )),
        '}' => MaybeOption::Some(( token_type::Brace, 1, 1, Option::None )),
        _ => MaybeOption::None,
      }
    },
    None => MaybeOption::None,
  }
}