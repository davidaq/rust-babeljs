#[derive(INode)] enum NodeDefintionBegin {}

node_type! {
  Statement {}
}

node_type! {
  Literal {}
}

node_type! {
  Program {
    body          : Vec<NodeBox> = vec![]
    directives    : Vec<NodeBox> = vec![]
  }
}

node_type! {
  ExpressionStatement <: Statement {
    expression    : NodeBox = None
  }
}

node_type! {
  Directive <: Statement {
    value         : NodeBox/*DirectiveLiteral*/ = None
  }
}

node_type! {
  DirectiveLiteral <: StringLiteral {
    value         : String = String::from("")
  }
}

node_type! {
  BreakStatement <: Statement {
    label        : NodeBox/*Identifier*/ = None
  }
}

node_type! {
  StringLiteral <: Literal {
    value        : String = String::from("")
  }
}

#[derive(INode)] enum NodeDefintionEnd {}
