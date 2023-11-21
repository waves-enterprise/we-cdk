#![no_std]
#![no_main]
use we_contract_sdk::*;

#[action]
fn _constructor() {
    set_storage!(integer :: "counter" => 0);
}

#[action]
fn increment_1() {
    let counter = get_storage!(integer :: "counter");
    set_storage!(integer :: "counter" => counter + 1);
}

#[action]
fn increment(step: Integer) {
    let counter = get_storage!(integer :: "counter");
    set_storage!(integer :: "counter" => counter + step);
}
