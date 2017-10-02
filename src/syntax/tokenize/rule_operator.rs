use syntax::tokenize::token_type;
use syntax::tokenize::input_reader::InputReader;

pub fn all (reader: &mut InputReader) -> Option<( u16, u32, usize )> {
  match reader.next() {
    ':' => Option::Some(( token_type::COLON, 0, 1 )),
    ';' => Option::Some(( token_type::SEMI_COLON, 0, 1 )),
    ',' => Option::Some(( token_type::COMMA, 0, 1 )),
    '(' => Option::Some(( token_type::PARENTHESIS, token_type::brace::LEFT, 1 )),
    ')' => Option::Some(( token_type::PARENTHESIS, token_type::brace::RIGHT, 1 )),
    '[' => Option::Some(( token_type::BRACKET, token_type::brace::LEFT, 1 )),
    ']' => Option::Some(( token_type::BRACKET, token_type::brace::RIGHT, 1 )),
    '{' => Option::Some(( token_type::BRACE, token_type::brace::LEFT, 1 )),
    '}' => Option::Some(( token_type::BRACE, token_type::brace::RIGHT, 1 )),
    '+' => match reader.next() {
      '+' => Option::Some(( token_type::OPERATOR, token_type::operator::INCRE, 2 )),
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::PLUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::PLUS, 1 )),
    },
    '-' => match reader.next() {
      '-' => Option::Some(( token_type::OPERATOR, token_type::operator::DECRE, 2 )),
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::MINUS_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::MINUS, 1 )),
    },
    '*' => match reader.next() {
      '*' => match reader.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::POW_ASSIGN, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::POW, 2 )),
      },
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::MULTIPLY_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::MULTIPLY, 1 )),
    },
    '/' => match reader.next() {
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::DIVIDE_ASSIGN, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::DIVIDE, 1 )),
    },
    '!' => match reader.next() {
      '=' => match reader.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_NOT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT_EQUAL, 2 )),
      },
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::NOT, 1 )),
    },
    '?' => Option::Some(( token_type::OPERATOR, token_type::operator::QUERY, 1 )),
    '@' => Option::Some(( token_type::OPERATOR, token_type::operator::AT, 1 )),
    '>' => match reader.next() {
      '>' => match reader.next() {
        '>' => Option::Some(( token_type::OPERATOR, token_type::operator::U_RIGHT_SHIFT, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::RIGHT_SHIFT, 2 )),
      },
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::EGT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::GT, 1 )),
    },
    '<' => match reader.next() {
      '<' => Option::Some(( token_type::OPERATOR, token_type::operator::LEFT_SHIFT, 3 )),
      '=' => Option::Some(( token_type::OPERATOR, token_type::operator::ELT, 2 )),
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::LT, 1 )),
    },
    '=' => match reader.next() {
      '=' => match reader.next() {
        '=' => Option::Some(( token_type::OPERATOR, token_type::operator::STRICT_EQUAL, 3 )),
        _ => Option::Some(( token_type::OPERATOR, token_type::operator::EQUAL, 2 )),
      },
      _ => Option::Some(( token_type::OPERATOR, token_type::operator::ASSIGN, 1 )),
    },
    _ => Option::None,
  }
}
