#![no_std]
#![no_main]
use we_contract_sdk::*;

// Declaring an interface to interact with another contract.
// The types used, for specifying the type of arguments,
// are also available for use within the contract.
#[interface]
trait i_contract {
    fn data_fn(integer: Integer, boolean: Boolean, binary: Binary, string: String);
    fn payment_fn();
}

// Declaring a function available for calling.
// `#[action]` keyword is used for this purpose.
// The called function always returns 0 inside itself.
// All available functions have an error handler which,
// in case of an error, exits the function with an error code.
#[action]
fn _constructor() {
    // Converting a string address to a byte address.
    let contract = base58!("4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2");
    let asset = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");

    // Values that will serve as arguments for the function call.
    // The types used are aliases over Rust types.
    // These types are used in interface declaration.
    let integer: Integer = 42;
    let boolean: Boolean = true;
    let binary: Binary = &[0, 1];
    let string: String = "test";

    // Calling a contract.
    // In this case, the `i_contract` interface and the address of the contract to be called are used.
    // And accordingly the function that needs to be called.
    call_contract! {
        i_contract(contract)::data_fn(integer, boolean, binary, string)
    };

    let payment_system_token: Payment = (SYSTEM_TOKEN, 4200000000);
    let payment: Payment = (asset, 2400000000);

    call_contract! {
        i_contract(contract)::payment_fn()::payments(payment_system_token, payment)
    }
}
