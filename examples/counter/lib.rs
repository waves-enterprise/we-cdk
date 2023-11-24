#![no_std]
#![no_main]
use we_contract_sdk::*;

#[action]
fn _constructor(int_value: Integer) {
    set_storage!(integer :: "count" => int_value);
    set_storage!(binary :: "owner" => get_tx_sender!());
}

#[action]
fn counter(step: Integer) {
    let count: Integer = get_storage!(integer :: "count");
    set_storage!(integer :: "count" => count + step);
}

#[action]
fn restore_counter(new_count: Integer) {
    let owner: Binary = get_storage!(binary :: "owner");
    require!(equals!(binary :: owner, get_tx_sender!()));
    set_storage!(integer :: "count" => new_count);
}
