use syntax::tokenize::token_type;

pub fn all (source: &str) -> Option<( u16, u32, usize )> {
  let mut chars = source.chars();
  let mut len = 0;
  match chars.next() {
    Some(c) => {
      match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' |
        'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
        '$' | '_' => {
          let mut nomore = false;
          len = 1;
          while !nomore {
            match chars.next() {
              Some(c) => {
                match c {
                  'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' |
                  'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
                  '$' | '_' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    len += 1;
                  },
                  _ => {
                    nomore = true;
                  },
                }
              },
              None => {
                nomore = true;
              },
            }
          }
        },
        _ => {},
      }
    },
    None => {},
  };
  if len > 0 {
    let content = &source[0..len] as &str;
    let (token_type, flag) = match content {
      "true" => (token_type::BooleanLiteral, 1),
      "false" => (token_type::BooleanLiteral, 0),
      "function" => (token_type::Function, 0),
      "return" => (token_type::Return, 0),
      "async" => (token_type::Async, 0),
      "await" => (token_type::Await, 0),
      "throw" => (token_type::Throw, 0),
      "yield" => (token_type::Yield, 0),
      "class" => (token_type::Class, 0),
      "extends" => (token_type::Extends, 0),
      "if" => (token_type::If, 0),
      "else" => (token_type::Else, 0),
      "while" => (token_type::While, 0),
      "for" => (token_type::For, 0),
      "break" => (token_type::Break, 0),
      "continue" => (token_type::Continue, 0),
      "in" => (token_type::In, 0),
      "of" => (token_type::Of, 0),
      "try" => (token_type::Try, 0),
      "catch" => (token_type::Catch, 0),
      "var" => (token_type::Var, 0),
      "let" => (token_type::Let, 0),
      "const" => (token_type::Const, 0),
      "null" => (token_type::Null, 0),
      _ => (token_type::Identifier, 0),
    };
    Option::Some(( token_type, flag, len ))
  } else {
    return Option::None
  }
}
