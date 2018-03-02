use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn try (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
  let mut len = 0;
  loop {
    let c = context.next();
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
        let expr_len = match_unicode_expr(context);
        if expr_len == 0 {
          break;
        }
        len += 1 + expr_len;
      },
      _ => {
        if c > '\x7f' {
          len += 1;
        } else {
          break;
        }
      },
    };
  }
  if len > 0 {
    let token_type = match context.get_source_from_cursor(len) {
      "true" | "false"      => tt::BOOLEAN_LITERAL,
      "function"  => tt::FUNCTION,
      "return"    => tt::RETURN,
      "async"     => tt::ASYNC,
      "await"     => tt::AWAIT,
      "throw"     => tt::THROW,
      "yield"     => tt::YIELD,
      "class"     => tt::CLASS,
      "extends"   => tt::EXTENDS,
      "static"    => tt::STATIC,
      "if"        => tt::IF,
      "else"      => tt::ELSE,
      "switch"    => tt::SWITCH,
      "case"      => tt::CASE,
      "while"     => tt::WHILE,
      "for"       => tt::FOR,
      "break"     => tt::BREAK,
      "continue"  => tt::CONTINUE,
      "do"        => tt::DO,
      "with"      => tt::WITH,
      "var"       => tt::VAR,
      "let"       => tt::LET,
      "void"      => tt::VOID,
      "delete"    => tt::DELETE,
      "const"     => tt::CONST,
      "typeof"    => tt::TYPEOF,
      "new"       => tt::NEW,
      
      "import"    => tt::IMPORT,
      "from"      => tt::FROM,
      "export"    => tt::EXPORT,
      "default"   => tt::DEFAULT,
      "try"       => tt::TRY,
      "catch"     => tt::CATCH,

      "in"          => tt::IN,
      "of"          => tt::OF,
      "instanceof"  => tt::INSTANCEOF,

      _           => tt::IDENTIFIER,
    };
    return Option::Some(( token_type, len ));
  } else {
    return Option::None;
  }
}

fn match_unicode_expr (context: &mut Tokenizer) -> usize {
  match context.next() {
    'u' => {
      match context.next() {
        '{' => {
          let mut len = 2;
          loop {
            match context.next() {
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
            match context.next() {
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

