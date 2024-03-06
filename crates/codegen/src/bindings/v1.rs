#![allow(warnings)]
#[link(wasm_import_module = "env1")]
extern "C" {
    // Asset
    #[no_mangle]
    pub fn get_balance(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_address: *const u8,
        length_address: usize,
        type_: i32,
        version: i32,
    ) -> (i32, i64);

    #[no_mangle]
    pub fn transfer(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_recipient: *const u8,
        length_recipient: usize,
        type_: i32,
        version: i32,
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

    // Tx
    #[no_mangle]
    pub fn get_tx_payments() -> (i32, i64);

    #[no_mangle]
    pub fn get_tx_payment_asset_id(number: i64) -> (i32, *const u8, usize);

    #[no_mangle]
    pub fn get_tx_payment_amount(number: i64) -> (i32, i64);

    #[no_mangle]
    pub fn tx(offset_field: *const u8, length_field: usize) -> (i32, *const u8, usize);
}
