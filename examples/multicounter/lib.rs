#![no_std]
#![no_main]
use we_contract_sdk::*;

const PREFIX_KEY: String = "shard_";
const NUMBERS: String = "0123456789";

#[action]
fn _constructor() {
    for i in 0..10 {
        let x = NUMBERS.get_unchecked(i..i + 1);
        let key = join!(string :: PREFIX_KEY, &x);
        set_storage!(integer :: key => 0);
    }
}

#[action]
fn increment_1(shard: String) {
    let key = join!(string :: PREFIX_KEY, shard);
    let counter = get_storage!(integer :: key);
    set_storage!(integer :: key => counter + 1);
}
