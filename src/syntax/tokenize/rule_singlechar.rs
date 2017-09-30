use syntax::tokenize::token_type;

pub fn all (source: &str) -> Option<( u16, u32, usize )> {
  match source.chars().next() {
    Some(c) => {
      match c {
        ':' => Option::Some(( token_type::Colon, 0, 1 )),
        ';' => Option::Some(( token_type::Semicolon, 0, 1 )),
        ',' => Option::Some(( token_type::Comma, 0, 1 )),
        '(' => Option::Some(( token_type::Parenthesis, 0, 1 )),
        ')' => Option::Some(( token_type::Parenthesis, 1, 1 )),
        '[' => Option::Some(( token_type::Bracket, 0, 1 )),
        ']' => Option::Some(( token_type::Bracket, 1, 1 )),
        '{' => Option::Some(( token_type::Brace, 0, 1 )),
        '}' => Option::Some(( token_type::Brace, 1, 1 )),
        _ => Option::None,
      }
    },
    None => Option::None,
  }
}