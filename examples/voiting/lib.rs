use we_contract_sdk::*;

use std::str;

#[action]
fn _constructor() {
    set_storage!(binary => ("owner", get_tx_sender!()));
}

#[action]
fn init_vote(address_contract: String, start_vote: Integer, end_vote: Integer) {
    require!(get_storage!(binary "owner") == get_tx_sender!());
    
    set_storage!(string => ("address_contract_voting", address_contract));
    set_storage!(integer => (format!("{}_{}", address_contract, "start_vote"), start_vote));
    set_storage!(integer => (format!("{}_{}", address_contract, "end_vote"), end_vote));
    set_storage!(integer => (format!("{}_{}", address_contract, "count_vote"), 0));
}

#[action]
fn vote(address_contract: String, address_voters: String) {
    let address_contract_voting = get_storage!(string "address_contract_voting");
    require!(address_contract_voting == address_contract);
    require!(base58!(address_voters) == get_tx_sender!());

    require!(get_block_timestamp!() > get_storage!(integer "start_vote") 
            && get_block_timestamp!() < get_storage!(integer "end_vote"));
    match get_storage!(boolean address_voters) {
        false => {
            set_storage!(boolean => (address_voters, true));
            let count = get_storage!(integer "count_vote");
            set_storage!(integer => ("count_vote", count + 1));
        }
        _ => {require!(false);}
    }
}

#[action]
fn result() {
    require!(get_storage!(binary "owner") == get_tx_sender!());
    require!( get_block_timestamp!() > get_storage!(integer "end_vote"));
    let count_vote = get_storage!(integer "count_vote");
    set_storage!(integer => ("result", count_vote));
}

//coming soon
#[action]
fn info_contract_vote(address_contract: String) {
    require!(get_storage!(string "address_contract_voting") == address_contract);


}

            