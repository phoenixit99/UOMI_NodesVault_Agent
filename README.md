# 🤖 UOMI Network AI Agent

<div align="center">

*An intelligent agent for interacting with the UOMI Network and managing UOMI tokens* 🦀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-%23654FF0.svg?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![UOMI](https://img.shields.io/badge/UOMI-Network-blue?style=for-the-badge)](https://uomi.network)
</div>

## 🌟 Overview

The UOMI Network AI Agent is a specialized assistant that helps users interact with the UOMI blockchain network. Built with WebAssembly and Rust, it provides real-time access to:

- 💰 Wallet Balance Information
- 📊 UOMI Token Analytics
- 🔄 Transaction History
- 📈 Network Statistics

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Node.js dependencies
npm install
```

### Running the Agent
```bash
# Start the local CORS proxy
npm install -g local-cors-proxy
lcp --proxyUrl https://explorer.uomi.ai

# Build and run the agent
sh ./bin/build_and_run_host.sh
```

## 💡 Features

### 1. Wallet Information
- Check UOMI token balances
- View native chain balance
- Monitor wallet activity

### 2. Network Integration
- Real-time blockchain data
- UOMI Explorer integration
- Secure API communication

### 3. User Interaction
- Natural language processing
- Command recognition
- Helpful responses

## 🔧 Usage Examples

### Check Wallet Balance
```bash
# Input
Check balance: 0xad749097119d27495987dA69F9D8E19366E2f287

# Output
Wallet Balance for 0xad749097119d27495987dA69F9D8E19366E2f287:
Native Balance: 10.000000 UOMI
Last updated at block: 5620584
```

### Get UOMI Information
```bash
# Input
What is UOMI?

# Output
[Agent provides information about UOMI Network and tokenomics]
```

## 🛠 Development

### Project Structure
```
UOMI_NodesVault_Agent/
├── agent-template/       # WASM Agent code
│   ├── src/
│   │   ├── lib.rs       # Main agent logic
│   │   └── utils.rs     # Utility functions
│   └── Cargo.toml       # Rust dependencies
├── host/                # Host runtime
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── blockchain_service.rs  # UOMI API integration
│   │   └── wasm_runtime.rs  # WASM execution
│   └── Cargo.toml       # Host dependencies
└── bin/                 # Build scripts
```

### Key Components

1. **Agent Template**
   - Natural language processing
   - Command recognition
   - Response formatting

2. **Blockchain Service**
   - UOMI Explorer integration
   - Balance checking
   - Transaction querying

3. **WASM Runtime**
   - Memory management
   - Host function calls
   - Error handling

## 🔧 Configuration

The agent uses a local CORS proxy to communicate with the UOMI Explorer API:
```json
{
  "api": {
    "timeout_ms": 10000,
    "retry_attempts": 3
  }
}
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📝 License

MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built with ❤️ for the UOMI Network

[Website](https://uomi.network) · [Documentation](https://docs.uomi.network) 

</div>
