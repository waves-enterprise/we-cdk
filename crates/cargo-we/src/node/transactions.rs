use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DataEntry {
    DataString {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: String,
    },
    DataInteger {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: u64,
    },
    DataBoolean {
        key: String,
        #[serde(rename = "type")]
        type_: String,
        value: bool,
    },
    DataBinary {
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
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_id: u64,
    pub version: u64,
    pub sender: String,
    pub password: String,
    pub contract_name: String,
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
    pub transaction: Transaction,
}
