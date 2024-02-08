pub mod transactions;

use reqwest::*;
use transactions::*;

pub struct Node {
    url: String,
}

impl Node {
    pub fn from_url(url: String) -> Self {
        Node { url }
    }

    pub async fn transactionSignAndBroadcast(
        &self,
        api_key: String,
        tx: TransactionCreateWasm,
    ) -> Result<(), Error> {
        let url = format!("{}/transactions/signAndBroadcast", self.url);
        let client = reqwest::Client::new();

        let response = client
            .post(url)
            .header("X-API-Key", api_key)
            .body(&tx)
            .send()
            .await?;
        Ok(())
    }
}
