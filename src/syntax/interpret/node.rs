pub enum Flag {
  Statement,
}

#[derive(INode)] enum NodeDefintionBegin {}

node_type!(Program {
  body          : Vec<NodeBox> = vec![];
  directives    : Vec<NodeBox> = vec![];
});

node_type!(ExpressionStatement < Statement {
  body          : Vec<NodeBox> = vec![];
  directives    : Vec<NodeBox> = vec![];
});

#[derive(INode)] enum NodeDefintionEnd {}
