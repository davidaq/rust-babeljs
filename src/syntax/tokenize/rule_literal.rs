use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn string (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let start_char = reader.next();
  match start_char {
    '\'' | '"' => {
      let mut len = 1;
      let mut escaped = false;
      let mut flag = 0;
      loop {
        let c = reader.next();
        match c {
          '\0' => return Option::None,
          '\n' | '\r' => {
            flag = 1;
          },
          _ => (),
        }
        len += 1;
        if escaped {
          escaped = false;
        } else if c == '\\' {
          escaped = true;
        } else if c == start_char {
          break;
        }
      }
      return Option::Some(( token_type::STRING_LITERAL, flag, len ));
    },
    _ => Option::None,
  }
}

pub fn number (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  let len = number_len(reader);
  if len > 0 {
    Option::Some(( token_type::NUMERIC_LITERAL, 0, len ))
  } else {
    Option::None
  }
}

fn number_len (reader: &mut InputReader) -> usize {
  match reader.next() {
    '0' => {
      let (accept_zero, len) = match reader.next() {
        'b' => (false, bin_number_len(reader)),
        'x' => (false, hex_number_len(reader)),
        '.' => (true, deci_number_len(reader, false, true)),
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' =>
          (true, deci_number_len(reader, true, true)),
        'e' | 'E' => (false, deci_number_len(reader, false, false)),
        _ => (false, 0),
      };
      if !accept_zero && len == 0 {
        return 1;
      } else {
        return len + 2;
      }
    },
    '.' => {
      1 + deci_number_len(reader, false, true)
    },
    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
      1 + deci_number_len(reader, true, true)
    },
    _ => 0,
  }
}

fn bin_number_len (reader: &mut InputReader) -> usize {
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

fn hex_number_len (reader: &mut InputReader) -> usize {
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

fn deci_number_len (reader: &mut InputReader, accept_dot: bool, accept_exp: bool) -> usize {
  let mut len = 0;
  loop {
    match reader.next() {
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
        len += 1;
      },
      '.' => {
        if accept_dot {
          len += 1 + deci_number_len(reader, false, true);
        }
        break;
      },
      'e' | 'E' => {
        if accept_exp {
          match reader.next() {
            '-' => {
              let sublen = deci_number_len(reader, false, false);
              if sublen > 0 {
                len += 2 + sublen;
              }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
              len += 2 + deci_number_len(reader, false, false);
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
