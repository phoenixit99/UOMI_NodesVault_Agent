use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),
    #[error("API error: {status}, {message}")]
    ApiError { status: u16, message: String },
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerResponse {
    pub block_number_balance_updated_at: u64,
    pub coin_balance: String,
    pub creation_transaction_hash: Option<String>,
    pub creator_address_hash: Option<String>,
    pub ens_domain_name: Option<String>,
    pub exchange_rate: Option<f64>,
    pub has_beacon_chain_withdrawals: bool,
    pub has_logs: bool,
    pub has_token_transfers: bool,
    pub has_tokens: bool,
    pub has_validated_blocks: bool,
    pub hash: String,
    pub implementations: Vec<String>,
    pub is_contract: bool,
    pub is_scam: bool,
    pub is_verified: bool,
    pub metadata: Option<serde_json::Value>,
    pub name: Option<String>,
    pub private_tags: Vec<String>,
    pub proxy_type: Option<String>,
    pub public_tags: Vec<String>,
    pub token: Option<serde_json::Value>,
    pub watchlist_address_id: Option<String>,
    pub watchlist_names: Vec<String>,
}



pub async fn get_wallet_balance(address: &str) -> Result<ExplorerResponse, BlockchainError> {
    // Validate address format
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(BlockchainError::InvalidAddress(
            "Invalid Ethereum address format".to_string(),
        ));
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // Use UOMI Explorer API endpoint
    let api_url = format!("https://explorer.uomi.ai/api/v2/addresses/{}", address);
    
    let mut headers = HeaderMap::new();
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("user-agent", "uomi-agent/1.0".parse().unwrap());
    
    println!("Calling UOMI API: {}", api_url);
    
    let response = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        println!("API Error: {} - {}", status, error_text);
        return Err(BlockchainError::ApiError {
            status: status.as_u16(),
            message: error_text,
        });
    }

    let response_text = response.text().await?;
    println!("API Response: {}", response_text);

    match serde_json::from_str::<ExplorerResponse>(&response_text) {
        Ok(response) => Ok(response),
        Err(e) => {
            println!("Failed to parse response: {}", e);
            Err(BlockchainError::Json(e))
        }
    }
}
