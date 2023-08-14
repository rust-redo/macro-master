mod heap_size;

#[proc_macro_derive(HeapSize)]
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    heap_size::derive_heap_size(input)
}
