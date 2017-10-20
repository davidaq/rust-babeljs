use syntax::basic_types::SourceLoc;
use super::node_type_id::*;
use std::any::Any;

pub type NodeList = Vec<NodeBox>;
pub struct NodeBox {
  pub val: Option<Box<INode>>,
}
impl NodeBox {
  pub fn is_none () -> bool {
    match self.val {
      Some (x) => false,
      None => true,
    }
  }
}

pub trait INode {
  fn loc (&self) -> &SourceLoc;
  fn type_id (&self) -> u32;
  fn type_name (&self) -> &'static str;
  fn as_any (&self) -> &Any;
  fn as_any_mut (&mut self) -> &mut Any;
}

macro_rules! node_type {
  ($name:ident {
    $($field_name:ident: $field_type:ty,)*
  }) => {
    #[derive(INode)]
    pub struct $name {
      pub loc: SourceLoc,
      $(pub $field_name: $field_type,)*
    }
  }
}

node_type! {
  Program {
    body: NodeList,
    directives: NodeList,
  }
}
