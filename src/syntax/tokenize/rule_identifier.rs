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

      "function"  => (token_type::FUNCTION, 0),
      "return"    => (token_type::RETURN, 0),
      "async"     => (token_type::ASYNC, 0),
      "await"     => (token_type::AWAIT, 0),
      "throw"     => (token_type::THROW, 0),
      "yield"     => (token_type::YIELD, 0),
      "class"     => (token_type::CLASS, 0),
      "extends"   => (token_type::EXTENDS, 0),
      "static"    => (token_type::STATIC, 0),
      "if"        => (token_type::IF, 0),
      "else"      => (token_type::ELSE, 0),
      "switch"    => (token_type::SWITCH, 0),
      "case"      => (token_type::CASE, 0),
      "while"     => (token_type::WHILE, 0),
      "for"       => (token_type::FOR, 0),
      "break"     => (token_type::BREAK, 0),
      "continue"  => (token_type::CONTINUE, 0),
      "do"        => (token_type::DO, 0),
      "with"      => (token_type::WITH, 0),
      "var"       => (token_type::VAR, 0),
      "let"       => (token_type::LET, 0),
      "const"     => (token_type::CONST, 0),
      "typeof"    => (token_type::TYPEOF, 0),
      "new"       => (token_type::NEW, 0),
      
      "import"    => (token_type::IMPORT, 0),
      "from"      => (token_type::FROM, 0),
      "export"    => (token_type::EXPORT, 0),
      "default"   => (token_type::DEFAULT, 0),
      "try"       => (token_type::TRY, 0),
      "catch"     => (token_type::CATCH, 0),

      "in"          => (token_type::IN, 0),
      "of"          => (token_type::OF, 0),
      "instanceof"  => (token_type::INSTANCEOF, 0),

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
