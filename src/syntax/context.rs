use syntax::tokenize::tt;

pub struct Context {
  pub source: String,
  pub tokens: Vec<tt::Token>,
}

impl Context {
  pub fn new () -> Self {
    Context {
      source: String::new(),
      tokens: vec![],
    }
  }

  pub fn append_source (&mut self, code: &str) {
    self.source += code;
  }

  pub fn append_token (&mut self, token: tt::Token) {
    self.tokens.push(token);
  }
}

