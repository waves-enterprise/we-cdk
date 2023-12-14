#![no_std]
#![no_main]
use we_cdk::*;

#[action]
fn _constructor() {
    let tx_sender = get_tx_sender!();
    set_storage!(binary :: "owner" => tx_sender);
}

#[action]
fn init_vote(address_contract: String, start_vote: Integer, end_vote: Integer) {
    let tx_sender = get_tx_sender!();
    let owner = get_storage!(binary :: "owner");
    require!(equals!(binary :: owner, tx_sender));

    let key = join!(string :: address_contract, "_start_vote");
    set_storage!(integer :: key => start_vote);

    let key = join!(string :: address_contract, "_end_vote");
    set_storage!(integer :: key => end_vote);

    let key = join!(string :: address_contract, "_count_vote");
    set_storage!(integer :: key => 0);
}

#[action]
fn vote(address_contract: String) {
    let tx_sender = get_tx_sender!();
    let string_tx_sender = to_base58_string!(tx_sender);

    let start_vote_key = join!(string :: address_contract, "_start_vote");
    let end_vote_key = join!(string :: address_contract, "_end_vote");
    let count_vote_key = join!(string :: address_contract, "_count_vote");

    require!(
        get_block_timestamp!() > get_storage!(integer :: start_vote_key)
            && get_block_timestamp!() < get_storage!(integer :: end_vote_key)
    );

    let key = join!(string :: string_tx_sender, "_", address_contract);
    match get_storage!(boolean :: key) {
        false => {
            set_storage!(boolean :: key => true);
            let count_vote = get_storage!(integer :: count_vote_key);
            set_storage!(integer :: count_vote_key => count_vote + 1);
        }
        _ => require!(false),
    }
}
