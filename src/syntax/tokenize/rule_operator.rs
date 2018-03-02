use syntax::tokenize::tt;
use syntax::tokenize::Tokenizer;

pub fn all (context: &mut Tokenizer) -> Option<( tt::TokenType, usize )> {
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
      '.' => Option::Some(( token_type::QUESTION_DOT, 2 )),
      _ => Option::Some(( token_type::QUESTION, 1 )),
    },
    '.' => match context.next() {
      '.' => match context.next() {
        '.' => Option::Some(( token_type::ELIPSIS, 3 )),
        _ => Option::Some(( token_type::DOT, 1 )),
      },
      _ => Option::Some(( token_type::DOT, 1 )),
    },
    '%' => Option::Some(( token_type::OPERATOR, token_type::operator::MOD, 1 )),
    '^' => Option::Some(( token_type::OPERATOR, token_type::operator::XOR, 1 )),
    '@' => Option::Some(( token_type::AT, 0, 1 )),
    '#' => Option::Some(( token_type::HASH, 0, 1 )),
    '+' => match context.next() {
      '+' => Option::Some(( token_type::OPERATOR, token_type::operator::INCRE, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::PLUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::PLUS, 1 )),
    },
    '-' => match context.next() {
      '-' => Option::Some(( token_type::OPERATOR, token_type::operator::DECRE, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::MINUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::MINUS, 1 )),
    },
    '*' => match context.next() {
      '*' => match context.next() {
        '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::POW_ASSIGN, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::POW, 2 )),
      },
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::MULTIPLY_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::STAR, 1 )),
    },
    '/' => match context.next() {
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::DIVIDE_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::DIVIDE, 1 )),
    },
    '!' => match context.next() {
      '=' => match context.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_NOT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT_EQUAL, 2 )),
      },
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT, 1 )),
    },
    '>' => match context.next() {
      '>' => match context.next() {
        '>' => Option::Some(( token_type::OPERATOR, token_type::operator::U_RIGHT_SHIFT, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::RIGHT_SHIFT, 2 )),
      },
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::EGT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::GT, 1 )),
    },
    '<' => match context.next() {
      '<' => Option::Some(( token_type::OPERATOR, token_type::operator::LEFT_SHIFT, 2 )),
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::ELT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::LT, 1 )),
    },
    '=' => match context.next() {
      '=' => match context.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::EQUAL, 2 )),
      },
      '>' => Option::Some(( token_type::ARROW, 0, 2 )),
      _ => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::ASSIGN, 1 )),
    },
    '&' => match context.next() {
      '&' => Option::Some(( token_type::OPERATOR, token_type::operator::AND, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::B_AND_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::B_AND, 1 )),
    },
    '|' => match context.next() {
      '|' => Option::Some(( token_type::OPERATOR, token_type::operator::OR, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::B_OR_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::B_OR, 1 )),
    },
    '~' => Option::Some(( token_type::OPERATOR, token_type::operator::B_INVERT, 1 )),
    _ => Option::None,
  }
}
