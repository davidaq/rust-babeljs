use std::str::Chars;
use util::Queue;
use syntax::basic_types::{ SourceLoc };
use syntax::tokenize::*;
use syntax::context::Context;

struct State {
  pub expr_allowed: bool,
  pub brace_stack: Vec<bool>,
}

impl State {
  fn new () -> Self {
    State {
      expr_allowed: true,
      brace_stack: vec![],
    }
  }
}

pub struct Tokenizer<'a> {
  state: State,
  context: &'a mut Context,
  source_chars: Vec<char>,
  buffer: Vec<char>,
  head: usize,
  cursor: usize,
  ended: bool,
  line: usize,
  col: usize,
  commit: usize,

  tmp_commit: usize,
  tmp_len: usize,
  tmp_line: usize,
  tmp_col: usize,
  tmp_content: String,
}

impl<'a> Tokenizer<'a> {

  pub fn tokenize (ctx: &mut Context) {
    let source_chars = ctx.source.chars().collect();
    let mut inst = Tokenizer {
      state: State::new(),
      context: ctx,
      source_chars: source_chars,
      buffer: vec![],
      head: 0,
      cursor: 0,
      ended: false,
      line: 1,
      col: 0,
      commit: 0,

      tmp_commit: 0,
      tmp_len: 0,
      tmp_line: 1,
      tmp_col: 1,
      tmp_content: String::with_capacity(100),
    };
    inst.run()
  }

  pub fn next (&mut self) -> char {
    if self.cursor >= self.source_chars.len() {
      return '\0';
    } else {
      let c = self.source_chars[self.cursor];
      self.cursor += 1;
      return c;
    }
  }

  fn try_rules (&mut self, start: usize) -> bool {
    macro_rules! match_token_rule {
      ( $rule:ident ) => {
        self.cursor = start;
        match $rule::try(self) {
          None => (),
          Some (result) => {
            // match tt::stringify(result.0) {
            //   Some (s) => println!("{}", s),
            //   None => {
            //     println!("*vary* '{}'", content);
            //   },
            // }
            let end = start + result.1;
            // let content = &(self.context.source).get(start..end).unwrap();
            self.cursor = end;
            let token = tt::Token {
              token_type: result.0,
              context: self.context as *const Context,
              start: start,
              end: end,
            };
            self.context.append_token(token);
            println!("T: {}", self.context.tokens[0].content());
            return true;
          }
        }
      }
    }
    match_token_rule!(rule_whitespace);
    return false;
  }

  fn run (&mut self) {
    let mut prev_type = tt::UNEXPECTED;

    while !self.ended {
      let mut token_type = tt::UNEXPECTED;
      let start = self.cursor;
      if self.try_rules(start) {
        break;
      }
      // let mut content = Option::None;
      // let start = self.context.pos();

  //    macro_rules! match_token_rule {
  //      ( $rule:path ) => {
  //        if token_type == token_type::UNEXPECTED {
  //          match $rule (&mut self.context) {
  //            None => self.context.reset(),
  //            Some (result) => {
  //              token_type = result.0;
  //              flag = result.1;
  //              let len = result.2;
  //              if (token_type::COPY_SOURCE & token_type) == token_type::COPY_SOURCE {
  //                content = Option::Some(String::from(self.context.content(len)));
  //              }
  //              self.context.commit(len);
  //            },
  //          }
  //        };
  //      }
  //    }
  //    match_token_rule!(rule_whitespace::all);
  //    match_token_rule!(rule_comment::all);
  //    match_token_rule!(rule_literal::string);
  //    match_token_rule!(rule_literal::number);
  //    match_token_rule!(rule_tplstr::all);
  //    match_token_rule!(rule_regex::all);
  //    match_token_rule!(rule_identifier::all);
  //    match_token_rule!(rule_operator::all);

  //    if token_type == token_type::UNEXPECTED {
  //      content = Option::Some(String::from(self.context.content(5)));
  //      self.context.commit(1);
  //    }
  //    self.output.push(Token {
  //      token_type: token_type,
  //      flag: flag,
  //      loc: SourceLoc {
  //        start: start,
  //        end: self.context.pos(),
  //      },
  //      content: content,
  //    });
  //    if token_type == token_type::UNEXPECTED {
  //      break;
  //    }
  //    if token_type != token_type::WHITE_SPACE {
  //      if (token_type == token_type::IDENTIFIER || (token_type & token_type::KEYWORD) > 0) && match prev_type { token_type::QUESTION_DOT | token_type::DOT => true, _ => false } {
  //        self.context.state.expr_allowed = true;
  //      } else {
  //        self.context.state.expr_allowed = (token_type & token_type::BEFORE_EXPR) > 0;
  //      }
  //      prev_type = token_type;
  //    }
    }
  }
}

