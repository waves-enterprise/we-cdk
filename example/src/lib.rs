use we_contract_sdk::*;

#[action]
fn _constructor() {
    call_contract("test", "run", "args");
}
