extern crate crossbeam;
extern crate std_semaphore;

#[macro_use]
extern crate derives;

mod syntax;
mod util;
mod run;

fn main() {
  run::main();
}
