use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub node_url: String,
    pub api_key: String,
    pub transaction: ContractTransaction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTransaction {
    #[serde(rename = "type")]
    type_id: u64,
    version: u64,
    sender: String,
    password: Option<String>,
    stored_contract: Option<StoredContractWasm>,
    #[serde(flatten)]
    data: TransactionData,
    fee: u64,
    fee_asset_id: Option<String>,
    validation_policy: ValidationPolicy,
    group_participants: Vec<String>,
    group_owners: Vec<String>,
}

impl ContractTransaction {
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize struct to JSON")
    }

    pub fn set_stored_contract(&mut self, stored_contract: StoredContractWasm) {
        self.stored_contract = Some(stored_contract);
    }
}

impl fmt::Display for ContractTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_json())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredContractWasm {
    pub bytecode: String,
    pub bytecode_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TransactionData {
    #[serde(rename_all = "camelCase")]
    CreateContract {
        contract_name: String,
        params: Vec<DataEntry>,
        payments: Vec<ContractTransferInV1>,
        is_confidential: bool,
    },
    #[serde(rename_all = "camelCase")]
    UpdateContract { contract_id: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum DataEntry {
    Integer { key: String, value: u64 },
    Boolean { key: String, value: bool },
    Binary { key: String, value: String },
    String { key: String, value: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTransferInV1 {
    asset_id: Option<String>,
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ValidationPolicy {
    Any,
    Majority,
    MajorityWithOneOf { addresses: Vec<String> },
}
