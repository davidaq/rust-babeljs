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
    '+' => Option::Some(( token_type::OPERATOR, token_type::operator::PLUS, 1 )),
    '-' => Option::Some(( token_type::OPERATOR, token_type::operator::MINUS, 1 )),
    '*' => Option::Some(( token_type::OPERATOR, token_type::operator::MULTIPLY, 1 )),
    '/' => Option::Some(( token_type::OPERATOR, token_type::operator::DIVIDE, 1 )),
    '!' => Option::Some(( token_type::OPERATOR, token_type::operator::NOT, 1 )),
    '?' => Option::Some(( token_type::OPERATOR, token_type::operator::QUERY, 1 )),
    '@' => Option::Some(( token_type::OPERATOR, token_type::operator::AT, 1 )),
    '>' => Option::Some(( token_type::OPERATOR, token_type::operator::GT, 1 )),
    '<' => Option::Some(( token_type::OPERATOR, token_type::operator::LT, 1 )),
    _ => Option::None,
  }
}
