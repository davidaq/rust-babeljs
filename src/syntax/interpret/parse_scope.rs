use syntax::tokenize::{ Token, token_type };
use super::*;
use super::node::{ NodeBox };

pub fn parse_program (context: &mut Interpretor) -> NodeBox {
  let mut program = node::Program::new();
  parse_body(context, &mut program.body, &mut program.directives);
  return program.to_box();
}

pub fn parse_body (context: &mut Interpretor, body: &mut Vec<NodeBox>, directives: &mut Vec<NodeBox>) {
  // let mut body: node::NodeList = vec![];
  // let mut directives: node::NodeList = vec![];
  // let mut allow_scope_directive = true;
  // loop {
  //   match parse_statement::parse(context) {
  //     Some (node) => {
  //       if allow_scope_directive && node.node_type == node_type_id::EXPRESSION_STMT {
  //         allow_scope_directive = false;
  //         match node.expression {
  //           Some (ref expression) => {
  //             if expression.node_type == node_type_id::STRING_LITERAL {
  //               match expression.str_value {
  //                 Some (ref val) => {
  //                   allow_scope_directive = true;
  //                   body.push(Box::new(node::directive(val.clone())));
  //                 },
  //                 None => (),
  //               }
  //             }
  //           },
  //           None => (),
  //         }
  //       }
  //       if !allow_scope_directive {
  //         body.push(Box::new(node));
  //       }
  //     },
  //     None => break,
  //   }
  // };
  // return (body, directives);
}
