#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn the_proc_macro(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    function
}
