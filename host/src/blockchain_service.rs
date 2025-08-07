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
    pub hash: String,
    pub is_contract: bool,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
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
        .danger_accept_invalid_certs(true)
        .use_native_tls()
        .build()?;

    // Use UOMI Explorer API endpoint
    let api_url = format!("http://localhost:8010/proxy/api/v2/addresses/{}", address);
    
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    
    // Add UOMI API key if you have one
    if let Ok(api_key) = std::env::var("UOMI_API_KEY") {
        headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());
    }
    
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
