mod generator;

use proc_macro::TokenStream;

/// Marks trait as an interface to another contract.
#[proc_macro_attribute]
pub fn interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match generator::interface(item.into()) {
        Ok(result) => result.into(),
        Err(_) => panic!(),
    }
}

/// Marks function as a called function.
#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match generator::action(item.into()) {
        Ok(result) => result.into(),
        Err(_) => panic!(),
    }
}
