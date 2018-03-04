use std::{ env, process, io, fs };
use std::io::BufRead;
use syntax::context::Context;
use syntax::tokenize::{ Token, Tokenizer, token_type };
//use syntax::interpret::{ Interpretor };
use syntax::tokenize::tt;
use util::Queue;

pub fn main () {
  println!("{}", tt::stringify(tt::TRY).unwrap());
  let mut context = Context::new();

  match env::args().nth(1) {
    Some (mode) => match &mode as &str {
      "debug" => {
        context.append_source("#!/bin/bash\n");
        context.append_source("//Hello\n");
        context.append_source("#!/bin/bash\n");
        context.append_source("/*Hello*/console.log\n");
        context.append_source("    var a=  true;\t\t   +`42`\n");
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
  if !Tokenizer::tokenize(&mut context) {
    panic!("Tokenize fail");
  }

  for i in 0..context.tokens.len() {
    let token = &context.tokens[i];
    println!("{} \t: '{}'", tt::name_of(token.token_type), token.content());
  }
  
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

