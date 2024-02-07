use serde::Deserialize;

macro_rules! transaction_generator {
    (struct $name:ident {
        $(
            $( #[$attr:meta] )?
            $field_name:ident: $field_type:ty,
        )*
    }) => {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(
                $( #[$attr] )?
                $field_name: $field_type,
            )*
        }

        impl $name {
            $(
                pub fn $field_name(&self) -> $field_type {
                    self.$field_name.clone()
                }
            )*
        }
    }
}
struct StoredContractWasm {
    bytecode: String,
    bytecodeHash: String,
}

struct ParamsTransaction {
    key: String,
    #[serde(alias = "type")]
    type_id: String,
    value: String,
}

struct AtomicBadge {
    trustedSender: String,
}

struct ValidationPolicy {
    #[serde(alias = "type")]
    type_id: String,
}

transaction_generator! {
    struct TransactionCreateContractWasm {
        #[serde(alias = "type")]
        type_id: u64,
        version: u64,
        sender: &'a str,
        password: &'a str,
        contractName: &'a str,
        storedContract: StoredContractWasm,
        params: Vec<ParamsTransaction>,
        fee: u64,
        timestamp: u64,
        feeAssetId: &'a str,
        atomicBadge: AtomicBadge,
        proofs: Vec<&'a str>,
        validationPolicy: ValidationPolicy,
        apiVersion: &'a str
    }
}
