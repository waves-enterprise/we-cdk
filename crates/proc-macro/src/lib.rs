mod generator;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match generator::interface(item.into()) {
        Ok(result) => result.into(),
        Err(_) => panic!(),
    }
}

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match generator::action(item.into()) {
        Ok(result) => result.into(),
        Err(_) => panic!(),
    }
}
