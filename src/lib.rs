//! A crate re-exporting some macros
//!
//! Doesn't do much

extern crate proc_macro_re_export_normal_macro;
extern crate proc_macro_re_export_proc_macro;

pub use proc_macro_re_export_normal_macro::the_normal_macro;
pub use proc_macro_re_export_proc_macro::the_proc_macro;
