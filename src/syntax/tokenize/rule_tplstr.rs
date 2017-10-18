use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    '`' => read_tpl(reader, false),
    '}' => {
      let is_tplstr = match reader.state.brace_stack.last() {
        Some(is_tplstr) => *is_tplstr,
        None => false,
      };
      if is_tplstr {
        reader.state.brace_stack.pop();
        read_tpl(reader, true)
      } else {
        Option::None
      }
    },
    _ => Option::None,
  }
}

fn read_tpl (reader: &mut InputReader, left_open: bool) -> Option<( u16, u32, usize )> {
  let mut escaped = false;
  let mut len: usize = 1;
  let mut dollar = false;
  loop {
    let c = reader.next();
    if c == '\0' {
      return Option::None;
    }
    len += 1;
    if escaped {
      escaped = false;
    } else {
      match c {
        '`' => return Option::Some(( (if left_open { token_type::TPL_STR_R } else { token_type::TPL_STR_LITERAL }), 0, len )),
        '$' => dollar = true,
        '\\' => {
          escaped = true;
          dollar = false;
        },
        '{' => {
          if dollar {
            reader.state.brace_stack.push(true);
            return Option::Some(( (if left_open { token_type::TPL_STR_RL } else { token_type::TPL_STR_L }), 0, len ));
          }
          dollar = false;
        },
        _ => dollar = false,
      }
    }
  }
}
