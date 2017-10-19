use syntax::basic_types::SourceLoc;
use super::node_type_id::*;

pub type NodeList = Vec<Box<Node>>;

pub struct Node {
  pub node_type: u32,
  pub loc: Option<SourceLoc>,
  pub body: Option<NodeList>,
  pub directives: Option<NodeList>,
  pub node_value: Option<Box<Node>>,
  pub str_value: Option<String>,
  pub expression: Option<Box<Node>>,
}

pub fn node (type_id: u32) -> Node {
  Node {
    node_type: type_id,
    loc: None,
    body: None,
    directives: None,
    node_value: None,
    str_value: None,
    expression: None,
  }
}

pub fn program (body: NodeList, directives: NodeList) -> Node {
  let mut node = node(PROGRAM);
  node.body = Some(body);
  node.directives = Some(directives);
  return node;
}

pub fn expression_statement (expression: Node) -> Node {
  let mut node = node(EXPRESSION_STMT);
  node.expression = Some(Box::new(expression));
  return node;
}

pub fn literal (node_type: u32, value: String) -> Node {
  let mut node = node(node_type);
  node.str_value = Some(value);
  return node;
}

pub fn directive (value: String) -> Node {
  let mut node = node(DIRECTIVE);
  node.node_value = Some(Box::new(literal(DIRECTIVE_LITERAL, value)));
  return node;
}
