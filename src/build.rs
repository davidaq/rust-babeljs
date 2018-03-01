use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

fn gen_token_type () {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("token_type.rs");
  let mut f = File::create(&dest_path).unwrap();

  let mut flag_counter = 1;
  let mut id_counter = 0;

  let mut flag_hash = HashMap::new();
  
  let mut content_mapping = String::new();
  let mut flag_mapping = String::new();

  macro_rules! def {
    (flag $name:ident) => {
      {
        flag_counter = flag_counter << 1;
        write!(
          f,
          "pub const {} : Flag = {};\n",
          stringify!($name),
          flag_counter
        ).unwrap();
        flag_hash.insert(stringify!($name), flag_counter);
      };
    };
    (token $name:ident ($content:expr) <: $($flag:expr),*) => {
      {
        id_counter += 1;
        write!(
          f,
          "pub const {} : TokenType = {};\n",
          stringify!($name),
          id_counter
        ).unwrap();
        let mut flag_val = 0;
        $(
          match flag_hash.get(stringify!($flag)) {
            Some (val) => {
              flag_val |= val;
            },
            _ => (),
          }
        )*
        flag_mapping += &format!(
          ", {}",
          flag_val
        );
        content_mapping += &format!(
          ", {}",
          stringify!($content)
        );
      }
    };
    (token $name:ident <: $($flag:expr),*) => {
      def!(token $name (Option::None) <: $($flag),*);
    };
    (token $name:ident := $content:tt <: $($flag:expr),*) => {
      def!(token $name (Option::Some($content)) <: $($flag),*);
    };
    (token $name:ident = $content:tt <: $($flag:expr),*) => {
      def!(token $name (Option::Some(stringify!($content))) <: $($flag),*);
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

  let type_count = id_counter + 1;
  write!(
    f,
    "const TOK_STRING : [Option<&'static str>;{}] = [Option::None {}];\n",
    type_count,
    &content_mapping
  ).unwrap();

  write!(
    f,
    "const TOK_FLAG : [Flag;{}] = [0 {}];\n",
    type_count,
    &flag_mapping
  ).unwrap();
}

pub fn main () {
  gen_token_type();
}

