#![no_std]
#![no_main]
use we_cdk::*;

const PREFIX_KEY: String = "shard_";

#[action]
fn _constructor() {
    for i in 0..10 {
        let key = join!(string :: PREFIX_KEY, to_string_int!(i));
        set_storage!(integer :: key => 0);
    }
}

#[action]
fn increment_1(shard: String) {
    let key = join!(string :: PREFIX_KEY, shard);
    let counter = get_storage!(integer :: key);
    set_storage!(integer :: key => counter + 1);
}
