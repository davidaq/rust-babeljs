use syntax::tokenize::{ Token, token_type };
use super::*;
use super::node::{ NodeBox };

pub fn parse_program (context: &mut Interpretor) -> NodeBox {
  let mut program = node::Program::new();
  parse_body(context, &mut program.body, &mut program.directives);
  return program.to_box();
}

pub fn parse_body (context: &mut Interpretor, body: &mut Vec<NodeBox>, directives: &mut Vec<NodeBox>) {
  let mut allow_scope_directive = true;
  loop {
    let stmt = parse_statement::parse(context);
    if match stmt { None => true, _ => false } {
      break;
    }
    if allow_scope_directive {
      allow_scope_directive = false;
      match node::ExpressionStatement::cast(&stmt) {
        Some (ref expr_stmt) => {
          match node::StringLiteral::cast(&expr_stmt.expression) {
            Some (ref literal) => {
              allow_scope_directive = true;
              let mut stmt = node::Directive::new();
              let mut value = node::DirectiveLiteral::new();
              value.value = literal.value.clone();
              stmt.value = value.to_box();
              body.push(stmt.to_box());
            },
            None => (),
          }
        },
        None => (),
      }
    }
    if !allow_scope_directive {
      body.push(stmt);
    }
  };
}
