pub use we_contracts_proc_macro::*;

#[link(wasm_import_module = "env")]
extern "C" {
    #[no_mangle]
    pub fn call_contract(contract_id: &str, func_name: &str, func_args: &str);
}
