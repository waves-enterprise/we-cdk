pub mod transactions;

use reqwest::Error;
use transactions::*;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Node {
    url: String,
    api_key: String,
}

impl Node {
    pub fn new(url: String, api_key: String) -> Self {
        Node { url, api_key }
    }

    pub async fn transaction_sign_and_broadcast(&self, tx: Transaction) -> Result<()> {
        let url = format!("{}/transactions/signAndBroadcast", self.url);
        let client = reqwest::Client::new();
        let json_temp = serde_json::to_string(&tx).expect("Failed to serialize json");
        let response = client
            .post(url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-API-Key", &self.api_key)
            .body(json_temp)
            .send()
            .await?;

        println!("Status: {}", response.status());

        let response_body = response.text().await?;

        println!("Response body:\n{}", response_body);

        Ok(())
    }
}
