use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredContractWasm {
    pub bytecode: String,
    pub bytecode_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamsTransaction {
    pub key: String,
    #[serde(rename = "type")]
    pub type_id: String,
    pub value: u64,
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
pub struct TransactionContractWasm {
    #[serde(rename = "type")]
    pub type_id: Option<u64>,
    pub version: Option<u64>,
    pub sender: String,
    pub password: String,
    pub contract_name: String,
    pub stored_contract: Option<StoredContractWasm>,
    pub params: Option<Vec<ParamsTransaction>>,
    pub payments: Option<Vec<ContractTransferInV1>>,
    pub fee: Option<u64>,
    pub fee_asset_id: Option<String>,
    pub validation_policy: ValidationPolicy,
    pub is_confidential: bool,
    pub group_participants: Vec<String>,
    pub group_owners: Vec<String>,
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
    pub transaction: TransactionContractWasm,
}
// sender: String,
// contractName: String,
// storedContract: StoredContract,
// params: List[DataEntry[_]],
// payments: List[ContractTransferInV1],
// fee: Long,
// feeAssetId: Option[String],
// timestamp: Option[Long] = None,
// atomicBadge: Option[AtomicBadge],
// validationPolicy: ValidationPolicy,
// password: Option[String] = None,
// isConfidential: Boolean,
// groupParticipants: List[String],
// groupOwners: List[String])
