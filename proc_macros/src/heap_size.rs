use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

/// description: 
///     calculate struct instance's allocated heap size,
///     help struct implement the heap_size::HeapSize trait.
///     see heap_size::HeapSize and test cases in proc_macros/tests/heap_size.rs
///     
/// e.g: 
///     #[derive(HeapSize)]
///     struct Demo<'a, T: ?Sized> {
///         a: Box<T>,
///         b: u8,
///         c: &'a str,
///         d: String
///     }
///
///     let demo = Demo {
///         a: b"bytestring".to_vec().into_boxed_slice(),
///         b: 255,
///         c: "&'static str",
///         d: "String".to_owned()
///     }
/// 
///     demo.heap_size_of_children() == 16
///
/// run test: 
///     `cd proc_macros && cargo run heap_size_tests`
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
}

