use we_contract_sdk::*;

// Smart contract used in WASMServiceSpec.scala

// Declaring an interface to interact with another contract.
// The types used, for specifying the type of arguments,
// are also available for use within the contract.
#[interface]
trait i_contract {
    fn function(
        integer: Integer,
        boolean: Boolean,
        binary: Binary,
        string: String
    );
    fn close();
}

// Declaring a function available for calling.
// `#[action]` keyword is used for this purpose.
// The called function always returns 0 inside itself.
// All available functions have an error handler which,
// in case of an error, exits the function with an error code.
#[action]
fn _constructor() {
    // Converting a string address to a byte address.
    
    set_storage!(integer => ("counter", 0));
}

#[action]
fn increment_1() {
    let counter = get_storage!(integer "counter");
    set_storage!(integer => ("counter", counter + 1));
}

#[action]
fn increment(
    step: Integer
) {
    let counter = get_storage!(integer "counter");
    set_storage!(integer => ("counter", counter + step));
}

