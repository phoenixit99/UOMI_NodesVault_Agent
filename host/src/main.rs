use anyhow::Result;
use std::path::Path;

mod wasm_runtime;
mod api_service;
mod utils;
mod blockchain_service;

fn main() -> Result<()> {
    let input_path = Path::new("./src/input.txt");
    let output_path = Path::new("./src/output.txt");
    
    if !input_path.exists() {
        println!("File ./input.txt does not exist");
        return Ok(());
    }

    let input = std::fs::read(input_path)?;

    //check if plugin.wasm exists
    if !Path::new("./src/agent_template.wasm").exists() {
        println!("File ./agent_template.wasm does not exist");
        return Ok(());
    }
    let wasm = include_bytes!("agent_template.wasm");

    match wasm_runtime::run_wasm(wasm.to_vec(), input) {
        Ok(output) => {
            std::fs::write(output_path, output)?;
        }
        Err(err) => {
            println!("Error executing WASM: {:?}", err);
        }
    }

    Ok(())
}