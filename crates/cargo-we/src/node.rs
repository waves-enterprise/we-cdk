pub mod transactions;

use transactions::ContractTransaction;

pub struct Node {
    url: String,
    api_key: String,
}

impl Node {
    pub fn new(url: String, api_key: String) -> Self {
        Node { url, api_key }
    }

    pub async fn transaction_sign_and_broadcast(
        &self,
        tx: ContractTransaction,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/transactions/signAndBroadcast", self.url);
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-API-Key", &self.api_key)
            .body(tx.as_json())
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        println!("Response body:\n{}", response_body);
        if status == 200 {
            println!("Successful send transaction");
        } else {
            println!("Error send transaction. HTTP Response status: {}", status);
        }

        Ok(())
    }
}
