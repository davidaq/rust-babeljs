use syntax::tokenize::{ Token2 };

pub struct Context<'a> {
  pub source: String,
  pub tokens: Vec<Box<Token2<'a> >>,
}

impl<'a> Context<'a> {
  pub fn new () -> Self {
    Context {
      source: String::new(),
      tokens: vec![],
    }
  }

  pub fn append_source (&mut self, code: &str) {
    self.source += code;
  }
}

