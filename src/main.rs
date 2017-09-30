extern crate crossbeam;
extern crate std_semaphore;

mod syntax;
mod util;

use syntax::tokenize::{ Token, Tokenizer, token_type };
use util::Queue;

fn main() {
    let source_queue = Queue::<String>::new();
    let token_queue = Queue::<Token>::new();
    let mut tokenizer = Tokenizer::new(&source_queue, &token_queue);
    crossbeam::scope(|scope| {
        scope.spawn(|| {
            tokenizer.run();
        });
        source_queue.push(String::from(" if (1 + 1> 2) {"));
        source_queue.push(String::from("alert ('hello world');"));
        source_queue.push(String::from("} else if{}"));
        source_queue.end();
        loop {
            match token_queue.pop() {
                Some(token) => {
                    match token.token_type {
                        token_type::Unexpected => println!("Unexpected"),
                        _ => {
                            match token.content {
                                Some (content) => {
                                    println!("token: {} content: {}", token.token_type - token_type::CopySource, content);
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
