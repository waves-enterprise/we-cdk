use we_contract_sdk::*;

#[action]
fn _constructor() {
    let address = base58!("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU");
    let balance = get_balance!(own);
    require!(balance > 0);
    transfer!(address, 42);
}
