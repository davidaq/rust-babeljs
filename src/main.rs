extern crate crossbeam;
extern crate std_semaphore;

mod syntax;
mod util;

use std::{ env, process, io, fs };
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
                            match token.content {
                                Some (content) => {
                                    println!("token: {} content: {}", plain_token_type, content);
                                },
                                None => {
                                    println!("token: {} flag: {}", plain_token_type, token.flag);
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
