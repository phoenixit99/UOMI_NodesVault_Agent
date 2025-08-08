use serde::{Deserialize, Serialize};
use utils::log;
use regex::Regex;

mod utils;

#[link(wasm_import_module = "env")]
extern "C" {
    fn get_input_data(ptr: i32, len: i32);
    fn set_output(ptr: i32, len: i32);
    fn console_log(ptr: i32, len: i32);
    fn call_ai(model: i32, ptr: i32, len: i32, output_ptr: i32, output_len: i32);
    fn call_blockchain(ptr: i32, len: i32, output_ptr: i32, output_len: i32) -> i32;
    fn get_cid_file(ptr: i32, len: i32, output_ptr: i32, output_len: i32);
    fn get_input_file(ptr: i32, len: i32);
}

const WALLET_REGEX: &str = r"0x[a-fA-F0-9]{40}";

fn extract_wallet_address(text: &str) -> Option<String> {
    let re = Regex::new(WALLET_REGEX).unwrap();
    re.find(text).map(|m| m.as_str().to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenInfo {
    balance: String,
    contract_address: String,
    decimals: u8,
    name: String,
    symbol: String,
    type_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    items: Vec<TokenInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExplorerResponse {
    block_number_balance_updated_at: u64,
    coin_balance: String,
    hash: String,
    is_contract: bool,
    #[serde(default)]
    exchange_rate: Option<f64>,
}

#[no_mangle]
pub extern "C" fn run() {
    log("UOMI Agent initialized");
    
    // Get and parse user input
    let input = utils::read_input();
    let messages = utils::parse_messages(&input);
    
    // Get the last user message
    let last_message = messages.iter()
        .filter(|msg| msg.role == "user")
        .last()
        .map(|msg| msg.content.clone())
        .unwrap_or_default();
    
    // Check if it's a balance request
    if let Some(wallet_address) = extract_wallet_address(&last_message) {
        log(&format!("Processing balance request for wallet: {}", wallet_address));
        
        // Call blockchain service
        let balance_request = serde_json::json!({
            "action": "get_balance",
            "address": wallet_address
        });
        
        let response = utils::call_blockchain_service(&serde_json::to_string(&balance_request).unwrap());
        
        match response {
            Ok(balance_data) => {
                // Parse balance response
                if let Ok(balance) = serde_json::from_slice::<ExplorerResponse>(&balance_data) {
                    // Convert wei to ETH (1 ETH = 10^18 wei)
                    let eth_balance = balance.coin_balance.parse::<f64>().unwrap_or_default() / 1e18;
                    
                    let mut response_parts = vec![
                        format!("Wallet Balance for {}:", balance.hash),
                        format!("\nNative Balance: {:.6} UOMI", eth_balance)
                    ];

                    if let Some(rate) = balance.exchange_rate {
                        let usd_value = eth_balance * rate;
                        response_parts.push(format!("(${:.2} USD)", usd_value));
                    }

                    // Get token balances
                    let token_request = serde_json::json!({
                        "action": "get_tokens",
                        "address": balance.hash
                    });

                    if let Ok(token_data) = utils::call_blockchain_service(&serde_json::to_string(&token_request).unwrap()) {
                        if let Ok(tokens) = serde_json::from_slice::<TokenResponse>(&token_data) {
                            response_parts.push("\nToken Balances:".to_string());
                            
                            // Look for UOMI token first
                            let mut found_uomi = false;
                            for token in &tokens.items {
                                if token.symbol.to_uppercase() == "UOMI" {
                                    found_uomi = true;
                                    let token_balance = token.balance.parse::<f64>().unwrap_or_default() / 10f64.powi(token.decimals as i32);
                                    response_parts.push(format!("🔹 UOMI: {:.2} tokens", token_balance));
                                }
                            }

                            // Show other tokens
                            for token in &tokens.items {
                                if token.symbol.to_uppercase() != "UOMI" {
                                    let token_balance = token.balance.parse::<f64>().unwrap_or_default() / 10f64.powi(token.decimals as i32);
                                    response_parts.push(format!("- {} ({}): {:.2}", token.name, token.symbol, token_balance));
                                }
                            }

                            if !found_uomi {
                                response_parts.push("\nNote: No UOMI tokens found in this wallet".to_string());
                            }
                        }
                    }

                    response_parts.push(format!("\nLast updated at block: {}", balance.block_number_balance_updated_at));
                    
                    if balance.is_contract {
                        response_parts.push("\nNote: This is a smart contract address".to_string());
                    }

                    let response_text = response_parts.join("\n");
                    
                    // Format as JSON response
                    let json_response = serde_json::json!({
                        "response": response_text,
                        "time_taken": 0.5, // Placeholder - could be calculated
                        "tokens_per_second": 50.0, // Placeholder
                        "total_tokens_generated": 100 // Placeholder
                    });
                    
                    let response_json = serde_json::to_string(&json_response).unwrap();
                    utils::save_output(response_json.as_bytes());
                } else {
                    let error_response = serde_json::json!({
                        "response": "Sorry, I couldn't parse the balance data. Please try again later.",
                        "time_taken": 0.1,
                        "tokens_per_second": 10.0,
                        "total_tokens_generated": 20
                    });
                    let error_json = serde_json::to_string(&error_response).unwrap();
                    utils::save_output(error_json.as_bytes());
                }
            }
            Err(e) => {
                log(&format!("Error fetching balance: {:?}", e));
                let error_response = serde_json::json!({
                    "response": "Sorry, I couldn't fetch your balance at the moment. Please try again later.",
                    "time_taken": 0.1,
                    "tokens_per_second": 10.0,
                    "total_tokens_generated": 20
                });
                let error_json = serde_json::to_string(&error_response).unwrap();
                utils::save_output(error_json.as_bytes());
            }
        }
    } else {
        // Handle regular conversation
        let system_message = utils::system_message(
            "You are UOMI Agent, a helpful assistant focused on blockchain and financial services.".to_string()
        );
        let modified_messages = utils::process_messages(system_message, messages);
        let modified_messages_str = format!("{{\"messages\": {}}}", serde_json::to_string(&modified_messages).unwrap());
        
        let request = utils::prepare_request(&modified_messages_str);
        let response = utils::call_ai_service(1, request);
        
        // Ensure AI response is also JSON formatted
        let ai_response_str = String::from_utf8_lossy(&response);
        let json_response = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&ai_response_str) {
            // If AI already returns JSON, use it
            parsed
        } else {
            // If AI returns plain text, wrap it in JSON
            serde_json::json!({
                "response": ai_response_str.trim(),
                "time_taken": 0.3,
                "tokens_per_second": 30.0,
                "total_tokens_generated": 50
            })
        };
        
        let response_json = serde_json::to_string(&json_response).unwrap();
        utils::save_output(response_json.as_bytes());
    }
}

