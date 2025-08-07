pub fn generate_wasm_data(data: Vec<u8>) -> Vec<u8> {
    let data_len = data.len();
    let mut wasm_data = Vec::with_capacity(data_len + 4);
    
    wasm_data.extend(&(data_len as u32).to_le_bytes());
    wasm_data.extend(data);
    
    wasm_data
}