use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn gen_token_type () {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("token_type.rs");
  let mut f = File::create(&dest_path).unwrap();

  let mut flag_counter = 1;
  let mut id_counter = 0;
  let mut content_mapping = String::new();

  macro_rules! def {
    (flag $name:ident) => {
      {
        flag_counter = flag_counter << 1;
        write!(
          f,
          "pub const {} : u32 = {};\n",
          stringify!($name),
          flag_counter
        ).unwrap();
      };
    };
    (token $name:ident <: $($flag:expr),*) => {
      {
        id_counter += 1;
        write!(
          f,
          "pub const {} : TokenType = TokenType {{ id: {}, flag: {} }};\n",
          stringify!($name),
          id_counter,
          stringify!($($flag)|*),
        ).unwrap();
      };
    };
    (token $name:ident ($content:expr) <: $($flag:expr),*) => {
      {
        def!(token $name <: $($flag),*);
        content_mapping += &format!(
          "{} => {},\n",
          id_counter,
          stringify!(Option::Some($content))
        );
      }
    };
    (token $name:ident := $content:tt <: $($flag:expr),*) => {
      def!(token $name ($content) <: $($flag),*);
    };
    (token $name:ident = $content:tt <: $($flag:expr),*) => {
      def!(token $name (stringify!($content)) <: $($flag),*);
    };
    (token $name:ident = $content:tt) => {
      def!(token $name = $content <: 0);
    };
    (token $name:ident := $content:tt) => {
      def!(token $name := $content <: 0);
    };
    (token $name:ident) => {
      def!(token $name <: 0);
    };
  };

  include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/syntax/tokenize/token_type.rs"));

  write!(
    f,
    "fn stringify (id: u32) -> Option<&'static str> {{ match (id) {{ {} _ => Option::None, }}  }}",
    &content_mapping
  ).unwrap();
  f.write_all(b"\n").unwrap();
}

pub fn main () {
  gen_token_type();
}

