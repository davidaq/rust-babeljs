use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let mut len = 0;
  loop {
    let c = reader.next();
    match c {
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
        if len == 0 {
          return Option::None;
        }
        len += 1;
      },
      'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' |
      'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
      'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' |
      'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' |
      '$' | '_' => {
        len += 1;
      },
      '\\' => {
        let expr_len = match_unicode_expr(reader);
        if expr_len == 0 {
          break;
        }
        len += 1 + expr_len;
      },
      _ => {
        if c > '\x7f' {
          len += c.len_utf8();
        } else {
          break;
        }
      },
    };
  }
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
      "do"        => (token_type::KEYWORD, token_type::keyword::DO),
      "var"       => (token_type::KEYWORD, token_type::keyword::VAR),
      "let"       => (token_type::KEYWORD, token_type::keyword::LET),
      "const"     => (token_type::KEYWORD, token_type::keyword::CONST),
      "import"    => (token_type::KEYWORD, token_type::keyword::IMPORT),
      "from"      => (token_type::KEYWORD, token_type::keyword::FROM),
      "export"    => (token_type::KEYWORD, token_type::keyword::EXPORT),
      "default"   => (token_type::KEYWORD, token_type::keyword::DEFAULT),
      "try"       => (token_type::KEYWORD, token_type::keyword::TRY),
      "catch"     => (token_type::KEYWORD, token_type::keyword::CATCH),

      "in"          => (token_type::OPERATOR, token_type::operator::IN),
      "of"          => (token_type::OPERATOR, token_type::operator::OF),
      "instanceof"  => (token_type::OPERATOR, token_type::operator::INSTANCEOF),

      _           => (token_type::IDENTIFIER, 0),
    };
    return Option::Some(( token_type, flag, len ))
  } else {
    return Option::None
  }
}

fn match_unicode_expr (reader: &mut InputReader) -> usize {
  match reader.next() {
    'u' => {
      match reader.next() {
        '{' => {
          let mut len = 2;
          loop {
            match reader.next() {
              'a' | 'A' | 'b' | 'B' | 'c' | 'C' | 'd' | 'D' | 'e' | 'E' | 'f' | 'F' |
              '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                len += 1;
              },
              '}' => {
                len += 1;
                break;
              },
              _ => {
                return 0;
              },
            }
          }
          return len;
        },
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          for _ in 0..3 {
            match reader.next() {
              '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
              _ => return 0,
            }
          }
          return 5;
        },
        _ => return 0,
      }
    },
    _ => return 0,
  }
}
