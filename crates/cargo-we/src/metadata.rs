use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug)]
struct Arg {
    name: String,
    ty: String, // TODO: rename name field
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
        serde_json::to_string(self).expect("Unable to ser struct")
    }

    fn parse_ast(ast: syn::File) -> Vec<Func> {
        let mut result: Vec<Func> = vec![];

        for item in ast.items {
            if let syn::Item::Fn(func) = item {
                // TODO: action
                if func.attrs.len() > 0 {
                    let func_name = func.sig.ident.to_string();

                    let mut args: Vec<Arg> = vec![];

                    for arg in func.sig.inputs.iter() {
                        if let syn::FnArg::Typed(a) = arg {
                            if let syn::Pat::Ident(pat_ident) = &*a.pat {
                                let arg_name = pat_ident.ident.to_string();

                                if let syn::Type::Path(type_path) = &*a.ty {
                                    let path_seg = &type_path.path.segments[0];
                                    let ty = path_seg.ident.to_string();

                                    args.push(Arg { name: arg_name, ty });
                                }
                            }
                        }
                    }

                    result.push(Func {
                        name: func_name,
                        args,
                    });
                }
            }
        }

        result
    }
}
