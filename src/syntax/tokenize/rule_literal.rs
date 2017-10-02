use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn string (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let start_char = reader.next();
  match start_char {
    '\'' | '"' => {
      let mut len = 1;
      let mut escaped = false;
      loop {
        let c = reader.next();
        if c == '\0' {
          break;
        }
        len += 1;
        if escaped {
          escaped = false;
        } else if c == '\\' {
          escaped = true;
        } else if c == start_char {
          return Option::Some(( token_type::STRING_LITERAL, 0, len ));
        }
      }
      return Option::None;
    },
    _ => Option::None,
  }
}

pub fn number (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let len = numberWithLen(reader);
  if len > 0 {
    Option::Some(( token_type::NUMERIC_LITERAL, 0, len ))
  } else {
    Option::None
  }
}

fn numberWithLen (reader: &mut InputReader) -> usize {
  match reader.next() {
    '0' => {
      let (acceptZero, len) = match reader.next() {
        'b' => (false, binNumberWithLen(reader)),
        'x' => (false, hexNumberWithLen(reader)),
        '.' => (true, deciNumberWithLen(reader, false, true)),
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' =>
          (true, deciNumberWithLen(reader, true, true)),
        'e' | 'E' => (false, deciNumberWithLen(reader, false, false)),
        _ => (false, 0),
      };
      if !acceptZero && len == 0 {
        return 1;
      } else {
        return len + 2;
      }
    },
    '.' => {
      1 + deciNumberWithLen(reader, false, true)
    },
    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
      1 + deciNumberWithLen(reader, true, true)
    },
    _ => 0,
  }
}

fn binNumberWithLen (reader: &mut InputReader) -> usize {
  let mut len = 0;
  loop {
    match reader.next() {
      '0' | '1' => {
        len += 1;
      },
      _ => {
        break;
      },
    }
  }
  return len;
}

fn hexNumberWithLen (reader: &mut InputReader) -> usize {
  let mut len = 0;
  loop {
    match reader.next() {
      'a' | 'A' | 'b' | 'B' | 'c' | 'C' | 'd' | 'D' | 'e' | 'E' | 'f' | 'F' |
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
        len += 1;
      },
      _ => {
        break;
      },
    }
  }
  return len;
}

fn deciNumberWithLen (reader: &mut InputReader, acceptDot: bool, acceptExp: bool) -> usize {
  let mut len = 0;
  loop {
    match reader.next() {
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
        len += 1;
      },
      '.' => {
        if acceptDot {
          len += 1 + deciNumberWithLen(reader, false, true);
        }
        break;
      },
      'e' | 'E' => {
        if acceptExp {
          match reader.next() {
            '-' => {
              let sublen = deciNumberWithLen(reader, false, false);
              if sublen > 0 {
                len += 2 + sublen;
              }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
              len += 2 + deciNumberWithLen(reader, false, false);
            },
            _ => (),
          }
        }
        break;
      },
      _ => {
        break;
      },
    }
  }
  return len;
}
