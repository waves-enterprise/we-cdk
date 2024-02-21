use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DataEntry {
    String {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: String,
    },
    Integer {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: u64,
    },
    Boolean {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: bool,
    },
    Binary {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: Vec<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TxContract {
    CreateContract {
        #[serde(rename = "contractName")]
        contract_name: String,
        params: Vec<DataEntry>,
        payments: Vec<ContractTransferInV1>,
    },
    UpdateContract {
        #[serde(rename = "contractId")]
        contract_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionContract {
    #[serde(rename = "type")]
    pub type_id: u64,
    pub version: u64,
    pub sender: String,
    pub password: String,
    pub stored_contract: Option<StoredContractWasm>,
    #[serde(flatten)]
    pub tx: TxContract,
    pub fee: u64,
    pub fee_asset_id: Option<String>,
    pub validation_policy: ValidationPolicy,
    pub is_confidential: bool,
    pub group_participants: Vec<String>,
    pub group_owners: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredContractWasm {
    pub bytecode: String,
    pub bytecode_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicBadge {
    pub trusted_sender: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationPolicy {
    #[serde(rename = "type")]
    pub type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTransferInV1 {
    asset_id: Option<String>,
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub node_url: String,
    pub api_key: String,
    pub transaction: TransactionContract,
}

impl TransactionContract {
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize struct to JSON")
    }
}

impl fmt::Display for TransactionContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_json())
    }
}
