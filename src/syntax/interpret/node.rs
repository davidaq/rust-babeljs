use syntax::basic_types::SourceLoc;
use super::node_type_id::*;

pub type NodeList = Vec<Box<Node>>;

pub enum NodeOrStr {
  n (Box<Node>),
  s (String),
}

pub struct Node {
  pub node_type: u32,
  pub loc: Option<SourceLoc>,
  pub body: Option<NodeList>,
  pub directives: Option<NodeList>,
  pub value: Option<NodeOrStr>,
}

pub fn node (type_id: u32) -> Node {
  Node {
    node_type: type_id,
    loc: None,
    body: None,
    directives: None,
    value: None,
  }
}

pub fn program (body: NodeList, directives: NodeList) -> Node {
  let mut node = node(Program);
  node.body = Option::Some(body);
  node.directives = Option::Some(directives);
  return node;
}
