use std::{ env, process, io, fs };
use std::io::BufRead;
use syntax::tokenize::{ Token, Tokenizer, token_type };
use syntax::interpret::{ Interpretor };
use util::Queue;
use crossbeam;

pub fn main() {
  let source_queue = Queue::<String>::new("source");
  let token_queue = Queue::<Token>::new("token");
  crossbeam::scope(|scope| {
    scope.spawn(|| {
      match env::args().nth(2) {
        Some (out) => match &out as &str {
          "print_tokens" => print_tokens(&token_queue),
          _ => (),
        },
        None => {
          Interpretor::new(&token_queue).run();
        },
      };
    });
    scope.spawn(|| {
      Tokenizer::new(&source_queue, &token_queue).run();
    });
    match env::args().nth(1) {
      Some (mode) => match &mode as &str {
        "test" => {
          source_queue.push(String::from("`42`"));
          source_queue.end();
        },
        "pipe" => {
          let stdin = io::stdin();
          let mut input = stdin.lock();
          loop {
            let mut line = String::new();
            match input.read_line(&mut line) {
              Ok (size) => {
                if size == 0 {
                  break;
                }
                source_queue.push(line);
              },
              Err (..) => break,
            }
          }
          source_queue.end();
        },
        "ipc" => {
          println!("IPC not implemented");
          process::exit(1);
        },
        _ => {
          let filename = &mode;
          let filein = fs::File::open(filename).expect("File not found");
          let mut input = io::BufReader::new(filein);
          loop {
            let mut line = String::new();
            match input.read_line(&mut line) {
              Ok (size) => {
                if size == 0 {
                  break;
                }
                source_queue.push(line);
              },
              Err (..) => break,
            }
          }
          source_queue.end();
        },
      },
      None => {
        println!("Must provide a mode!");
        println!("Options are: test, ipc, pipe");
        process::exit(1);
      },
    }
  });
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
