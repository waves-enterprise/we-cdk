use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Convert the described interface into a
/// WASM method set for calling contract methods.
pub fn interface(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let mut mod_func: Vec<TokenStream2> = vec![];

    let input = syn::parse2::<syn::ItemTrait>(input)?;
    let mod_name = input.ident;

    for item in input.items {
        if let syn::TraitItem::Fn(func) = item {
            let func_name_str = func.sig.ident.to_string();
            let func_name = func.sig.ident;

            let mut args: Vec<TokenStream2> = vec![];
            let mut call_args: Vec<TokenStream2> = vec![];
            for arg in func.sig.inputs.iter() {
                if let syn::FnArg::Typed(a) = arg {
                    if let syn::Pat::Ident(pat_ident) = &*a.pat {
                        let arg_name = &pat_ident.ident;

                        if let syn::Type::Path(type_path) = &*a.ty {
                            let path_seg = &type_path.path.segments[0];
                            let name = path_seg.ident.to_string();
                            match name.as_str() {
                                "Integer" => {
                                    args.push(quote!(
                                        #arg_name: i64
                                    ));

                                    call_args.push(quote!(
                                        wevm::v0::bindings::call_arg_int(#arg_name);
                                    ));
                                }
                                "Boolean" => {
                                    args.push(quote!(
                                        #arg_name: bool
                                    ));

                                    call_args.push(quote!(
                                        wevm::v0::bindings::call_arg_bool(#arg_name);
                                    ));
                                }
                                "Binary" => {
                                    args.push(quote!(
                                        #arg_name: &[u8]
                                    ));

                                    call_args.push(quote!(
                                        let error = wevm::v0::bindings::call_arg_binary(#arg_name.as_ptr(), #arg_name.len());
                                        if error != 0 {
                                            return error;
                                        }
                                    ));
                                }
                                "String" => {
                                    args.push(quote!(
                                        #arg_name: &str
                                    ));

                                    call_args.push(quote!(
                                        let error = wevm::v0::bindings::call_arg_string(#arg_name.as_ptr(), #arg_name.len());
                                        if error != 0 {
                                            return error;
                                        }
                                    ));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }

            mod_func.push(quote!(
                pub fn #func_name(contract_id: &[u8], #( #args ),*) -> i32 {
                    unsafe {
                        #( #call_args )*
                        wevm::v0::bindings::call_contract(contract_id.as_ptr(), contract_id.len(), #func_name_str.as_ptr(), #func_name_str.len())
                    }
                }
            ));
        }
    }

    Ok(quote!(
        mod #mod_name {
            use we_cdk::*;

            #( #mod_func )*
        }
    ))
}

/// Converting contract methods to valid WASM methods.
pub fn action(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let input = syn::parse2::<syn::ItemFn>(input)?;

    let name = &input.sig.ident;
    let block = &input.block;

    let mut args: Vec<TokenStream2> = vec![];
    let mut args_build: Vec<TokenStream2> = vec![];

    for arg in input.sig.inputs.iter() {
        if let syn::FnArg::Typed(a) = arg {
            if let syn::Pat::Ident(pat_ident) = &*a.pat {
                let arg_name = &pat_ident.ident;

                let offset = format_ident!("offset_{}", arg_name);
                let length = format_ident!("length_{}", arg_name);

                if let syn::Type::Path(type_path) = &*a.ty {
                    let path_seg = &type_path.path.segments[0];
                    let name = path_seg.ident.to_string();
                    match name.as_str() {
                        "Binary" => {
                            args.push(quote!(
                                #offset: *const u8
                            ));

                            args.push(quote!(
                                #length: usize
                            ));

                            args_build.push(quote!(
                                let #arg_name = core::slice::from_raw_parts(#offset, #length);
                            ));
                        }
                        "String" => {
                            args.push(quote!(
                                #offset: *const u8
                            ));

                            args.push(quote!(
                                #length: usize
                            ));

                            args_build.push(quote!(
                                let #arg_name = {
                                    let bytes = core::slice::from_raw_parts(#offset, #length);
                                    core::str::from_utf8_unchecked(bytes)
                                };
                            ));
                        }
                        _ => args.push(quote!(
                            #arg_name: #type_path
                        )),
                    }
                }
            }
        }
    }

    Ok(quote!(
        #[no_mangle]
        pub extern "C" fn #name ( #( #args ),* ) -> i32 {
            unsafe {
                #( #args_build )*
                #block
            }

            0
        }
    ))
}
