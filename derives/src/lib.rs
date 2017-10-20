#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn gen_type_id () -> u32 {
  static mut COUNTER: u32 = 0;
  unsafe {
    COUNTER += 1;
    return COUNTER;
  }
}

#[proc_macro_derive(INode)]
pub fn derive_node (input: TokenStream) -> TokenStream {
  let source = input.to_string();
  let ast = syn::parse_derive_input(&source).unwrap();
  let name = &ast.ident;
  let type_name = name.to_string();
  let fields_setup = match ast.body {
    syn::Body::Struct (ref body) => {
      body.fields().iter().map(|field| {
        match field.ident {
          Some (ref ident) => {
            match &field.ty {
              &syn::Ty::Path (.., ref path) => {
                return match &path.segments[0].ident.to_string() as &str {
                  "Vec" => return quote! { #ident: vec![] },
                  "Option" => return quote! { #ident: None },
                  _ => quote! { #ident: #path::new() },
                };
              },
              &syn::Ty::Array (..) => return quote! { #ident: vec![] },
              _ => panic!("Field `{}` on `{}` has unsupported field type for INode", ident.to_string(), type_name),
            }
          },
          None => panic!("INode cannot derive on a tupple"),
        }
      })
    },
    _ => panic!("INode can only derive on structs"),
  };
  let type_id = gen_type_id();
  let expanded = quote! {
    impl #name {
      pub fn new () -> Self {
        #name {
          #(#fields_setup),*
        }
      }
      pub fn is_typeof<T: INode> (inst: &T) -> bool {
        return inst.type_id() == #type_id;
      }
      pub fn to_box (self) -> node::NodeBox {
        return node::NodeBox { val: Some (Box::new(self)) };
      }
      pub fn cast (boxed: &Option<Box<INode>>) -> Option<&#name> {
        match boxed {
          &None => None,
          &Some (ref boxed) => boxed.as_any().downcast_ref::<#name>(),
        }
      }
      pub fn cast_mut (boxed: &mut Option<Box<INode>>) -> Option<&mut #name> {
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
    }
  };
  let ret: TokenStream = expanded.parse().unwrap();
  // println!("{}", ret.to_string());
  return ret;
}
