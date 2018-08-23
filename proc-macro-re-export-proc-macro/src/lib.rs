#![feature(proc_macro)]

//! A crate defining a proc macro
//!
//! Doesn't do much

extern crate proc_macro;

use proc_macro::TokenStream;

/// A proc macro
///
/// Doesn't do much
#[proc_macro_attribute]
pub fn the_proc_macro(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    function
}
