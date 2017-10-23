#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod node;

#[proc_macro_derive(INode)]
pub fn derive_node (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  node::derive_node(input)
}
