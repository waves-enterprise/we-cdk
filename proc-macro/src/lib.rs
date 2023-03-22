#![no_std]
use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let action_name = &item.sig.ident;
    let action_args = &item.sig.inputs;
    let action_block = &item.block;

    quote!(
        #[no_mangle]
        pub extern "C" fn #action_name (#action_args) {
            unsafe {
                #action_block
            }
        }
    )
    .into()
}
