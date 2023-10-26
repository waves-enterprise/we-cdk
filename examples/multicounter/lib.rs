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
fn _constructor(num_shards: Integer) {
    // Converting a string address to a byte address.
    set_storage!(integer => ("shard_0", 0));
    set_storage!(integer => ("shard_1", 0));
    set_storage!(integer => ("shard_2", 0));
    set_storage!(integer => ("shard_3", 0));
    set_storage!(integer => ("shard_4", 0));
    set_storage!(integer => ("shard_5", 0));
    set_storage!(integer => ("shard_6", 0));
    set_storage!(integer => ("shard_7", 0));
    set_storage!(integer => ("shard_8", 0));
    set_storage!(integer => ("shard_9", 0));

}

#[action]
fn increment_1(shard: Integer) {
    match shard {
        0 => {
            let counter = get_storage!(integer "shard_0");
            set_storage!(integer => ("shard_0", counter + 1));
        }
        1 => {
            let counter = get_storage!(integer "shard_1");
            set_storage!(integer => ("shard_1", counter + 1));
        }
        2 => {
            let counter = get_storage!(integer "shard_2");
            set_storage!(integer => ("shard_2", counter + 1));
        }
        3 => {
            let counter = get_storage!(integer "shard_3");
            set_storage!(integer => ("shard_3", counter + 1));
        }
        4 => {
            let counter = get_storage!(integer "shard_4");
            set_storage!(integer => ("shard_4", counter + 1));
        }
        5 => {
            let counter = get_storage!(integer "shard_5");
            set_storage!(integer => ("shard_5", counter + 1));
        }
        6 => {
            let counter = get_storage!(integer "shard_6");
            set_storage!(integer => ("shard_6", counter + 1));
        }
        7 => {
            let counter = get_storage!(integer "shard_7");
            set_storage!(integer => ("shard_7", counter + 1));
        }
        8 => {
            let counter = get_storage!(integer "shard_8");
            set_storage!(integer => ("shard_8", counter + 1));
        }
        9 => {
            let counter = get_storage!(integer "shard_9");
            set_storage!(integer => ("shard_9", counter + 1));
        }
        _ => {}
    }
}
