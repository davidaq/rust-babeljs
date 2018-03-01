use std::{ env, process, io, fs };
use std::io::BufRead;
use syntax::context::Context;
use syntax::tokenize::{ Token, Tokenizer, token_type };
//use syntax::interpret::{ Interpretor };
use syntax::tokenize::tt;
use util::Queue;

pub fn main() {
  println!("{}", tt::ARROW.stringify().unwrap());
  println!("{}", tt::LINE_COMMENT == tt::LINE_COMMENT);
  let mut context = Context::new();
  let source_queue = Queue::<String>::new("source");
  let token_queue = Queue::<Token>::new("token");

  match env::args().nth(1) {
    Some (mode) => match &mode as &str {
      "debug" => {
        context.append_source("`42`");
      },
      _ => {
        let filename = &mode;
        let filein = fs::File::open(filename).expect("File not found");
        let mut input = io::BufReader::new(filein);
        let mut line = String::new();
        loop {
          match input.read_line(&mut line) {
            Ok (size) => {
              if size == 0 {
                break;
              }
              context.append_source(&line);
            },
            Err (..) => break,
          }
        }
      },
    },
    None => {
      println!("Must provide a mode!");
      println!("Options are: test, ipc, pipe");
      process::exit(1);
    },
  }
  Tokenizer::tokenize(&mut context);
  
  //Tokenizer::new(&source_queue, &token_queue).run();

  //match env::args().nth(2) {
  //  Some (out) => match &out as &str {
  //    "print_tokens" => print_tokens(&token_queue),
  //    _ => (),
  //  },
  //  None => {
  //    //Interpretor::new(&token_queue).run();
  //  },
  //};

}

fn print_tokens (token_queue: &Queue<Token>) {
  loop {
    match token_queue.pop() {
      Some(token) => {
        match token.token_type {
          token_type::UNEXPECTED => match token.content {
            Some (content) => {
              panic!("Unexpected: {}", content);
            },
            None => {
              panic!("Unexpected");
            },
          },
          _ => {
            let plain_token_type = !((!token.token_type) | token_type::ALL_MARKER);
            let loc = format!("line: {} \t col: {}", token.loc.start.line, token.loc.start.col);
            match token.content {
              Some (content) => {
                println!("token: {} \t content: {} \t {}", plain_token_type, content, loc);
              },
              None => {
                println!("token: {} \t flag: {} \t {}", plain_token_type, token.flag, loc);
              },
            }
          },
        };
      },
      None => break,
    }
  }
}
