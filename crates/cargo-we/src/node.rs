mod transactions;

use serde::{Deserialize, Serialize};

use reqwest::*;
use response::*;

pub struct Node<'a> {
    url: &'a str,
}

impl<'a> Node<'a> {
    pub fn from_url(url: &'a str) -> Self {
        Node { url }
    }

    pub async fn transactionSignAndBroadcast(
        &self,
        api_key: &'a str,
        tx: TransactionCreateContractWasm,
    ) -> Result<(), Error> {
        let url = format!("{}/transactions/signAndBroadcast", self.url);
        let client = reqwest::Client::new();
        let json_data = serde_json::to_string(&tx)?;

        let response = client
            .post(url)
            .header("X-API-Key", api_key)
            .body(json_data)
            .send()
            .await?;
        Ok(())
    }
}
