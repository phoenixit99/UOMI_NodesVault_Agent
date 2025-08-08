cd agent-template 
cargo build --target wasm32-unknown-unknown --release
cd ..

# Copy WASM file with correct filename
cp ./target/wasm32-unknown-unknown/release/uomi_nodes_vault_agent.wasm ./host/src/agent_template.wasm

cd host
cargo run