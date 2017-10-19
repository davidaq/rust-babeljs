use syntax::tokenize::{ Token, token_type };
use super::{ Interpretor, node, node_type_id };

fn parse (context: &mut Interpretor) -> Option<node::Node> {
  let tok = context.cur_token();
  match tok.token_type {
    token_type::STRING_LITERAL => {
      match tok.content {
        Some (ref val) => return Some(node::literal(node_type_id::STRING_LITERAL, val.clone())),
        None => (),
      }
    },
    _ => (),
  };
  return None;
}
