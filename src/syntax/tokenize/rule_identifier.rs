use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let mut len = 0;
  match reader.next() {
    'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' |
    'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
    '$' | '_' => {
      len = 1;
      loop {
        match reader.next() {
          'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' |
          'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
          '$' | '_' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            len += 1;
          },
          _ => {
            break;
          },
        };
      }
    },
    _ => {},
  };
  if len > 0 {
    let (token_type, flag) = match reader.content(len) {
      "true"      => (token_type::BOOLEAN_LITERAL, token_type::boolean::TRUE),
      "false"     => (token_type::BOOLEAN_LITERAL, token_type::boolean::FALSE),

      "function"  => (token_type::KEYWORD, token_type::keyword::FUNCTION),
      "return"    => (token_type::KEYWORD, token_type::keyword::RETURN),
      "async"     => (token_type::KEYWORD, token_type::keyword::ASYNC),
      "await"     => (token_type::KEYWORD, token_type::keyword::AWAIT),
      "throw"     => (token_type::KEYWORD, token_type::keyword::THROW),
      "yield"     => (token_type::KEYWORD, token_type::keyword::YIELD),
      "class"     => (token_type::KEYWORD, token_type::keyword::CLASS),
      "extends"   => (token_type::KEYWORD, token_type::keyword::EXTENDS),
      "static"    => (token_type::KEYWORD, token_type::keyword::STATIC),
      "if"        => (token_type::KEYWORD, token_type::keyword::IF),
      "else"      => (token_type::KEYWORD, token_type::keyword::ELSE),
      "while"     => (token_type::KEYWORD, token_type::keyword::WHILE),
      "for"       => (token_type::KEYWORD, token_type::keyword::FOR),
      "break"     => (token_type::KEYWORD, token_type::keyword::BREAK),
      "continue"  => (token_type::KEYWORD, token_type::keyword::CONTINUE),
      "continue"  => (token_type::KEYWORD, token_type::keyword::DO),
      "var"       => (token_type::KEYWORD, token_type::keyword::VAR),
      "let"       => (token_type::KEYWORD, token_type::keyword::LET),
      "const"     => (token_type::KEYWORD, token_type::keyword::CONST),
      "in"        => (token_type::KEYWORD, token_type::keyword::IN),
      "of"        => (token_type::KEYWORD, token_type::keyword::OF),
      "try"       => (token_type::KEYWORD, token_type::keyword::TRY),
      "catch"     => (token_type::KEYWORD, token_type::keyword::CATCH),

      _           => (token_type::IDENTIFIER, 0),
    };
    return Option::Some(( token_type, flag, len ))
  } else {
    return Option::None
  }
}
