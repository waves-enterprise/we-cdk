use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug)]
struct Arg {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Func {
    name: String,
    args: Vec<Arg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    name: String,
    abi: Vec<Func>,
}

impl Metadata {
    pub fn new(name: &str) -> Self {
        let mut file = File::open("lib.rs").expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

        let ast = syn::parse_file(&src).expect("Unable to parse file");
        let abi = Self::parse_ast(ast);

        Self {
            name: name.to_string(),
            abi,
        }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize struct to JSON")
    }

    fn parse_ast(ast: syn::File) -> Vec<Func> {
        let mut result: Vec<Func> = vec![];

        for item in ast.items {
            // You only need to get the functions
            if let syn::Item::Fn(func) = item {
                for attr in func.attrs {
                    // It is necessary to get functions with the attribute `action`
                    if Self::is_ident(&attr.path, "action") {
                        // Get the name of the function
                        let func_name = func.sig.ident.to_string();

                        let mut args: Vec<Arg> = vec![];

                        // Going through all the arguments
                        for arg in func.sig.inputs.iter() {
                            if let syn::FnArg::Typed(a) = arg {
                                if let syn::Pat::Ident(pat_ident) = &*a.pat {
                                    // Get the name of the argument
                                    let arg_name = pat_ident.ident.to_string();

                                    if let syn::Type::Path(type_path) = &*a.ty {
                                        args.push(Arg {
                                            name: arg_name,
                                            ty: Self::get_ty(&type_path.path)
                                                .expect("Unable to get ty")
                                                .to_string(),
                                        })
                                    }
                                }
                            }
                        }
                        // Add a function
                        result.push(Func {
                            name: func_name,
                            args,
                        });
                    }
                }
            }
        }

        result
    }

    fn get_ty(path: &syn::Path) -> Option<&syn::Ident> {
        if let Some(ident) = Self::get_ident(path) {
            Some(ident)
        } else {
            path.get_ident()
        }
    }

    fn is_ident(path: &syn::Path, ident: &str) -> bool {
        match Self::get_ident(path) {
            Some(id) => id == ident,
            None => path.is_ident(ident),
        }
    }

    fn get_ident(path: &syn::Path) -> Option<&syn::Ident> {
        if path.segments.len() == 2 && path.segments[0].ident == "we_cdk" {
            Some(&path.segments[1].ident)
        } else {
            None
        }
    }
}
