use util::Queue;

pub struct InputReader<'a> {
  input: &'a Queue<String>,
}

impl<'a> InputReader<'a> {
  pub fn new (input: &'a Queue<String>) -> Self {
    InputReader {
      input: input,
    }
  }
}
