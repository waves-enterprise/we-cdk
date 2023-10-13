use we_contract_sdk::*;

#[action]
fn _constructor(
    bool_value: Boolean,
    int_value: Integer,
    string_value: String,
    address_value: String,
) {
    // Set storage on deploy
    set_storage!(boolean => ("bool_value", bool_value));
    set_storage!(integer => ("int_value", int_value));
    set_storage!(string => ("string_value", string_value));
    set_storage!(binary => ("address_value", base58!(address_value)));
    set_storage!(binary => ("owner", get_tx_sender!()));
}

#[action]
fn update_storage(
    bool_value: Boolean,
    int_value: Integer,
    string_value: String,
    address_value: String,
) {
    // Check owner
    let owner: Binary = get_storage!(binary "owner");
    require!(owner == get_tx_sender!());

    // Update storage
    set_storage!(boolean => ("bool_value", bool_value));
    set_storage!(integer => ("int_value", int_value));
    set_storage!(string => ("string_value", string_value));
    set_storage!(binary => ("address_value", base58!(address_value)));
}
