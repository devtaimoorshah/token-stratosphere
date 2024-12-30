# Token-Stratosphere Deployment Guide

This README provides a comprehensive guide to deploying your contract on the DuKong Testnet using TypeScript. You will deploy a CosmWasm contract written in Rust and interact with it via a TypeScript deployment script. The deployment process works across Windows, Linux, and MacOS. Follow this guide to deploy your contract and verify it on the MANTRA Chain Explorer.

## Prerequisites

Before proceeding, ensure you have the following prerequisites:

- **Rust** installed for compiling CosmWasm contracts.
- **Node.js** (with npm) for running deployment scripts.
- **Docker** to use the CosmWasm optimizer.
- **OM test tokens** in your wallet to cover gas fees and contract instantiation costs.
- A configured **mnemonic** for your wallet and RPC endpoint for DuKong Testnet.

## Setting Up Your Development Environment

### Install Rust

1. Open your terminal and run the following command to install Rust:
    ```bash
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
2. Follow the prompts during installation. Choose the default installation option to install Rust on your system.

### Set Rust to Stable Version

Run the following command to set Rust to the stable version:
```bash
$ rustup default stable
```

### Verify Cargo Installation

Cargo is the Rust package manager, which should be installed by default. Verify its installation:
```bash
$ cargo --version
```

### Add WebAssembly Target

CosmWasm contracts need to be compiled into WebAssembly (WASM) format. Add the WASM target to Rust:
```bash
$ rustup target add wasm32-unknown-unknown
```

### Install cargo-generate

`cargo-generate` is used to generate new Rust projects from templates. Install it using:
```bash
$ cargo install cargo-generate --features vendored-openssl
```

## Boilerplate Setup

We will use a boilerplate project to manage our contract.

### Clone the Repository

Clone the repository containing the boilerplate:
```bash
git clone https://github.com/devtaimoorshah/token-stratosphere.git
```

### Check Dependencies

Open the `Cargo.toml` file in the root directory of your project and ensure the following dependencies are added:
```toml
[dependencies]
cw20-base = { version = "0.13.2", features = ["library"] }
cw20 = "2.0.0"
cw-utils = "0.12.1"
cosmwasm-std = "1.0.0"
cosmwasm-storage = "1.1.1"
cw-storage-plus = "0.15.0"
cw2 = "0.15.0"
schemars = "0.8.10"
serde = { version = "1.0.144", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.31" }
cosmwasm-schema = "1.1.4"
prost = "0.12"
```

## Build and Deploy the Contract

### Build the Contract

1. Open the terminal in your project directory and run the following Docker command to optimize your contract:
    ```bash
    docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.16.0
    ```
   This will generate necessary artifacts including the `.wasm` file.

### Deployment Script Setup

1. Create a `deployer` folder to manage the deployment scripts:
    ```bash
    mkdir deployer
    ```

2. Navigate to the `deployer` folder and initialize a new Node.js project:
    ```bash
    cd deployer
    npm init -y
    ```

3. Install the required dependencies:
    ```bash
    npm install typescript --save-dev
    npx tsc --init
    npm install @cosmjs/cosmwasm-stargate
    npm install @cosmjs/proto-signing @cosmjs/stargate dotenv fs
    ```

4. Ensure the `package.json` in the `deployer` folder includes the following dependencies:
    ```json
    "dependencies": {
        "@cosmjs/cosmwasm-stargate": "^0.32.4",
        "@cosmjs/proto-signing": "^0.32.4",
        "@cosmjs/stargate": "^0.32.4",
        "dotenv": "^16.4.5",
        "fs": "^0.0.1-security"
    }
    ```

5. Manually copy the `.wasm` file from your artifacts folder into the `deployer` folder.

### Create the Deployment Script

Create a new `index.ts` file inside the `deployer` folder with the following content:

```typescript
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { SigningCosmWasmClient, CosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const { coins, GasPrice } = require("@cosmjs/stargate");
const fs = require("fs");
require("dotenv").config();

const mnemonic = process.env.MNEMONIC; // Replace with your mnemonic
const rpcEndpoint = "https://rpc.dukong.mantrachain.io"; // RPC endpoint for your chain
const contractWasmPath = "./stratocoin.wasm"; // Path to your Stratocoin contract(wasm file)

async function deploy() {
  try {
    // Step 1: Set up wallet and client
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
      prefix: "mantra", // Replace with the correct prefix for your chain
    });
    const [account] = await wallet.getAccounts();
    console.log(`Wallet address: ${account.address}`);

    // Step 2: Connect to the blockchain with the correct gasPrice denom
    const client = await SigningCosmWasmClient.connectWithSigner(
      rpcEndpoint,
      wallet,
      { gasPrice: GasPrice.fromString("0.01uom") } // Correct denom here: uom
    );
    console.log("Connected to blockchain");

    // Step 3: Upload contract
    const wasmCode = fs.readFileSync(contractWasmPath); // Using the path to stratocoin.wasm
    const uploadReceipt = await client.upload(
      account.address,
      wasmCode,
      "auto", // Automatically manage gas limit
      "Upload Stratocoin Contract"
    );
    const codeId = uploadReceipt.codeId;
    console.log(`Contract uploaded with Code ID: ${codeId}`);

    // Step 4: Instantiate contract with proper feeAmount
    const initMsg = {
      token_name: "StratoCoin",
      symbol: "STRAT",
      denom: "ustrat",
      decimals: 6,
      total_supply: "1000000000000000",
      initial_circulating_supply: "100000000000000",
      distribution: {
        validators: "400000000000000",
        community_reserve: "300000000000000",
        staking_rewards: "200000000000000",
        liquidity_provision: "50000000000000",
        foundation: "50000000000000",
      },
      inflation: {
        rate: 0.14, // Change from "14%" to 0.14
        decay_rate: 0.5,
        decay_time: "365 days",
        min_inflation: 0.07, // Change from "7%" to 0.07
        capped: true,
      },
      governance: {
        staking_requirement: "1000000 ustrat",
        voting_power: "1 token = 1 vote",
        quorum: 0.334, // Change from "33.4%" to 0.334
        threshold: 0.5, // Change from "50%" to 0.5
      },
      staking: {
        unbonding_time: "14 days",
        max_validators: 150,
        reward_distribution: "pro_rata",
      },
      liquidity_pools: [
        { pair: "STRAT/OSMO", pool_type: "default" },
        { pair: "STRAT/ATOM", pool_type: "default" },
      ],
      ibc_channels: [
        { target_chain: "osmosis-1", transfer_enabled: true },
      ],
    };

    // Explicitly set feeAmount as an array of coin objects
    const feeAmount = [{ denom: "uom", amount: "5000" }];  // Corrected feeAmount format

    const instantiateReceipt = await client.instantiate(
      account.address,
      codeId,
      initMsg,
      "My Stratocoin Contract",
      { gas: "200000", feeAmount }  // Use the correct feeAmount array format
    );

    const contractAddress = instantiateReceipt.contractAddress;
    console.log(`Contract instantiated at address: ${contractAddress}`);
  } catch (error) {
    console.error("Error during deployment:", error);
  }
}

deploy().catch(console.error);

```

### Set Up Environment Variables

Create a `.env` file in the `deployer` folder and add your mnemonic and other environment variables:

```
MNEMONIC="your mnemonic here"
```

### Compile and Deploy the Contract

1. Compile the TypeScript file:
    ```bash
    tsc index.ts
    ```

2. Run the deployment script:
    ```bash
    npx ts-node index.ts
    ```

   This will deploy your CosmWasm contract on the DuKong Testnet.

### Verify Contract Deployment

Once the contract is deployed, you will receive confirmation messages with the contract address and other details. Verify the contract deployment on the **MANTRA Chain Explorer**:

[MANTRA Chain Explorer](https://explorer.mantrachain.io)

## Conclusion

Congratulations! Your CosmWasm contract is now deployed on the DuKong Testnet. You can interact with it via the provided contract address on the blockchain.

For further details and the full code repository, visit [Token-Stratosphere GitHub Repository](https://github.com/devtaimoorshah/token-stratosphere.git).

## Contact Us 📧

If you encounter any issues or need any assistance, feel free to reach out to us:

- Email: [info@techsurge.co.uk](mailto:info@techsurge.co.uk)
- Phone: +44 7404 925516
