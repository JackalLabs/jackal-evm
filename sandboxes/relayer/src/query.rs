use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use anyhow::Result; // Import anyhow for error handling

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PubKey {
    #[serde(rename = "@type")]
    pub key_type: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Account {
    #[serde(rename = "@type")]
    pub account_type: String,
    pub address: String,
    pub pub_key: PubKey,
    pub account_number: String,
    pub sequence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccountResponse {
    pub account: Account,
}

pub(crate) async fn query_account(client: &Client, url: &str) -> Result<AccountResponse> {
    // Send the GET request
    let response = client.get(url).send().await?;

    // Check if the response was successful
    if response.status().is_success() {
        // Parse the JSON response body
        let account_response: AccountResponse = response.json().await?;
        Ok(account_response)
    } else {
        Err(anyhow::anyhow!("Failed to query API: {}", response.status()))
    }
}

