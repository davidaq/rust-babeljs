use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    ':' => Option::Some(( token_type::COLON, 0, 1 )),
    ';' => Option::Some(( token_type::SEMI_COLON, 0, 1 )),
    ',' => Option::Some(( token_type::COMMA, 0, 1 )),
    '(' => Option::Some(( token_type::PARENTHESIS_L, 0, 1 )),
    ')' => Option::Some(( token_type::PARENTHESIS_R, 0, 1 )),
    '[' => Option::Some(( token_type::BRACKET_L, 0, 1 )),
    ']' => Option::Some(( token_type::BRACKET_R, 0, 1 )),
    '{' => {
      reader.state.brace_stack.push(false);
      Option::Some(( token_type::BRACE_L, 0, 1 ))
    },
    '}' => {
      reader.state.brace_stack.pop();
      Option::Some(( token_type::BRACE_R, 0, 1 ))
    },
    '?' => match reader.next() {
      '.' => Option::Some(( token_type::QUESTION_DOT, 0, 2 )),
      _ => Option::Some(( token_type::QUESTION, 0, 1 )),
    },
    '.' => match reader.next() {
      '.' => match reader.next() {
        '.' => Option::Some(( token_type::ELIPSIS, 0, 3 )),
        _ => Option::Some(( token_type::DOT, 0, 1 )),
      },
      _ => Option::Some(( token_type::DOT, 0, 1 )),
    },
    '%' => Option::Some(( token_type::OPERATOR, token_type::operator::MOD, 1 )),
    '^' => Option::Some(( token_type::OPERATOR, token_type::operator::XOR, 1 )),
    '@' => Option::Some(( token_type::AT, 0, 1 )),
    '#' => Option::Some(( token_type::HASH, 0, 1 )),
    '+' => match reader.next() {
      '+' => Option::Some(( token_type::OPERATOR, token_type::operator::INCRE, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::PLUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::PLUS, 1 )),
    },
    '-' => match reader.next() {
      '-' => Option::Some(( token_type::OPERATOR, token_type::operator::DECRE, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::MINUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::MINUS, 1 )),
    },
    '*' => match reader.next() {
      '*' => match reader.next() {
        '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::POW_ASSIGN, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::POW, 2 )),
      },
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::MULTIPLY_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::MULTIPLY, 1 )),
    },
    '/' => match reader.next() {
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::DIVIDE_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::DIVIDE, 1 )),
    },
    '!' => match reader.next() {
      '=' => match reader.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_NOT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT_EQUAL, 2 )),
      },
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT, 1 )),
    },
    '>' => match reader.next() {
      '>' => match reader.next() {
        '>' => Option::Some(( token_type::OPERATOR, token_type::operator::U_RIGHT_SHIFT, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::RIGHT_SHIFT, 2 )),
      },
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::EGT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::GT, 1 )),
    },
    '<' => match reader.next() {
      '<' => Option::Some(( token_type::OPERATOR, token_type::operator::LEFT_SHIFT, 2 )),
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::ELT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::LT, 1 )),
    },
    '=' => match reader.next() {
      '=' => match reader.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::EQUAL, 2 )),
      },
      '>' => Option::Some(( token_type::ARROW, 0, 2 )),
      _ => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::ASSIGN, 1 )),
    },
    '&' => match reader.next() {
      '&' => Option::Some(( token_type::OPERATOR, token_type::operator::AND, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::B_AND_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::B_AND, 1 )),
    },
    '|' => match reader.next() {
      '|' => Option::Some(( token_type::OPERATOR, token_type::operator::OR, 2 )),
      '=' => Option::Some(( token_type::ASSIGN_OPERATOR, token_type::operator::B_OR_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::B_OR, 1 )),
    },
    '~' => Option::Some(( token_type::OPERATOR, token_type::operator::B_INVERT, 1 )),
    _ => Option::None,
  }
}
