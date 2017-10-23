use syntax::tokenize::{ Token, token_type };
use super::{ Interpretor, node, node_type_id };
use super::node::INode;

pub fn parse (context: &mut Interpretor) -> node::NodeBox {
  match context.cur_token().token_type {
    token_type::BREAK | token_type::CONTINUE => {
      return parse_break_continue(context);
    },
    token_type::IMPORT | token_type::EXPORT => {
      return parse_module_decl(context);
    },
    _ => (),
  }
  return None;
}

fn parse_break_continue (context: &mut Interpretor) -> node::NodeBox {
  unimplemented!();
}

fn parse_module_decl (context: &mut Interpretor) -> node::NodeBox {
  unimplemented!();
}
