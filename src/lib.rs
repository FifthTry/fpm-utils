#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

#[proc_macro_attribute]
pub fn wasm_backend(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let visibility = input_fn.vis;
    let sig = input_fn.sig;
    let ident = sig.ident;
    let inputs = sig.inputs;
    let output = sig.output;
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;
    let block = input_fn.block;
    // let host_stream: proc_macro2::TokenStream = include_str!("guest_imports.rs").parse().unwrap();
    let guest_stream: proc_macro2::TokenStream = include_str!("guest_exports.rs").parse().unwrap();

    quote!(
        // #host_stream

        #guest_stream

        struct GuestBackend;

        impl guest_backend::GuestBackend for GuestBackend {
            #visibility fn #ident #generics (#inputs) #output #where_clause {
                #block
            }
        }
    )
    .into()
}
