extern crate crossbeam;
extern crate std_semaphore;

mod syntax;
mod util;

use std::{ env, process, io };
use io::BufRead;
use syntax::tokenize::{ Token, Tokenizer, token_type };
use util::Queue;

fn main() {
    let source_queue = Queue::<String>::new("source");
    let token_queue = Queue::<Token>::new("token");
    crossbeam::scope(|scope| {
        scope.spawn(|| {
            Tokenizer::new(&source_queue, &token_queue).run();
        });
        match env::args().nth(1) {
            Some (mode) => match &mode as &str {
                "test" => {
                    source_queue.push(String::from(" if (/avc+1/i.test(/*hehe*/'hehe')) {"));
                    source_queue.push(String::from("alert ('hello world');//alert\n"));
                    source_queue.push(String::from("} else if {}"));
                    source_queue.push(String::from("var a = 1; a++;"));
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
                _ => {
                    println!("No such mode: {}", mode);
                    println!("Options are: test, ipc, pipe");
                    process::exit(1);
                },
            },
            None => {
                println!("Must provide a mode!");
                println!("Options are: test, ipc, pipe");
                process::exit(1);
            },
        }
        loop {
            match token_queue.pop() {
                Some(token) => {
                    match token.token_type {
                        token_type::UNEXPECTED => panic!("Unexpected"),
                        _ => {
                            match token.content {
                                Some (content) => {
                                    println!("token: {} content: {}", token.token_type - token_type::COPY_SOURCE, content);
                                },
                                None => {
                                    println!("token: {}", token.token_type);
                                },
                            }
                        },
                    };
                },
                None => break,
            }
        }
    });
}
