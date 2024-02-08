use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredContractWasm {
    pub bytecode: String,
    pub bytecode_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ParamsTransaction {
    pub key: String,
    #[serde(alias = "type")]
    pub type_id: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AtomicBadge {
    pub trustedSender: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValidationPolicy {
    #[serde(alias = "type")]
    pub type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionContractWasm {
    #[serde(alias = "type")]
    pub type_id: u64,
    pub version: u64,
    pub sender: String,
    pub password: String,
    pub contract_name: String,
    pub stored_contract: StoredContractWasm,
    pub params: Option(Vec<ParamsTransaction>),
    pub fee: u64,
    pub timestamp: u64,
    pub fee_asset_id: String,
    pub atomic_badge: AtomicBadge,
    pub validation_policy: ValidationPolicy,
    pub api_version: String,
}
