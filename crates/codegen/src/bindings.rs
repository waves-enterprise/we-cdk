#![allow(warnings)]
#[link(wasm_import_module = "env0")]
extern "C" {
    // Asset
    #[no_mangle]
    pub fn get_balance(asset_id: &[u8], address: &[u8]) -> (i32, i64);

    #[no_mangle]
    pub fn transfer(asset_id: &[u8], recipient: &[u8], amount: i64) -> i32;

    #[no_mangle]
    pub fn issue<'a>(
        name: &'a str,
        description: &'a str,
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> (i32, &'a [u8]);

    #[no_mangle]
    pub fn burn(asset_id: &[u8], amount: i64) -> i32;

    #[no_mangle]
    pub fn reissue(asset_id: &[u8], amount: i64, is_reissuable: bool) -> i32;

    // Block
    #[no_mangle]
    pub fn get_block_timestamp() -> (i32, i64);

    #[no_mangle]
    pub fn get_block_height() -> (i32, i64);

    // Call contract
    #[no_mangle]
    pub fn call_arg_int(value: i64);

    #[no_mangle]
    pub fn call_arg_bool(value: bool);

    #[no_mangle]
    pub fn call_arg_binary(value: &[u8]) -> i32;

    #[no_mangle]
    pub fn call_arg_string(value: &str) -> i32;

    #[no_mangle]
    pub fn call_payment(asset_id: &[u8], amount: i64) -> i32;

    #[no_mangle]
    pub fn call_contract(contract_id: &[u8], func_name: &str) -> i32;

    // Lease
    #[no_mangle]
    pub fn lease<'a>(recipient: &'a [u8], amount: i64) -> (i32, &'a [u8]);

    #[no_mangle]
    pub fn cancel_lease(lease_id: &[u8]) -> i32;

    // Payments
    #[no_mangle]
    pub fn get_tx_payments() -> (i32, i32);

    #[no_mangle]
    pub fn get_tx_payment_asset_id<'a>(number: i32) -> (i32, &'a [u8]);

    #[no_mangle]
    pub fn get_tx_payment_amount(number: i32) -> (i32, i64);

    // Storage
    #[no_mangle]
    pub fn get_storage_int(address: &[u8], key: &str) -> (i32, i64);

    #[no_mangle]
    pub fn get_storage_bool(address: &[u8], key: &str) -> (i32, bool);

    #[no_mangle]
    pub fn get_storage_binary<'a>(address: &'a [u8], key: &'a str) -> (i32, &'a [u8]);

    #[no_mangle]
    pub fn get_storage_string<'a>(address: &'a [u8], key: &'a str) -> (i32, &'a str);

    #[no_mangle]
    pub fn set_storage_int(key: &str, value: i64) -> i32;

    #[no_mangle]
    pub fn set_storage_bool(key: &str, value: bool) -> i32;

    #[no_mangle]
    pub fn set_storage_binary(key: &str, value: &[u8]) -> i32;

    #[no_mangle]
    pub fn set_storage_string(key: &str, value: &str) -> i32;

    // Tx
    #[no_mangle]
    pub fn get_tx_sender<'a>() -> (i32, &'a [u8]);

    // Utils
    #[no_mangle]
    pub fn base_58<'a>(value: &'a str) -> (i32, &'a [u8]);
}
