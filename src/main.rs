extern crate crossbeam;
extern crate std_semaphore;

#[macro_use]
extern crate macross;

mod syntax;
mod util;
mod run;

fn main() {
  run::main();
}
