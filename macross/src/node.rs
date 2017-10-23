use syn;
use quote;
use proc_macro::TokenStream;
use std::sync::{ Once, ONCE_INIT };
use std::mem;


fn gen_type_id () -> u32 {
  static mut COUNTER: u32 = 0;
  unsafe {
    COUNTER += 1;
    return COUNTER;
  }
}

fn type_names<T> (cb: T) where T: FnOnce(&mut Vec<String>) {
  static mut INST: *mut Vec<String> = 0 as *mut Vec<String>;
  static ONCE: Once = ONCE_INIT;
  unsafe {
    ONCE.call_once(|| {
      let inst: Vec<String> = vec![];
      INST = mem::transmute(Box::new(inst));
    });
    cb(&mut *INST);
  }
}

pub fn derive_node (input: TokenStream) -> TokenStream {
  let source = input.to_string();
  let ast = syn::parse_derive_input(&source).unwrap();
  let name = &ast.ident;
  let type_name = name.to_string();
  match ast.body {
    syn::Body::Enum (_) => {
      let ret: TokenStream = (match &type_name as &str {
        "NodeDefintionBegin" => prepare_derive_node(),
        "NodeDefintionEnd" => process_derive_node(),
        _ => quote! {},
      }).parse().unwrap();
      // println!("{}", ret.to_string());
      return ret;
    },
    _ => (),
  };
  let type_id = gen_type_id();
  type_names(|list| {
    list.push(type_name.clone());
  });
  let expanded = quote! {
    impl #name {
      pub fn is_typeof<T: INode> (inst: &T) -> bool {
        return inst.type_id() == #type_id;
      }
      pub fn to_box (self) -> NodeBox {
        return Some(Box::new(self));
      }
      pub fn cast (boxed: &NodeBox) -> Option<&#name> {
        match boxed {
          &None => None,
          &Some (ref boxed) => boxed.as_any().downcast_ref::<#name>(),
        }
      }
      pub fn cast_mut (boxed: &mut NodeBox) -> Option<&mut #name> {
        match boxed.as_mut() {
          None => None,
          Some (boxed) => boxed.as_any_mut().downcast_mut::<#name>(),
        }
      }
    }
    impl INode for #name {
      fn loc (&self) -> &SourceLoc {
        return &self.loc;
      }
      fn type_id (&self) -> u32 {
        return #type_id;
      }
      fn type_name (&self) -> &'static str {
        return #type_name;
      }
      fn as_any (&self) -> &Any {
        return self;
      }
      fn as_any_mut (&mut self) -> &mut Any {
        return self;
      }
      fn has_flag (&self, flag: Flag) -> bool {
        return self.has_flag_(flag);
      }
    }
  };
  let ret: TokenStream = expanded.parse().unwrap();
  // println!("{}", ret.to_string());
  return ret;
}

pub fn prepare_derive_node () -> quote::Tokens {
  quote! {
    use syntax::basic_types::SourceLoc;
    use std::any::Any;

    pub trait INode {
      fn loc (&self) -> &SourceLoc;
      fn type_id (&self) -> u32;
      fn type_name (&self) -> &'static str;
      fn as_any (&self) -> &Any;
      fn as_any_mut (&mut self) -> &mut Any;
      fn has_flag (&self, Flag) -> bool;
    }
    trait IFlagged {
      fn has_flag_ (&self, Flag) -> bool;
    }

    macro_rules! node_type_base {
      ($name:ident {
        $($field_name:ident : $field_type:ty = $def_val:expr)*
      }) => {
        #[derive(INode)]
        pub struct $name {
          pub loc: SourceLoc,
          $(pub $field_name: $field_type,)*
        }
        impl $name {
          pub fn new () -> Self {
            $name {
              loc: SourceLoc::new(),
              $($field_name: $def_val,)*
            }
          }
        }
      };
    }

    macro_rules! node_type {
      ($name:ident <: $($flag:ident),* {
        $($field_name:ident: $field_type:ty = $def_val:expr)*
      }) => {
        node_type_base!($name {
          $($field_name : $field_type = $def_val)*
        });
        impl IFlagged for $name {
          fn has_flag_ (&self, flag: Flag) -> bool {
            match flag {
              $(Flag::$flag)|* => true,
              _ => false,
            }
          }
        }
      };
      ($name:ident {
        $($field_name:ident : $field_type:ty = $def_val:expr)*
      }) => {
        node_type_base!($name {
          $($field_name : $field_type = $def_val)*
        });
        impl IFlagged for $name {
          fn has_flag_ (&self, flag: Flag) -> bool {
            false
          }
        }
      };
    }

    pub type NodeBox = Option<Box<INode>>;
  }
}

pub fn process_derive_node () -> quote::Tokens {
  let mut names: Vec<quote::Tokens> = vec![];
  type_names(|list| {
    for i in 0..list.len() {
      let name = syn::Ident::from(list[i].clone());
      names.push(quote! {
        #name,
      });
    }
  });
  quote! {
    enum Flag {
      #(#names)*
    }
  }
}
