use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn try (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
  match context.next() {
    ':' => Option::Some(( tt::COLON, 1 )),
    ';' => Option::Some(( tt::SEMI_COLON, 1 )),
    ',' => Option::Some(( tt::COMMA, 1 )),
    '(' => Option::Some(( tt::PARENTHESIS_L, 1 )),
    ')' => Option::Some(( tt::PARENTHESIS_R, 1 )),
    '[' => Option::Some(( tt::BRACKET_L, 1 )),
    ']' => Option::Some(( tt::BRACKET_R, 1 )),
    '{' => {
      context.state.brace_stack.push(false);
      Option::Some(( tt::BRACE_L, 1 ))
    },
    '}' => {
      context.state.brace_stack.pop();
      Option::Some(( tt::BRACE_R, 1 ))
    },
    '?' => match context.next() {
      '.' => Option::Some(( tt::QUESTION_DOT, 2 )),
      _ => Option::Some(( tt::QUESTION, 1 )),
    },
    '.' => match context.next() {
      '.' => match context.next() {
        '.' => Option::Some(( tt::ELIPSIS, 3 )),
        _ => Option::Some(( tt::DOT, 1 )),
      },
      _ => Option::Some(( tt::DOT, 1 )),
    },
    '%' => Option::Some(( tt::OP_MOD, 1 )),
    '^' => Option::Some(( tt::OP_BIT_XOR, 1 )),
    '@' => Option::Some(( tt::AT, 1 )),
    '#' => Option::Some(( tt::HASH, 1 )),
    '+' => match context.next() {
      '+' => Option::Some(( tt::OP_INCRE, 2 )),
      '=' => Option::Some(( tt::OP_INCRE_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_PLUS, 1 )),
    },
    '-' => match context.next() {
      '-' => Option::Some(( tt::OP_DECRE, 2 )),
      '=' => Option::Some(( tt::OP_DECRE_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_MINUS, 1 )),
    },
    '*' => match context.next() {
      '*' => match context.next() {
        '=' => Option::Some(( tt::OP_POW_ASSIGN, 3 )),
         _ => Option::Some(( tt::OP_POW, 2 )),
      },
      '=' => Option::Some(( tt::OP_MULTIPLY_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_STAR, 1 )),
    },
    '/' => match context.next() {
      '=' => Option::Some(( tt::OP_DIVIDE_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_DIVIDE, 1 )),
    },
    '!' => match context.next() {
      '=' => match context.next() {
        '=' => Option::Some(( tt::OP_NOT_EQUAL_STRICT, 3 )),
        _ => Option::Some(( tt::OP_NOT_EQUAL, 2 )),
      },
      _ => Option::Some(( tt::OP_EXCLAIM, 1 )),
    },
    '>' => match context.next() {
      '>' => match context.next() {
        '>' => match context.next() {
          '=' => Option::Some(( tt::OP_BIT_URIGHT_ASSIGN, 4 )),
          _ => Option::Some(( tt::OP_BIT_URIGHT, 3 )),
        },
        '=' => Option::Some(( tt::OP_BIT_RIGHT_ASSIGN, 3 )),
        _ => Option::Some(( tt::OP_BIT_RIGHT, 2 )),
      },
      '=' => Option::Some(( tt::OP_EGT, 2 )),
      _ => Option::Some(( tt::OP_GT, 1 )),
    },
    '<' => match context.next() {
      '<' => match context.next() {
        '=' => Option::Some(( tt::OP_BIT_LEFT_ASSIGN, 3 )),
        _ => Option::Some(( tt::OP_BIT_LEFT, 2 )),
      },
      '=' => Option::Some(( tt::OP_ELT, 2 )),
      _ => Option::Some(( tt::OP_LT, 1 )),
    },
    '=' => match context.next() {
      '=' => match context.next() {
        '=' => Option::Some(( tt::OP_EQUAL_STRICT, 3 )),
        _ => Option::Some(( tt::OP_EQUAL, 2 )),
      },
      '>' => Option::Some(( tt::ARROW, 2 )),
      _ => Option::Some(( tt::OP_ASSIGN, 1 )),
    },
    '&' => match context.next() {
      '&' => Option::Some(( tt::OP_AND, 2 )),
      '=' => Option::Some(( tt::OP_BIT_AND_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_BIT_AND, 1 )),
    },
    '|' => match context.next() {
      '|' => Option::Some(( tt::OP_OR, 2 )),
      '=' => Option::Some(( tt::OP_BIT_OR_ASSIGN, 2 )),
      _ => Option::Some(( tt::OP_BIT_OR, 1 )),
    },
    '~' => Option::Some(( tt::OP_BIT_INVERT, 1 )),
    _ => Option::None,
  }
}
