#![allow(warnings)]
#[link(wasm_import_module = "env0")]
extern "C" {
    // Asset
    #[no_mangle]
    pub fn get_balance(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_address: *const u8,
        length_address: usize,
    ) -> (i32, i64);

    #[no_mangle]
    pub fn transfer(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_recipient: *const u8,
        length_recipient: usize,
        amount: i64,
    ) -> i32;

    #[no_mangle]
    pub fn issue(
        offset_name: *const u8,
        length_name: usize,
        offset_description: *const u8,
        length_description: usize,
        quantity: i64,
        decimals: i64,
        is_reissuable: bool,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn burn(offset_asset_id: *const u8, length_asset_id: usize, amount: i64) -> i32;

    #[no_mangle]
    pub fn reissue(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        amount: i64,
        is_reissuable: bool,
    ) -> i32;

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
    pub fn call_arg_binary(offset_value: *const u8, length_value: usize) -> i32;

    #[no_mangle]
    pub fn call_arg_string(offset_value: *const u8, length_value: usize) -> i32;

    #[no_mangle]
    pub fn call_payment(offset_asset_id: *const u8, length_asset_id: usize, amount: i64) -> i32;

    #[no_mangle]
    pub fn call_contract(
        offset_contract_id: *const u8,
        length_contract_id: usize,
        offset_func_name: *const u8,
        length_func_name: usize,
    ) -> i32;

    // Lease
    #[no_mangle]
    pub fn lease_address(
        offset_address: *const u8,
        length_address: usize,
        amount: i64,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn lease_alias(
        offset_alias: *const u8,
        length_alias: usize,
        amount: i64,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn cancel_lease(offset_lease_id: *const u8, length_lease_id: usize) -> i32;

    // Payments
    #[no_mangle]
    pub fn get_tx_payments() -> (i32, i64);

    #[no_mangle]
    pub fn get_tx_payment_asset_id(number: i64) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn get_tx_payment_amount(number: i64) -> (i32, i64);

    // Storage
    #[no_mangle]
    pub fn get_storage_int(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, i64);

    #[no_mangle]
    pub fn get_storage_bool(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, bool);

    #[no_mangle]
    pub fn get_storage_binary(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn get_storage_string(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn set_storage_int(offset_key: *const u8, length_key: usize, value: i64) -> i32;

    #[no_mangle]
    pub fn set_storage_bool(offset_key: *const u8, length_key: usize, value: bool) -> i32;

    #[no_mangle]
    pub fn set_storage_binary(
        offset_key: *const u8,
        length_key: usize,
        offset_value: *const u8,
        length_value: usize,
    ) -> i32;

    #[no_mangle]
    pub fn set_storage_string(
        offset_key: *const u8,
        length_key: usize,
        offset_value: *const u8,
        length_value: usize,
    ) -> i32;

    // Tx
    #[no_mangle]
    pub fn get_tx_sender() -> (i32, *const u8, usize);

    // Utils
    #[no_mangle]
    pub fn base_58(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn to_base_58_string(
        offset_bytes: *const u8,
        length_bytes: usize,
    ) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn binary_equals(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, bool);

    #[no_mangle]
    pub fn string_equals(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, bool);

    #[no_mangle]
    pub fn join(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, *const u8, usize);
}
