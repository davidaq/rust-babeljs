use syntax::tokenize::token_type;
use syntax::tokenize::context::Context;

pub fn all (context: &mut Context) -> Option<( u16, u32, usize )> {
  match context.next() {
    '`' => read_tpl(context, false),
    '}' => {
      let is_tplstr = match context.state.brace_stack.last() {
        Some(is_tplstr) => *is_tplstr,
        None => false,
      };
      if is_tplstr {
        context.state.brace_stack.pop();
        read_tpl(context, true)
      } else {
        Option::None
      }
    },
    _ => Option::None,
  }
}

fn read_tpl (context: &mut Context, left_open: bool) -> Option<( u16, u32, usize )> {
  let mut escaped = false;
  let mut len: usize = 1;
  let mut dollar = false;
  loop {
    let c = context.next();
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
            context.state.brace_stack.push(true);
            return Option::Some(( (if left_open { token_type::TPL_STR_RL } else { token_type::TPL_STR_L }), 0, len ));
          }
          dollar = false;
        },
        _ => dollar = false,
      }
    }
  }
}
