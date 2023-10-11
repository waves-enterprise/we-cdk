use we_contract_sdk::*;

// Declaring a function available for calling.
// `#[action]` keyword is used for this purpose.
// The called function always returns 0 inside itself.
// All available functions have an error handler which,
// in case of an error, exits the function with an error code.
#[action]
fn _constructor() {
    // Converting a string address to a byte address.
    let address = base58!("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU");
    let balance = get_balance!(this);

    // Verify inputs and conditions.
    // In this case, a balance check is performed.
    require!(balance > 42);
    // Transfer of funds to the `address`.
    transfer!(address, 42);
}
